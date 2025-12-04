mod cmd;
pub mod msg;

use std::{fmt::Display, future, time::Duration};

use msg::Message;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use tokio::sync::mpsc;
use wreq::{
    header::HeaderValue,
    ws::{self, WebSocketResponse, message::Utf8Bytes},
};

use crate::{
    client::SocketAddr,
    cookie::Cookie,
    error::Error,
    header::HeaderMap,
    http::{StatusCode, Version},
};

/// A WebSocket response.
#[pyclass(subclass, frozen, str)]
pub struct WebSocket {
    /// Returns the status code of the response.
    #[pyo3(get)]
    version: Version,

    /// Returns the HTTP version of the response.
    #[pyo3(get)]
    status: StatusCode,

    /// Returns the remote address of the response.
    #[pyo3(get)]
    remote_addr: Option<SocketAddr>,

    /// Returns the local address of the response.
    #[pyo3(get)]
    local_addr: Option<SocketAddr>,

    /// Returns the headers of the response.
    #[pyo3(get)]
    headers: HeaderMap,
    protocol: Option<HeaderValue>,
    cmd: mpsc::UnboundedSender<cmd::Command>,
}

/// A blocking WebSocket response.
#[pyclass(name = "WebSocket", subclass, frozen, str)]
pub struct BlockingWebSocket(WebSocket);

// ===== impl WebSocket =====

impl WebSocket {
    /// Creates a new [`WebSocket`] instance.
    pub async fn new(response: WebSocketResponse) -> wreq::Result<WebSocket> {
        let (version, status, remote_addr, local_addr, headers) = (
            Version::from_ffi(response.version()),
            StatusCode::from(response.status()),
            response.remote_addr().map(SocketAddr),
            response.local_addr().map(SocketAddr),
            HeaderMap(response.headers().clone()),
        );
        let websocket = response.into_websocket().await?;
        let protocol = websocket.protocol().cloned();
        let (cmd, rx) = mpsc::unbounded_channel();
        tokio::spawn(cmd::task(websocket, rx));

        Ok(WebSocket {
            version,
            status,
            remote_addr,
            local_addr,
            headers,
            protocol,
            cmd,
        })
    }
}

#[pymethods]
impl WebSocket {
    /// Returns the cookies of the response.
    #[getter]
    pub fn cookies(&self, py: Python) -> Vec<Cookie> {
        py.detach(|| Cookie::extract_headers_cookies(&self.headers.0))
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

    /// Receive a message from the WebSocket.
    #[pyo3(signature = (timeout=None))]
    pub fn recv<'py>(
        &self,
        py: Python<'py>,
        timeout: Option<Duration>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let tx = self.cmd.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, cmd::recv(tx, timeout))
    }

    /// Send a message to the WebSocket.
    #[pyo3(signature = (message))]
    pub fn send<'py>(&self, py: Python<'py>, message: Message) -> PyResult<Bound<'py, PyAny>> {
        let tx = self.cmd.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, cmd::send(tx, message))
    }

    /// Send multiple messages to the WebSocket.
    #[pyo3(signature = (messages))]
    pub fn send_all<'py>(
        &self,
        py: Python<'py>,
        messages: Vec<Message>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let tx = self.cmd.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, cmd::send_all(tx, messages))
    }

    /// Close the WebSocket connection.
    #[pyo3(signature = (code=None, reason=None))]
    pub fn close<'py>(
        &self,
        py: Python<'py>,
        code: Option<u16>,
        reason: Option<PyBackedStr>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let tx = self.cmd.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, cmd::close(tx, code, reason))
    }
}

#[pymethods]
impl WebSocket {
    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, future::ready(Ok(slf)))
    }

    #[inline]
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

impl Display for WebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} [{}] >", stringify!(WebSocket), self.status.0,)
    }
}

// ===== impl BlockingWebSocket =====

#[pymethods]
impl BlockingWebSocket {
    /// Returns the status code of the response.
    #[getter]
    pub fn status(&self) -> StatusCode {
        self.0.status
    }

    /// Returns the HTTP version of the response.
    #[getter]
    pub fn version(&self) -> Version {
        self.0.version
    }

    /// Returns the headers of the response.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        self.0.headers.clone()
    }

    /// Returns the cookies of the response.
    #[getter]
    pub fn cookies(&self, py: Python) -> Vec<Cookie> {
        self.0.cookies(py)
    }

    /// Returns the remote address of the response.
    #[getter]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.0.remote_addr
    }

    /// Returns the local address of the response.
    #[getter]
    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.0.local_addr
    }

    /// Returns the WebSocket protocol.
    #[getter]
    pub fn protocol(&self) -> Option<&str> {
        self.0.protocol()
    }

    /// Receive a message from the WebSocket.
    #[pyo3(signature = (timeout=None))]
    pub fn recv(&self, py: Python, timeout: Option<Duration>) -> PyResult<Option<Message>> {
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(cmd::recv(self.0.cmd.clone(), timeout))
        })
    }

    /// Send a message to the WebSocket.
    #[pyo3(signature = (message))]
    pub fn send(&self, py: Python, message: Message) -> PyResult<()> {
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(cmd::send(self.0.cmd.clone(), message))
        })
    }

    /// Send multiple messages to the WebSocket.
    #[pyo3(signature = (messages))]
    pub fn send_all(&self, py: Python, messages: Vec<Message>) -> PyResult<()> {
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(cmd::send_all(self.0.cmd.clone(), messages))
        })
    }

    /// Close the WebSocket connection.
    #[pyo3(signature = (code=None, reason=None))]
    pub fn close(
        &self,
        py: Python,
        code: Option<u16>,
        reason: Option<PyBackedStr>,
    ) -> PyResult<()> {
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(cmd::close(
                self.0.cmd.clone(),
                code,
                reason,
            ))
        })
    }
}

#[pymethods]
impl BlockingWebSocket {
    #[inline]
    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __exit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<()> {
        self.close(py, None, None)
    }
}

impl From<WebSocket> for BlockingWebSocket {
    fn from(inner: WebSocket) -> Self {
        Self(inner)
    }
}

impl Display for BlockingWebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{} [{}] >",
            stringify!(BlockingWebSocket),
            self.0.status,
        )
    }
}
