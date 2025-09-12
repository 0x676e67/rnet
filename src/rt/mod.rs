//! Rust Bindings to the Python Asyncio Event Loop

mod err;
mod generic;
pub mod tokio;

#[pymodule]
fn pyo3_async_runtimes(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("RustPanic", py.get_type::<err::RustPanic>())?;
    Ok(())
}

use std::sync::Arc;

use ::tokio::sync::oneshot;
use pyo3::{call::PyCallArgs, prelude::*, sync::PyOnceLock, types::PyDict};

static ASYNCIO: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static CONTEXTVARS: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static ENSURE_FUTURE: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
static GET_RUNNING_LOOP: PyOnceLock<Py<PyAny>> = PyOnceLock::new();

#[inline]
fn ensure_future<'p>(py: Python<'p>, awaitable: &Bound<'p, PyAny>) -> PyResult<Bound<'p, PyAny>> {
    ENSURE_FUTURE
        .get_or_try_init(py, || -> PyResult<Py<PyAny>> {
            Ok(asyncio(py)?.getattr("ensure_future")?.into())
        })?
        .bind(py)
        .call1((awaitable,))
}

#[inline]
fn create_future(event_loop: Bound<'_, PyAny>) -> PyResult<Bound<'_, PyAny>> {
    event_loop.call_method0("create_future")
}

#[inline]
fn asyncio(py: Python<'_>) -> PyResult<&Bound<'_, PyAny>> {
    ASYNCIO
        .get_or_try_init(py, || Ok(py.import("asyncio")?.into()))
        .map(|asyncio| asyncio.bind(py))
}

/// Task-local data to store for Python conversions.
#[derive(Debug, Clone)]
pub struct TaskLocals {
    /// Track the event loop of the Python task
    event_loop: Arc<Py<PyAny>>,
    /// Track the contextvars of the Python task
    _context: Arc<Py<PyAny>>,
}

impl TaskLocals {
    /// At a minimum, TaskLocals must store the event loop.
    pub fn new(event_loop: Bound<PyAny>) -> Self {
        Self {
            _context: Arc::new(event_loop.py().None()),
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

    /// Manually provide the contextvars for the current task.
    pub fn with_context(self, context: Bound<PyAny>) -> Self {
        Self {
            _context: Arc::new(context.into()),
            ..self
        }
    }

    /// Capture the current task's contextvars
    pub fn copy_context(self, py: Python) -> PyResult<Self> {
        let copy_context = CONTEXTVARS
            .get_or_try_init(py, || py.import("contextvars").map(|m| m.into()))?
            .bind(py)
            .call_method0("copy_context")?;
        Ok(self.with_context(copy_context))
    }

    /// Get a reference to the event loop
    pub fn event_loop<'p>(&self, py: Python<'p>) -> Bound<'p, PyAny> {
        self.event_loop.clone_ref(py).into_bound(py)
    }
}

#[pyclass]
struct PyTaskCompleter {
    tx: Option<oneshot::Sender<PyResult<Py<PyAny>>>>,
}

#[pymethods]
impl PyTaskCompleter {
    #[pyo3(signature = (task))]
    pub fn __call__(&mut self, task: &Bound<PyAny>) -> PyResult<()> {
        debug_assert!(task.call_method0("done")?.extract()?);
        let result = match task.call_method0("result") {
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
struct PyEnsureFuture {
    awaitable: Py<PyAny>,
    tx: Option<oneshot::Sender<PyResult<Py<PyAny>>>>,
}

#[pymethods]
impl PyEnsureFuture {
    pub fn __call__(&mut self) -> PyResult<()> {
        Python::attach(|py| {
            let task = ensure_future(py, self.awaitable.bind(py))?;
            let on_complete = PyTaskCompleter { tx: self.tx.take() };
            task.call_method1("add_done_callback", (on_complete,))?;

            Ok(())
        })
    }
}

fn call_soon_threadsafe<'py>(
    event_loop: &Bound<'py, PyAny>,
    context: &Bound<PyAny>,
    args: impl PyCallArgs<'py>,
) -> PyResult<()> {
    let py = event_loop.py();

    let kwargs = PyDict::new(py);
    kwargs.set_item("context", context)?;
    event_loop.call_method("call_soon_threadsafe", args, Some(&kwargs))?;
    Ok(())
}

fn dump_err(py: Python<'_>) -> impl FnOnce(PyErr) + '_ {
    move |e| {
        // We can't display Python exceptions via std::fmt::Display,
        // so print the error here manually.
        e.print_and_set_sys_last_vars(py);
    }
}
