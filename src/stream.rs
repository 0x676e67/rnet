use bytes::Bytes;
use futures_util::Stream;
use pyo3::{
    types::{PyBytes, PyBytesMethods},
    PyObject, PyResult, Python,
};
use std::pin::Pin;

pub struct SyncStream {
    iter: PyObject,
}

pub struct AsyncStream {
    stream: Pin<Box<dyn futures_util::Stream<Item = PyObject> + Send + Sync + 'static>>,
}

impl SyncStream {
    #[inline]
    pub fn new(iter: PyObject) -> Self {
        SyncStream { iter }
    }
}

impl AsyncStream {
    #[inline]
    pub fn new(stream: impl futures_util::Stream<Item = PyObject> + Send + Sync + 'static) -> Self {
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
        let next = Python::with_gil(|py| {
            self.iter
                .call_method0(py, "__next__")
                .ok()
                .map(|item| downcast_bound_bytes(py, item))
        });
        std::task::Poll::Ready(next)
    }
}

impl Stream for AsyncStream {
    type Item = PyResult<Bytes>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.stream
            .as_mut()
            .poll_next(cx)
            .map(|item| item.map(|item| Python::with_gil(|py| downcast_bound_bytes(py, item))))
    }
}

#[inline]
fn downcast_bound_bytes<'p>(py: Python<'p>, ob: PyObject) -> PyResult<Bytes> {
    ob.downcast_bound::<PyBytes>(py)
        .map(move |b| b.as_bytes().to_vec())
        .map(Bytes::from)
        .map_err(|_| pyo3::exceptions::PyTypeError::new_err("Stream must yield bytes-like objects"))
}

impl From<SyncStream> for rquest::Body {
    #[inline]
    fn from(iterator: SyncStream) -> Self {
        rquest::Body::wrap_stream(iterator)
    }
}

impl From<SyncStream> for rquest::multipart::Part {
    #[inline]
    fn from(iterator: SyncStream) -> Self {
        rquest::multipart::Part::stream(iterator)
    }
}

impl From<AsyncStream> for rquest::Body {
    #[inline]
    fn from(stream: AsyncStream) -> Self {
        rquest::Body::wrap_stream(stream)
    }
}

impl From<AsyncStream> for rquest::multipart::Part {
    #[inline]
    fn from(stream: AsyncStream) -> Self {
        rquest::multipart::Part::stream(stream)
    }
}
