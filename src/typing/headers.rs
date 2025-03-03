use crate::error::{wrap_invali_header_name_error, wrap_invali_header_value_error};
use pyo3::{
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyBytes, PyDict, PyList},
};
use rquest::header::{self, HeaderName, HeaderValue};
use std::str::FromStr;

/// A HTTP header map.
pub struct FromPyHeaderMap(pub header::HeaderMap);

/// A HTTP reference to a header map.
pub struct IntoPyHeaderMapRef<'a>(pub &'a header::HeaderMap);

/// A list of header names in order.
pub struct FromPyHeaderOrderList(pub Vec<HeaderName>);

impl FromPyObject<'_> for FromPyHeaderMap {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;

        dict.iter()
            .try_fold(
                header::HeaderMap::with_capacity(dict.len()),
                |mut headers, (key, value)| {
                    let key = key.extract::<PyBackedStr>()?;
                    let name = HeaderName::from_bytes(key.as_bytes())
                        .map_err(wrap_invali_header_name_error)?;
                    let value = value.extract::<PyBackedStr>()?;
                    let value = HeaderValue::from_bytes(value.as_bytes())
                        .map_err(wrap_invali_header_value_error)?;
                    headers.insert(name, value);
                    Ok(headers)
                },
            )
            .map(Self)
    }
}

impl<'py> FromPyObject<'py> for FromPyHeaderOrderList {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let list = ob.downcast::<PyList>()?;
        list.iter()
            .try_fold(Vec::with_capacity(list.len()), |mut order, item| {
                let name = HeaderName::from_str(item.extract::<&str>()?)
                    .map_err(wrap_invali_header_name_error)?;
                order.push(name);
                Ok(order)
            })
            .map(Self)
    }
}

impl<'py> IntoPyObject<'py> for IntoPyHeaderMapRef<'_> {
    type Target = PyDict;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0
            .iter()
            .try_fold(PyDict::new(py), |dict, (name, value)| {
                dict.set_item(name.as_str(), PyBytes::new(py, value.as_bytes()))?;
                Ok(dict)
            })
    }
}
