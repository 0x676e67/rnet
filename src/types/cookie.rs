use crate::error::wrap_invali_header_value_error;

use super::IndexMap;
use pyo3::FromPyObject;
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::{PyStubType, TypeInfo};
use rquest::header::HeaderValue;

pub type CookieMap = super::IndexMap<String, String>;

impl TryFrom<CookieMap> for HeaderValue {
    type Error = PyErr;

    fn try_from(cookies: CookieMap) -> Result<Self, Self::Error> {
        let mut kv = String::with_capacity(cookies.len() * 8);
        for (k, v) in cookies.iter() {
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

impl PyStubType for CookieMap {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.Dict[str, str]", "typing".into())
    }
}

impl FromPyObject<'_> for CookieMap {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(IndexMap)
    }
}

impl<'py> IntoPyObject<'py> for CookieMap {
    type Target = PyDict;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0
            .iter()
            .try_fold(PyDict::new(py), |dict, (header, value)| {
                dict.set_item(header, value)?;
                Ok(dict)
            })
    }
}
