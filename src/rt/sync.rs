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
use pyo3::{IntoPyObjectExt, exceptions::PyBaseException, prelude::*};

use super::{
    dump_err, future_into_py_with_locals,
    task::{TaskLocals, cancelled},
};

pin_project! {
    /// A cancellable future wrapper.
    ///
    /// This wraps an inner future and a oneshot cancellation receiver.
    /// The wrapper will poll the inner future normally, but will also
    /// observe the cancellation receiver and complete early if a cancel
    /// notification is received.
    ///
    /// Typical use: run a Rust future that should be aborted when the
    /// corresponding Python `asyncio.Future` is cancelled.
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
    /// Create a new cancellable wrapper.
    ///
    /// `future` is the inner future to drive. `cancel_rx` is a oneshot
    /// receiver that will resolve when cancellation is requested.
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

    /// Poll the inner future and also check for cancellation.
    ///
    /// If the inner future completes, its result is returned. If the
    /// cancellation receiver resolves first, the wrapper returns an
    /// error indicating cancellation (this path is generally unreachable
    /// in normal use because the Python side will observe cancellation).
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
                    Poll::Ready(Err(PyBaseException::new_err("unreachable")))
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

/// Python-callable callback used to notify Rust of Python future completion.
///
/// Instances carry an optional oneshot sender that will be fired when the
/// Python future is observed to be cancelled. The callback is intended to be
/// added as a done-callback on an `asyncio.Future`.
#[pyclass]
pub struct PyDoneCallback {
    pub cancel_tx: Option<oneshot::Sender<()>>,
}

#[pymethods]
impl PyDoneCallback {
    /// Called by Python when the associated Future is done.
    ///
    /// If the Python future is cancelled, sends a cancellation signal on the
    /// internal oneshot sender so the Rust side can abort the corresponding task.
    pub fn __call__(&mut self, fut: &Bound<PyAny>) -> PyResult<()> {
        if cancelled(fut).map_err(dump_err(fut.py())).unwrap_or(false) {
            if let Some(tx) = self.cancel_tx.take() {
                let _ = tx.send(());
            }
        }

        Ok(())
    }
}

/// A sending endpoint that forwards Python-owned objects into a Rust async mpsc channel.
///
/// [`Sender`] holds the task-local context (`TaskLocals`) and a [`futures::mpsc::Sender`]
/// of owned `Py<PyAny>` objects which are safe to move across threads.
#[pyclass]
pub struct Sender {
    locals: TaskLocals,
    tx: mpsc::Sender<Py<PyAny>>,
}

impl Sender {
    /// Construct a new [`Sender`] with the given task locals and channel sender.
    #[inline]
    pub fn new(locals: TaskLocals, tx: mpsc::Sender<Py<PyAny>>) -> Sender {
        Sender { locals, tx }
    }
}

#[pymethods]
impl Sender {
    /// Send an item into the channel.
    ///
    /// This method first attempts a non-blocking `try_send`. If the channel is
    /// full, it schedules an async operation (via `future_into_py_with_locals`)
    /// to flush and send the item without blocking the Python thread.
    ///
    /// Returns a Python boolean indicating success.
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

    /// Close the underlying channel.
    ///
    /// After calling `close`, no further sends will succeed.
    pub fn close(&mut self) -> PyResult<()> {
        self.tx.close_channel();
        Ok(())
    }
}
