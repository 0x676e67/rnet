use crate::error::wrap_invali_header_value_error;
use bytes::Bytes;
use pyo3::FromPyObject;
use pyo3::pybacked::PyBackedStr;
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use pyo3_stub_gen::{PyStubType, TypeInfo};
use rquest::cookie::{self, Expiration};
use rquest::header::{self, HeaderMap, HeaderValue};
use std::time::SystemTime;

/// A cookie.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct Cookie(pub rquest::cookie::Cookie<'static>);

impl Cookie {
    pub(crate) fn extract_cookies(headers: &HeaderMap) -> Vec<Self> {
        headers
            .get_all(header::SET_COOKIE)
            .iter()
            .map(cookie::Cookie::parse)
            .flat_map(Result::ok)
            .map(cookie::Cookie::into_owned)
            .map(Cookie)
            .collect()
    }
}

#[gen_stub_pymethods]
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
    pub fn new(
        name: String,
        value: String,
        domain: Option<String>,
        path: Option<String>,
        max_age: Option<std::time::Duration>,
        expires: Option<SystemTime>,
        http_only: bool,
        secure: bool,
        same_site: Option<crate::typing::SameSite>,
    ) -> Cookie {
        let mut builder = rquest::cookie::Cookie::builder(name, value);
        if let Some(domain) = domain {
            builder = builder.domain(domain);
        }

        if let Some(path) = path {
            builder = builder.path(path);
        }

        if let Some(max_age) = max_age {
            if let Ok(max_age) = rquest::cookie::Duration::try_from(max_age) {
                builder = builder.max_age(max_age);
            }
        }

        if let Some(expires) = expires {
            builder = builder.expires(Expiration::DateTime(expires.into()));
        }

        if http_only {
            builder = builder.http_only(true);
        }

        if secure {
            builder = builder.secure(true);
        }

        if let Some(same_site) = same_site {
            builder = builder.same_site(same_site.into_ffi());
        }

        Self(builder.build())
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
        self.0.http_only()
    }

    /// Returns true if the 'Secure' directive is enabled.
    #[getter]
    #[inline(always)]
    pub fn secure(&self) -> bool {
        self.0.secure()
    }

    /// Returns true if  'SameSite' directive is 'Lax'.
    #[getter]
    #[inline(always)]
    pub fn same_site_lax(&self) -> bool {
        self.0.same_site_lax()
    }

    /// Returns true if  'SameSite' directive is 'Strict'.
    #[getter]
    #[inline(always)]
    pub fn same_site_strict(&self) -> bool {
        self.0.same_site_strict()
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
        self.0.expires()
    }

    #[inline]
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    #[inline]
    fn __repr__(&self) -> String {
        self.__str__()
    }
}

/// Parse a cookie header from a Python dictionary.
pub struct CookieFromPyDict(pub HeaderValue);

impl FromPyObject<'_> for CookieFromPyDict {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;
        dict.iter()
            .try_fold(
                String::with_capacity(dict.len() * 8),
                |mut cookies, (k, v)| {
                    if !cookies.is_empty() {
                        cookies.push_str("; ");
                    }
                    cookies.push_str(k.extract::<PyBackedStr>()?.as_ref());
                    cookies.push('=');
                    cookies.push_str(v.extract::<PyBackedStr>()?.as_ref());
                    Ok(cookies)
                },
            )
            .and_then(|cookies| {
                HeaderValue::from_maybe_shared(Bytes::from(cookies))
                    .map(Self)
                    .map_err(wrap_invali_header_value_error)
            })
    }
}

impl PyStubType for CookieFromPyDict {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.Dict[str, str]", "typing".into())
    }
}
