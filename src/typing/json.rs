use indexmap::IndexMap;
use pyo3::prelude::*;
#[cfg(feature = "docs")]
use pyo3_stub_gen::{PyStubType, TypeInfo};
use serde::{Deserialize, Serialize};

#[derive(Clone, FromPyObject, IntoPyObject, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Json {
    Object(IndexMap<String, Json>),
    Boolean(bool),
    Number(isize),
    Float(f64),
    String(String),
    Null(Option<isize>),
    Array(Vec<Json>),
}

#[cfg(feature = "docs")]
impl PyStubType for Json {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.Dict[str, typing.Any]", "typing".into())
    }
}
