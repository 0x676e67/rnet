use crate::define_constants;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A HTTP method.
#[gen_stub_pyclass]
#[pyclass(eq)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(rquest::Method);

impl From<Method> for rquest::Method {
    fn from(method: Method) -> rquest::Method {
        method.0
    }
}

#[pymethods]
impl Method {
    /// Returns a string representation of the method.
    fn __str__(&self) -> &str {
        self.0.as_str()
    }

    /// Returns a string representation of the method.
    fn __repr__(&self) -> &str {
        self.__str__()
    }
}

define_constants!(
    Method,
    rquest::Method,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
);
