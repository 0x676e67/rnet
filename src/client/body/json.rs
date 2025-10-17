use indexmap::IndexMap;
use pyo3::{FromPyObject, prelude::*, pybacked::PyBackedStr};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a JSON value for HTTP requests.
/// Supports objects, arrays, numbers, strings, booleans, and null.
#[derive(FromPyObject, IntoPyObject, Serialize, Deserialize)]
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

#[derive(IntoPyObject, PartialEq, Eq, Hash)]
pub enum String {
    PyString(PyBackedStr),
    String(std::string::String),
}

impl FromPyObject<'_> for String {
    #[inline]
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(Self::PyString)
    }
}

impl Serialize for String {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            String::PyString(pb) => serializer.serialize_str(pb.as_ref()),
            String::String(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for String {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        std::string::String::deserialize(deserializer).map(String::String)
    }
}
