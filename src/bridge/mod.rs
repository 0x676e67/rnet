//! Rust Bindings to the Python Asyncio Event Loop

mod sync;
mod task;

use std::{cell::OnceCell, future::Future, pin::Pin, sync::LazyLock};

use futures::channel::{mpsc, oneshot};
use pyo3::{IntoPyObjectExt, call::PyCallArgs, intern, prelude::*, types::PyDict};
use sync::{Cancellable, PyDoneCallback, Sender};
use task::{TaskLocals, cancelled, create_future};
use tokio::runtime::{Builder, Runtime as TokioRuntime};

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
    fn scope<F, R>(locals: TaskLocals, fut: F) -> Pin<Box<dyn Future<Output = R> + Send>>
    where
        F: Future<Output = R> + Send + 'static,
    {
        let cell = OnceCell::new();
        cell.set(locals).unwrap();
        Box::pin(TASK_LOCALS.scope(cell, fut))
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
            Ok(TaskLocals::with_running_loop(py)?.copy_context(py)?)
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

    /// **This API is marked as unstable** and is only available when the
    /// `unstable-streams` crate feature is enabled. This comes with no
    /// stability guarantees, and could be changed or removed at any time.
    #[inline]
    pub fn into_stream(
        g: Bound<'_, PyAny>,
    ) -> PyResult<impl futures::Stream<Item = Py<PyAny>> + 'static> {
        into_stream_with_locals(Runtime::get_current_locals(g.py())?, g)
    }
}

fn set_result(
    py: Python,
    event_loop: Bound<PyAny>,
    future: &Bound<PyAny>,
    result: PyResult<Py<PyAny>>,
) -> PyResult<()> {
    let none = py.None().into_bound(py);
    let (complete, val) = match result {
        Ok(val) => (
            future.getattr(intern!(py, "set_result"))?,
            val.into_pyobject(py)?,
        ),
        Err(err) => (
            future.getattr(intern!(py, "set_exception"))?,
            err.into_bound_py_any(py)?,
        ),
    };
    call_soon_threadsafe(
        &event_loop,
        &none,
        (CheckedCompletor, future, complete, val),
    )?;

    Ok(())
}

fn call_soon_threadsafe<'py>(
    event_loop: &Bound<'py, PyAny>,
    context: &Bound<PyAny>,
    args: impl PyCallArgs<'py>,
) -> PyResult<()> {
    let py = event_loop.py();
    let kwargs = PyDict::new(py);
    kwargs.set_item(intern!(py, "context"), context)?;
    event_loop.call_method(intern!(py, "call_soon_threadsafe"), args, Some(&kwargs))?;
    Ok(())
}

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

#[allow(unused_must_use)]
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

    let py_fut = create_future(locals.event_loop.bind(py).clone())?;
    py_fut.call_method1(
        intern!(py, "add_done_callback"),
        (PyDoneCallback {
            cancel_tx: Some(cancel_tx),
        },),
    )?;

    let future_tx = py_fut.clone().unbind();
    let locals = locals.clone();

    Runtime::spawn(async move {
        // create a scope for the task locals
        let result = Runtime::scope(locals.clone(), Cancellable::new(fut, cancel_rx)).await;

        // spawn a blocking task to set the result of the future
        tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                if cancelled(future_tx.bind(py))
                    .map_err(dump_err(py))
                    .unwrap_or(false)
                {
                    return;
                }

                set_result(
                    py,
                    locals.event_loop(py),
                    future_tx.bind(py),
                    result.and_then(|val| val.into_py_any(py)),
                )
                .map_err(dump_err(py));
            });
        })
        .await
        .expect("tokio::task::spawn_blocking failed");
    });

    Ok(py_fut)
}

const STREAM_GLUE: &str = r#"
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
"#;

fn into_stream_with_locals(
    locals: TaskLocals,
    g: Bound<'_, PyAny>,
) -> PyResult<impl futures::Stream<Item = Py<PyAny>> + 'static> {
    use std::ffi::CString;

    use pyo3::sync::PyOnceLock;

    static GLUE_MOD: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
    let py = g.py();
    let glue = GLUE_MOD
        .get_or_try_init(py, || -> PyResult<Py<PyAny>> {
            PyModule::from_code(
                py,
                &CString::new(STREAM_GLUE).unwrap(),
                &CString::new("pyo3_async_runtimes/pyo3_async_runtimes_glue.py").unwrap(),
                &CString::new("pyo3_async_runtimes_glue").unwrap(),
            )
            .map(Into::into)
        })?
        .bind(py);

    let (tx, rx) = mpsc::channel(10);

    locals.event_loop(py).call_method1(
        intern!(py, "call_soon_threadsafe"),
        (
            locals.event_loop(py).getattr(intern!(py, "create_task"))?,
            glue.call_method1(intern!(py, "forward"), (g, Sender::new(locals, tx)))?,
        ),
    )?;
    Ok(rx)
}
