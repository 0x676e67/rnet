use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;
use pyo3::prelude::*;

use crate::rt::Runtime;

pin_project! {
     /// A future that can be either a Rust Future to be executed in a thread pool.
    pub struct PyFuture<Fut> {
        #[pin]
        inner: Fut,
    }
}

impl<Fut, T> PyFuture<Fut>
where
    Fut: Future<Output = PyResult<T>> + Send + 'static,
    T: Send + for<'py> IntoPyObject<'py> + 'static,
{
    #[inline(always)]
    pub fn future_into_py<'py>(py: Python<'py>, future: Fut) -> PyResult<Bound<'py, PyAny>> {
        Runtime::future_into_py(py, Self { inner: future })
    }
}

impl<Fut> Future for PyFuture<Fut>
where
    Fut: Future + Send,
    Fut::Output: Send,
{
    type Output = Fut::Output;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}
