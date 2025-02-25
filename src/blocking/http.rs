use crate::{
    async_impl,
    error::{py_stop_iteration_error, wrap_rquest_error, wrap_serde_error},
    types::{HeaderMap, Json, SocketAddr, StatusCode, Version},
};
use futures_util::{Stream, StreamExt};
use indexmap::IndexMap;
use pyo3::{prelude::*, IntoPyObjectExt};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use serde_json::Value;

/// A bloking response from a request.
#[gen_stub_pyclass]
#[pyclass]
pub struct BlockingResponse {
    inner: async_impl::Response,
}

impl From<async_impl::Response> for BlockingResponse {
    fn from(response: async_impl::Response) -> Self {
        Self {
            inner: async_impl::Response::from(response),
        }
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
        self.inner.url()
    }

    /// Returns whether the response is successful.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the response is successful.
    #[getter]
    #[inline(always)]
    pub fn ok(&self) -> bool {
        self.inner.ok()
    }

    /// Returns the status code as integer of the response.
    ///
    /// # Returns
    ///
    /// An integer representing the HTTP status code.
    #[getter]
    #[inline(always)]
    pub fn status(&self) -> u16 {
        self.inner.status()
    }

    /// Returns the status code of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the HTTP status code.
    #[getter]
    #[inline(always)]
    pub fn status_code(&self) -> StatusCode {
        self.inner.status_code()
    }

    /// Returns the HTTP version of the response.
    ///
    /// # Returns
    ///
    /// A `Version` object representing the HTTP version of the response.
    #[getter]
    #[inline(always)]
    pub fn version(&self) -> Version {
        self.inner.version()
    }

    /// Returns the headers of the response.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` object representing the headers of the response.
    #[getter]
    #[inline(always)]
    pub fn headers(&self) -> HeaderMap {
        self.inner.headers()
    }

    /// Returns the content length of the response.
    ///
    /// # Returns
    ///
    /// An integer representing the content length of the response.
    #[getter]
    #[inline(always)]
    pub fn content_length(&self) -> u64 {
        self.inner.content_length()
    }

    /// Returns the remote address of the response.
    ///
    /// # Returns
    ///
    /// An `IpAddr` object representing the remote address of the response.
    #[getter]
    #[inline(always)]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.inner.remote_addr()
    }

    /// Encoding to decode with when accessing text.
    ///
    /// # Returns
    ///
    /// A string representing the encoding to decode with when accessing text.
    #[getter]
    pub fn encoding(&self) -> String {
        self.inner.encoding()
    }

    /// Returns the TLS peer certificate of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the TLS peer certificate of the response.
    pub fn peer_certificate(&self) -> PyResult<Option<Vec<u8>>> {
        self.inner.peer_certificate()
    }

    /// Returns the text content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the text content of the response.
    pub fn text(&self, py: Python) -> PyResult<String> {
        py.allow_threads(|| {
            let resp = self.into_inner()?;
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
            let resp = self.into_inner()?;
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
            let resp = self.into_inner()?;
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.json::<Json>())
                .map_err(wrap_rquest_error)
        })
    }

    /// Returns the JSON string content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json_str(&self, py: Python) -> PyResult<String> {
        py.allow_threads(|| {
            let resp = self.into_inner()?;
            let vlaue = pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.json::<Value>())
                .map_err(wrap_rquest_error)?;
            serde_json::to_string(&vlaue).map_err(wrap_serde_error)
        })
    }

    /// Returns the JSON pretty string content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json_str_pretty(&self, py: Python) -> PyResult<String> {
        py.allow_threads(|| {
            let resp = self.into_inner()?;
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                let json = resp.json::<Value>().await.map_err(wrap_rquest_error)?;
                serde_json::to_string_pretty(&json).map_err(wrap_serde_error)
            })
        })
    }

    /// Returns the bytes content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the bytes content of the response.
    pub fn bytes(&self, py: Python) -> PyResult<PyObject> {
        py.allow_threads(|| {
            let resp = self.into_inner()?;
            let bytes = pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.bytes())
                .map_err(wrap_rquest_error)?;
            Python::with_gil(|py| bytes.into_bound_py_any(py).map(|obj| obj.unbind()))
        })
    }

    /// Convert the response into a `Stream` of `Bytes` from the body.
    ///
    /// # Returns
    ///
    /// A Python object representing the stream content of the response.
    pub fn stream(&self) -> PyResult<BlockingStreamer> {
        self.into_inner()
            .map(rquest::Response::bytes_stream)
            .map(BlockingStreamer::new)
    }

    /// Closes the response connection.
    pub fn close(&self) {
        let _ = self.into_inner().map(drop);
    }
}

#[pymethods]
impl BlockingResponse {
    /// Returns the cookies of the response.
    ///
    /// # Returns
    ///
    /// A Python dictionary representing the cookies of the response.
    #[getter]
    pub fn cookies(&self) -> IndexMap<String, String> {
        self.inner.cookies()
    }
}

impl BlockingResponse {
    /// Consumes the `Response` and returns the inner `rquest::Response`.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing the inner `rquest::Response` if successful, or an error if the
    /// response has already been taken or cannot be unwrapped.
    ///
    /// # Errors
    ///
    /// Returns a memory error if the response has already been taken or if the `Arc` cannot be unwrapped.
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    fn into_inner(&self) -> PyResult<rquest::Response> {
        self.inner.into_inner()
    }
}

/// A bloking bytes streaming response.
/// This is an asynchronous iterator that yields chunks of data from the response stream.
/// This is used to stream the response content.
/// This is used in the `stream` method of the `Response` class.
/// This is used in an asynchronous for loop in Python.
#[gen_stub_pyclass]
#[pyclass]
pub struct BlockingStreamer(async_impl::Streamer);

impl BlockingStreamer {
    /// Create a new `Streamer` instance.
    ///
    /// # Arguments
    ///
    /// * `stream` - A stream of bytes.
    ///
    /// # Returns
    ///
    /// A new `Streamer` instance.
    fn new(
        stream: impl Stream<Item = Result<bytes::Bytes, rquest::Error>> + Send + 'static,
    ) -> BlockingStreamer {
        BlockingStreamer(async_impl::Streamer::new(stream))
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BlockingStreamer {
    #[inline(always)]
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> PyResult<Option<PyObject>> {
        // Here we clone the inner field, so we can use it
        // in our future.
        let streamer = self.0.clone();
        pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
            // Here we lock the mutex to access the data inside
            // and call next() method to get the next value.
            let mut lock = streamer.0.lock().await;
            let val = lock
                .as_mut()
                .ok_or_else(py_stop_iteration_error)?
                .next()
                .await;

            drop(lock);

            match val {
                Some(Ok(val)) => {
                    // If we have a value, we return it as a PyObject.
                    Python::with_gil(|py| Ok(Some(val.into_bound_py_any(py)?.unbind())))
                }
                Some(Err(err)) => Err(wrap_rquest_error(err)),
                // Here we return PyStopAsyncIteration error,
                // because python needs exceptions to tell that iterator
                // has ended.
                None => Err(py_stop_iteration_error()),
            }
        })
    }

    #[inline(always)]
    fn __enter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        slf.into_bound_py_any(py)
    }

    fn __exit__<'a>(
        &'a mut self,
        _exc_type: &Bound<'a, PyAny>,
        _exc_value: &Bound<'a, PyAny>,
        _traceback: &Bound<'a, PyAny>,
    ) -> PyResult<()> {
        let streamer = self.0.clone();
        pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
            let mut lock = streamer.0.lock().await;
            Ok(drop(lock.take()))
        })
    }
}
