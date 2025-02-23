use std::sync::Arc;

use crate::{
    error::{py_stop_async_iteration_error, wrap_rquest_error},
    types::{HeaderMap, Json, SocketAddr, StatusCode, Version},
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
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
    pub fn recv<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let websocket = self.receiver.clone();
        future_into_py(py, async move {
            let mut lock = websocket.lock().await;
            if let Some(recv) = lock.as_mut() {
                if let Ok(Some(val)) = recv.try_next().await {
                    return Ok(Some(Message(val)));
                }
            }
            Ok(None)
        })
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
    pub fn send<'rt>(&self, py: Python<'rt>, message: Message) -> PyResult<Bound<'rt, PyAny>> {
        let sender = self.sender.clone();
        future_into_py(py, async move {
            let mut lock = sender.lock().await;
            if let Some(send) = lock.as_mut() {
                return send.send(message.0).await.map_err(wrap_rquest_error);
            }
            Ok(())
        })
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
    pub fn close<'rt>(
        &self,
        py: Python<'rt>,
        code: Option<u16>,
        reason: Option<String>,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        future_into_py(py, async move {
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
    }
}

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
    fn __anext__<'rt>(&self, py: Python<'rt>) -> PyResult<Option<Bound<'rt, PyAny>>> {
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

            recv.map(|val| val.map(Message))
                .map(Some)
                .map_err(wrap_rquest_error)
        })
        .map(Some)
    }
}

/// A WebSocket message.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct Message(rquest::Message);

#[pymethods]
impl Message {
    /// Returns the JSON representation of the message.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing the JSON representation of the message.
    pub fn json(&self) -> PyResult<Json> {
        self.0.json::<Json>().map_err(wrap_rquest_error)
    }

    /// Returns the data of the message as bytes.
    ///
    /// # Returns
    ///
    /// A byte slice representing the data of the message.
    #[getter]
    pub fn data(&self) -> &[u8] {
        match &self.0 {
            rquest::Message::Text(text) => text.as_bytes(),
            rquest::Message::Binary(data) => data,
            rquest::Message::Ping(data) => data,
            rquest::Message::Pong(data) => data,
            _ => &[],
        }
    }

    /// Returns the binary data of the message if it is a binary message.
    ///
    /// # Returns
    ///
    /// An optional byte slice representing the binary data of the message.
    #[getter]
    pub fn binary(&self) -> Option<&[u8]> {
        match &self.0 {
            rquest::Message::Binary(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the ping data of the message if it is a ping message.
    ///
    /// # Returns
    ///
    /// An optional byte slice representing the ping data of the message.
    #[getter]
    pub fn ping(&self) -> Option<&[u8]> {
        match &self.0 {
            rquest::Message::Ping(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the pong data of the message if it is a pong message.
    ///
    /// # Returns
    ///
    /// An optional byte slice representing the pong data of the message.
    #[getter]
    pub fn pong(&self) -> Option<&[u8]> {
        match &self.0 {
            rquest::Message::Pong(data) => Some(data),
            _ => None,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Message {
    /// Returns a string representation of the message.
    ///
    /// # Returns
    ///
    /// A string representing the message.
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    /// Returns a string representation of the message.
    ///
    /// # Returns
    ///
    /// A string representing the message.
    fn __repr__(&self) -> String {
        self.__str__()
    }

    /// Returns the text of the message if it is a text message.
    ///
    /// # Returns
    ///
    /// An optional string representing the text of the message.
    #[getter]
    pub fn text(&self) -> Option<&str> {
        match &self.0 {
            rquest::Message::Text(text) => Some(text),
            _ => None,
        }
    }

    /// Returns the close code and reason of the message if it is a close message.
    ///
    /// # Returns
    ///
    /// An optional tuple containing the close code and reason.
    #[getter]
    pub fn close(&self) -> Option<(u16, Option<&str>)> {
        match &self.0 {
            rquest::Message::Close { code, reason } => Some((
                match *code {
                    rquest::CloseCode::Normal => 1000,
                    rquest::CloseCode::Away => 1001,
                    rquest::CloseCode::Protocol => 1002,
                    rquest::CloseCode::Unsupported => 1003,
                    rquest::CloseCode::Status => 1005,
                    rquest::CloseCode::Abnormal => 1006,
                    rquest::CloseCode::Invalid => 1007,
                    rquest::CloseCode::Policy => 1008,
                    rquest::CloseCode::Size => 1009,
                    rquest::CloseCode::Extension => 1010,
                    rquest::CloseCode::Error => 1011,
                    rquest::CloseCode::Restart => 1012,
                    rquest::CloseCode::Again => 1013,
                    rquest::CloseCode::Tls => 1015,
                    rquest::CloseCode::Reserved(v)
                    | rquest::CloseCode::Iana(v)
                    | rquest::CloseCode::Library(v)
                    | rquest::CloseCode::Bad(v) => v,
                    _ => return None,
                },
                reason.as_deref(),
            )),
            _ => None,
        }
    }

    /// Creates a new text message.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the text message.
    #[staticmethod]
    #[pyo3(signature = (text))]
    #[inline]
    pub fn from_text(text: &str) -> Self {
        Message(rquest::Message::Text(text.to_owned()))
    }

    /// Creates a new binary message.
    ///
    /// # Arguments
    ///
    /// * `data` - The binary data of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the binary message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline]
    pub fn from_binary(data: Vec<u8>) -> Self {
        Message(rquest::Message::Binary(data))
    }

    /// Creates a new ping message.
    ///
    /// # Arguments
    ///
    /// * `data` - The ping data of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the ping message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline]
    pub fn from_ping(data: Vec<u8>) -> Self {
        Message(rquest::Message::Ping(data))
    }

    /// Creates a new pong message.
    ///
    /// # Arguments
    ///
    /// * `data` - The pong data of the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the pong message.
    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline]
    pub fn from_pong(data: Vec<u8>) -> Self {
        Message(rquest::Message::Pong(data))
    }

    /// Creates a new close message.
    ///
    /// # Arguments
    ///
    /// * `code` - The close code.
    /// * `reason` - An optional reason for closing.
    ///
    /// # Returns
    ///
    /// A new `Message` instance containing the close message.
    #[staticmethod]
    #[pyo3(signature = (code, reason=None))]
    #[inline]
    pub fn from_close(code: u16, reason: Option<String>) -> Self {
        Message(rquest::Message::Close {
            code: rquest::CloseCode::from(code),
            reason,
        })
    }
}
