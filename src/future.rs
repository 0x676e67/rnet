use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;
use pyo3::prelude::*;

use crate::rt::Runtime;

pin_project! {
    /// Async future wrapper
    pub struct AsyncFuture<Fut> {
        #[pin]
        inner: Fut,
    }
}

pin_project! {
    /// Blocking closure wrapper
    pub struct BlockingFuture<F> {
        inner: Option<F>,
    }
}

impl<Fut, T> AsyncFuture<Fut>
where
    Fut: Future<Output = PyResult<T>> + Send + 'static,
    T: Send + for<'py> IntoPyObject<'py> + 'static,
{
    #[inline(always)]
    pub fn future_into_py<'py>(py: Python<'py>, future: Fut) -> PyResult<Bound<'py, PyAny>> {
        Runtime::future_into_py(py, Self { inner: future })
    }
}

impl<F, R> BlockingFuture<F>
where
    F: FnOnce() -> Result<R, PyErr> + Send + 'static,
    R: Send + for<'py> IntoPyObject<'py> + 'static,
{
    #[inline(always)]
    pub fn future_into_py<'py>(py: Python<'py>, closure: F) -> PyResult<Bound<'py, PyAny>> {
        Runtime::future_into_py(
            py,
            Self {
                inner: Some(closure),
            },
        )
    }
}

impl<Fut> Future for AsyncFuture<Fut>
where
    Fut: Future + Send,
    Fut::Output: Send,
{
    type Output = Fut::Output;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        Python::attach(|py| {
            py.detach(|| self.project().inner.poll(&mut Context::from_waker(waker)))
        })
    }
}

impl<F, R> Future for BlockingFuture<F>
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    type Output = R;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let closure = self
            .project()
            .inner
            .take()
            .expect("Closure already executed");
        Poll::Ready(closure())
    }
}
