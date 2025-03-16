use crate::{
    buffer::{Buffer, BytesBuffer, PyBufferProtocol},
    error::{memory_error, py_stop_async_iteration_error, wrap_rquest_error},
    typing::{Cookie, HeaderMap, Json, SocketAddr, StatusCode, Version},
};
use arc_swap::ArcSwapOption;
use futures_util::{Stream, TryStreamExt};
use mime::Mime;
use pyo3::{IntoPyObjectExt, prelude::*};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::{TlsInfo, Url, header};
use std::{ops::Deref, pin::Pin, sync::Arc};
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
///     print("Headers: ", response.headers)
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
pub struct Response {
    url: Url,
    version: Version,
    status_code: StatusCode,
    remote_addr: Option<SocketAddr>,
    content_length: Option<u64>,
    headers: rquest::header::HeaderMap,
    response: ArcSwapOption<rquest::Response>,
}

impl Response {
    /// Create a new `Response` instance.
    pub fn new(mut response: rquest::Response) -> Self {
        Response {
            url: response.url().clone(),
            version: Version::from_ffi(response.version()),
            status_code: StatusCode::from(response.status()),
            remote_addr: response.remote_addr().map(SocketAddr::from),
            content_length: response.content_length(),
            headers: std::mem::take(response.headers_mut()),
            response: ArcSwapOption::from_pointee(response),
        }
    }

