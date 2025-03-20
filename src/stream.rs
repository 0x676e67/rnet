use bytes::Bytes;
use futures_util::Stream;
use pyo3::{
    PyObject, PyResult, Python,
    pybacked::{PyBackedBytes, PyBackedStr},
};
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
fn downcast_bound_bytes(py: Python<'_>, ob: PyObject) -> PyResult<Bytes> {
    if let Ok(str_chunk) = ob.extract::<PyBackedBytes>(py) {
        return Ok(Bytes::from_owner(str_chunk));
    }

    ob.extract::<PyBackedStr>(py)
        .map(Bytes::from_owner)
        .map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("Stream must yield bytes/str - like objects")
        })
}
