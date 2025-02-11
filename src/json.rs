use indexmap::IndexMap;
use pyo3::{prelude::*, types::PyBool};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromPyObject, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PyJson {
    Object(IndexMap<String, PyJson>),
    Boolean(bool),
    Number(isize),
    Float(f64),
    String(String),
    Null(Option<isize>),
    Array(Vec<PyJson>),
}

impl<'py> IntoPyObject<'py> for PyJson {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = anyhow::Error;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Null(_) => Option::<isize>::None.into_pyobject(py).map_err(Into::into),
            Self::Boolean(inner) => inner
                .into_pyobject(py)
                .map(|x| <pyo3::Bound<'_, PyBool> as Clone>::clone(&x).into_any())
                .map_err(Into::into),

            Self::String(inner) => inner
                .into_pyobject(py)
                .map(|x| x.into_any())
                .map_err(Into::into),

            Self::Float(inner) => inner
                .into_pyobject(py)
                .map(|x| x.into_any())
                .map_err(Into::into),

            Self::Number(inner) => inner
                .into_pyobject(py)
                .map(|x| x.into_any())
                .map_err(Into::into),

            Self::Array(inner) => {
                let mut new_holder = Vec::with_capacity(inner.len());
                for item in inner {
                    new_holder.push(item.into_pyobject(py)?);
                }
                new_holder.into_pyobject(py).map_err(Into::into)
            }

            Self::Object(inner) => {
                let mut new_holder = IndexMap::with_capacity(inner.len());
                for (key, elem) in inner {
                    new_holder.insert(key, elem.into_pyobject(py).map(|x| x.into_any())?);
                }
                new_holder
                    .into_pyobject(py)
                    .map(|x| x.into_any())
                    .map_err(Into::into)
            }
        }
    }
}
