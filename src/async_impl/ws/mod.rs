mod message;

use crate::{
    error::{py_stop_async_iteration_error, websocket_disconnect_error, wrap_rquest_error},
    typing::{HeaderMap, SocketAddr, StatusCode, Version},
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
pub use message::Message;
use pyo3::{prelude::*, IntoPyObjectExt};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use std::sync::Arc;
use tokio::sync::Mutex;

type Sender = Arc<Mutex<Option<SplitSink<rquest::WebSocket, rquest::Message>>>>;
type Receiver = Arc<Mutex<Option<SplitStream<rquest::WebSocket>>>>;

/// A WebSocket response.
#[gen_stub_pyclass]
#[pyclass]
pub struct WebSocket {
    version: Version,
    status_code: StatusCode,
    remote_addr: Option<SocketAddr>,
    headers: HeaderMap,
    protocol: Option<String>,
    sender: Sender,
    receiver: Receiver,
}

impl WebSocket {
    pub async fn new(builder: rquest::WebSocketRequestBuilder) -> crate::Result<WebSocket> {
        let response = builder.send().await.map_err(wrap_rquest_error)?;

        let version = Version::from_ffi(response.version());
        let status_code = StatusCode::from(response.status());
        let remote_addr = response.remote_addr().map(SocketAddr::from);
        let headers = HeaderMap::from(response.headers().clone());
        let websocket = response.into_websocket().await.map_err(wrap_rquest_error)?;
        let protocol = websocket.protocol().map(ToOwned::to_owned);
        let (sender, receiver) = websocket.split();

        Ok(WebSocket {
            version,
            status_code,
            remote_addr,
            headers,
            protocol,
            sender: Arc::new(Mutex::new(Some(sender))),
            receiver: Arc::new(Mutex::new(Some(receiver))),
        })
    }
}

impl WebSocket {
    #[inline(always)]
    pub fn sender(&self) -> Sender {
        self.sender.clone()
    }

    #[inline(always)]
    pub fn receiver(&self) -> Receiver {
        self.receiver.clone()
    }

    pub async fn _recv(receiver: Receiver) -> PyResult<Option<Message>> {
        let mut lock = receiver.lock().await;
        lock.as_mut()
            .ok_or_else(websocket_disconnect_error)?
            .try_next()
            .await
            .map(|val| val.map(Message))
            .map_err(wrap_rquest_error)
    }

    pub async fn _send(sender: Sender, message: Message) -> PyResult<()> {
        let mut lock = sender.lock().await;
        lock.as_mut()
            .ok_or_else(websocket_disconnect_error)?
            .send(message.0)
            .await
            .map_err(wrap_rquest_error)
    }

    pub async fn _close(
        receiver: Receiver,
        sender: Sender,
        code: Option<u16>,
        reason: Option<String>,
    ) -> PyResult<()> {
        #[cfg(feature = "logging")]
        log::debug!("Closing WebSocket connection");

        let mut lock = receiver.lock().await;
        let receiver = lock.take();
        drop(lock);
        drop(receiver);

        let mut lock = sender.lock().await;
        let sender = lock.take();
        drop(lock);

        if let Some(mut sender) = sender {
            sender
                .send(rquest::Message::Close {
                    code: code
                        .map(rquest::CloseCode::from)
                        .unwrap_or(rquest::CloseCode::Normal),
                    reason,
                })
                .await
                .map_err(wrap_rquest_error)?;
            sender.flush().await.map_err(wrap_rquest_error)?;
            sender.close().await.map_err(wrap_rquest_error)?;

            #[cfg(feature = "logging")]
            log::debug!("WebSocket connection closed");
        }

        Ok(())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl WebSocket {
    /// Returns whether the response is successful.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the response is successful.
    #[getter]
    #[inline(always)]
    pub fn ok(&self) -> bool {
        self.status_code.as_int() == rquest::StatusCode::SWITCHING_PROTOCOLS
    }

    /// Returns the status code as integer of the response.
    ///
    /// # Returns
    ///
    /// An integer representing the HTTP status code.
    #[getter]
    #[inline(always)]
    pub fn status(&self) -> u16 {
        self.status_code.as_int()
    }

    /// Returns the status code of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the HTTP status code.
    #[getter]
    #[inline(always)]
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Returns the HTTP version of the response.
    ///
    /// # Returns
    ///
    /// A `Version` object representing the HTTP version of the response.
    #[getter]
    #[inline(always)]
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns the headers of the response.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` object representing the headers of the response.
    #[getter]
    #[inline(always)]
    pub fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    /// Returns the remote address of the response.
    ///
    /// # Returns
    ///
    /// An `IpAddr` object representing the remote address of the response.
    #[getter]
    #[inline(always)]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    /// Returns the WebSocket protocol.
    ///
    /// # Returns
    ///
    /// An optional string representing the WebSocket protocol.
    #[inline(always)]
    pub fn protocol(&self) -> Option<&str> {
        self.protocol.as_deref()
    }

    /// Receives a message from the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python runtime.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing a `Bound` object with the received message, or `None` if no message is received.
    #[inline(always)]
    pub fn recv<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        future_into_py(py, Self::_recv(self.receiver.clone()))
    }

    /// Sends a message to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python runtime.
    /// * `message` - The message to send.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing a `Bound` object.
    #[pyo3(signature = (message))]
    #[inline(always)]
    pub fn send<'rt>(&self, py: Python<'rt>, message: Message) -> PyResult<Bound<'rt, PyAny>> {
        future_into_py(py, Self::_send(self.sender.clone(), message))
    }

    /// Closes the WebSocket connection.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python runtime.
    /// * `code` - An optional close code.
    /// * `reason` - An optional reason for closing.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing a `Bound` object.
    #[pyo3(signature = (code=None, reason=None))]
    #[inline(always)]
    pub fn close<'rt>(
        &self,
        py: Python<'rt>,
        code: Option<u16>,
        reason: Option<String>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        future_into_py(py, Self::_close(receiver, sender, code, reason))
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl WebSocket {
    /// Returns the WebSocket instance itself as an asynchronous iterator.
    ///
    /// This method is used to make the WebSocket instance iterable in an asynchronous context.
    ///
    /// # Arguments
    ///
    /// * `slf` - A reference to the WebSocket instance.
    ///
    /// # Returns
    ///
    /// Returns the WebSocket instance itself.
    #[inline(always)]
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Returns the next message from the WebSocket.
    ///
    /// This method is used to retrieve the next message from the WebSocket in an asynchronous iteration.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python runtime.
    ///
    /// # Returns
    ///
    /// Returns a `PyResult` containing an `Option` with a `Bound` object representing the received message.
    /// If no message is received, returns `None`.
    fn __anext__<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let recv = self.receiver.clone();
        future_into_py(py, async move {
            // Here we lock the mutex to access the data inside
            // and call try_next() method to get the next value.
            let mut lock = recv.lock().await;
            let recv = lock
                .as_mut()
                .ok_or_else(py_stop_async_iteration_error)?
                .try_next()
                .await;

            drop(lock);

            recv.map(|val| val.map(Message)).map_err(wrap_rquest_error)
        })
    }

    fn __aenter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'a>(
        &self,
        py: Python<'a>,
        _exc_type: &Bound<'a, PyAny>,
        _exc_value: &Bound<'a, PyAny>,
        _traceback: &Bound<'a, PyAny>,
    ) -> PyResult<Bound<'a, PyAny>> {
        self.close(py, None, None)
    }
}
