//! Generic implementations of PyO3 Asyncio utilities that can be used for any Rust runtime

use std::{
    fmt::Debug,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures::{
    SinkExt,
    channel::{mpsc, oneshot},
};
use pin_project_lite::pin_project;
use pyo3::{IntoPyObjectExt, prelude::*};

use super::{TaskLocals, call_soon_threadsafe, create_future, dump_err};

/// Generic Rust async/await runtime
pub trait Runtime: Send + 'static {
    /// The error returned by a JoinHandle after being awaited
    type JoinError: Send + Debug;
    /// A future that completes with the result of the spawned task
    type JoinHandle: Future<Output = Result<(), Self::JoinError>> + Send;

    /// Spawn a future onto this runtime's event loop
    fn spawn<F>(fut: F) -> Self::JoinHandle
    where
        F: Future<Output = ()> + Send + 'static;

    /// Spawn a function onto this runtime's blocking event loop
    fn spawn_blocking<F>(f: F) -> Self::JoinHandle
    where
        F: FnOnce() + Send + 'static;
}

/// Exposes the utilities necessary for using task-local data in the Runtime
pub trait ContextExt: Runtime {
    /// Set the task locals for the given future
    fn scope<F, R>(locals: TaskLocals, fut: F) -> Pin<Box<dyn Future<Output = R> + Send>>
    where
        F: Future<Output = R> + Send + 'static;

    /// Get the task locals for the current task
    fn get_task_locals() -> Option<TaskLocals>;
}

/// Either copy the task locals from the current task OR get the current running loop and
/// contextvars from Python.
pub fn get_current_locals<'py, R>(py: Python<'py>) -> PyResult<TaskLocals>
where
    R: ContextExt,
{
    if let Some(locals) = R::get_task_locals() {
        Ok(locals)
    } else {
        Ok(TaskLocals::with_running_loop(py)?.copy_context(py)?)
    }
}

