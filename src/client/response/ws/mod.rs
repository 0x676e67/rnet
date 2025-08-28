pub mod msg;

use std::sync::Arc;

use futures_util::{
    StreamExt,
    stream::{SplitSink, SplitStream},
};
use msg::Message;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use tokio::sync::Mutex;
use wreq::{
    header::HeaderValue,
    ws::{self, WebSocketResponse, message::Utf8Bytes},
};

use crate::{
    client::SocketAddr,
    error::Error,
    http::{Version, cookie::Cookie, header::HeaderMap, status::StatusCode},
};

/// Type aliases for WebSocket sender and receiver.
type Sender = Arc<Mutex<Option<SplitSink<ws::WebSocket, ws::message::Message>>>>;

/// Type alias for WebSocket receiver.
type Receiver = Arc<Mutex<Option<SplitStream<ws::WebSocket>>>>;

/// A WebSocket response.
#[pyclass(subclass)]
pub struct WebSocket {
    version: Version,
    status: StatusCode,
    remote_addr: Option<SocketAddr>,
    local_addr: Option<SocketAddr>,
    headers: HeaderMap,
    protocol: Option<HeaderValue>,
    sender: Sender,
    receiver: Receiver,
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
        let protocol = websocket.protocol().cloned();
        let (sender, receiver) = websocket.split();

        Ok(WebSocket {
            version,
            status,
            remote_addr,
            local_addr,
            headers,
            protocol,
            sender: Arc::new(Mutex::new(Some(sender))),
            receiver: Arc::new(Mutex::new(Some(receiver))),
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
    pub fn recv<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, util::recv(self.receiver.clone()))
    }

    /// Sends a message to the WebSocket.
    #[inline]
    #[pyo3(signature = (message))]
    pub fn send<'py>(&self, py: Python<'py>, message: Message) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, util::send(self.sender.clone(), message))
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
        future_into_py(py, util::close(receiver, sender, code, reason))
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
        let fut = util::anext(self.receiver.clone(), || Error::StopAsyncIteration.into());
        future_into_py(py, fut)
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
    pub fn recv(&self, py: Python) -> PyResult<Option<Message>> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(util::recv(self.0.receiver.clone()))
        })
    }

    /// Sends a message to the WebSocket.
    #[pyo3(signature = (message))]
    pub fn send(&self, py: Python, message: Message) -> PyResult<()> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(util::send(self.0.sender.clone(), message))
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
            pyo3_async_runtimes::tokio::get_runtime().block_on(util::close(
                self.0.receiver.clone(),
                self.0.sender.clone(),
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
    fn __next__(&self, py: Python) -> PyResult<Message> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(util::anext(self.0.receiver.clone(), || {
                    Error::StopIteration.into()
                }))
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

mod util {

    use bytes::Bytes;
    use futures_util::{SinkExt, TryStreamExt};
    use pyo3::{prelude::*, pybacked::PyBackedStr};

    use super::{Error, Message, Receiver, Sender, Utf8Bytes, ws};

    pub async fn recv(receiver: Receiver) -> PyResult<Option<Message>> {
        receiver
            .lock()
            .await
            .as_mut()
            .ok_or_else(|| Error::WebSocketDisconnect)?
            .try_next()
            .await
            .map(|val| val.map(Message))
            .map_err(Error::Library)
            .map_err(Into::into)
    }

    pub async fn send(sender: Sender, message: Message) -> PyResult<()> {
        sender
            .lock()
            .await
            .as_mut()
            .ok_or_else(|| Error::WebSocketDisconnect)?
            .send(message.0)
            .await
            .map_err(Error::Library)
            .map_err(Into::into)
    }

    pub async fn close(
        receiver: Receiver,
        sender: Sender,
        code: Option<u16>,
        reason: Option<PyBackedStr>,
    ) -> PyResult<()> {
        // Take and drop receiver to close the stream
        {
            let mut lock = receiver.lock().await;
            lock.take();
        }

        // Take sender for closing handshake
        let sender = {
            let mut lock = sender.lock().await;
            lock.take()
        };

        if let Some(mut sender) = sender {
            let code = code
                .map(ws::message::CloseCode)
                .unwrap_or(ws::message::CloseCode::NORMAL);
            let reason = reason
                .map(Bytes::from_owner)
                .map(Utf8Bytes::from_bytes_unchecked)
                .unwrap_or_else(|| Utf8Bytes::from_static("Goodbye"));
            let msg = ws::message::Message::Close(Some(ws::message::CloseFrame { code, reason }));

            sender.send(msg).await.map_err(Error::Library)?;
            sender.flush().await.map_err(Error::Library)?;
            sender.close().await.map_err(Error::Library)?;
        }

        Ok(())
    }

    pub async fn anext(
        receiver: Receiver,
        py_stop_iteration_error: fn() -> PyErr,
    ) -> PyResult<Message> {
        let val = {
            let mut lock = receiver.lock().await;
            lock.as_mut()
                .ok_or_else(py_stop_iteration_error)?
                .try_next()
                .await
        };

        val.map(|opt| opt.map(Message))
            .map_err(Error::Library)?
            .ok_or_else(py_stop_iteration_error)
    }
}
