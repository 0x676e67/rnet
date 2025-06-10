mod message;

use crate::{
    error::Error,
    typing::{Cookie, HeaderMap, SocketAddr, StatusCode, Version},
};
use bytes::Bytes;
use futures_util::{
    SinkExt, StreamExt, TryStreamExt,
    stream::{SplitSink, SplitStream},
};
pub use message::Message;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use std::sync::Arc;
use tokio::sync::Mutex;
use wreq::{
    Utf8Bytes,
    header::{self, HeaderValue},
};

type Sender = Arc<Mutex<Option<SplitSink<wreq::WebSocket, wreq::Message>>>>;
type Receiver = Arc<Mutex<Option<SplitStream<wreq::WebSocket>>>>;

/// A WebSocket response.
#[pyclass(subclass)]
pub struct WebSocket {
    version: Version,
    status_code: StatusCode,
    remote_addr: Option<SocketAddr>,
    headers: header::HeaderMap,
    protocol: Option<HeaderValue>,
    sender: Sender,
    receiver: Receiver,
}

impl WebSocket {
    pub async fn new(builder: wreq::WebSocketRequestBuilder) -> wreq::Result<WebSocket> {
        let response = builder.send().await?;

        let version = Version::from_ffi(response.version());
        let status_code = StatusCode::from(response.status());
        let remote_addr = response.remote_addr().map(SocketAddr);
        let headers = response.headers().clone();
        let websocket = response.into_websocket().await?;
        let protocol = websocket.protocol().cloned();
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

    pub fn sender(&self) -> Sender {
        self.sender.clone()
    }

    pub fn receiver(&self) -> Receiver {
        self.receiver.clone()
    }

    pub async fn _recv(receiver: Receiver) -> PyResult<Option<Message>> {
        let mut lock = receiver.lock().await;
        lock.as_mut()
            .ok_or_else(|| Error::WebSocketDisconnect)?
            .try_next()
            .await
            .map(|val| val.map(Message))
            .map_err(Error::Request)
            .map_err(Into::into)
    }

    pub async fn _send(sender: Sender, message: Message) -> PyResult<()> {
        let mut lock = sender.lock().await;
        lock.as_mut()
            .ok_or_else(|| Error::WebSocketDisconnect)?
            .send(message.0)
            .await
            .map_err(Error::Request)
            .map_err(Into::into)
    }

    pub async fn _close(
        receiver: Receiver,
        sender: Sender,
        code: Option<u16>,
        reason: Option<PyBackedStr>,
    ) -> PyResult<()> {
        let mut lock = receiver.lock().await;
        let receiver = lock.take();
        drop(lock);
        drop(receiver);

        let mut lock = sender.lock().await;
        let sender = lock.take();
        drop(lock);

        if let Some(mut sender) = sender {
            let reason = reason
                .map(Bytes::from_owner)
                .map(Utf8Bytes::from_bytes_unchecked)
                .unwrap_or_else(|| wreq::Utf8Bytes::from_static("Goodbye"));
            sender
                .send(wreq::Message::Close(Some(wreq::CloseFrame {
                    code: code.map(wreq::CloseCode).unwrap_or(wreq::CloseCode::NORMAL),

                    reason,
                })))
                .await
                .map_err(Error::Request)?;
            sender.flush().await.map_err(Error::Request)?;
            sender.close().await.map_err(Error::Request)?;
        }

        Ok(())
    }

    pub async fn _anext(
        receiver: Receiver,
        py_stop_iteration_error: fn() -> PyErr,
    ) -> PyResult<Message> {
        let mut lock = receiver.lock().await;
        let val = lock
            .as_mut()
            .ok_or_else(py_stop_iteration_error)?
            .try_next()
            .await;

        drop(lock);

        val.map(|val| val.map(Message))
            .map_err(Error::Request)?
            .ok_or_else(py_stop_iteration_error)
    }
}

#[pymethods]
impl WebSocket {
    /// Returns whether the response is successful.
    #[getter]
    pub fn ok(&self) -> bool {
        self.status_code.as_int() == wreq::StatusCode::SWITCHING_PROTOCOLS
    }

    /// Returns the status code as integer of the response.
    #[getter]
    pub fn status(&self) -> u16 {
        self.status_code.as_int()
    }

    /// Returns the status code of the response.
    #[getter]
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Returns the HTTP version of the response.
    #[getter]
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns the headers of the response.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        HeaderMap(self.headers.clone())
    }

    /// Returns the cookies of the response.
    #[getter]
    pub fn cookies(&self, py: Python) -> Vec<Cookie> {
        py.allow_threads(|| Cookie::extract_cookies(&self.headers))
    }

    /// Returns the remote address of the response.
    #[getter]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    /// Returns the WebSocket protocol.
    #[getter]
    pub fn protocol(&self) -> Option<&str> {
        self.protocol
            .as_ref()
            .map(HeaderValue::to_str)
            .transpose()
            .ok()
            .flatten()
    }

    /// Receives a message from the WebSocket.
    pub fn recv<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, Self::_recv(self.receiver.clone()))
    }

    /// Sends a message to the WebSocket.
    #[pyo3(signature = (message))]
    pub fn send<'py>(&self, py: Python<'py>, message: Message) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, Self::_send(self.sender.clone(), message))
    }

    /// Closes the WebSocket connection.
    #[pyo3(signature = (code=None, reason=None))]
    pub fn close<'py>(
        &self,
        py: Python<'py>,
        code: Option<u16>,
        reason: Option<PyBackedStr>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        future_into_py(py, Self::_close(receiver, sender, code, reason))
    }
}

#[pymethods]
impl WebSocket {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(
            py,
            WebSocket::_anext(self.receiver.clone(), || Error::StopAsyncIteration.into()),
        )
    }

    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.close(py, None, None)
    }
}
