use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;
use pyo3::{
    coroutine::CancelHandle,
    exceptions::{PyRuntimeError, asyncio::CancelledError},
    prelude::*,
};
use tokio::task::JoinHandle;

pin_project! {
    /// A future that allows Python threads to run while it is being polled or executed.
    /// It also handles cancellation and spawns the task in tokio runtime.
    pub struct AllowThreads<T> {
        #[pin]
        handle: JoinHandle<PyResult<T>>,
    }
}

impl<T> AllowThreads<T>
where
    T: Send + 'static,
{
    /// Create [`AllowThreads`] from a future
    #[inline]
    pub fn new<Fut>(fut: Fut, mut cancel: CancelHandle) -> Self
    where
        Fut: Future<Output = PyResult<T>> + Send + 'static,
    {
        Self { handle:  pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
            tokio::select! {
                result = fut => result,
                _ = cancel.cancelled() => Err(CancelledError::new_err("Operation was cancelled")),
            }
        }) }
    }
}

impl<T> Future for AllowThreads<T>
where
    T: Send + 'static,
{
    type Output = PyResult<T>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        Python::attach(|py| {
            py.detach(
                || match self.project().handle.poll(&mut Context::from_waker(waker)) {
                    Poll::Ready(Ok(result)) => Poll::Ready(result),
                    Poll::Ready(Err(e)) => Poll::Ready(Err(PyRuntimeError::new_err(e.to_string()))),
                    Poll::Pending => Poll::Pending,
                },
            )
        })
    }
}
