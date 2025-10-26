//! Rust Bindings to the Python Asyncio Event Loop

mod sync;
mod task;

use std::{cell::OnceCell, ffi::CStr, future::Future, sync::LazyLock};

use futures_util::Stream;
use pyo3::{
    IntoPyObjectExt, call::PyCallArgs, ffi::c_str, intern, prelude::*, sync::PyOnceLock,
    types::PyDict,
};
use sync::{Cancellable, PyDoneCallback, ReceiverStream, Sender};
use task::{TaskLocals, cancelled};
use tokio::{
    runtime::{Builder, Runtime as TokioRuntime},
    sync::{mpsc, oneshot},
};

tokio::task_local! {
    /// Task-local storage for Python context (`TaskLocals`), used to propagate
    /// Python async context (such as the current event loop and contextvars)
    /// across Rust async boundaries. This is set when a Rust future is spawned
    /// from Python, ensuring that Python context is preserved for the duration
    /// of the task. It is initialized at the start of each task and should not
    /// be accessed outside of an async task context.
    static TASK_LOCALS: OnceCell<TaskLocals>;
}

/// The global Tokio runtime instance.
static TOKIO_RUNTIME: LazyLock<TokioRuntime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to build Tokio runtime")
});

/// A small runtime bridge that manages task-local context and exposes utilities
/// for converting Rust futures into Python awaitables.
///
/// This type wraps a global Tokio runtime and helpers to:
/// - install task-local `TaskLocals` for spawned tasks (`scope`)
/// - retrieve or create `TaskLocals` from the current Python context
/// - spawn and run futures on the global runtime
/// - convert Rust `Future<Output = PyResult<T>>` into Python `asyncio.Future` objects
pub struct Runtime;

impl Runtime {
    /// Set the task locals for the given future
    fn scope<F, R>(locals: TaskLocals, fut: F) -> impl Future<Output = R> + Send
    where
        F: Future<Output = R> + Send + 'static,
    {
        let cell = OnceCell::new();
        cell.set(locals).unwrap();
        TASK_LOCALS.scope(cell, fut)
    }

    /// Get the task locals for the current task
    fn get_task_locals() -> Option<TaskLocals> {
        TASK_LOCALS
            .try_with(|c| c.get().cloned())
            .unwrap_or_default()
    }

    /// Either copy the task locals from the current task OR get the current running loop and
    /// contextvars from Python.
    fn get_current_locals<'py>(py: Python<'py>) -> PyResult<TaskLocals> {
        if let Some(locals) = Self::get_task_locals() {
            Ok(locals)
        } else {
            TaskLocals::with_running_loop(py)
        }
    }

    /// Spawn a future onto the runtime
    #[inline]
    pub fn spawn<F>(fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        TOKIO_RUNTIME.spawn(fut);
    }

    /// Spawn a blocking function onto the runtime
    #[inline]
    pub fn spawn_blocking<F, R>(f: F)
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        TOKIO_RUNTIME.spawn_blocking(f);
    }

    /// Block on a future using the runtime
    #[inline]
    pub fn block_on<F, R>(fut: F) -> R
    where
        F: Future<Output = R>,
    {
        TOKIO_RUNTIME.block_on(fut)
    }

    /// Convert a Rust Future into a Python awaitable with a generic runtime
    #[inline]
    pub fn future_into_py<F, T>(py: Python, fut: F) -> PyResult<Bound<PyAny>>
    where
        F: Future<Output = PyResult<T>> + Send + 'static,
        T: for<'py> IntoPyObject<'py> + Send + 'static,
    {
        future_into_py_with_locals::<F, T>(py, Runtime::get_current_locals(py)?, fut)
    }

    /// Convert a Python async generator into a Rust Stream with a generic runtime
    #[inline]
    pub fn into_stream(g: Borrowed<PyAny>) -> PyResult<impl Stream<Item = Py<PyAny>> + 'static> {
        into_stream_with_locals(Runtime::get_current_locals(g.py())?, g)
    }
}

fn set_result<T>(
    py: Python,
    event_loop: &Bound<PyAny>,
    future: &Bound<PyAny>,
    result: PyResult<T>,
) -> PyResult<()>
where
    T: for<'py> IntoPyObject<'py> + Send + 'static,
{
    let context = py.None().into_bound(py);
    let (complete, val) = match result {
        Ok(val) => (
            future.getattr(intern!(py, "set_result"))?,
            val.into_bound_py_any(py)?,
        ),
        Err(err) => (
            future.getattr(intern!(py, "set_exception"))?,
            err.into_bound_py_any(py)?,
        ),
    };

    call_soon_threadsafe(
        event_loop,
        context,
        (CheckedCompletor, future, complete, val),
    )
}

