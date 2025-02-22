use crate::define_enum_with_conversion;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

define_enum_with_conversion!(
    const,
    /// A HTTP version.
    Version, rquest::Version, {
    HTTP_09,
    HTTP_10,
    HTTP_11,
    HTTP_2,
    HTTP_3,
});
