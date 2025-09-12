use std::{cell::OnceCell, future::Future, pin::Pin, sync::OnceLock};

use ::tokio::{
    runtime::{Builder, Runtime},
    task,
};
use futures_util::Stream;
use pyo3::prelude::*;

use super::{
    TaskLocals,
    generic::{self, ContextExt, Runtime as GenericRuntime},
};

static TOKIO_RUNTIME: OnceLock<Runtime> = OnceLock::new();

struct TokioRuntime;

tokio::task_local! {
    static TASK_LOCALS: OnceCell<TaskLocals>;
}

impl GenericRuntime for TokioRuntime {
    type JoinError = task::JoinError;
    type JoinHandle = task::JoinHandle<()>;

    #[inline]
    fn spawn<F>(fut: F) -> Self::JoinHandle
    where
        F: Future<Output = ()> + Send + 'static,
    {
        get_runtime().spawn(async move {
            fut.await;
        })
    }

    #[inline]
    fn spawn_blocking<F>(f: F) -> Self::JoinHandle
    where
        F: FnOnce() + Send + 'static,
    {
        get_runtime().spawn_blocking(f)
    }
}

impl ContextExt for TokioRuntime {
    fn scope<F, R>(locals: TaskLocals, fut: F) -> Pin<Box<dyn Future<Output = R> + Send>>
    where
        F: Future<Output = R> + Send + 'static,
    {
        let cell = OnceCell::new();
        cell.set(locals).unwrap();
        Box::pin(TASK_LOCALS.scope(cell, fut))
    }

    fn get_task_locals() -> Option<TaskLocals> {
        TASK_LOCALS
            .try_with(|c| c.get().map(Clone::clone))
            .unwrap_or_default()
    }
}

/// Get a reference to the current tokio runtime
#[inline]
pub fn get_runtime<'a>() -> &'a Runtime {
    TOKIO_RUNTIME.get_or_init(|| {
        Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Unable to build Tokio runtime")
    })
}

/// Convert a Rust Future into a Python awaitable
///
/// If the `asyncio.Future` returned by this conversion is cancelled via `asyncio.Future.cancel`,
/// the Rust future will be cancelled as well (new behaviour in `v0.15`).
///
/// Python `contextvars` are preserved when calling async Python functions within the Rust future
/// via [`into_future`] (new behaviour in `v0.15`).
///
/// > Although `contextvars` are preserved for async Python functions, synchronous functions will
/// > unfortunately fail to resolve them when called within the Rust future. This is because the
/// > function is being called from a Rust thread, not inside an actual Python coroutine context.
/// >
/// > As a workaround, you can get the `contextvars` from the current task locals using
/// > [`get_current_locals`] and [`TaskLocals::context`](`crate::TaskLocals::context`), then wrap
/// > your
/// > synchronous function in a call to `contextvars.Context.run`. This will set the context, call
/// > the
/// > synchronous function, and restore the previous context when it returns or raises an exception.
///
/// # Arguments
/// * `py` - The current PyO3 GIL guard
/// * `fut` - The Rust future to be converted
#[inline]
pub fn future_into_py<F, T>(py: Python, fut: F) -> PyResult<Bound<PyAny>>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: for<'py> IntoPyObject<'py> + Send + 'static,
{
    generic::future_into_py::<TokioRuntime, _, T>(py, fut)
}

/// <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>unstable-streams</code></span> Convert an async generator into a stream
///
/// **This API is marked as unstable** and is only available when the
/// `unstable-streams` crate feature is enabled. This comes with no
/// stability guarantees, and could be changed or removed at any time.
///
/// # Arguments
/// * `gen` - The Python async generator to be converted
#[inline]
pub fn into_stream_v2(
    r#gen: Bound<'_, PyAny>,
) -> PyResult<impl Stream<Item = Py<PyAny>> + 'static> {
    generic::into_stream_v2::<TokioRuntime>(r#gen)
}