#[inline]
fn cancelled(future: &Bound<PyAny>) -> PyResult<bool> {
    future.getattr("cancelled")?.call0()?.is_truthy()
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

fn set_result(
    py: Python,
    event_loop: Bound<PyAny>,
    future: &Bound<PyAny>,
    result: PyResult<Py<PyAny>>,
) -> PyResult<()> {
    let none = py.None().into_bound(py);

    let (complete, val) = match result {
        Ok(val) => (future.getattr("set_result")?, val.into_pyobject(py)?),
        Err(err) => (future.getattr("set_exception")?, err.into_bound_py_any(py)?),
    };
    call_soon_threadsafe(
        &event_loop,
        &none,
        (CheckedCompletor, future, complete, val),
    )?;

    Ok(())
}

/// Convert a Rust Future into a Python awaitable with a generic runtime
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
/// * `py` - PyO3 GIL guard
/// * `locals` - The task-local data for Python
/// * `fut` - The Rust future to be converted
#[allow(unused_must_use)]
pub fn future_into_py_with_locals<R, F, T>(
    py: Python,
    locals: TaskLocals,
    fut: F,
) -> PyResult<Bound<PyAny>>
where
    R: Runtime + ContextExt,
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: for<'py> IntoPyObject<'py> + Send + 'static,
{
    let (cancel_tx, cancel_rx) = oneshot::channel();

    let py_fut = create_future(locals.event_loop.bind(py).clone())?;
    py_fut.call_method1(
        "add_done_callback",
        (PyDoneCallback {
            cancel_tx: Some(cancel_tx),
        },),
    )?;

    let future_tx = py_fut.clone().unbind();
    let locals = locals.clone();

    R::spawn(async move {
        // create a scope for the task locals
        let result = R::scope(
            locals.clone(),
            Cancellable::new_with_cancel_rx(fut, cancel_rx),
        )
        .await;

        // spawn a blocking task to set the result of the future
        R::spawn_blocking(move || {
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
        .expect("Runtime::spawn_blocking failed");
    });

    Ok(py_fut)
}

pin_project! {
    /// Future returned by [`timeout`](timeout) and [`timeout_at`](timeout_at).
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[derive(Debug)]
    struct Cancellable<T> {
        #[pin]
        future: T,
        #[pin]
        cancel_rx: oneshot::Receiver<()>,
        poll_cancel_rx: bool
    }
}

impl<T> Cancellable<T> {
    #[inline]
    fn new_with_cancel_rx(future: T, cancel_rx: oneshot::Receiver<()>) -> Self {
        Self {
            future,
            cancel_rx,
            poll_cancel_rx: true,
        }
    }
}

impl<'py, F, T> Future for Cancellable<F>
where
    F: Future<Output = PyResult<T>>,
    T: IntoPyObject<'py>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        // First, try polling the future
        if let Poll::Ready(v) = this.future.poll(cx) {
            return Poll::Ready(v);
        }

        // Now check for cancellation
        if *this.poll_cancel_rx {
            match this.cancel_rx.poll(cx) {
                Poll::Ready(Ok(())) => {
                    *this.poll_cancel_rx = false;
                    // The python future has already been cancelled,
                    // so this return value will never be used.
                    Poll::Ready(Err(pyo3::exceptions::PyBaseException::new_err(
                        "unreachable",
                    )))
                }
                Poll::Ready(Err(_)) => {
                    *this.poll_cancel_rx = false;
                    Poll::Pending
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Pending
        }
    }
}

#[pyclass]
struct PyDoneCallback {
    cancel_tx: Option<oneshot::Sender<()>>,
}

#[pymethods]
impl PyDoneCallback {
    pub fn __call__(&mut self, fut: &Bound<PyAny>) -> PyResult<()> {
        let py = fut.py();

        if cancelled(fut).map_err(dump_err(py)).unwrap_or(false) {
            let _ = self.cancel_tx.take().unwrap().send(());
        }

        Ok(())
    }
}

/// Convert a Rust Future into a Python awaitable with a generic runtime
#[inline]
pub fn future_into_py<R, F, T>(py: Python, fut: F) -> PyResult<Bound<PyAny>>
where
    R: Runtime + ContextExt,
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: for<'py> IntoPyObject<'py> + Send + 'static,
{
    future_into_py_with_locals::<R, F, T>(py, get_current_locals::<R>(py)?, fut)
}

trait Sender: Send + 'static {
    fn send(&mut self, py: Python, locals: TaskLocals, item: Py<PyAny>) -> PyResult<Py<PyAny>>;
    fn close(&mut self) -> PyResult<()>;
}

struct GenericSender<R>
where
    R: Runtime,
{
    runtime: PhantomData<R>,
    tx: mpsc::Sender<Py<PyAny>>,
}

impl<R> Sender for GenericSender<R>
where
    R: Runtime + ContextExt,
{
    fn send(&mut self, py: Python, locals: TaskLocals, item: Py<PyAny>) -> PyResult<Py<PyAny>> {
        match self.tx.try_send(item.clone_ref(py)) {
            Ok(_) => true.into_py_any(py),
            Err(e) => {
                if e.is_full() {
                    let mut tx = self.tx.clone();

                    future_into_py_with_locals::<R, _, bool>(py, locals, async move {
                        if tx.flush().await.is_err() {
                            // receiving side disconnected
                            return Ok(false);
                        }
                        if tx.send(item).await.is_err() {
                            // receiving side disconnected
                            return Ok(false);
                        }
                        Ok(true)
                    })
                    .map(Bound::unbind)
                } else {
                    false.into_py_any(py)
                }
            }
        }
    }
    fn close(&mut self) -> PyResult<()> {
        self.tx.close_channel();
        Ok(())
    }
}

#[pyclass]
struct SenderGlue {
    locals: TaskLocals,
    tx: Arc<Mutex<dyn Sender>>,
}

#[pymethods]
impl SenderGlue {
    pub fn send(&mut self, item: Py<PyAny>) -> PyResult<Py<PyAny>> {
        Python::attach(|py| self.tx.lock().unwrap().send(py, self.locals.clone(), item))
    }
    pub fn close(&mut self) -> PyResult<()> {
        self.tx.lock().unwrap().close()
    }
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

/// <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>unstable-streams</code></span> Convert an async generator into a stream
///
/// **This API is marked as unstable** and is only available when the
/// `unstable-streams` crate feature is enabled. This comes with no
/// stability guarantees, and could be changed or removed at any time.
///
/// # Arguments
/// * `locals` - The current task locals
/// * `g` - The Python async generator to be converted
pub fn into_stream_with_locals_v2<R>(
    locals: TaskLocals,
    g: Bound<'_, PyAny>,
) -> PyResult<impl futures::Stream<Item = Py<PyAny>> + 'static>
where
    R: Runtime + ContextExt,
{
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
        "call_soon_threadsafe",
        (
            locals.event_loop(py).getattr("create_task")?,
            glue.call_method1(
                "forward",
                (
                    g,
                    SenderGlue {
                        locals,
                        tx: Arc::new(Mutex::new(GenericSender {
                            runtime: PhantomData::<R>,
                            tx,
                        })),
                    },
                ),
            )?,
        ),
    )?;
    Ok(rx)
}

/// <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>unstable-streams</code></span> Convert an async generator into a stream
///
/// **This API is marked as unstable** and is only available when the
/// `unstable-streams` crate feature is enabled. This comes with no
/// stability guarantees, and could be changed or removed at any time.
///
/// # Arguments
/// * `g` - The Python async generator to be converted
#[inline]
pub fn into_stream_v2<R>(
    g: Bound<'_, PyAny>,
) -> PyResult<impl futures::Stream<Item = Py<PyAny>> + 'static>
where
    R: Runtime + ContextExt,
{
    into_stream_with_locals_v2::<R>(get_current_locals::<R>(g.py())?, g)
}
