use std::{pin::Pin, sync::Arc};

use futures_util::{Stream, TryStreamExt};
use pyo3::{IntoPyObjectExt, prelude::*};
use pyo3_async_runtimes::tokio::future_into_py;
use tokio::sync::Mutex;

use crate::{
    buffer::{BytesBuffer, PyBufferProtocol},
    error::Error,
};

type InnerStreamer =
    Arc<Mutex<Option<Pin<Box<dyn Stream<Item = wreq::Result<bytes::Bytes>> + Send + 'static>>>>>;

/// A byte stream response.
/// An asynchronous iterator yielding data chunks from the response stream.
/// Used to stream response content.
/// Implemented in the `stream` method of the `Response` class.
/// Can be used in an asynchronous for loop in Python.
#[derive(Clone)]
#[pyclass(subclass)]
pub struct Streamer(InnerStreamer);

impl Streamer {
    /// Create a new `Streamer` instance.
    pub fn new(
        stream: impl Stream<Item = wreq::Result<bytes::Bytes>> + Send + 'static,
    ) -> Streamer {
        Streamer(Arc::new(Mutex::new(Some(Box::pin(stream)))))
    }

    pub async fn _anext(streamer: InnerStreamer, error: fn() -> PyErr) -> PyResult<Py<PyAny>> {
        let mut lock = streamer.lock().await;
        let val = lock.as_mut().ok_or_else(error)?.try_next().await;

        drop(lock);

        let buffer = val
            .map_err(Error::Request)?
            .map(BytesBuffer::new)
            .ok_or_else(error)?;

        Python::with_gil(|py| buffer.into_bytes(py))
    }
}

/// Asynchronous iterator implementation for `Streamer`.
#[pymethods]
impl Streamer {
    fn __aiter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(
            py,
            Streamer::_anext(self.0.clone(), || Error::StopAsyncIteration.into()),
        )
    }

    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let streamer = self.0.clone();
        future_into_py(py, async move {
            drop(streamer.lock().await.take());
            Ok(())
        })
    }
}

/// Synchronous iterator implementation for `Streamer`.
#[pymethods]
impl Streamer {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&self, py: Python) -> PyResult<Py<PyAny>> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(Streamer::_anext(self.0.clone(), || {
                    Error::StopIteration.into()
                }))
        })
    }

    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

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
