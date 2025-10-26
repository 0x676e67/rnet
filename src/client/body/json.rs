use indexmap::IndexMap;
use pyo3::{FromPyObject, prelude::*, pybacked::PyBackedStr};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a JSON value for HTTP requests.
/// Supports objects, arrays, numbers, strings, booleans, and null.
#[derive(FromPyObject, IntoPyObject, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Json {
    Object(IndexMap<JsonString, Json>),
    Boolean(bool),
    Number(isize),
    Float(f64),
    String(JsonString),
    Null(Option<isize>),
    Array(Vec<Json>),
}

/// A string type that can represent either a Python-backed string
/// or a standard Rust `String`. This allows for zero-copy deserialization
/// of strings originating from Python, improving performance when handling
/// JSON data that includes string values.
#[derive(IntoPyObject, PartialEq, Eq, Hash)]
pub enum JsonString {
    PyString(PyBackedStr),
    RustString(String),
}

impl FromPyObject<'_, '_> for JsonString {
    type Error = PyErr;

    #[inline]
    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        ob.extract().map(Self::PyString)
    }
}

impl Serialize for JsonString {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            JsonString::PyString(pb) => serializer.serialize_str(pb.as_ref()),
            JsonString::RustString(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for JsonString {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(JsonString::RustString)
    }
}