fn call_soon_threadsafe<'py>(
    event_loop: &Bound<'py, PyAny>,
    context: Bound<PyAny>,
    args: impl PyCallArgs<'py>,
) -> PyResult<()> {
    let py = event_loop.py();
    let kwargs = PyDict::new(py);
    kwargs.set_item(intern!(py, "context"), context)?;
    event_loop.call_method(intern!(py, "call_soon_threadsafe"), args, Some(&kwargs))?;
    Ok(())
}

#[inline]
fn dump_err(py: Python<'_>) -> impl FnOnce(PyErr) + '_ {
    move |e| {
        // We can't display Python exceptions via std::fmt::Display,
        // so print the error here manually.
        e.print_and_set_sys_last_vars(py);
    }
}

#[pyclass]
struct CheckedCompletor;

#[pymethods]
impl CheckedCompletor {
    fn __call__(
        &self,
        future: &Bound<PyAny>,
        complete: &Bound<PyAny>,
        value: &Bound<PyAny>,
    ) -> PyResult<()> {
        if cancelled(future)? {
            return Ok(());
        }

        complete.call1((value,))?;
        Ok(())
    }
}

fn future_into_py_with_locals<F, T>(
    py: Python,
    locals: TaskLocals,
    fut: F,
) -> PyResult<Bound<PyAny>>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: for<'py> IntoPyObject<'py> + Send + 'static,
{
    let (cancel_tx, cancel_rx) = oneshot::channel();

    // Create the asyncio Future while holding the GIL briefly.
    let py_fut = locals
        .event_loop()
        .bind(py)
        .call_method0(intern!(py, "create_future"))?;
    py_fut.call_method1(
        intern!(py, "add_done_callback"),
        (PyDoneCallback {
            cancel_tx: Some(cancel_tx),
        },),
    )?;

    // Take an owned handle to the future so it can be moved to other threads.
    let future_tx = py_fut.clone().unbind();

    py.detach(|| {
        Runtime::spawn(async move {
            let event_loop = locals.event_loop().clone();

            // create a scope for the task locals
            let result = Runtime::scope(locals, Cancellable::new(fut, cancel_rx)).await;

            // spawn a blocking task to set the result of the future. We re-acquire
            // the GIL only inside `Python::attach`, bind the owned handles, and
            // perform a short-lived call to set the result.
            Runtime::spawn_blocking(move || {
                Python::attach(|py| {
                    let future_tx = future_tx.bind(py);
                    if cancelled(future_tx).map_err(dump_err(py)).unwrap_or(false) {
                        return;
                    }

                    // bind the owned event loop and use it to schedule the callback
                    let event_loop = event_loop.bind(py);
                    #[allow(unused_must_use)]
                    set_result(py, event_loop, future_tx, result).map_err(dump_err(py));
                })
            })
        })
    });

    Ok(py_fut)
}

const STREAM_GLUE: &CStr = c_str!(
    r#"
import asyncio

async def forward(gen, sender):
    async for item in gen:
        should_continue = sender.send(item)

        if asyncio.iscoroutine(should_continue):
            should_continue = await should_continue

        if should_continue:
            continue
        else:
            break

    sender.close()
"#
);
const FILE_NAME: &CStr = c_str!("pyo3_async_runtimes/pyo3_async_runtimes_glue.py");
const MODULE_NAME: &CStr = c_str!("pyo3_async_runtimes_glue");

fn into_stream_with_locals(
    locals: TaskLocals,
    g: Borrowed<PyAny>,
) -> PyResult<impl Stream<Item = Py<PyAny>> + 'static> {
    static GLUE_MOD: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
    let py = g.py();
    let glue = GLUE_MOD
        .get_or_try_init(py, || {
            PyModule::from_code(py, STREAM_GLUE, FILE_NAME, MODULE_NAME).map(Into::into)
        })?
        .bind(py);

    let (tx, rx) = mpsc::channel(10);

    locals
        .event_loop()
        .clone_ref(py)
        .into_bound(py)
        .call_method1(
            intern!(py, "call_soon_threadsafe"),
            (
                locals
                    .event_loop()
                    .bind(py)
                    .getattr(intern!(py, "create_task"))?,
                glue.call_method1(intern!(py, "forward"), (g, Sender::new(locals, tx)))?,
            ),
        )?;

    py.detach(|| Ok(ReceiverStream::new(rx)))
}
