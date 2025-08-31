mod cmd;
pub mod msg;

use std::time::Duration;

use msg::Message;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use tokio::sync::mpsc;
use wreq::{
    header::HeaderValue,
    ws::{self, WebSocketResponse, message::Utf8Bytes},
};

use crate::{
    client::SocketAddr,
    error::Error,
    http::{Version, cookie::Cookie, header::HeaderMap, status::StatusCode},
};

/// A WebSocket response.
#[pyclass(subclass)]
pub struct WebSocket {
    version: Version,
    status: StatusCode,
    remote_addr: Option<SocketAddr>,
    local_addr: Option<SocketAddr>,
    headers: HeaderMap,
    protocol: Option<HeaderValue>,
    tx: mpsc::UnboundedSender<cmd::Command>,
}

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
        let protocol = websocket.protocol();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        tokio::spawn(cmd::task(websocket, command_rx));

        Ok(WebSocket {
            version,
            status,
            remote_addr,
            local_addr,
            headers,
            protocol,
            tx: command_tx,
        })
    }
}

#[pymethods]
impl WebSocket {
    /// Returns the status code of the response.
    #[inline]
    #[getter]
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Returns the HTTP version of the response.
    #[inline]
    #[getter]
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns the headers of the response.
    #[inline]
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    /// Returns the cookies of the response.
    #[inline]
    #[getter]
    pub fn cookies(&self, py: Python) -> Vec<Cookie> {
        py.allow_threads(|| Cookie::extract_headers_cookies(&self.headers.0))
    }

    /// Returns the remote address of the response.
    #[inline]
    #[getter]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    /// Returns the local address of the response.
    #[inline]
    #[getter]
    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }

    /// Returns the WebSocket protocol.
    #[inline]
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
    #[inline]
    #[pyo3(signature = (timeout=None))]
    pub fn recv<'py>(
        &self,
        py: Python<'py>,
        timeout: Option<Duration>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let tx = self.tx.clone();
        future_into_py(py, cmd::recv(tx, timeout))
    }

    /// Sends a message to the WebSocket.
    #[inline]
    #[pyo3(signature = (message))]
    pub fn send<'py>(&self, py: Python<'py>, message: Message) -> PyResult<Bound<'py, PyAny>> {
        let tx = self.tx.clone();
        future_into_py(py, cmd::send(tx, message))
    }

    /// Closes the WebSocket connection.
    #[pyo3(signature = (code=None, reason=None))]
    pub fn close<'py>(
        &self,
        py: Python<'py>,
        code: Option<u16>,
        reason: Option<PyBackedStr>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let tx = self.tx.clone();
        future_into_py(py, cmd::close(tx, code, reason))
    }
}

#[pymethods]
impl WebSocket {
    #[inline]
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[inline]
    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.recv(py, None)
    }

    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
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

/// A blocking WebSocket response.
#[pyclass(name = "WebSocket", subclass)]
pub struct BlockingWebSocket(WebSocket);

#[pymethods]
impl BlockingWebSocket {
    /// Returns the status code of the response.
    #[getter]
    pub fn status(&self) -> StatusCode {
        self.0.status()
    }

    /// Returns the HTTP version of the response.
    #[getter]
    pub fn version(&self) -> Version {
        self.0.version()
    }

    /// Returns the headers of the response.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        self.0.headers()
    }

    /// Returns the cookies of the response.
    #[getter]
    pub fn cookies(&self, py: Python) -> Vec<Cookie> {
        self.0.cookies(py)
    }

    /// Returns the remote address of the response.
    #[getter]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.0.remote_addr()
    }

    /// Returns the local address of the response.
    #[getter]
    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.0.local_addr()
    }

    /// Returns the WebSocket protocol.
    #[getter]
    pub fn protocol(&self) -> Option<&str> {
        self.0.protocol()
    }

    /// Receives a message from the WebSocket.
    #[pyo3(signature = (timeout=None))]
    pub fn recv(&self, py: Python, timeout: Option<Duration>) -> PyResult<Option<Message>> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(cmd::recv(self.0.tx.clone(), timeout))
        })
    }

    /// Sends a message to the WebSocket.
    #[pyo3(signature = (message))]
    pub fn send(&self, py: Python, message: Message) -> PyResult<()> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(cmd::send(self.0.tx.clone(), message))
        })
    }

    /// Closes the WebSocket connection.
    #[pyo3(signature = (code=None, reason=None))]
    pub fn close(
        &self,
        py: Python,
        code: Option<u16>,
        reason: Option<PyBackedStr>,
    ) -> PyResult<()> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(cmd::close(
                self.0.tx.clone(),
                code,
                reason,
            ))
        })
    }
}

#[pymethods]
impl BlockingWebSocket {
    #[inline]
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[inline]
    fn __next__(&self, py: Python) -> PyResult<Option<Message>> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(cmd::recv(self.0.tx.clone(), None))
        })
    }

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
