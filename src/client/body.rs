//! Types and utilities for representing HTTP request bodies.

pub mod form;
pub mod json;
pub mod multipart;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::Stream;
use pyo3::{
    FromPyObject, PyAny, PyResult, Python,
    exceptions::PyTypeError,
    intern,
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};

use crate::rt::Runtime;

/// Represents the body of an HTTP request.
/// Supports text, bytes, synchronous and asynchronous streaming bodies.
pub enum Body {
    Form(form::Form),
    Json(json::Json),
    Bytes(Bytes),
    SyncStream(SyncStream),
    AsyncStream(AsyncStream),
}

impl TryFrom<Body> for wreq::Body {
    type Error = PyErr;

    /// Converts a `Body` into a `wreq::Body` for internal use.
    fn try_from(value: Body) -> PyResult<wreq::Body> {
        match value {
            Body::Form(form) => serde_urlencoded::to_string(form)
                .map(wreq::Body::from)
                .map_err(crate::Error::Form)
                .map_err(Into::into),
            Body::Json(json) => serde_json::to_vec(&json)
                .map_err(crate::Error::Json)
                .map(wreq::Body::from)
                .map_err(Into::into),
            Body::Bytes(bytes) => Ok(wreq::Body::from(bytes)),
            Body::SyncStream(stream) => Ok(wreq::Body::wrap_stream(stream)),
            Body::AsyncStream(stream) => Ok(wreq::Body::wrap_stream(stream)),
        }
    }
}

impl FromPyObject<'_, '_> for Body {
    type Error = PyErr;

    /// Extracts a `Body` from a Python object.
    /// Accepts str, bytes, sync iterator, or async iterator.
    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        if let Ok(bytes) = ob.extract::<PyBackedBytes>() {
            return Ok(Self::Bytes(Bytes::from_owner(bytes)));
        }

        if let Ok(text) = ob.extract::<PyBackedStr>() {
            return Ok(Self::Bytes(Bytes::from_owner(text)));
        }

        if let Ok(json) = ob.extract::<json::Json>() {
            return Ok(Self::Json(json));
        }

        if let Ok(form) = ob.extract::<form::Form>() {
            return Ok(Self::Form(form));
        }

        if ob.hasattr(intern!(ob.py(), "asend"))? {
            Runtime::into_stream(ob)
                .map(AsyncStream::new)
                .map(Self::AsyncStream)
        } else {
            ob.extract::<Py<PyAny>>()
                .map(SyncStream::new)
                .map(Self::SyncStream)
                .map_err(Into::into)
        }
    }
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
