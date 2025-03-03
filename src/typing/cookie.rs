use crate::error::wrap_invali_header_value_error;
use pyo3::FromPyObject;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyList;
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::{PyStubType, TypeInfo};
use rquest::header::{self, HeaderMap, HeaderValue};

pub struct FromPyCookieMap(pub HeaderValue);

pub struct IntoPyCookieMapRef<'a>(pub &'a HeaderMap);

pub struct IntoPyCookieList(pub Option<HeaderValue>);

pub struct FromPyCookieList(pub Vec<HeaderValue>);

impl FromPyObject<'_> for FromPyCookieMap {
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
                HeaderValue::from_maybe_shared(cookies)
                    .map(Self)
                    .map_err(wrap_invali_header_value_error)
            })
    }
}

impl<'py> IntoPyObject<'py> for IntoPyCookieMapRef<'py> {
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

impl<'py> IntoPyObject<'py> for IntoPyCookieList {
    type Target = PyList;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0
            .iter()
            .filter_map(|hv| hv.to_str().ok())
            .try_fold(PyList::empty(py), |list, item| {
                list.append(item).map(|_| list)
            })
    }
}

impl FromPyObject<'_> for FromPyCookieList {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let list = ob.downcast::<PyList>()?;
        list.iter()
            .try_fold(Vec::with_capacity(list.len()), |mut vec, item| {
                let str = item.extract::<PyBackedStr>()?;
                let header = HeaderValue::from_bytes(str.as_bytes())
                    .map_err(wrap_invali_header_value_error)?;
                vec.push(header);
                Ok(vec)
            })
            .map(Self)
    }
}

impl PyStubType for FromPyCookieList {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.List[str]", "typing".into())
    }
}
