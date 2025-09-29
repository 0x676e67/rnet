use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;
use pyo3::prelude::*;

use crate::bridge::Runtime;

pin_project! {
    /// A future that allows Python threads to run while it is being polled or executed.
    #[project = NoGILProj]
    pub enum NoGIL<Fut, F> {
        Future {
            #[pin]
            inner: Fut,
        },
        Closure {
            inner: Option<F>,
        },
    }
}

impl<Fut, T> NoGIL<Fut, ()>
where
    Fut: Future<Output = PyResult<T>> + Send + 'static,
    T: Send + for<'py> IntoPyObject<'py> + 'static,
{
    #[inline(always)]
    pub fn future<'py>(py: Python<'py>, future: Fut) -> PyResult<Bound<'py, PyAny>> {
        Runtime::future_into_py(py, NoGIL::Future { inner: future })
    }
}

impl<F, R> NoGIL<(), F>
where
    F: FnOnce() -> Result<R, PyErr> + Send + 'static,
    R: Send + for<'py> IntoPyObject<'py> + 'static,
{
    #[inline(always)]
    pub fn closure<'py>(py: Python<'py>, closure: F) -> PyResult<Bound<'py, PyAny>> {
        Runtime::future_into_py(
            py,
            NoGIL::Closure {
                inner: Some(closure),
            },
        )
    }
}

impl<Fut> Future for NoGIL<Fut, ()>
where
    Fut: Future + Send,
    Fut::Output: Send,
{
    type Output = Fut::Output;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        match self.project() {
            NoGILProj::Future { inner } => inner.poll(&mut Context::from_waker(waker)),
            _ => unreachable!("Future variant should not contain Closure"),
        }
    }
}

impl<F, R> Future for NoGIL<(), F>
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    type Output = R;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Python::attach(|py| {
            py.detach(|| match self.project() {
                NoGILProj::Closure { inner } => {
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
