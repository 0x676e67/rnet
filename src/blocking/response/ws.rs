use crate::{
    async_impl::{self, Message},
    error::Error,
    typing::{Cookie, HeaderMap, SocketAddr, StatusCode, Version},
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use std::ops::Deref;

/// A blocking WebSocket response.
#[pyclass]
pub struct BlockingWebSocket(async_impl::WebSocket);

impl From<async_impl::WebSocket> for BlockingWebSocket {
    fn from(inner: async_impl::WebSocket) -> Self {
        Self(inner)
    }
}

impl Deref for BlockingWebSocket {
    type Target = async_impl::WebSocket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[pymethods]
impl BlockingWebSocket {
    /// Returns whether the response is successful.
    #[getter]
    pub fn ok(&self) -> bool {
        self.0.ok()
    }

    /// Returns the status code as integer of the response.
    #[getter]
    pub fn status(&self) -> u16 {
        self.0.status()
    }

    /// Returns the status code of the response.
    #[getter]
    pub fn status_code(&self) -> StatusCode {
        self.0.status_code()
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
    pub fn cookies<'py>(&'py self, py: Python<'py>) -> Vec<Cookie> {
        self.0.cookies(py)
    }

    /// Returns the remote address of the response.
    #[getter]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.0.remote_addr()
    }

    /// Returns the WebSocket protocol.
    #[getter]
    pub fn protocol(&self) -> Option<&str> {
        self.0.protocol()
    }

    /// Receives a message from the WebSocket.
    pub fn recv(&self, py: Python) -> PyResult<Option<Message>> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(async_impl::WebSocket::_recv(self.receiver()))
        })
    }

    /// Sends a message to the WebSocket.
    #[pyo3(signature = (message))]
    pub fn send(&self, py: Python, message: Message) -> PyResult<()> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(async_impl::WebSocket::_send(self.sender(), message))
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
            pyo3_async_runtimes::tokio::get_runtime().block_on(async_impl::WebSocket::_close(
                self.receiver(),
                self.sender(),
                code,
                reason,
            ))
        })
    }
}

#[pymethods]
impl BlockingWebSocket {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self, py: Python) -> PyResult<Message> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(async_impl::WebSocket::_anext(self.receiver(), || {
                    Error::StopIteration.into()
                }))
        })
    }

    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __exit__<'a>(
        &self,
        py: Python<'a>,
        _exc_type: &Bound<'a, PyAny>,
        _exc_value: &Bound<'a, PyAny>,
        _traceback: &Bound<'a, PyAny>,
    ) -> PyResult<()> {
        self.close(py, None, None)
    }
}
