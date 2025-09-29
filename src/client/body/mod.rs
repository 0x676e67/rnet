//! Types and utilities for representing HTTP request bodies.

pub mod multipart;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::Stream;
use indexmap::IndexMap;
use pyo3::{
    FromPyObject, PyAny, PyResult, Python,
    exceptions::PyTypeError,
    intern,
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};
use serde::{Deserialize, Serialize};

use crate::bridge::Runtime;

/// Represents the body of an HTTP request.
/// Supports text, bytes, synchronous and asynchronous streaming bodies.
pub enum Body {
    Text(Bytes),
    Bytes(Bytes),
    SyncStream(SyncStream),
    AsyncStream(AsyncStream),
}

impl From<Body> for wreq::Body {
    /// Converts a `Body` into a `wreq::Body` for internal use.
    fn from(value: Body) -> wreq::Body {
        match value {
            Body::Text(bytes) | Body::Bytes(bytes) => wreq::Body::from(bytes),
            Body::SyncStream(stream) => wreq::Body::wrap_stream(stream),
            Body::AsyncStream(stream) => wreq::Body::wrap_stream(stream),
        }
    }
}

impl FromPyObject<'_> for Body {
    /// Extracts a `Body` from a Python object.
    /// Accepts str, bytes, sync iterator, or async iterator.
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(text) = ob.extract::<PyBackedStr>() {
            return Ok(Self::Text(Bytes::from_owner(text)));
        }

        if let Ok(bytes) = ob.extract::<PyBackedBytes>() {
            return Ok(Self::Bytes(Bytes::from_owner(bytes)));
        }

        if ob.hasattr(intern!(ob.py(), "asend"))? {
            Runtime::into_stream(ob.to_owned())
                .map(AsyncStream::new)
                .map(Self::AsyncStream)
        } else {
            ob.extract::<Py<PyAny>>()
                .map(SyncStream::new)
                .map(Self::SyncStream)
        }
    }
}

/// Represents a JSON value for HTTP requests.
/// Supports objects, arrays, numbers, strings, booleans, and null.
#[derive(Clone, FromPyObject, IntoPyObject, Serialize, Deserialize)]
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

/// Wraps a Python synchronous iterator for use as a streaming HTTP body.
pub struct SyncStream {
    iter: Py<PyAny>,
}

impl SyncStream {
    /// Creates a new [`SyncStream`] from a Python iterator.
    #[inline]
    pub fn new(iter: Py<PyAny>) -> Self {
        SyncStream { iter }
    }
}

impl Stream for SyncStream {
    type Item = PyResult<Bytes>;

    /// Yields the next chunk from the Python iterator as bytes.
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Python::attach(|py| {
            let next = self
                .iter
                .call_method0(py, intern!(py, "__next__"))
                .ok()
                .map(|item| extract_bytes(py, item));
            py.detach(|| Poll::Ready(next))
        })
    }
}

/// Wraps a Python asynchronous iterator for use as a streaming HTTP body.
pub struct AsyncStream {
    stream: Pin<Box<dyn Stream<Item = Py<PyAny>> + Send + Sync + 'static>>,
}

impl AsyncStream {
    /// Creates a new [`AsyncStream`] from a Rust or Python async stream.
    #[inline]
    pub fn new(stream: impl Stream<Item = Py<PyAny>> + Send + Sync + 'static) -> Self {
        AsyncStream {
            stream: Box::pin(stream),
        }
    }
}

impl Stream for AsyncStream {
    type Item = PyResult<Bytes>;

    /// Yields the next chunk from the async stream as bytes.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let waker = cx.waker();
        Python::attach(|py| {
            py.detach(|| {
                self.stream
                    .as_mut()
                    .poll_next(&mut Context::from_waker(waker))
            })
            .map(|item| item.map(|item| extract_bytes(py, item)))
        })
    }
}

/// Extracts a [`Bytes`] object from a Python object.
/// Accepts bytes-like or str-like objects, otherwise raises a `TypeError`.
#[inline]
fn extract_bytes(py: Python<'_>, ob: Py<PyAny>) -> PyResult<Bytes> {
    match ob.extract::<PyBackedBytes>(py) {
        Ok(chunk) => Ok(Bytes::from_owner(chunk)),
        Err(_) => ob
            .extract::<PyBackedStr>(py)
            .map(Bytes::from_owner)
            .map_err(|err| {
                PyTypeError::new_err(format!("Stream must yield bytes/str - like objects: {err}"))
            }),
    }
}
