use crate::define_constants;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A HTTP version.
#[gen_stub_pyclass]
#[pyclass(eq)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Version(rquest::Version);

impl From<Version> for rquest::Version {
    fn from(version: Version) -> rquest::Version {
        version.0
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Version {
    /// Returns a string representation of the version.
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    /// Returns a string representation of the version.
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
