use crate::{
    buffer::{Buffer, PyBufferProtocol},
    error::{wrap_invali_header_name_error, wrap_invali_header_value_error},
};
use pyo3::{
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyBytes, PyDict, PyList},
};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::header::{self, HeaderName, HeaderValue};
use std::str::FromStr;

/// A HTTP header map.
#[gen_stub_pyclass]
#[pyclass]
pub struct HeaderMap(pub header::HeaderMap);

#[gen_stub_pymethods]
#[pymethods]
impl HeaderMap {
    #[inline]
    fn __getitem__<'py>(&self, py: Python<'py>, key: PyBackedStr) -> Option<Bound<'py, PyAny>> {
        let value = self.0.get(key.as_ref() as &str)?;
        let buffer = Buffer::new(value.as_bytes().to_vec());
        buffer.into_bytes_ref(py).ok()
    }

    #[inline]
    fn __setitem__(&mut self, key: PyBackedStr, value: PyBackedStr) {
        if let (Ok(name), Ok(value)) = (
            HeaderName::from_bytes(key.as_bytes()),
            HeaderValue::from_bytes(value.as_bytes()),
        ) {
            self.0.insert(name, value);
        }
    }

    #[inline]
    fn __delitem__(&mut self, key: PyBackedStr) {
        self.0.remove(key.as_ref() as &str);
    }

    #[inline]
    fn __contains__(&self, key: PyBackedStr) -> bool {
        self.0.contains_key(key.as_ref() as &str)
    }

    #[inline]
    fn __len__(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn __iter__(&self) -> HeaderMapKeysIter {
        HeaderMapKeysIter {
            inner: self.0.keys().cloned().collect(),
        }
    }

    #[inline]
    fn items(&self) -> HeaderMapItemsIter {
        HeaderMapItemsIter {
            inner: self.0.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[inline]
    fn __repr__(&self) -> String {
        self.__str__()
    }
}

/// An iterator over the keys in a HeaderMap.
#[gen_stub_pyclass]
#[pyclass]
pub struct HeaderMapKeysIter {
    inner: Vec<HeaderName>,
}

#[gen_stub_pymethods]
#[pymethods]
impl HeaderMapKeysIter {
    #[inline]
    fn __iter__(slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf
    }

    #[inline]
    fn __next__(mut slf: PyRefMut<Self>) -> Option<String> {
        slf.inner.pop().map(|k| k.to_string())
    }
}

/// An iterator over the items in a HeaderMap.
#[gen_stub_pyclass]
#[pyclass]
pub struct HeaderMapItemsIter {
    inner: Vec<(HeaderName, HeaderValue)>,
}

#[gen_stub_pymethods]
#[pymethods]
impl HeaderMapItemsIter {
    #[inline]
    fn __iter__(slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf
    }

    #[inline]
    fn __next__(mut slf: PyRefMut<Self>) -> Option<(String, Option<String>)> {
        slf.inner
            .pop()
            .map(|(k, v)| (k.to_string(), v.to_str().ok().map(String::from)))
    }
}

/// A HTTP header map.
pub struct HeaderMapFromPyDict(pub header::HeaderMap);

/// A HTTP reference to a header map.
pub struct HeaderMapIntoPyDict<'a>(pub &'a header::HeaderMap);

/// A list of header names in order.
pub struct HeadersOrderFromPyList(pub Vec<HeaderName>);

impl FromPyObject<'_> for HeaderMapFromPyDict {
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

impl<'py> FromPyObject<'py> for HeadersOrderFromPyList {
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

impl<'py> IntoPyObject<'py> for HeaderMapIntoPyDict<'_> {
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
