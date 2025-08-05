use std::time::SystemTime;

use pyo3::prelude::*;
use wreq::{
    cookie::{self, Expiration},
    header::{self, HeaderMap},
};

define_enum_with_conversion!(
    /// The Cookie SameSite attribute.
    const,
    SameSite,
    wreq::cookie::SameSite,
    (Strict, Strict),
    (Lax, Lax),
    (Empty, None),
);

/// A cookie.
#[pyclass(subclass)]
#[derive(Clone)]
pub struct Cookie(pub cookie_crate::Cookie<'static>);

#[pymethods]
impl Cookie {
    /// Create a new cookie.
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
        let mut cookie = cookie_crate::Cookie::new(name, value);
        if let Some(domain) = domain {
            cookie.set_domain(domain);
        }

        if let Some(path) = path {
            cookie.set_path(path);
        }

        if let Some(max_age) = max_age {
            if let Ok(max_age) = cookie::Duration::try_from(max_age) {
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
    #[inline(always)]
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// The value of the cookie.
    #[getter]
    #[inline(always)]
    pub fn value(&self) -> &str {
        self.0.value()
    }

    /// Returns true if the 'HttpOnly' directive is enabled.
    #[getter]
    #[inline(always)]
    pub fn http_only(&self) -> bool {
        self.0.http_only().unwrap_or(false)
    }

    /// Returns true if the 'Secure' directive is enabled.
    #[getter]
    #[inline(always)]
    pub fn secure(&self) -> bool {
        self.0.secure().unwrap_or(false)
    }

    /// Returns true if  'SameSite' directive is 'Lax'.
    #[getter]
    #[inline(always)]
    pub fn same_site_lax(&self) -> bool {
        self.0.same_site() == Some(cookie_crate::SameSite::Lax)
    }

    /// Returns true if  'SameSite' directive is 'Strict'.
    #[getter]
    #[inline(always)]
    pub fn same_site_strict(&self) -> bool {
        self.0.same_site() == Some(cookie_crate::SameSite::Strict)
    }

    /// Returns the path directive of the cookie, if set.
    #[getter]
    #[inline(always)]
    pub fn path(&self) -> Option<&str> {
        self.0.path()
    }

    /// Returns the domain directive of the cookie, if set.
    #[getter]
    #[inline(always)]
    pub fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    /// Get the Max-Age information.
    #[getter]
    #[inline(always)]
    pub fn max_age(&self) -> Option<std::time::Duration> {
        self.0.max_age().and_then(|d| d.try_into().ok())
    }

    /// The cookie expiration time.
    #[getter]
    #[inline(always)]
    pub fn expires(&self) -> Option<SystemTime> {
        match self.0.expires() {
            Some(Expiration::DateTime(offset)) => Some(SystemTime::from(offset)),
            None | Some(Expiration::Session) => None,
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

impl Cookie {
    pub fn parse(headers: &HeaderMap) -> Vec<Cookie> {
        headers
            .get_all(header::SET_COOKIE)
            .iter()
            .map(|h| {
                std::str::from_utf8(h.as_bytes())
                    .map_err(cookie_crate::ParseError::from)
                    .and_then(cookie_crate::Cookie::parse)
            })
            .flat_map(Result::ok)
            .map(cookie_crate::Cookie::into_owned)
            .map(Cookie)
            .collect()
    }
}
