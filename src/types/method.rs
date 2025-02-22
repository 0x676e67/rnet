use crate::define_enum_with_conversion;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

define_enum_with_conversion!(
    /// A HTTP method.
    Method,
    rquest::Method,
    {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
});
