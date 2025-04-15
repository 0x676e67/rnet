use crate::{
    buffer::{HeaderNameBuffer, HeaderValueBuffer, PyBufferProtocol},
    define_into_pyobject_todo, define_py_stub_gen,
    error::Error,
};
use pyo3::{
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyDict, PyList},
};
#[cfg(feature = "docs")]
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::header::{self, HeaderName, HeaderValue};

/// A HTTP header map.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass]
pub struct HeaderMap(pub header::HeaderMap);

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl HeaderMap {
    #[new]
    #[inline]
    fn new(init: Option<&Bound<'_, PyDict>>) -> Self {
        let mut headers = header::HeaderMap::new();

        // This section of memory might be retained by the Rust object,
        // and we want to prevent Python's garbage collector from managing it.
        if let Some(dict) = init {
            for (name, value) in dict.iter() {
                if let (Ok(Ok(name)), Ok(Ok(value))) = (
                    name.extract::<PyBackedStr>()
                        .map(|n| HeaderName::from_bytes(n.as_bytes())),
                    value
                        .extract::<PyBackedStr>()
                        .map(|v| HeaderValue::from_bytes(v.as_bytes())),
                ) {
                    headers.insert(name, value);
                }
            }
        }

        Self(headers)
    }

    /// Returns multiple value sequences of key mapping
    fn get_all(&self, key: PyBackedStr) -> HeaderMapValuesIter {
        HeaderMapValuesIter {
            inner: self
                .0
                .get_all(key.as_ref() as &str)
                .iter()
                .cloned()
                .collect(),
        }
    }

    /// Returns key-value pairs in the order they were added.
    #[inline]
    fn items(&self) -> HeaderMapItemsIter {
        HeaderMapItemsIter {
            inner: self.0.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        }
    }
}

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
                self.0.append(name, value);
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

/// An iterator over the values in a HeaderMap.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass]
pub struct HeaderMapValuesIter {
    inner: Vec<HeaderValue>,
}
#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl HeaderMapValuesIter {
    #[inline]
    fn __iter__(slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf
    }

    #[inline]
    fn __next__(mut slf: PyRefMut<Self>) -> Option<Bound<'_, PyAny>> {
        slf.inner
            .pop()
            .and_then(|v| HeaderValueBuffer::new(v).into_bytes_ref(slf.py()).ok())
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
pub struct HeaderMapExtractor(pub header::HeaderMap);

/// A list of header names in order.
pub struct HeadersOrderExtractor(pub Vec<HeaderName>);

impl FromPyObject<'_> for HeaderMapExtractor {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(headers) = ob.downcast::<HeaderMap>() {
            return Ok(Self(headers.borrow().0.clone()));
        }

        let dict = ob.downcast::<PyDict>()?;
        dict.iter()
            .try_fold(
                header::HeaderMap::with_capacity(dict.len()),
                |mut headers, (name, value)| {
                    let name = name.extract::<PyBackedStr>()?;
                    let name = HeaderName::from_bytes(name.as_bytes()).map_err(Error::from)?;
                    let value = value.extract::<PyBackedStr>()?;
                    let value = HeaderValue::from_bytes(value.as_bytes()).map_err(Error::from)?;
                    headers.insert(name, value);
                    Ok(headers)
                },
            )
            .map(Self)
    }
}

impl<'py> FromPyObject<'py> for HeadersOrderExtractor {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let list = ob.downcast::<PyList>()?;
        list.iter()
            .try_fold(Vec::with_capacity(list.len()), |mut order, name| {
                let name = name.extract::<PyBackedStr>()?;
                let name = HeaderName::from_bytes(name.as_bytes()).map_err(Error::from)?;
                order.push(name);
                Ok(order)
            })
            .map(Self)
    }
}

define_into_pyobject_todo!(HeaderMapExtractor);

define_into_pyobject_todo!(HeadersOrderExtractor);

define_py_stub_gen!(
    HeaderMapExtractor,
    "typing.Union[typing.Dict[str, str], HeaderMap]",
    "typing"
);

define_py_stub_gen!(HeadersOrderExtractor, "typing.List[str]", "typing");
