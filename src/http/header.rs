use std::fmt;

use bytes::Bytes;
use pyo3::{
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
    types::{PyDict, PyList},
};
use wreq::header::{self, HeaderName, HeaderValue};

use crate::buffer::{BytesBuffer, HeaderNameBuffer, HeaderValueBuffer, PyBufferProtocol};

/// A HTTP header map.
#[pyclass(subclass, str)]
#[derive(Clone)]
pub struct HeaderMap(pub header::HeaderMap);

/// A HTTP original header map.
#[pyclass(subclass, str)]
#[derive(Clone)]
pub struct OrigHeaderMap(pub header::OrigHeaderMap);

/// An iterator over the keys in a HeaderMap.
#[pyclass(subclass)]
pub struct HeaderMapKeysIter {
    inner: Vec<HeaderName>,
}

/// An iterator over the values in a HeaderMap.
#[pyclass(subclass)]
pub struct HeaderMapValuesIter {
    inner: Vec<HeaderValue>,
}

/// An iterator over the items in a HeaderMap.
#[pyclass]
pub struct HeaderMapItemsIter {
    inner: Vec<(HeaderName, HeaderValue)>,
}

/// An iterator over the items in a OrigHeaderMap.
#[pyclass]
pub struct OrigHeaderMapIter {
    inner: Vec<(HeaderName, header::OrigHeaderName)>,
}

// ===== impl HeaderMap =====

#[pymethods]
impl HeaderMap {
    /// Creates a new `HeaderMap` from an optional dictionary.
    #[new]
    #[pyo3(signature = (dict=None, capacity=None))]
    fn new(dict: Option<&Bound<'_, PyDict>>, capacity: Option<usize>) -> HeaderMap {
        let mut headers = capacity
            .map(header::HeaderMap::with_capacity)
            .unwrap_or_default();

        // This section of memory might be retained by the Rust object,
        // and we want to prevent Python's garbage collector from managing it.
        if let Some(dict) = dict {
            for (name, value) in dict.iter() {
                let name = match name
                    .extract::<PyBackedStr>()
                    .map(|n| HeaderName::from_bytes(n.as_bytes()))
                {
                    Ok(Ok(n)) => n,
                    _ => continue,
                };

                let value = match value
                    .extract::<PyBackedStr>()
                    .map(Bytes::from_owner)
                    .map(HeaderValue::from_maybe_shared)
                {
                    Ok(Ok(v)) => v,
                    _ => continue,
                };

                headers.insert(name, value);
            }
        }

        HeaderMap(headers)
    }

    /// Returns a reference to the value associated with the key.
    ///
    /// If there are multiple values associated with the key, then the first one
    /// is returned. Use `get_all` to get all values associated with a given
    /// key. Returns `None` if there are no values associated with the key.
    #[pyo3(signature = (key, default=None))]
    fn get<'py>(
        &self,
        py: Python<'py>,
        key: PyBackedStr,
        default: Option<PyBackedBytes>,
    ) -> Option<Bound<'py, PyAny>> {
        let value = py.allow_threads(|| {
            self.0.get::<&str>(key.as_ref()).cloned().or_else(|| {
                match default
                    .map(Bytes::from_owner)
                    .map(HeaderValue::from_maybe_shared)
                {
                    Some(Ok(v)) => Some(v),
                    _ => None,
                }
            })
        });

        value.and_then(|v| HeaderValueBuffer::new(v).into_bytes_ref(py).ok())
    }

    /// Returns a view of all values associated with a key.
    #[pyo3(signature = (key))]
    fn get_all(&self, py: Python, key: PyBackedStr) -> HeaderMapValuesIter {
        py.allow_threads(|| HeaderMapValuesIter {
            inner: self
                .0
                .get_all::<&str>(key.as_ref())
                .iter()
                .cloned()
                .collect(),
        })
    }

    /// Insert a key-value pair into the header map.
    #[pyo3(signature = (key, value))]
    fn insert(&mut self, py: Python, key: PyBackedStr, value: PyBackedStr) {
        py.allow_threads(|| {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_maybe_shared(Bytes::from_owner(value)),
            ) {
                self.0.insert(name, value);
            }
        })
    }

    /// Append a key-value pair to the header map.
    #[pyo3(signature = (key, value))]
    fn append(&mut self, py: Python, key: PyBackedStr, value: PyBackedStr) {
        py.allow_threads(|| {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_maybe_shared(Bytes::from_owner(value)),
            ) {
                self.0.append(name, value);
            }
        })
    }

    /// Remove a key-value pair from the header map.
    #[pyo3(signature = (key))]
    fn remove(&mut self, py: Python, key: PyBackedStr) {
        py.allow_threads(|| {
            self.0.remove::<&str>(key.as_ref());
        })
    }

    /// Returns true if the map contains a value for the specified key.
    #[pyo3(signature = (key))]
    fn contains_key(&self, py: Python, key: PyBackedStr) -> bool {
        py.allow_threads(|| self.0.contains_key::<&str>(key.as_ref()))
    }

    /// Returns key-value pairs in the order they were added.
    #[inline]
    fn items(&self, py: Python) -> HeaderMapItemsIter {
        py.allow_threads(|| HeaderMapItemsIter {
            inner: self.0.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        })
    }

    /// Returns the number of headers stored in the map.
    ///
    /// This number represents the total number of **values** stored in the map.
    /// This number can be greater than or equal to the number of **keys**
    /// stored given that a single key may have more than one associated value.
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

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
}

