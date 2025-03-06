use bytes::Bytes;
use futures_util::Stream;
use pyo3::{PyObject, PyResult, Python, pybacked::PyBackedBytes, types::PyAnyMethods};
use std::{pin::Pin, task::Context};

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
                .map(|item| downcast_bound_bytes(py, item));
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
            .map(|item| item.map(|item| downcast_bound_bytes(py, item)))
        })
    }
}

#[inline]
fn downcast_bound_bytes<'p>(py: Python<'p>, ob: PyObject) -> PyResult<Bytes> {
    let bind = ob.bind(py);
    bind.extract::<PyBackedBytes>()
        .map(move |b| b.as_ref().to_vec())
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
