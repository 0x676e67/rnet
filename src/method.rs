use crate::define_constants;
use pyo3::prelude::*;

#[pyclass(eq)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(rquest::Method);

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
