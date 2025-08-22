use std::{
    fmt,
    sync::{Arc, RwLock},
    time::SystemTime,
};

use bytes::{BufMut, Bytes};
use cookie_crate::{Expiration, ParseError, time::Duration};
use cookie_store::RawCookie;
use pyo3::{prelude::*, pybacked::PyBackedStr};
use url::Url;
use wreq::{
    cookie::CookieStore,
    header::{self, HeaderMap, HeaderValue},
};

use crate::error::Error;

const EMPTY_DOMAIN: &str = "";
const COOKIE_SEPARATOR: &[u8] = b"=";

define_enum!(
    /// The Cookie SameSite attribute.
    const,
    SameSite,
    cookie_crate::SameSite,
    (Strict, Strict),
    (Lax, Lax),
    (Empty, None),
);

/// A single HTTP cookie.

#[derive(Clone)]
#[pyclass(subclass, str)]
pub struct Cookie(pub RawCookie<'static>);

/// A good default `CookieStore` implementation.
///
/// This is the implementation used when simply calling `cookie_store(true)`.
/// This type is exposed to allow creating one and filling it with some
/// existing cookies more easily, before creating a `Client`.
#[derive(Clone, Default)]
#[pyclass(subclass)]
pub struct Jar(Arc<RwLock<cookie_store::CookieStore>>);

// ===== impl Cookie =====

#[pymethods]
impl Cookie {
    /// Create a new [`Cookie`].
    #[new]
    #[pyo3(signature = (
        name,
        value,
        domain = None,
        path = None,
        max_age = None,
        expires = None,
        http_only = false,
        secure = false,
        same_site = None
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        value: String,
        domain: Option<String>,
        path: Option<String>,
        max_age: Option<std::time::Duration>,
        expires: Option<SystemTime>,
        http_only: bool,
        secure: bool,
        same_site: Option<SameSite>,
    ) -> Cookie {
        let mut cookie = RawCookie::new(name, value);
        if let Some(domain) = domain {
            cookie.set_domain(domain);
        }

        if let Some(path) = path {
            cookie.set_path(path);
        }

        if let Some(max_age) = max_age {
            if let Ok(max_age) = Duration::try_from(max_age) {
                cookie.set_max_age(max_age);
            }
        }

        if let Some(expires) = expires {
            cookie.set_expires(Expiration::DateTime(expires.into()));
        }

        if http_only {
            cookie.set_http_only(true);
        }

        if secure {
            cookie.set_secure(true);
        }

        if let Some(same_site) = same_site {
            cookie.set_same_site(same_site.into_ffi());
        }

        Self(cookie)
    }

    /// The name of the cookie.
    #[getter]
    #[inline]
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// The value of the cookie.
    #[getter]
    #[inline]
    pub fn value(&self) -> &str {
        self.0.value()
    }

    /// Returns true if the 'HttpOnly' directive is enabled.
    #[getter]
    #[inline]
    pub fn http_only(&self) -> bool {
        self.0.http_only().unwrap_or(false)
    }

    /// Returns true if the 'Secure' directive is enabled.
    #[getter]
    #[inline]
    pub fn secure(&self) -> bool {
        self.0.secure().unwrap_or(false)
    }

    /// Returns true if  'SameSite' directive is 'Lax'.
    #[getter]
    #[inline]
    pub fn same_site_lax(&self) -> bool {
        self.0.same_site() == Some(cookie_crate::SameSite::Lax)
    }

    /// Returns true if  'SameSite' directive is 'Strict'.
    #[getter]
    #[inline]
    pub fn same_site_strict(&self) -> bool {
        self.0.same_site() == Some(cookie_crate::SameSite::Strict)
    }

    /// Returns the path directive of the cookie, if set.
    #[getter]
    #[inline]
    pub fn path(&self) -> Option<&str> {
        self.0.path()
    }

    /// Returns the domain directive of the cookie, if set.
    #[getter]
    #[inline]
    pub fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    /// Get the Max-Age information.
    #[getter]
    #[inline]
    pub fn max_age(&self) -> Option<std::time::Duration> {
        self.0.max_age().and_then(|d| d.try_into().ok())
    }

    /// The cookie expiration time.
    #[getter]
    #[inline]
    pub fn expires(&self) -> Option<SystemTime> {
        match self.0.expires() {
            Some(Expiration::DateTime(offset)) => Some(SystemTime::from(offset)),
            None | Some(Expiration::Session) => None,
        }
    }
}

