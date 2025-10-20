use std::sync::Arc;

use ::tokio::sync::oneshot;
use pyo3::{intern, prelude::*, sync::PyOnceLock};

static ASYNCIO: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static ENSURE_FUTURE: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static GET_RUNNING_LOOP: PyOnceLock<Py<PyAny>> = PyOnceLock::new();

#[inline]
pub fn ensure_future<'p>(
    py: Python<'p>,
    awaitable: &Bound<'p, PyAny>,
) -> PyResult<Bound<'p, PyAny>> {
    ENSURE_FUTURE
        .get_or_try_init(py, || -> PyResult<Py<PyAny>> {
            Ok(asyncio(py)?.getattr(intern!(py, "ensure_future"))?.into())
        })?
        .bind(py)
        .call1((awaitable,))
}

#[inline]
pub fn cancelled(future: &Bound<PyAny>) -> PyResult<bool> {
    future
        .getattr(intern!(future.py(), "cancelled"))?
        .call0()?
        .is_truthy()
}

#[inline]
pub fn asyncio(py: Python<'_>) -> PyResult<&Bound<'_, PyAny>> {
    ASYNCIO
        .get_or_try_init(py, || Ok(py.import("asyncio")?.into()))
        .map(|asyncio| asyncio.bind(py))
}

/// Task-local data to store for Python conversions.
#[derive(Debug, Clone)]
pub struct TaskLocals {
    /// Track the event loop of the Python task
    event_loop: Arc<Py<PyAny>>,
}

impl TaskLocals {
    /// At a minimum, TaskLocals must store the event loop.
    #[inline]
    pub fn new(event_loop: Bound<PyAny>) -> Self {
        Self {
            event_loop: Arc::new(event_loop.into()),
        }
    }

    /// Construct TaskLocals with the event loop returned by `get_running_loop`
    pub fn with_running_loop(py: Python) -> PyResult<Self> {
        // Ideally should call get_running_loop, but calls get_event_loop for compatibility when
        // get_running_loop is not available.
        GET_RUNNING_LOOP
            .get_or_try_init(py, || -> PyResult<Py<PyAny>> {
                let asyncio = asyncio(py)?;
                Ok(asyncio.getattr("get_running_loop")?.into())
            })?
            .bind(py)
            .call0()
            .map(Self::new)
    }

    /// Get a reference to the event loop
    #[inline]
    pub fn event_loop(&self) -> &Arc<Py<PyAny>> {
        &self.event_loop
    }
}

#[pyclass]
struct PyTaskSender {
    tx: Option<oneshot::Sender<PyResult<Py<PyAny>>>>,
}

#[pymethods]
impl PyTaskSender {
    #[pyo3(signature = (task))]
    pub fn __call__(&mut self, task: &Bound<PyAny>) -> PyResult<()> {
        let py = task.py();
        debug_assert!(task.call_method0(intern!(py, "done"))?.extract()?);
        let result = match task.call_method0(intern!(py, "result")) {
            Ok(val) => Ok(val.into()),
            Err(e) => Err(e),
        };

        // unclear to me whether or not this should be a panic or silent error.
        //
        // calling PyTaskCompleter twice should not be possible, but I don't think it really hurts
        // anything if it happens.
        if let Some(tx) = self.tx.take() {
            if tx.send(result).is_err() {
                // cancellation is not an error
            }
        }

        Ok(())
    }
}

#[pyclass]
struct PyFuture {
    awaitable: Py<PyAny>,
    tx: Option<oneshot::Sender<PyResult<Py<PyAny>>>>,
}

#[pymethods]
impl PyFuture {
    pub fn __call__(&mut self, py: Python) -> PyResult<()> {
        let task = ensure_future(py, self.awaitable.bind(py))?;
        let on_complete = PyTaskSender { tx: self.tx.take() };
        task.call_method1(intern!(py, "add_done_callback"), (on_complete,))?;
        Ok(())
    }
}
