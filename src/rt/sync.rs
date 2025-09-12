use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{
    SinkExt,
    channel::{mpsc, oneshot},
};
use pin_project_lite::pin_project;
use pyo3::{IntoPyObjectExt, prelude::*};

use super::{
    future_into_py_with_locals,
    task::{TaskLocals, cancelled},
    util::dump_err,
};

pin_project! {
    /// Future returned by [`timeout`](timeout) and [`timeout_at`](timeout_at).
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[derive(Debug)]
    pub struct Cancellable<T> {
        #[pin]
        future: T,
        #[pin]
        cancel_rx: oneshot::Receiver<()>,
        poll_cancel_rx: bool
    }
}

impl<T> Cancellable<T> {
    #[inline]
    pub fn new(future: T, cancel_rx: oneshot::Receiver<()>) -> Self {
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
pub struct PyDoneCallback {
    pub cancel_tx: Option<oneshot::Sender<()>>,
}

#[pymethods]
impl PyDoneCallback {
    pub fn __call__(&mut self, fut: &Bound<PyAny>) -> PyResult<()> {
        if cancelled(fut).map_err(dump_err(fut.py())).unwrap_or(false) {
            let _ = self.cancel_tx.take().unwrap().send(());
        }

        Ok(())
    }
}

#[pyclass]
pub struct Sender {
    locals: TaskLocals,
    tx: mpsc::Sender<Py<PyAny>>,
}

impl Sender {
    #[inline]
    pub fn new(locals: TaskLocals, tx: mpsc::Sender<Py<PyAny>>) -> Sender {
        Sender { locals, tx }
    }
}

#[pymethods]
impl Sender {
    pub fn send(&mut self, py: Python, item: Py<PyAny>) -> PyResult<Py<PyAny>> {
        match self.tx.try_send(item.clone_ref(py)) {
            Ok(_) => true.into_py_any(py),
            Err(e) => {
                if e.is_full() {
                    let mut tx = self.tx.clone();
                    future_into_py_with_locals::<_, bool>(py, self.locals.clone(), async move {
                        if tx.flush().await.is_err() {
                            return Ok(false);
                        }
                        if tx.send(item).await.is_err() {
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
    pub fn close(&mut self) -> PyResult<()> {
        self.tx.close_channel();
        Ok(())
    }
}
