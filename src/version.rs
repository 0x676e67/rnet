use crate::define_constants;
use pyo3::prelude::*;

#[pyclass(eq)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(rquest::Version);

#[pymethods]
impl Version {
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

define_constants!(
    Version,
    rquest::Version,
    HTTP_09,
    HTTP_10,
    HTTP_11,
    HTTP_2,
    HTTP_3
);

impl From<rquest::Version> for Version {
    fn from(version: rquest::Version) -> Self {
        Version(version)
    }
}
