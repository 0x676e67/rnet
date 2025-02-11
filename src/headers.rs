use std::ops::Deref;

use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict},
};
use rquest::header;

#[derive(Clone)]
#[pyclass]
pub struct HeaderMap(header::HeaderMap);

#[pymethods]
impl HeaderMap {
    pub fn to_dict<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyDict>> {
        let new_dict = PyDict::new(py);
        for (header, value) in &self.0 {
            new_dict.set_item(header.as_str(), PyBytes::new(py, value.as_ref()))?;
        }
        Ok(new_dict)
    }

    fn __getitem__<'rt>(&'rt self, key: &str) -> PyResult<Option<&'rt [u8]>> {
        Ok(self.0.get(key).and_then(|v| Some(v.as_ref())))
    }
}

impl From<header::HeaderMap> for HeaderMap {
    fn from(map: header::HeaderMap) -> Self {
        HeaderMap(map)
    }
}

impl Deref for HeaderMap {
    type Target = header::HeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
