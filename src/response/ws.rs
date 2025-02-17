use std::sync::Arc;

use crate::{
    error::{memory_error, wrap_rquest_error},
    types::{HeaderMap, Json, SocketAddr, StatusCode, Version},
};
use arc_swap::ArcSwapOption;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
use pyo3::{prelude::*, IntoPyObjectExt};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::TlsInfo;
use tokio::sync::Mutex;

/// A response from a request.
///
/// # Examples
///
/// ```python
/// import asyncio
/// import rnet
///
/// async def main():
///     response = await rnet.get("https://www.rust-lang.org")
///     print("Status Code: ", response.status_code)
///     print("Version: ", response.version)
///     print("Response URL: ", response.url)
///     print("Headers: ", response.headers.to_dict())
///     print("Content-Length: ", response.content_length)
///     print("Encoding: ", response.encoding)
///     print("Remote Address: ", response.remote_addr)
///
///     text_content = await response.text()
///     print("Text: ", text_content)
///
/// if __name__ == "__main__":
///     asyncio.run(main())
/// ```
#[gen_stub_pyclass]
#[pyclass]
pub struct WebSocketResponse {
    version: Version,
    status_code: StatusCode,
    remote_addr: Option<SocketAddr>,
    headers: HeaderMap,
    response: ArcSwapOption<rquest::WebSocketResponse>,
}

impl From<rquest::WebSocketResponse> for WebSocketResponse {
    fn from(response: rquest::WebSocketResponse) -> Self {
        WebSocketResponse {
            version: Version::from(response.version()),
            status_code: StatusCode::from(response.status()),
            remote_addr: response.remote_addr().map(SocketAddr::from),
            headers: HeaderMap::from(response.headers().clone()),
            response: ArcSwapOption::from_pointee(response),
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl WebSocketResponse {
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

    /// Returns the TLS peer certificate of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the TLS peer certificate of the response.
    pub fn peer_certificate(&self) -> PyResult<Option<Vec<u8>>> {
        let resp_ref = self.response.load();
        let resp = resp_ref.as_ref().ok_or_else(memory_error)?;
        if let Some(val) = resp.extensions().get::<TlsInfo>() {
            return Ok(val.peer_certificate().map(ToOwned::to_owned));
        }

        Ok(None)
    }

    /// Returns the WebSocket of the response.
    pub fn into_websocket<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let response = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response
                .into_websocket()
                .await
                .map(WebSocket::from)
                .map_err(wrap_rquest_error)
        })
    }

    /// Closes the response connection.
    pub fn close(&self) {
        let _ = self.into_inner().map(drop);
    }
}

impl WebSocketResponse {
    /// Consumes the `WebSocketResponse` and returns the inner `rquest::RespWebSocketResponseonse`.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing the inner `rquest::WebSocketResponse` if successful, or an error if the
    /// response has already been taken or cannot be unwrapped.
    ///
    /// # Errors
    ///
    /// Returns a memory error if the response has already been taken or if the `Arc` cannot be unwrapped.
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    fn into_inner(&self) -> PyResult<rquest::WebSocketResponse> {
        self.response
            .swap(None)
            .and_then(Arc::into_inner)
            .ok_or_else(memory_error)
    }
}

type Sender = Arc<Mutex<SplitSink<rquest::WebSocket, rquest::Message>>>;
type Receiver = Arc<Mutex<SplitStream<rquest::WebSocket>>>;

#[gen_stub_pyclass]
#[pyclass]
pub struct WebSocket {
    protocol: Option<String>,
    sender: Sender,
    receiver: Receiver,
}

impl From<rquest::WebSocket> for WebSocket {
    fn from(ws: rquest::WebSocket) -> Self {
        let protocol = ws.protocol().map(ToOwned::to_owned);
        let (sender, receiver) = ws.split();
        WebSocket {
            protocol,
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}

#[pymethods]
impl WebSocket {
    pub fn protocol(&self) -> Option<&str> {
        self.protocol.as_deref()
    }

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

    #[pyo3(signature = (message))]
    pub fn send<'rt>(&self, py: Python<'rt>, message: Message) -> PyResult<Bound<'rt, PyAny>> {
        let sender = self.sender.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = sender.lock().await;
            ws.send(message.0).await.map_err(wrap_rquest_error)
        })
    }

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

#[pyclass]
#[derive(Clone)]
pub struct Message(rquest::Message);

#[pymethods]
impl Message {
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pymethods]
impl Message {
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

    pub fn json(&self) -> PyResult<Json> {
        self.0.json::<Json>().map_err(wrap_rquest_error)
    }

    #[getter]
    pub fn text(&self) -> Option<&str> {
        match &self.0 {
            rquest::Message::Text(text) => Some(text),
            _ => None,
        }
    }

    #[getter]
    pub fn binary(&self) -> Option<&[u8]> {
        match &self.0 {
            rquest::Message::Binary(data) => Some(data),
            _ => None,
        }
    }

    #[getter]
    pub fn ping(&self) -> Option<&[u8]> {
        match &self.0 {
            rquest::Message::Ping(data) => Some(data),
            _ => None,
        }
    }

    #[getter]
    pub fn pong(&self) -> Option<&[u8]> {
        match &self.0 {
            rquest::Message::Pong(data) => Some(data),
            _ => None,
        }
    }

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

    #[staticmethod]
    #[pyo3(signature = (text))]
    #[inline]
    pub fn from_text(text: &str) -> Self {
        Message(rquest::Message::Text(text.to_owned()))
    }

    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline]
    pub fn from_binary(data: Vec<u8>) -> Self {
        Message(rquest::Message::Binary(data))
    }

    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline]
    pub fn from_ping(data: Vec<u8>) -> Self {
        Message(rquest::Message::Ping(data))
    }

    #[staticmethod]
    #[pyo3(signature = (data))]
    #[inline]
    pub fn from_pong(data: Vec<u8>) -> Self {
        Message(rquest::Message::Pong(data))
    }

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
