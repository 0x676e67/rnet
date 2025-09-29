use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use pyo3::{IntoPyObjectExt, prelude::*};
use tokio::sync::mpsc::{self, error::TryRecvError};

use crate::{bridge::Runtime, buffer::PyBuffer, client::nogil::NoGIL, error::Error};

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
        Runtime::spawn(async move {
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
        let res = py.detach(|| match self.0.try_recv() {
            Ok(res) => res.map(PyBuffer::from).map(Some).map_err(Error::Library),
            Err(err) => match err {
                TryRecvError::Empty => Ok(None),
                TryRecvError::Disconnected => Err(Error::StopAsyncIteration),
            },
        })?;
        NoGIL::closure(py, move || Ok(res))
    }

    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        NoGIL::closure(py, move || Ok(slf))
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
        NoGIL::closure(py, move || Ok(()))
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
        py.detach(|| {
            self.0
                .blocking_recv()
                .ok_or(Error::StopIteration)?
                .map(PyBuffer::from)
                .map_err(Error::Library)
                .map_err(Into::into)
        })
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
