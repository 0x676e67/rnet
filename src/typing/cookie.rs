use crate::error::wrap_invali_header_value_error;
use indexmap::IndexMap;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyList;
use pyo3::FromPyObject;
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::{PyStubType, TypeInfo};
use rquest::header::{self, HeaderMap, HeaderValue};

pub struct FromPyCookieMap(pub IndexMap<String, String>);

pub struct IntoPyCookieMapRef<'a>(pub &'a HeaderMap);

pub struct IntoPyCookieHeader(pub Option<HeaderValue>);

pub struct FromPyCookieList(pub Vec<HeaderValue>);

impl TryFrom<FromPyCookieMap> for HeaderValue {
    type Error = PyErr;

    fn try_from(cookies: FromPyCookieMap) -> Result<Self, Self::Error> {
        let mut kv = String::with_capacity(cookies.0.len() * 8);
        for (k, v) in cookies.0.iter() {
            if !kv.is_empty() {
                kv.push_str("; ");
            }
            kv.push_str(k);
            kv.push('=');
            kv.push_str(v);
        }
        HeaderValue::from_str(&kv).map_err(wrap_invali_header_value_error)
    }
}

impl FromPyObject<'_> for FromPyCookieMap {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(FromPyCookieMap)
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
                dict.set_item(cookie.name(), cookie.value())?;
                Ok(dict)
            })
    }
}

impl<'py> IntoPyObject<'py> for IntoPyCookieHeader {
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
        let mut vec = Vec::with_capacity(list.len());
        for item in list.iter() {
            let str = item.extract::<PyBackedStr>()?;
            let header =
                HeaderValue::from_bytes(str.as_bytes()).map_err(wrap_invali_header_value_error)?;
            vec.push(header);
        }
        Ok(Self(vec))
    }
}

impl PyStubType for FromPyCookieList {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.List[str]", "typing".into())
    }
}