#[pymethods]
impl HeaderMap {
    #[inline]
    fn __getitem__<'py>(&self, py: Python<'py>, key: PyBackedStr) -> Option<Bound<'py, PyAny>> {
        self.get(py, key, None)
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
}

impl fmt::Display for HeaderMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// ===== impl OrigHeaderMap =====
#[pymethods]
impl OrigHeaderMap {
    /// Creates a new `OrigHeaderMap` from an optional list of header names.
    #[new]
    #[pyo3(signature = (init=None, capacity=None))]
    fn new(init: Option<&Bound<'_, PyList>>, capacity: Option<usize>) -> OrigHeaderMap {
        let mut headers = capacity
            .map(header::OrigHeaderMap::with_capacity)
            .unwrap_or_default();

        // This section of memory might be retained by the Rust object,
        // and we want to prevent Python's garbage collector from managing it.
        if let Some(init) = init {
            for name in init.iter() {
                let name = match name
                    .extract::<PyBackedStr>()
                    .map(|n| HeaderName::from_bytes(n.as_bytes()))
                {
                    Ok(Ok(n)) => n,
                    _ => continue,
                };

                headers.insert(name);
            }
        }

        OrigHeaderMap(headers)
    }

    /// Insert a new header name into the collection.
    ///
    /// If the map did not previously have this key present, then `false` is
    /// returned.
    ///
    /// If the map did have this key present, the new value is pushed to the end
    /// of the list of values currently associated with the key. The key is not
    /// updated, though; this matters for types that can be `==` without being
    /// identical.
    #[inline]
    pub fn insert(&mut self, value: PyBackedStr) -> bool {
        self.0.insert(Bytes::from_owner(value))
    }

    /// Extends the map with all entries from another [`OrigHeaderMap`], preserving order.
    #[inline]
    pub fn extend(&mut self, iter: &Bound<'_, OrigHeaderMap>) {
        self.0.extend(iter.borrow().0.clone());
    }

    /// Returns key-value pairs in the order they were added.
    #[inline]
    pub fn items(&self, py: Python) -> OrigHeaderMapIter {
        py.allow_threads(|| OrigHeaderMapIter {
            inner: self.0.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        })
    }
}

#[pymethods]
impl OrigHeaderMap {
    #[inline]
    fn __len__(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for OrigHeaderMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// ===== impl HeaderMapKeysIter =====

#[pymethods]
impl HeaderMapKeysIter {
    #[inline]
    fn __iter__(slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf
    }

    #[inline]
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Bound<'_, PyAny>> {
        slf.inner
            .pop()
            .and_then(|k| HeaderNameBuffer::new(k).into_bytes_ref(slf.py()).ok())
    }
}

// ===== impl HeaderMapValuesIter =====

#[pymethods]
impl HeaderMapValuesIter {
    #[inline]
    fn __iter__(slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf
    }

    #[inline]
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Bound<'_, PyAny>> {
        slf.inner
            .pop()
            .and_then(|v| HeaderValueBuffer::new(v).into_bytes_ref(slf.py()).ok())
    }
}

// ===== impl HeaderMapItemsIter =====

#[pymethods]
impl HeaderMapItemsIter {
    #[inline]
    fn __iter__(slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(Bound<'_, PyAny>, Bound<'_, PyAny>)> {
        if let Some((k, v)) = slf.inner.pop() {
            let name = HeaderNameBuffer::new(k).into_bytes_ref(slf.py()).ok()?;
            let value = HeaderValueBuffer::new(v).into_bytes_ref(slf.py()).ok()?;
            return Some((name, value));
        }
        None
    }
}

// ===== impl OrigHeaderMapItemsIter =====
#[pymethods]
impl OrigHeaderMapIter {
    #[inline]
    fn __iter__(slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(Bound<'_, PyAny>, Bound<'_, PyAny>)> {
        if let Some((name, orig_name)) = slf.inner.pop() {
            let name = HeaderNameBuffer::new(name).into_bytes_ref(slf.py()).ok()?;
            let orig_name = match orig_name {
                header::OrigHeaderName::Cased(bytes) => {
                    BytesBuffer::new(bytes).into_bytes_ref(slf.py()).ok()?
                }
                header::OrigHeaderName::Standard(name) => {
                    HeaderNameBuffer::new(name).into_bytes_ref(slf.py()).ok()?
                }
            };

            return Some((name, orig_name));
        }
        None
    }
}
