use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;
use pyo3::prelude::*;

pin_project! {
    /// A future that allows Python threads to run while it is being polled or executed.
    #[project = AllowThreadsProj]
    pub enum AllowThreads<Fut, F> {
        Future {
            #[pin]
            inner: Fut,
        },
        Closure {
            inner: Option<F>,
        },
    }
}

impl<Fut> AllowThreads<Fut, ()>
where
    Fut: Future + Send,
    Fut::Output: Send,
{
    /// Create [`AllowThreads`] from a future
    #[inline(always)]
    pub fn future(future: Fut) -> Self {
        AllowThreads::Future { inner: future }
    }
}

impl<F, R> AllowThreads<(), F>
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    /// Create [`AllowThreads`] from a closure
    #[inline(always)]
    pub fn closure(closure: F) -> Self {
        AllowThreads::Closure {
            inner: Some(closure),
        }
    }
}

impl<A, B> AllowThreads<A, B> {
    /// Convert to a pinned box
    #[inline(always)]
    pub fn future_into_py<'py, T>(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>>
    where
        Self: Future<Output = PyResult<T>> + Send + 'static,
        T: for<'py2> IntoPyObject<'py2> + Send + 'static,
    {
        pyo3_async_runtimes::tokio::future_into_py(py, self)
    }
}

impl<Fut> Future for AllowThreads<Fut, ()>
where
    Fut: Future + Send,
    Fut::Output: Send,
{
    type Output = Fut::Output;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        Python::attach(|py| {
            py.detach(|| match self.project() {
                AllowThreadsProj::Future { inner } => inner.poll(&mut Context::from_waker(waker)),
                _ => unreachable!("Future variant should not contain Closure"),
            })
        })
    }
}

impl<F, R> Future for AllowThreads<(), F>
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    type Output = R;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Python::attach(|py| {
            py.detach(|| match self.project() {
                AllowThreadsProj::Closure { inner } => {
                    let res = inner.take().expect("Closure already executed")();
                    Poll::Ready(res)
                }
                _ => {
                    unreachable!("Closure variant should not contain Future")
                }
            })
        })
    }
}
