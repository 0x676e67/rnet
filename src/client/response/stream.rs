use std::{pin::Pin, sync::Arc};

use bytes::Bytes;
use futures_util::{Stream, TryStreamExt};
use pyo3::{IntoPyObjectExt, prelude::*};
use pyo3_async_runtimes::tokio::future_into_py;
use tokio::sync::Mutex;

use crate::{
    buffer::{BytesBuffer, PyBufferProtocol},
    error::Error,
};

type BytesStream = Pin<Box<dyn Stream<Item = wreq::Result<Bytes>> + Send + 'static>>;

/// A byte stream response.
/// An asynchronous iterator yielding data chunks from the response stream.
/// Used to stream response content.
/// Implemented in the `stream` method of the `Response` class.
/// Can be used in an asynchronous for loop in Python.
#[derive(Clone)]
#[pyclass(subclass)]
pub struct Streamer(Arc<Mutex<Option<BytesStream>>>);

impl Streamer {
    /// Create a new `Streamer` instance.
    #[inline]
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> Streamer {
        Streamer(Arc::new(Mutex::new(Some(Box::pin(stream)))))
    }
}

/// Asynchronous iterator implementation for `Streamer`.
#[pymethods]
impl Streamer {
    #[inline]
    fn __aiter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(
            py,
            anext(self.0.clone(), || Error::StopAsyncIteration.into()),
        )
    }

    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    #[inline]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let streamer = self.0.clone();
        let fut = async move {
            let mut lock = streamer.lock().await;
            Ok(drop(lock.take()))
        };
        future_into_py(py, fut)
    }
}

/// Synchronous iterator implementation for `Streamer`.
#[pymethods]
impl Streamer {
    #[inline]
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __next__(&self, py: Python) -> PyResult<Py<PyAny>> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(anext(self.0.clone(), || Error::StopIteration.into()))
        })
    }

    #[inline]
    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __exit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<()> {
        py.allow_threads(|| {
            let streamer = self.0.clone();
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                let mut lock = streamer.lock().await;
                drop(lock.take());
                Ok(())
            })
        })
    }
}

async fn anext(
    streamer: Arc<Mutex<Option<BytesStream>>>,
    error: fn() -> PyErr,
) -> PyResult<Py<PyAny>> {
    let mut lock = streamer.lock().await;
    let val = lock.as_mut().ok_or_else(error)?.try_next().await;

    drop(lock);

    let buffer = val
        .map_err(Error::Library)?
        .map(BytesBuffer::new)
        .ok_or_else(error)?;

    Python::with_gil(|py| buffer.into_bytes(py))
}
