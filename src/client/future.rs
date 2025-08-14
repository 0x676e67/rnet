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
    pub fn new_future(future: Fut) -> Self {
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
    pub fn new_closure(closure: F) -> Self {
        AllowThreads::Closure {
            inner: Some(closure),
        }
    }
}

impl<Fut> Future for AllowThreads<Fut, ()>
where
    Fut: Future + Send,
    Fut::Output: Send,
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        Python::with_gil(|py| {
            py.allow_threads(|| match self.project() {
                AllowThreadsProj::Future { inner } => inner.poll(&mut Context::from_waker(waker)),
                AllowThreadsProj::Closure { .. } => {
                    unreachable!("Future variant should not contain Closure")
                }
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

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Python::with_gil(|py| {
            py.allow_threads(|| match self.project() {
                AllowThreadsProj::Closure { inner } => {
                    if let Some(closure) = inner.take() {
                        Poll::Ready(closure())
                    } else {
                        panic!("Closure already executed")
                    }
                }
                AllowThreadsProj::Future { .. } => {
                    unreachable!("Closure variant should not contain Future")
                }
            })
        })
    }
}
