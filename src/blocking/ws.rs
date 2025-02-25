use crate::{
    async_impl::{self, Message},
    error::{py_stop_iteration_error, wrap_rquest_error},
    types::{HeaderMap, SocketAddr, Version},
};
use futures_util::{SinkExt, TryStreamExt};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A blocking WebSocket response.
#[gen_stub_pyclass]
#[pyclass]
pub struct BlockingWebSocket {
    inner: async_impl::WebSocket,
}

impl From<async_impl::WebSocket> for BlockingWebSocket {
    fn from(inner: async_impl::WebSocket) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BlockingWebSocket {
    /// Returns whether the response is successful.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the response is successful.
    #[getter]
    #[inline(always)]
    pub fn ok(&self) -> bool {
        self.inner.ok()
    }

    /// Returns the status code as integer of the response.
    ///
    /// # Returns
    ///
    /// An integer representing the HTTP status code.
    #[getter]
    #[inline(always)]
    pub fn status(&self) -> u16 {
        self.inner.status()
    }

    /// Returns the HTTP version of the response.
    ///
    /// # Returns
    ///
    /// A `Version` object representing the HTTP version of the response.
    #[getter]
    #[inline(always)]
    pub fn version(&self) -> Version {
        self.inner.version()
    }

    /// Returns the headers of the response.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` object representing the headers of the response.
    #[getter]
    #[inline(always)]
    pub fn headers(&self) -> HeaderMap {
        self.inner.headers()
    }

    /// Returns the remote address of the response.
    ///
    /// # Returns
    ///
    /// An `IpAddr` object representing the remote address of the response.
    #[getter]
    #[inline(always)]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.inner.remote_addr()
    }

    /// Returns the WebSocket protocol.
    ///
    /// # Returns
    ///
    /// An optional string representing the WebSocket protocol.
    pub fn protocol(&self) -> Option<&str> {
        self.inner.protocol()
    }

    /// Receives a message from the WebSocket.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing a `Bound` object with the received message, or `None` if no message is received.
    pub fn recv(&self, py: Python) -> PyResult<Option<Message>> {
        py.allow_threads(|| {
            let websocket = self.inner.receiver.clone();
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                let mut lock = websocket.lock().await;
                if let Some(recv) = lock.as_mut() {
                    if let Ok(Some(val)) = recv.try_next().await {
                        return Ok(Some(Message(val)));
                    }
                }
                Ok(None)
            })
        })
    }

    /// Sends a message to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing a `Bound` object.
    #[pyo3(signature = (message))]
    pub fn send(&self, py: Python, message: Message) -> PyResult<()> {
        py.allow_threads(|| {
            let sender = self.inner.sender.clone();
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                let mut lock = sender.lock().await;
                if let Some(send) = lock.as_mut() {
                    return send.send(message.0).await.map_err(wrap_rquest_error);
                }
                Ok(())
            })
        })
    }

    /// Closes the WebSocket connection.
    ///
    /// # Arguments
    ///
    /// * `code` - An optional close code.
    /// * `reason` - An optional reason for closing.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing a `Bound` object.
    #[pyo3(signature = (code=None, reason=None))]
    pub fn close(&self, py: Python, code: Option<u16>, reason: Option<String>) -> PyResult<()> {
        py.allow_threads(|| {
            let sender = self.inner.sender.clone();
            let receiver = self.inner.receiver.clone();
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                let mut lock = receiver.lock().await;
                drop(lock.take());
                drop(lock);

                let mut lock = sender.lock().await;
                let send = lock.take();
                drop(lock);

                if let Some(mut send) = send {
                    if let Some(code) = code {
                        send.send(rquest::Message::Close {
                            code: rquest::CloseCode::from(code),
                            reason,
                        })
                        .await
                        .map_err(wrap_rquest_error)?;
                    }
                    return send.close().await.map_err(wrap_rquest_error);
                }

                Ok(())
            })
        })
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BlockingWebSocket {
    #[inline(always)]
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self, py: Python) -> PyResult<Option<Message>> {
        py.allow_threads(|| {
            let recv = self.inner.receiver.clone();
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                let mut lock = recv.lock().await;
                let recv = lock
                    .as_mut()
                    .ok_or_else(py_stop_iteration_error)?
                    .try_next()
                    .await;

                drop(lock);

                recv.map(|val| val.map(Message)).map_err(wrap_rquest_error)
            })
        })
    }
}
