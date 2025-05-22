use crate::{
    buffer::{Buffer, BytesBuffer, PyBufferProtocol},
    error::Error,
    typing::{Cookie, HeaderMap, Json, SocketAddr, StatusCode, Version},
};
use arc_swap::ArcSwapOption;
use futures_util::{Stream, TryStreamExt};
use mime::Mime;
use pyo3::{IntoPyObjectExt, prelude::*};
use pyo3_async_runtimes::tokio::future_into_py;
use rquest::{TlsInfo, Url, header};
use std::{ops::Deref, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

/// A response from a request.
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
            remote_addr: response.remote_addr().map(SocketAddr),
            content_length: response.content_length(),
            headers: std::mem::take(response.headers_mut()),
            response: ArcSwapOption::from_pointee(response),
        }
    }

    /// Consumes the `Response` and returns the inner `rquest::Response`.
    pub fn inner(&self) -> PyResult<rquest::Response> {
        self.response
            .swap(None)
            .and_then(Arc::into_inner)
            .ok_or_else(|| Error::Memory)
            .map_err(Into::into)
    }
}

#[pymethods]
impl Response {
    /// Returns the URL of the response.
    #[getter]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Returns whether the response is successful.
    #[getter]
    pub fn ok(&self) -> bool {
        self.status_code.is_success()
    }

    /// Returns the status code as integer of the response.
    #[getter]
    pub fn status(&self) -> u16 {
        self.status_code.as_int()
    }

    /// Returns the status code of the response.
    #[getter]
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Returns the HTTP version of the response.
    #[getter]
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns the headers of the response.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        HeaderMap(self.headers.clone())
    }

    /// Returns the cookies of the response.
    #[getter]
    pub fn cookies(&self, py: Python) -> Vec<Cookie> {
        py.allow_threads(|| Cookie::extract_cookies(&self.headers))
    }

    /// Returns the content length of the response.
    #[getter]
    pub fn content_length(&self) -> u64 {
        self.content_length.unwrap_or_default()
    }

    /// Returns the remote address of the response.
    #[getter]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    /// Encoding to decode with when accessing text.
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
    pub fn peer_certificate<'py>(
        &'py self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
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
    pub fn text<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let resp = self.inner()?;
        future_into_py(py, async move {
            resp.text()
                .await
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }

    /// Returns the text content of the response with a specific charset.
    pub fn text_with_charset<'py>(
        &self,
        py: Python<'py>,
        encoding: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let resp = self.inner()?;
        future_into_py(py, async move {
            resp.text_with_charset(&encoding)
                .await
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }

    /// Returns the JSON content of the response.
    pub fn json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let resp = self.inner()?;
        future_into_py(py, async move {
            resp.json::<Json>()
                .await
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }

    /// Returns the bytes content of the response.
    pub fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let resp = self.inner()?;
        future_into_py(py, async move {
            let buffer = resp
                .bytes()
                .await
                .map(BytesBuffer::new)
                .map_err(Error::Request)?;
            Python::with_gil(|py| buffer.into_bytes(py))
        })
    }

    /// Convert the response into a `Stream` of `Bytes` from the body.
    pub fn stream(&self, py: Python) -> PyResult<Streamer> {
        py.allow_threads(|| {
            self.inner()
                .map(rquest::Response::bytes_stream)
                .map(Streamer::new)
        })
    }

    /// Closes the response connection.
    pub fn close<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let res = self.inner();
        future_into_py(py, async move {
            let _ = res.map(drop);
            Ok(())
        })
    }
}

#[pymethods]
impl Response {
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'py>(
        &'py self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.close(py)
    }
}

type InnerStreamer = Pin<Box<dyn Stream<Item = rquest::Result<bytes::Bytes>> + Send + 'static>>;

/// A byte stream response.
/// An asynchronous iterator yielding data chunks from the response stream.
/// Used to stream response content.
/// Implemented in the `stream` method of the `Response` class.
/// Can be used in an asynchronous for loop in Python.
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
    pub fn new(
        stream: impl Stream<Item = rquest::Result<bytes::Bytes>> + Send + 'static,
    ) -> Streamer {
        Streamer(Arc::new(Mutex::new(Some(Box::pin(stream)))))
    }

    pub async fn _anext(
        streamer: Arc<Mutex<Option<InnerStreamer>>>,
        error: fn() -> PyErr,
    ) -> PyResult<Py<PyAny>> {
        let mut lock = streamer.lock().await;
        let val = lock.as_mut().ok_or_else(error)?.try_next().await;

        drop(lock);

        let buffer = val
            .map_err(Error::Request)?
            .map(BytesBuffer::new)
            .ok_or_else(error)?;

        Python::with_gil(|py| buffer.into_bytes(py))
    }
}

#[pymethods]
impl Streamer {
    fn __aiter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(
            py,
            Streamer::_anext(self.0.clone(), || Error::StopAsyncIteration.into()),
        )
    }

    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let streamer = self.0.clone();
        future_into_py(py, async move {
            drop(streamer.lock().await.take());
            Ok(())
        })
    }
}
