use crate::error::wrap_invali_header_value_error;
use bytes::Bytes;
use pyo3::FromPyObject;
use pyo3::pybacked::PyBackedStr;
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::{PyStubType, TypeInfo};
use rquest::header::{self, HeaderMap, HeaderValue};

/// Parse a cookie header from a Python list.
pub struct CookiesFromPyDict(pub Vec<HeaderValue>);

/// Parse a cookie header from a Python dictionary.
pub struct CookieFromPyDict(pub HeaderValue);

/// Convert a header value into a Python dictionary.
pub struct CookieIntoPyDict(pub Option<HeaderValue>);

/// Convert a headers header map into a Python dictionary.
pub struct CookieMapIntoPyDict<'a>(pub &'a HeaderMap);

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

impl<'py> IntoPyObject<'py> for CookieMapIntoPyDict<'py> {
    type Target = PyDict;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0
            .get_all(header::SET_COOKIE)
            .iter()
            .map(|value| {
                py.allow_threads(|| {
                    std::str::from_utf8(value.as_bytes())
                        .map_err(cookie::ParseError::from)
                        .and_then(cookie::Cookie::parse)
                })
            })
            .filter_map(Result::ok)
            .try_fold(PyDict::new(py), |dict, cookie| {
                dict.set_item(cookie.name(), cookie.value()).map(|_| dict)
            })
    }
}

impl<'py> IntoPyObject<'py> for CookieIntoPyDict {
    type Target = PyDict;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0
            .iter()
            .map(|value| {
                py.allow_threads(|| {
                    std::str::from_utf8(value.as_bytes())
                        .map_err(cookie::ParseError::from)
                        .and_then(cookie::Cookie::parse)
                })
            })
            .filter_map(Result::ok)
            .try_fold(PyDict::new(py), |dict, cookie| {
                dict.set_item(cookie.name(), cookie.value()).map(|_| dict)
            })
    }
}

impl FromPyObject<'_> for CookiesFromPyDict {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;
        dict.iter()
            .try_fold(Vec::with_capacity(dict.len()), |mut cookies, (k, v)| {
                let key = k.extract::<PyBackedStr>()?;
                let value = v.extract::<PyBackedStr>()?;
                let kv = format!("{}={}", key.as_ref() as &str, value.as_ref() as &str);
                HeaderValue::from_maybe_shared(Bytes::from(kv))
                    .map_err(wrap_invali_header_value_error)
                    .map(|value| {
                        cookies.push(value);
                        cookies
                    })
            })
            .map(Self)
    }
}

impl PyStubType for CookiesFromPyDict {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.List[str]", "typing".into())
    }
}
