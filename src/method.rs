use crate::define_constants;
use pyo3::prelude::*;

#[pyclass(eq)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(rquest::Method);

impl Method {
    pub fn into_inner(self) -> rquest::Method {
        self.0
    }
}

#[pymethods]
impl Method {
    fn __str__(&self) -> &str {
        self.0.as_str()
    }

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
