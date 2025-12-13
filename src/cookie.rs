use std::{fmt, sync::Arc, time::SystemTime};

use cookie::{Cookie as RawCookie, Expiration, ParseError, time::Duration};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use wreq::header::{self, HeaderMap, HeaderValue};

define_enum!(
    /// The Cookie SameSite attribute.
    const,
    SameSite,
    cookie::SameSite,
    (Strict, Strict),
    (Lax, Lax),
    (Empty, None),
);

/// A single HTTP cookie.

#[derive(Clone)]
#[pyclass(subclass, str, frozen)]
pub struct Cookie(RawCookie<'static>);

/// A good default `CookieStore` implementation.
///
/// This is the implementation used when simply calling `cookie_store(true)`.
/// This type is exposed to allow creating one and filling it with some
/// existing cookies more easily, before creating a `Client`.
#[derive(Clone, Default)]
#[pyclass(subclass, frozen)]
pub struct Jar(pub Arc<wreq::cookie::Jar>);

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
        http_only = None,
        secure = None,
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
        http_only: Option<bool>,
        secure: Option<bool>,
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

        cookie.set_http_only(http_only);
        cookie.set_secure(secure);
        cookie.set_same_site(same_site.map(|s| s.into_ffi()));

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
        self.0.same_site() == Some(cookie::SameSite::Lax)
    }

    /// Returns true if  'SameSite' directive is 'Strict'.
    #[getter]
    #[inline]
    pub fn same_site_strict(&self) -> bool {
        self.0.same_site() == Some(cookie::SameSite::Strict)
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
            .map_err(cookie::ParseError::from)
            .and_then(RawCookie::parse)
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// ===== impl Jar =====

#[pymethods]
impl Jar {
    /// Create a new [`Jar`] with an empty cookie store.
    #[new]
    #[pyo3(signature = (compression = None))]
    pub fn new(compression: Option<bool>) -> Self {
        Self(Arc::new(compression.map_or_else(
            wreq::cookie::Jar::default,
            wreq::cookie::Jar::new,
        )))
    }

    /// Clone this [`Jar`], sharing storage but enabling compression.
    pub fn compreessed(&self) -> Self {
        Self(self.0.compressed())
    }

    /// Clone this [`Jar`], sharing storage but disabling compression.
    pub fn uncompressed(&self) -> Self {
        Self(self.0.uncompressed())
    }

    /// Get a cookie by name and URL.
    #[pyo3(signature = (name, url))]
    pub fn get(&self, py: Python, name: PyBackedStr, url: PyBackedStr) -> Option<Cookie> {
        py.detach(|| {
            self.0
                .get(&name, AsRef::<str>::as_ref(&url))
                .map(RawCookie::from)
                .map(Cookie)
        })
    }

    /// Get all cookies.
    pub fn get_all(&self, py: Python) -> Vec<Cookie> {
        py.detach(|| self.0.get_all().map(RawCookie::from).map(Cookie).collect())
    }

    /// Add a cookie to this jar.
    #[pyo3(signature = (cookie, url))]
    pub fn add_cookie(&self, py: Python, cookie: Cookie, url: PyBackedStr) {
        py.detach(|| self.0.add_cookie(cookie.0, AsRef::<str>::as_ref(&url)))
    }

    /// Add a cookie str to this jar.
    #[pyo3(signature = (cookie, url))]
    pub fn add_cookie_str(&self, py: Python, cookie: PyBackedStr, url: PyBackedStr) {
        py.detach(|| self.0.add_cookie_str(&cookie, AsRef::<str>::as_ref(&url)))
    }

    /// Remove a cookie from this jar by name and URL.
    #[pyo3(signature = (name, url))]
    pub fn remove(&self, py: Python, name: PyBackedStr, url: PyBackedStr) {
        py.detach(|| {
            self.0.remove(
                AsRef::<str>::as_ref(&name).to_owned(),
                AsRef::<str>::as_ref(&url),
            )
        })
    }

    /// Clear all cookies in this jar.
    pub fn clear(&self, py: Python) {
        py.detach(|| {
            self.0.clear();
        });
    }
}
