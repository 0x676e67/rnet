use indexmap::IndexMap;
use pyo3::{FromPyObject, pybacked::PyBackedStr};
use serde::{
    Serialize, Serializer,
    ser::{SerializeMap, SerializeSeq},
};

/// Represents a form submission in Python, either as a mapping or a sequence of key-value pairs.
///
/// This enum supports extracting form data from Python objects such as dictionaries (`dict`) or
/// sequences of tuples (e.g., `list` of `(key, value)` pairs). It is used for handling HTTP form
/// submissions where keys and values are strings.
#[derive(FromPyObject)]
pub enum Form {
    Map(IndexMap<PyBackedStr, PyBackedStr>),
    Tuple(Vec<(PyBackedStr, PyBackedStr)>),
}

impl Serialize for Form {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Form::Map(map) => {
                let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
                for (key, value) in map {
                    map_serializer.serialize_entry(
                        <PyBackedStr as AsRef<str>>::as_ref(key),
                        <PyBackedStr as AsRef<str>>::as_ref(value),
                    )?;
                }
                map_serializer.end()
            }
            Form::Tuple(vec) => {
                let mut seq_serializer = serializer.serialize_seq(Some(vec.len()))?;
                for (key, value) in vec {
                    seq_serializer
                        .serialize_element::<(&str, &str)>(&(key.as_ref(), value.as_ref()))?;
                }
                seq_serializer.end()
            }
        }
    }
}
