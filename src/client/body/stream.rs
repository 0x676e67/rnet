use std::{
    future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::{Stream, StreamExt, TryStreamExt};
use pyo3::{
    IntoPyObjectExt,
    exceptions::PyTypeError,
    intern,
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};
use tokio::sync::Mutex;

use crate::{buffer::PyBuffer, error::Error};

type BoxedStream<T> = Pin<Box<dyn Stream<Item = T> + Send + 'static>>;

/// Represents a Python streaming body, either synchronous or asynchronous.
pub enum PyStream {
    Sync(Py<PyAny>),
    Async(BoxedStream<Py<PyAny>>),
}

/// A bytes stream response.
#[pyclass(subclass)]
pub struct Streamer(Arc<Mutex<Option<BoxedStream<wreq::Result<Bytes>>>>>);

async fn anext(
    streamer: Arc<Mutex<Option<BoxedStream<wreq::Result<Bytes>>>>>,
    error: fn() -> Error,
) -> PyResult<PyBuffer> {
    let val = streamer
        .lock()
        .await
        .as_mut()
        .ok_or_else(error)?
        .try_next()
        .await;

    val.map_err(Error::Library)?
        .map(PyBuffer::from)
        .ok_or_else(error)
        .map_err(Into::into)
}

// ===== impl Streamer =====

impl Streamer {
    /// Create a new [`Streamer`] instance.
    #[inline]
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> Streamer {
        Streamer(Arc::new(Mutex::new(Some(stream.boxed()))))
    }
}

#[pymethods]
impl Streamer {
    #[inline]
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __aiter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __next__(&mut self, py: Python) -> PyResult<PyBuffer> {
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(anext(self.0.clone(), || Error::StopIteration))
        })
    }

    #[inline]
    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            anext(self.0.clone(), || Error::StopAsyncIteration),
        )
    }

    #[inline]
    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, future::ready(Ok(slf)))
    }

    #[inline]
    fn __exit__<'py>(
        &mut self,
        py: Python,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) {
        py.detach(|| self.0.blocking_lock().take());
    }

    #[inline]
    fn __aexit__<'py>(
        &mut self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            this.lock()
                .await
                .take()
                .map(drop)
                .map(PyResult::Ok)
                .transpose()
        })
    }
}

// ===== impl PyStream =====

impl FromPyObject<'_, '_> for PyStream {
    type Error = PyErr;

    /// Extracts a [`PyStream`] from a Python object.
    /// Accepts sync or async iterators.
    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        if ob.hasattr(intern!(ob.py(), "asend"))? {
            pyo3_async_runtimes::tokio::into_stream_v2(ob.to_owned())
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
