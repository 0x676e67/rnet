use std::sync::Arc;

use crate::{
    error::wrap_rquest_error,
    types::{HeaderMap, Json, SocketAddr, StatusCode, Version},
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
use pyo3::{prelude::*, IntoPyObjectExt};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use tokio::sync::Mutex;

type Sender = Arc<Mutex<SplitSink<rquest::WebSocket, rquest::Message>>>;
type Receiver = Arc<Mutex<SplitStream<rquest::WebSocket>>>;

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
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
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
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = websocket.lock().await;
            if let Ok(Some(val)) = ws.try_next().await {
                return Ok(Some(Message(val)));
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
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = sender.lock().await;
            ws.send(message.0).await.map_err(wrap_rquest_error)
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
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut sender = sender.lock().await;
            sender
                .send(rquest::Message::Close {
                    code: rquest::CloseCode::from(code.unwrap_or_default()),
                    reason,
                })
                .await
                .map_err(wrap_rquest_error)?;
            Ok(())
        })
    }
}

#[pymethods]
impl WebSocket {
    fn __aenter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let slf = slf.into_py_any(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'a>(
        &'a mut self,
        py: Python<'a>,
        _exc_type: &Bound<'a, PyAny>,
        _exc_value: &Bound<'a, PyAny>,
        _traceback: &Bound<'a, PyAny>,
    ) -> PyResult<Bound<'a, PyAny>> {
        self.close(py, None, None)
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
