use crate::{
    buffer::{HeaderNameBuffer, HeaderValueBuffer, PyBufferProtocol},
    error::Error,
};
use pyo3::{
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyDict, PyList},
};
use wreq::header::{self, HeaderName, HeaderValue};

/// A HTTP header map.
#[pyclass(subclass)]
pub struct HeaderMap(pub header::HeaderMap);

#[pymethods]
impl HeaderMap {
    #[new]
    #[pyo3(signature = (init=None, capacity=None))]
    fn new(init: Option<&Bound<'_, PyDict>>, capacity: Option<usize>) -> Self {
        let mut headers = capacity
            .map(header::HeaderMap::with_capacity)
            .unwrap_or_else(header::HeaderMap::new);

        // This section of memory might be retained by the Rust object,
        // and we want to prevent Python's garbage collector from managing it.
        if let Some(dict) = init {
            for (name, value) in dict.iter() {
                if let (Ok(Ok(name)), Ok(Ok(value))) = (
                    name.extract::<PyBackedStr>()
                        .map(|n| HeaderName::from_bytes(n.as_bytes())),
                    value
                        .extract::<PyBackedStr>()
                        .map(HeaderValue::from_maybe_shared),
                ) {
                    headers.insert(name, value);
                }
            }
        }

        HeaderMap(headers)
    }

    /// Returns a reference to the value associated with the key.
    ///
    /// If there are multiple values associated with the key, then the first one
    /// is returned. Use `get_all` to get all values associated with a given
    /// key. Returns `None` if there are no values associated with the key.
    fn get<'py>(&self, py: Python<'py>, key: PyBackedStr) -> Option<Bound<'py, PyAny>> {
        let value = self.0.get::<&str>(key.as_ref())?;
        let buffer = HeaderValueBuffer::new(value.clone());
        buffer.into_bytes_ref(py).ok()
    }

    /// Returns a view of all values associated with a key.
    fn get_all(&self, key: PyBackedStr) -> HeaderMapValuesIter {
        HeaderMapValuesIter {
            inner: self
                .0
                .get_all::<&str>(key.as_ref())
                .iter()
                .cloned()
                .collect(),
        }
    }

    /// Insert a key-value pair into the header map.
    fn insert(&mut self, py: Python, key: PyBackedStr, value: PyBackedStr) {
        py.allow_threads(|| {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_maybe_shared(value),
            ) {
                self.0.insert(name, value);
            }
        })
    }

    /// Append a key-value pair to the header map.
    fn append(&mut self, py: Python, key: PyBackedStr, value: PyBackedStr) {
        py.allow_threads(|| {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_maybe_shared(value),
            ) {
                self.0.append(name, value);
            }
        })
    }

    /// Remove a key-value pair from the header map.
    fn remove(&mut self, py: Python, key: PyBackedStr) {
        py.allow_threads(|| {
            self.0.remove::<&str>(key.as_ref());
        })
    }

    /// Returns true if the map contains a value for the specified key.
    fn contains_key(&self, py: Python, key: PyBackedStr) -> bool {
        py.allow_threads(|| self.0.contains_key::<&str>(key.as_ref()))
    }

    /// Returns key-value pairs in the order they were added.
    fn items(&self) -> HeaderMapItemsIter {
        HeaderMapItemsIter {
            inner: self.0.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        }
    }

    /// Returns the number of headers stored in the map.
    ///
    /// This number represents the total number of **values** stored in the map.
    /// This number can be greater than or equal to the number of **keys**
    /// stored given that a single key may have more than one associated value.
    ///
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of keys stored in the map.
    ///
    /// This number will be less than or equal to `len()` as each key may have
    /// more than one associated value.
    #[inline]
    fn keys_len(&self) -> usize {
        self.0.keys_len()
    }

    /// Returns true if the map contains no elements.
    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
}

#[pymethods]
impl HeaderMap {
    #[inline]
    fn __getitem__<'py>(&self, py: Python<'py>, key: PyBackedStr) -> Option<Bound<'py, PyAny>> {
        self.get(py, key)
    }

    #[inline]
    fn __setitem__(&mut self, py: Python, key: PyBackedStr, value: PyBackedStr) {
        self.insert(py, key, value);
    }

    #[inline]
    fn __delitem__(&mut self, py: Python, key: PyBackedStr) {
        self.remove(py, key);
    }

    #[inline]
    fn __contains__(&self, py: Python, key: PyBackedStr) -> bool {
        self.contains_key(py, key)
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
#[pyclass(subclass)]
pub struct HeaderMapKeysIter {
    inner: Vec<HeaderName>,
}

#[pymethods]
impl HeaderMapKeysIter {
    #[inline]
    fn __iter__(slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<Bound<'_, PyAny>> {
        slf.inner
            .pop()
            .and_then(|k| HeaderNameBuffer::new(k).into_bytes_ref(slf.py()).ok())
    }
}

/// An iterator over the values in a HeaderMap.
#[pyclass(subclass)]
pub struct HeaderMapValuesIter {
    inner: Vec<HeaderValue>,
}
#[pymethods]
impl HeaderMapValuesIter {
    #[inline]
    fn __iter__(slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<Bound<'_, PyAny>> {
        slf.inner
            .pop()
            .and_then(|v| HeaderValueBuffer::new(v).into_bytes_ref(slf.py()).ok())
    }
}

/// An iterator over the items in a HeaderMap.
#[pyclass]
pub struct HeaderMapItemsIter {
    inner: Vec<(HeaderName, HeaderValue)>,
}

#[pymethods]
impl HeaderMapItemsIter {
    #[inline]
    fn __iter__(slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf
    }

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
                    let name = {
                        let name = name.extract::<PyBackedStr>()?;
                        HeaderName::from_bytes(name.as_bytes()).map_err(Error::from)?
                    };

                    let value = {
                        let value = value.extract::<PyBackedStr>()?;
                        HeaderValue::from_maybe_shared(value).map_err(Error::from)?
                    };

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
                let name = {
                    let name = name.extract::<PyBackedStr>()?;
                    HeaderName::from_bytes(name.as_bytes()).map_err(Error::from)?
                };

                order.push(name);
                Ok(order)
            })
            .map(Self)
    }
}
