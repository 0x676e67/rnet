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

/// Represents the body of an HTTP request.
/// Supports text, bytes, form, json, synchronous and asynchronous streaming bodies.
#[derive(FromPyObject)]
pub enum Body {
    Text(PyBackedStr),
    Bytes(PyBackedBytes),
    Form(form::Form),
    Json(json::Json),
    Stream(PyStream),
}

impl TryFrom<Body> for wreq::Body {
    type Error = PyErr;

    /// Converts a [`Body`] into a [`wreq::Body`] for internal use.
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
            Body::Text(s) => Ok(wreq::Body::from(Bytes::from_owner(s))),
            Body::Bytes(bytes) => Ok(wreq::Body::from(Bytes::from_owner(bytes))),
            Body::Stream(stream) => Ok(wreq::Body::wrap_stream(stream)),
        }
    }
}

/// Represents a Python streaming body, either synchronous or asynchronous.
pub enum PyStream {
    Sync(Py<PyAny>),
    Async(Pin<Box<dyn Stream<Item = Py<PyAny>> + Send + 'static>>),
}

impl FromPyObject<'_, '_> for PyStream {
    type Error = PyErr;

    /// Extracts a [`PyStream`] from a Python object.
    /// Accepts sync or async iterators.
    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        if ob.hasattr(intern!(ob.py(), "asend"))? {
            crate::rt::Runtime::into_stream(ob)
                .map(Box::pin)
                .map(|stream| PyStream::Async(stream))
        } else {
            ob.extract::<Py<PyAny>>()
                .map(PyStream::Sync)
                .map_err(Into::into)
        }
    }
}

impl Stream for PyStream {
    type Item = PyResult<Bytes>;

    /// Yields the next chunk from the Python stream as bytes.
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.get_mut() {
            PyStream::Sync(iter) => Python::attach(|py| {
                let next = iter
                    .call_method0(py, intern!(py, "__next__"))
                    .ok()
                    .map(|item| extract_bytes(py, item));
                py.detach(|| Poll::Ready(next))
            }),
            PyStream::Async(stream) => {
                let waker = cx.waker();
                Python::attach(|py| {
                    py.detach(|| stream.as_mut().poll_next(&mut Context::from_waker(waker)))
                        .map(|item| item.map(|item| extract_bytes(py, item)))
                })
            }
        }
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
