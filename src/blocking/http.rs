use std::ops::Deref;

use crate::{
    async_impl::{self},
    buffer::{BytesBuffer, PyBufferProtocol},
    error::{py_stop_iteration_error, wrap_rquest_error},
    typing::{Cookie, HeaderMap, Json, SocketAddr, StatusCode, Version},
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A bloking response from a request.
#[gen_stub_pyclass]
#[pyclass]
pub struct BlockingResponse(async_impl::Response);

impl From<async_impl::Response> for BlockingResponse {
    fn from(response: async_impl::Response) -> Self {
        Self(response)
    }
}

impl Deref for BlockingResponse {
    type Target = async_impl::Response;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BlockingResponse {
    /// Returns the URL of the response.
    ///
    /// # Returns
    ///
    /// A string representing the URL of the response.
    #[getter]
    #[inline(always)]
    pub fn url(&self) -> &str {
        self.0.url()
    }

    /// Returns whether the response is successful.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the response is successful.
    #[getter]
    #[inline(always)]
    pub fn ok(&self) -> bool {
        self.0.ok()
    }

    /// Returns the status code as integer of the response.
    ///
    /// # Returns
    ///
    /// An integer representing the HTTP status code.
    #[getter]
    #[inline(always)]
    pub fn status(&self) -> u16 {
        self.0.status()
    }

    /// Returns the status code of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the HTTP status code.
    #[getter]
    #[inline(always)]
    pub fn status_code(&self) -> StatusCode {
        self.0.status_code()
    }

    /// Returns the HTTP version of the response.
    ///
    /// # Returns
    ///
    /// A `Version` object representing the HTTP version of the response.
    #[getter]
    #[inline(always)]
    pub fn version(&self) -> Version {
        self.0.version()
    }

    /// Returns the headers of the response.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` object representing the headers of the response.
    #[getter]
    #[inline(always)]
    pub fn headers(&self) -> HeaderMap {
        self.0.headers()
    }

    /// Returns the cookies of the response.
    ///
    /// # Returns
    ///
    /// A Python cookies object representing the cookies of the response.
    #[getter]
    #[inline(always)]
    pub fn cookies<'py>(&'py self, py: Python<'py>) -> Vec<Cookie> {
        self.0.cookies(py)
    }

    /// Returns the content length of the response.
    ///
    /// # Returns
    ///
    /// An integer representing the content length of the response.
    #[getter]
    #[inline(always)]
    pub fn content_length(&self) -> u64 {
        self.0.content_length()
    }

    /// Returns the remote address of the response.
    ///
    /// # Returns
    ///
    /// An `IpAddr` object representing the remote address of the response.
    #[getter]
    #[inline(always)]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.0.remote_addr()
    }

    /// Encoding to decode with when accessing text.
    ///
    /// # Returns
    ///
    /// A string representing the encoding to decode with when accessing text.
    #[getter]
    #[inline(always)]
    pub fn encoding(&self, py: Python) -> String {
        self.0.encoding(py)
    }

    /// Returns the TLS peer certificate of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the TLS peer certificate of the response.
    #[inline(always)]
    pub fn peer_certificate<'rt>(
        &'rt self,
        py: Python<'rt>,
    ) -> PyResult<Option<Bound<'rt, PyAny>>> {
        self.0.peer_certificate(py)
    }

    /// Returns the text content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the text content of the response.
    pub fn text(&self, py: Python) -> PyResult<String> {
        py.allow_threads(|| {
            let resp = self.inner()?;
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.text())
                .map_err(wrap_rquest_error)
        })
    }

    /// Returns the text content of the response with a specific charset.
    ///
    /// # Arguments
    ///
    /// * `default_encoding` - The default encoding to use if the charset is not specified.
    ///
    /// # Returns
    ///
    /// A Python object representing the text content of the response.
    pub fn text_with_charset(&self, py: Python, encoding: String) -> PyResult<String> {
        py.allow_threads(|| {
            let resp = self.inner()?;
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.text_with_charset(&encoding))
                .map_err(wrap_rquest_error)
        })
    }

    /// Returns the JSON content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json(&self, py: Python) -> PyResult<Json> {
        py.allow_threads(|| {
            let resp = self.inner()?;
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.json::<Json>())
                .map_err(wrap_rquest_error)
        })
    }

    /// Returns the bytes content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the bytes content of the response.
    pub fn bytes(&self, py: Python) -> PyResult<Py<PyAny>> {
        py.allow_threads(|| {
            let resp = self.inner()?;
            let buffer = pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.bytes())
                .map(BytesBuffer::new)
                .map_err(wrap_rquest_error)?;

            Python::with_gil(|py| buffer.into_bytes(py))
        })
    }

    /// Convert the response into a `Stream` of `Bytes` from the body.
    ///
    /// # Returns
    ///
    /// A Python object representing the stream content of the response.
    #[inline(always)]
    pub fn stream(&self, py: Python) -> PyResult<BlockingStreamer> {
        self.0.stream(py).map(BlockingStreamer)
    }

    /// Closes the response connection.
    #[inline(always)]
    pub fn close(&self, py: Python) -> PyResult<()> {
        self.0.close(py)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BlockingResponse {
    #[inline(always)]
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
        self.close(py)
    }
}

/// A blocking byte stream response.
/// An asynchronous iterator yielding data chunks from the response stream.
/// Used for streaming response content.
/// Employed in the `stream` method of the `Response` class.
/// Utilized in an asynchronous for loop in Python.
#[gen_stub_pyclass]
#[pyclass]
pub struct BlockingStreamer(async_impl::Streamer);

#[gen_stub_pymethods]
#[pymethods]
impl BlockingStreamer {
    #[inline(always)]
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&self, py: Python) -> PyResult<Py<PyAny>> {
        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(async_impl::Streamer::_anext(
                self.0.deref().clone(),
                py_stop_iteration_error,
            ))
        })
    }

    #[inline(always)]
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
        py.allow_threads(|| {
            let streamer = self.0.clone();
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                let mut lock = streamer.lock().await;
                Ok(drop(lock.take()))
            })
        })
    }
}
