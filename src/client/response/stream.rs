use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use pyo3::{IntoPyObjectExt, prelude::*};
use tokio::sync::mpsc;

use crate::{buffer::PyBuffer, client::response::future::AllowThreads, error::Error};

/// A byte stream response.
/// An asynchronous iterator yielding data chunks from the response stream.
/// Used to stream response content.
/// Implemented in the `stream` method of the `Response` class.
/// Can be used in an asynchronous for loop in Python.
#[pyclass(subclass)]
pub struct Streamer(mpsc::Receiver<wreq::Result<Bytes>>);

impl Streamer {
    /// Create a new `Streamer` instance.
    #[inline]
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> Streamer {
        let (tx, rx) = mpsc::channel(8);
        pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
            futures_util::pin_mut!(stream);
            while let Some(item) = stream.next().await {
                if tx.send(item).await.is_err() {
                    break;
                }
            }
        });

        Streamer(rx)
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
    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let res = next(py, &mut self.0, Error::StopAsyncIteration);
        AllowThreads::closure(move || res).future_into_py(py)
    }

    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        AllowThreads::closure(move || Ok(slf)).future_into_py(py)
    }

    #[inline]
    fn __aexit__<'py>(
        &mut self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.0.close();
        AllowThreads::closure(move || Ok(())).future_into_py(py)
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
    fn __next__(&mut self, py: Python) -> PyResult<PyBuffer> {
        next(py, &mut self.0, Error::StopIteration)
    }

    #[inline]
    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __exit__<'py>(
        &mut self,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) {
        self.0.close();
    }
}

fn next(
    py: Python,
    rx: &mut mpsc::Receiver<wreq::Result<Bytes>>,
    error: Error,
) -> PyResult<PyBuffer> {
    py.allow_threads(|| {
        rx.blocking_recv()
            .ok_or(error)?
            .map(PyBuffer::from)
            .map_err(Error::Library)
            .map_err(Into::into)
    })
}
