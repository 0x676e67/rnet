use crate::{
    buffer::{HeaderNameBuffer, HeaderValueBuffer, PyBufferProtocol},
    error::Error,
};
use pyo3::{
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyDict, PyList},
};
#[cfg(feature = "docs")]
use pyo3_stub_gen::{
    PyStubType, TypeInfo,
    derive::{gen_stub_pyclass, gen_stub_pymethods},
};
use rquest::header::{self, HeaderName, HeaderValue};
use std::str::FromStr;

/// A HTTP header map.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass]
pub struct HeaderMap(pub header::HeaderMap);

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl HeaderMap {
    #[inline]
    fn __getitem__<'py>(&self, py: Python<'py>, key: PyBackedStr) -> Option<Bound<'py, PyAny>> {
        let value = self.0.get(key.as_ref() as &str)?;
        let buffer = HeaderValueBuffer::new(value.clone());
        buffer.into_bytes_ref(py).ok()
    }

    #[inline]
    fn __setitem__(&mut self, py: Python, key: PyBackedStr, value: PyBackedStr) {
        py.allow_threads(|| {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_bytes(value.as_bytes()),
            ) {
                self.0.insert(name, value);
            }
        })
    }

    #[inline]
    fn __delitem__(&mut self, py: Python, key: PyBackedStr) {
        py.allow_threads(|| {
            self.0.remove(key.as_ref() as &str);
        })
    }

    #[inline]
    fn __contains__(&self, py: Python, key: PyBackedStr) -> bool {
        py.allow_threads(|| self.0.contains_key(key.as_ref() as &str))
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
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass]
pub struct HeaderMapKeysIter {
    inner: Vec<HeaderName>,
}

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl HeaderMapKeysIter {
    #[inline]
    fn __iter__(slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf
    }

    #[inline]
    fn __next__(mut slf: PyRefMut<Self>) -> Option<Bound<'_, PyAny>> {
        slf.inner
            .pop()
            .and_then(|k| HeaderNameBuffer::new(k).into_bytes_ref(slf.py()).ok())
    }
}

/// An iterator over the items in a HeaderMap.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass]
pub struct HeaderMapItemsIter {
    inner: Vec<(HeaderName, HeaderValue)>,
}

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl HeaderMapItemsIter {
    #[inline]
    fn __iter__(slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf
    }

    #[inline]
    fn __next__(
        mut slf: PyRefMut<'_, Self>,
    ) -> Option<(Bound<'_, PyAny>, Option<Bound<'_, PyAny>>)> {
        if let Some((k, v)) = slf.inner.pop() {
            let key = HeaderNameBuffer::new(k).into_bytes_ref(slf.py()).ok()?;
            let value = HeaderValueBuffer::new(v).into_bytes_ref(slf.py()).ok();
            return Some((key, value));
        }
        None
    }
}

/// A HTTP header map.
pub struct HeaderMapFromPyDict(pub header::HeaderMap);

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
                    let name = HeaderName::from_bytes(key.as_bytes()).map_err(Error::from)?;
                    let value = value.extract::<PyBackedStr>()?;
                    let value = HeaderValue::from_bytes(value.as_bytes()).map_err(Error::from)?;
                    headers.insert(name, value);
                    Ok(headers)
                },
            )
            .map(Self)
    }
}

impl<'py> IntoPyObject<'py> for HeaderMapFromPyDict {
    type Target = HeaderMap;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        HeaderMap(self.0).into_pyobject(py)
    }
}

#[cfg(feature = "docs")]
impl PyStubType for HeaderMapFromPyDict {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.Dict[str, str]", "typing".into())
    }
}

impl<'py> FromPyObject<'py> for HeadersOrderFromPyList {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let list = ob.downcast::<PyList>()?;
        list.iter()
            .try_fold(Vec::with_capacity(list.len()), |mut order, item| {
                let name = HeaderName::from_str(item.extract::<&str>()?).map_err(Error::from)?;
                order.push(name);
                Ok(order)
            })
            .map(Self)
    }
}
