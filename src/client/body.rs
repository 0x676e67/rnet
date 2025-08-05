use std::{pin::Pin, task::Context};

use bytes::Bytes;
use futures_util::Stream;
use pyo3::{
    FromPyObject, PyAny, PyObject, PyResult, Python,
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};

/// The body to use for the request.
pub enum Body {
    Text(Bytes),
    Bytes(Bytes),
    SyncStream(SyncStream),
    AsyncStream(AsyncStream),
}

impl From<Body> for wreq::Body {
    fn from(value: Body) -> wreq::Body {
        match value {
            Body::Text(bytes) | Body::Bytes(bytes) => wreq::Body::from(bytes),
            Body::SyncStream(stream) => wreq::Body::wrap_stream(stream),
            Body::AsyncStream(stream) => wreq::Body::wrap_stream(stream),
        }
    }
}

impl FromPyObject<'_> for Body {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(text) = ob.extract::<PyBackedStr>() {
            return Ok(Self::Text(Bytes::from_owner(text)));
        }

        if let Ok(bytes) = ob.extract::<PyBackedBytes>() {
            return Ok(Self::Bytes(Bytes::from_owner(bytes)));
        }

        if ob.hasattr("asend")? {
            pyo3_async_runtimes::tokio::into_stream_v2(ob.to_owned())
                .map(AsyncStream::new)
                .map(Self::AsyncStream)
        } else {
            ob.extract::<PyObject>()
                .map(SyncStream::new)
                .map(Self::SyncStream)
        }
    }
}

pub struct SyncStream {
    iter: PyObject,
}

pub struct AsyncStream {
    stream: Pin<Box<dyn Stream<Item = PyObject> + Send + Sync + 'static>>,
}

impl SyncStream {
    #[inline]
    pub fn new(iter: PyObject) -> Self {
        SyncStream { iter }
    }
}

impl AsyncStream {
    #[inline]
    pub fn new(stream: impl Stream<Item = PyObject> + Send + Sync + 'static) -> Self {
        AsyncStream {
            stream: Box::pin(stream),
        }
    }
}

impl Stream for SyncStream {
    type Item = PyResult<Bytes>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        Python::with_gil(|py| {
            let next = self
                .iter
                .call_method0(py, "__next__")
                .ok()
                .map(|item| extract_bytes(py, item));
            py.allow_threads(|| std::task::Poll::Ready(next))
        })
    }
}

impl Stream for AsyncStream {
    type Item = PyResult<Bytes>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let waker = cx.waker();
        Python::with_gil(|py| {
            py.allow_threads(|| {
                self.stream
                    .as_mut()
                    .poll_next(&mut Context::from_waker(waker))
            })
            .map(|item| item.map(|item| extract_bytes(py, item)))
        })
    }
}

#[inline]
fn extract_bytes(py: Python<'_>, ob: PyObject) -> PyResult<Bytes> {
    if let Ok(str_chunk) = ob.extract::<PyBackedBytes>(py) {
        return Ok(Bytes::from_owner(str_chunk));
    }

    ob.extract::<PyBackedStr>(py)
        .map(Bytes::from_owner)
        .map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("Stream must yield bytes/str - like objects")
        })
}
