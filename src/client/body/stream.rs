use std::{
    future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::{FutureExt, Stream, StreamExt, TryStreamExt, stream::BoxStream};
use pyo3::{
    IntoPyObjectExt,
    exceptions::PyTypeError,
    intern,
    prelude::*,
    pybacked::{PyBackedBytes, PyBackedStr},
};
use tokio::{runtime::Handle, sync::Mutex, task::JoinHandle};

use crate::{buffer::PyBuffer, error::Error};

type Pending = Option<JoinHandle<Option<PyResult<Bytes>>>>;

/// Python stream source (sync or async iterator)
enum StreamSource {
    Sync(Arc<Py<PyAny>>),
    Async(Arc<Mutex<BoxStream<'static, Py<PyAny>>>>),
}

/// A Python stream wrapper.
pub struct PyStream {
    source: StreamSource,
    pending: Option<JoinHandle<Option<PyResult<Bytes>>>>,
}

/// A bytes stream response.
#[derive(Clone)]
#[pyclass(subclass)]
pub struct Streamer(Arc<Mutex<Option<BoxStream<'static, wreq::Result<Bytes>>>>>);

// ===== impl Streamer =====

impl Streamer {
    /// Create a new [`Streamer`] instance.
    #[inline]
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> Streamer {
        Streamer(Arc::new(Mutex::new(Some(stream.boxed()))))
    }

    async fn next(self, error: fn() -> Error) -> PyResult<PyBuffer> {
        let val = self
            .0
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
                .block_on(self.clone().next(|| Error::StopIteration))
        })
    }

    #[inline]
    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            self.clone().next(|| Error::StopAsyncIteration),
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
                .map(StreamExt::boxed)
                .map(Mutex::new)
                .map(Arc::new)
                .map(StreamSource::Async)
                .map(|source| PyStream {
                    source,
                    pending: None,
                })
        } else {
            ob.extract::<Py<PyAny>>()
                .map(Arc::new)
                .map(StreamSource::Sync)
                .map(|source| PyStream {
                    source,
                    pending: None,
                })
                .map_err(Into::into)
        }
    }
}

impl Stream for PyStream {
    type Item = PyResult<Bytes>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        fn poll_or_create<F>(
            pending: &mut Pending,
            cx: &mut Context<'_>,
            create_task: F,
        ) -> Poll<Option<PyResult<Bytes>>>
        where
            F: FnOnce() -> JoinHandle<Option<PyResult<Bytes>>>,
        {
            if let Some(mut fut) = pending.take() {
                match fut.poll_unpin(cx) {
                    Poll::Ready(Ok(res)) => return Poll::Ready(res),
                    Poll::Ready(Err(_)) => return Poll::Ready(None),
                    Poll::Pending => {
                        *pending = Some(fut);
                        return Poll::Pending;
                    }
                }
            }

            let mut fut = create_task();
            match fut.poll_unpin(cx) {
                Poll::Ready(Ok(res)) => Poll::Ready(res),
                Poll::Ready(Err(_)) => Poll::Ready(None),
                Poll::Pending => {
                    *pending = Some(fut);
                    Poll::Pending
                }
            }
        }

        match &self.source {
            StreamSource::Sync(ob) => {
                let ob = ob.clone();
                poll_or_create(&mut self.get_mut().pending, cx, || {
                    pyo3_async_runtimes::tokio::get_runtime().spawn_blocking(move || {
                        Python::attach(|py| {
                            ob.call_method0(py, intern!(py, "__next__"))
                                .ok()
                                .map(|ob| extract_bytes(py, ob))
                        })
                    })
                })
            }
            StreamSource::Async(stream) => {
                let stream = stream.clone();
                poll_or_create(&mut self.get_mut().pending, cx, || {
                    pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                        let ob = stream.lock().await.next().await;
                        Handle::current()
                            .spawn_blocking(move || {
                                Python::attach(|py| ob.map(|ob| extract_bytes(py, ob)))
                            })
                            .await
                            .ok()?
                    })
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