impl Cookie {
    /// Parse cookies from a `HeaderMap`.
    pub fn extract_headers_cookies(headers: &HeaderMap) -> Vec<Cookie> {
        headers
            .get_all(header::SET_COOKIE)
            .iter()
            .map(Cookie::parse)
            .flat_map(Result::ok)
            .map(RawCookie::into_owned)
            .map(Cookie)
            .collect()
    }

    fn parse<'a>(value: &'a HeaderValue) -> Result<RawCookie<'a>, ParseError> {
        std::str::from_utf8(value.as_bytes())
            .map_err(cookie_crate::ParseError::from)
            .and_then(RawCookie::parse)
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ===== impl Jar =====

impl CookieStore for Jar {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url) {
        let iter = cookie_headers.filter_map(|val| Cookie::parse(val).map(|c| c.into_owned()).ok());

        self.0.write().unwrap().store_response_cookies(iter, url);
    }

    fn cookies(&self, url: &url::Url) -> Vec<HeaderValue> {
        self.0
            .read()
            .unwrap()
            .get_request_values(url)
            .filter_map(|(name, value)| {
                let name = name.as_bytes();
                let value = value.as_bytes();
                let mut cookie = bytes::BytesMut::with_capacity(name.len() + 1 + value.len());

                cookie.put(name);
                cookie.put(COOKIE_SEPARATOR);
                cookie.put(value);

                HeaderValue::from_maybe_shared(Bytes::from(cookie)).ok()
            })
            .collect()
    }
}

#[pymethods]
impl Jar {
    /// Create a new [`Jar`] with an empty cookie store.
    #[new]
    pub fn new() -> Self {
        Jar(Arc::new(RwLock::new(cookie_store::CookieStore::default())))
    }

    /// Get a cookie by name and URL.
    #[pyo3(signature = (name, url))]
    pub fn get(&self, py: Python, name: PyBackedStr, url: PyBackedStr) -> PyResult<Option<Cookie>> {
        py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(Error::UrlParse)?;
            let store = self.0.read().unwrap();
            let cookie = store.get(
                url.host_str().unwrap_or(EMPTY_DOMAIN),
                url.path(),
                name.as_ref(),
            );

            cookie
                .map(|cookie| {
                    // Convert the cookie to a static lifetime to match the Cookie type.
                    // This is safe because we are only returning a reference to the cookie,
                    // not the underlying data.
                    let cookie = cookie.clone().into_owned();
                    Cookie(RawCookie::from(cookie))
                })
                .map(PyResult::Ok)
                .transpose()
        })
    }

    /// Get all cookies.
    pub fn get_all(&self, py: Python) -> Vec<Cookie> {
        py.allow_threads(|| {
            let store = self.0.read().unwrap();
            store
                .iter_any()
                .map(Clone::clone)
                .map(RawCookie::from)
                .map(Cookie)
                .collect()
        })
    }

    /// Add a cookie to this jar.
    #[pyo3(signature = (cookie, url))]
    pub fn add(&self, py: Python, cookie: Cookie, url: PyBackedStr) -> PyResult<()> {
        py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(Error::UrlParse)?;
            self.0
                .write()
                .unwrap()
                .store_response_cookies(std::iter::once(cookie.0), &url);
            Ok(())
        })
    }

    /// Add a cookie str to this jar.
    #[pyo3(signature = (cookie, url))]
    pub fn add_cookie_str(
        &self,
        py: Python,
        cookie: PyBackedStr,
        url: PyBackedStr,
    ) -> PyResult<()> {
        py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(Error::UrlParse)?;
            let cookies = RawCookie::parse::<&str>(cookie.as_ref())
                .map(RawCookie::into_owned)
                .into_iter();
            self.0
                .write()
                .unwrap()
                .store_response_cookies(cookies, &url);
            Ok(())
        })
    }

    /// Remove a cookie from this jar by name and URL.
    #[pyo3(signature = (name, url))]
    pub fn remove(&self, py: Python, name: PyBackedStr, url: PyBackedStr) -> PyResult<()> {
        py.allow_threads(|| {
            let url = Url::parse(url.as_ref()).map_err(Error::UrlParse)?;
            self.0.write().unwrap().remove(
                url.host_str().unwrap_or(EMPTY_DOMAIN),
                url.path(),
                name.as_ref(),
            );
            Ok(())
        })
    }

    /// Clear all cookies in this jar.
    pub fn clear(&self, py: Python) {
        py.allow_threads(|| {
            self.0.write().unwrap().clear();
        });
    }
}