    /// Consumes the `Response` and returns the inner `rquest::Response`.
    #[inline(always)]
    pub fn inner(&self) -> PyResult<rquest::Response> {
        self.response
            .swap(None)
            .and_then(Arc::into_inner)
            .ok_or_else(memory_error)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Response {
    /// Returns the URL of the response.
    ///
    /// # Returns
    ///
    /// A string representing the URL of the response.
    #[getter]
    #[inline(always)]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Returns whether the response is successful.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the response is successful.
    #[getter]
    #[inline(always)]
    pub fn ok(&self) -> bool {
        self.status_code.is_success()
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

    /// Returns the status code of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the HTTP status code.
    #[getter]
    #[inline(always)]
    pub fn status_code(&self) -> StatusCode {
        self.status_code
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
        HeaderMap(self.headers.clone())
    }

    /// Returns the cookies of the response.
    ///
    /// # Returns
    ///
    /// A Python cookies object representing the cookies of the response.
    #[getter]
    #[inline(always)]
    pub fn cookies<'py>(&'py self, py: Python<'py>) -> Vec<Cookie> {
        py.allow_threads(|| Cookie::extract_cookies(&self.headers))
    }

    /// Returns the content length of the response.
    ///
    /// # Returns
    ///
    /// An integer representing the content length of the response.
    #[getter]
    #[inline(always)]
    pub fn content_length(&self) -> u64 {
        self.content_length.unwrap_or_default()
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

    /// Encoding to decode with when accessing text.
    ///
    /// # Returns
    ///
    /// A string representing the encoding to decode with when accessing text.
    #[getter]
    pub fn encoding(&self, py: Python) -> String {
        py.allow_threads(|| {
            self.headers
                .get(header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<Mime>().ok())
                .and_then(|mime| {
                    mime.get_param("charset")
                        .map(|charset| charset.as_str().to_owned())
                })
                .unwrap_or_else(|| "utf-8".to_owned())
        })
    }

    /// Returns the TLS peer certificate of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the TLS peer certificate of the response.
    pub fn peer_certificate<'rt>(
        &'rt self,
        py: Python<'rt>,
    ) -> PyResult<Option<Bound<'rt, PyAny>>> {
        let s = py.allow_threads(|| {
            let resp_ref = self.response.load();
            let resp = resp_ref.as_ref()?;
            let val = resp.extensions().get::<TlsInfo>()?;
            val.peer_certificate()
                .map(ToOwned::to_owned)
                .map(Buffer::new)
        });

        s.map(|buffer| buffer.into_bytes_ref(py)).transpose()
    }

    /// Returns the text content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the text content of the response.
    pub fn text<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.inner()?;
        future_into_py(
            py,
            async move { resp.text().await.map_err(wrap_rquest_error) },
        )
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
    pub fn text_with_charset<'rt>(
        &self,
        py: Python<'rt>,
        encoding: String,
    ) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.inner()?;
        future_into_py(py, async move {
            resp.text_with_charset(&encoding)
                .await
                .map_err(wrap_rquest_error)
        })
    }

    /// Returns the JSON content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.inner()?;
        future_into_py(py, async move {
            resp.json::<Json>().await.map_err(wrap_rquest_error)
        })
    }

    /// Returns the bytes content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the bytes content of the response.
    pub fn bytes<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.inner()?;
        future_into_py(py, async move {
            let buffer = resp
                .bytes()
                .await
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
    pub fn stream(&self, py: Python) -> PyResult<Streamer> {
        py.allow_threads(|| {
            self.inner()
                .map(rquest::Response::bytes_stream)
                .map(Streamer::new)
        })
    }

    /// Closes the response connection.
    pub fn close(&self, py: Python) -> PyResult<()> {
        py.allow_threads(|| {
            let _ = self.inner().map(drop);
            Ok(())
        })
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Response {
    fn __aenter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'a>(
        &self,
        py: Python<'a>,
        _exc_type: &Bound<'a, PyAny>,
        _exc_value: &Bound<'a, PyAny>,
        _traceback: &Bound<'a, PyAny>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let res = self.close(py);
        future_into_py(py, async move { res })
    }
}

type InnerStreamer =
    Pin<Box<dyn Stream<Item = Result<bytes::Bytes, rquest::Error>> + Send + 'static>>;

/// A byte stream response.
/// An asynchronous iterator yielding data chunks from the response stream.
/// Used to stream response content.
/// Implemented in the `stream` method of the `Response` class.
/// Can be used in an asynchronous for loop in Python.
///
/// # Examples
///
/// ```python
/// import asyncio
/// import rnet
/// from rnet import Method, Impersonate
///
/// async def main():
///     resp = await rnet.get("https://httpbin.org/stream/20")
///     print("Status Code: ", resp.status_code)
///     print("Version: ", resp.version)
///     print("Response URL: ", resp.url)
///     print("Headers: ", resp.headers)
///     print("Content-Length: ", resp.content_length)
///     print("Encoding: ", resp.encoding)
///     print("Remote Address: ", resp.remote_addr)
///
///     async with resp.stream() as streamer:
///         async for chunk in streamer:
///             print("Chunk: ", chunk)
///             await asyncio.sleep(0.1)
///
/// if __name__ == "__main__":
///     asyncio.run(main())
/// ```
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct Streamer(Arc<Mutex<Option<InnerStreamer>>>);

impl Deref for Streamer {
    type Target = Arc<Mutex<Option<InnerStreamer>>>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Streamer {
    /// Create a new `Streamer` instance.
    #[inline(always)]
    pub fn new(
        stream: impl Stream<Item = Result<bytes::Bytes, rquest::Error>> + Send + 'static,
    ) -> Streamer {
        Streamer(Arc::new(Mutex::new(Some(Box::pin(stream)))))
    }

    pub async fn _anext<'rt>(
        streamer: Arc<Mutex<Option<InnerStreamer>>>,
        py_stop_iteration_error: fn() -> PyErr,
    ) -> PyResult<Py<PyAny>> {
        let mut lock = streamer.lock().await;
        let val = lock
            .as_mut()
            .ok_or_else(py_stop_iteration_error)?
            .try_next()
            .await;

        drop(lock);

        let buffer = val
            .map_err(wrap_rquest_error)?
            .map(BytesBuffer::new)
            .ok_or_else(&py_stop_iteration_error)?;

        Python::with_gil(|py| buffer.into_bytes(py))
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Streamer {
    #[inline(always)]
    fn __aiter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline(always)]
    fn __anext__<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        future_into_py(
            py,
            Streamer::_anext(self.0.clone(), py_stop_async_iteration_error),
        )
    }

    fn __aenter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'a>(
        &self,
        py: Python<'a>,
        _exc_type: &Bound<'a, PyAny>,
        _exc_value: &Bound<'a, PyAny>,
        _traceback: &Bound<'a, PyAny>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let streamer = self.0.clone();
        future_into_py(py, async move { Ok(drop(streamer.lock().await.take())) })
    }
}
