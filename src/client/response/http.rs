use futures_util::TryFutureExt;
use mime::Mime;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use wreq::{Url, header, tls::TlsInfo};

use super::Streamer;
use crate::{
    buffer::{Buffer, BytesBuffer, PyBufferProtocol},
    client::{SocketAddr, body::Json},
    error::Error,
    http::{Version, cookie::Cookie, header::HeaderMap, status::StatusCode},
};

/// A response from a request.
#[pyclass(subclass)]
pub struct Response {
    url: Url,
    version: Version,
    status: StatusCode,
    remote_addr: Option<SocketAddr>,
    local_addr: Option<SocketAddr>,
    content_length: Option<u64>,
    headers: HeaderMap,
    response: Option<wreq::Response>,
}

/// A blocking response from a request.
#[pyclass(name = "Response", subclass)]
pub struct BlockingResponse(Response);

// ===== impl Response =====

impl Response {
    /// Create a new [`Response`] instance.
    pub fn new(mut response: wreq::Response) -> Self {
        Response {
            url: response.url().clone(),
            version: Version::from_ffi(response.version()),
            status: StatusCode::from(response.status()),
            remote_addr: response.remote_addr().map(SocketAddr),
            local_addr: response.local_addr().map(SocketAddr),
            content_length: response.content_length(),
            headers: HeaderMap(std::mem::take(response.headers_mut())),
            response: Some(response),
        }
    }

    fn inner(&mut self) -> PyResult<wreq::Response> {
        self.response
            .take()
            .ok_or_else(|| Error::Memory)
            .map_err(Into::into)
    }
}

#[pymethods]
impl Response {
    /// Returns the URL of the response.
    #[inline]
    #[getter]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

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

    /// Returns the content length of the response.
    #[inline]
    #[getter]
    pub fn content_length(&self) -> u64 {
        self.content_length.unwrap_or_default()
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

    /// Encoding to decode with when accessing text.
    #[getter]
    pub fn encoding(&self, py: Python) -> String {
        py.allow_threads(|| {
            self.headers
                .0
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
            let resp = self.response.as_ref()?;
            let val = resp.extensions().get::<TlsInfo>()?;
            val.peer_certificate()
                .map(ToOwned::to_owned)
                .map(Buffer::new)
        });

        s.map(|buffer| buffer.into_bytes_ref(py)).transpose()
    }

    /// Returns the text content of the response.
    pub fn text<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let resp = self.inner()?;
        future_into_py(py, resp.text().map_err(Error::Library).map_err(Into::into))
    }

    /// Returns the text content of the response with a specific charset.
    pub fn text_with_charset<'py>(
        &mut self,
        py: Python<'py>,
        encoding: PyBackedStr,
    ) -> PyResult<Bound<'py, PyAny>> {
        let resp = self.inner()?;
        let fut = async move {
            resp.text_with_charset(&encoding)
                .await
                .map_err(Error::Library)
                .map_err(Into::into)
        };
        future_into_py(py, fut)
    }

    /// Returns the JSON content of the response.
    pub fn json<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let resp = self.inner()?;
        future_into_py(
            py,
            resp.json::<Json>()
                .map_err(Error::Library)
                .map_err(Into::into),
        )
    }

    /// Returns the bytes content of the response.
    pub fn bytes<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let resp = self.inner()?;
        let fut = async move {
            let buffer = resp
                .bytes()
                .await
                .map(BytesBuffer::new)
                .map_err(Error::Library)?;
            Python::with_gil(|py| buffer.into_bytes(py))
        };
        future_into_py(py, fut)
    }

    /// Convert the response into a `Stream` of `Bytes` from the body.
    pub fn stream(&mut self, py: Python) -> PyResult<Streamer> {
        py.allow_threads(|| {
            self.inner()
                .map(wreq::Response::bytes_stream)
                .map(Streamer::new)
        })
    }

    /// Closes the response connection.
    pub fn close<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner();
        future_into_py(py, async move { Ok(inner.ok().map(drop)) })
    }
}

#[pymethods]
impl Response {
    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    #[inline]
    fn __aexit__<'py>(
        &'py mut self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.close(py)
    }
}

// ===== impl BlockingResponse =====

#[pymethods]
impl BlockingResponse {
    /// Returns the URL of the response.
    #[inline]
    #[getter]
    pub fn url(&self) -> &str {
        self.0.url()
    }

    /// Returns the status code of the response.
    #[inline]
    #[getter]
    pub fn status(&self) -> StatusCode {
        self.0.status()
    }

    /// Returns the HTTP version of the response.
    #[inline]
    #[getter]
    pub fn version(&self) -> Version {
        self.0.version()
    }

    /// Returns the headers of the response.
    #[inline]
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        self.0.headers()
    }

    /// Returns the cookies of the response.
    #[inline]
    #[getter]
    pub fn cookies(&self, py: Python) -> Vec<Cookie> {
        self.0.cookies(py)
    }

    /// Returns the content length of the response.
    #[inline]
    #[getter]
    pub fn content_length(&self) -> u64 {
        self.0.content_length()
    }

    /// Returns the remote address of the response.
    #[inline]
    #[getter]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.0.remote_addr()
    }

    /// Returns the local address of the response.
    #[inline]
    #[getter]
    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.0.local_addr()
    }

    /// Encoding to decode with when accessing text.
    #[inline]
    #[getter]
    pub fn encoding(&self, py: Python) -> String {
        self.0.encoding(py)
    }

    /// Returns the TLS peer certificate of the response.
    #[inline]
    pub fn peer_certificate<'py>(
        &'py self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        self.0.peer_certificate(py)
    }

    /// Returns the text content of the response.
    pub fn text(&mut self, py: Python) -> PyResult<String> {
        py.allow_threads(|| {
            let resp = self.0.inner()?;
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.text())
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Returns the text content of the response with a specific charset.
    pub fn text_with_charset(&mut self, py: Python, encoding: PyBackedStr) -> PyResult<String> {
        py.allow_threads(|| {
            let resp = self.0.inner()?;
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.text_with_charset(&encoding))
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Returns the JSON content of the response.
    pub fn json(&mut self, py: Python) -> PyResult<Json> {
        py.allow_threads(|| {
            let resp = self.0.inner()?;
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.json::<Json>())
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Returns the bytes content of the response.
    pub fn bytes(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        py.allow_threads(|| {
            let resp = self.0.inner()?;
            let buffer = pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.bytes())
                .map(BytesBuffer::new)
                .map_err(Error::Library)?;

            Python::with_gil(|py| buffer.into_bytes(py))
        })
    }

    /// Convert the response into a `Stream` of `Bytes` from the body.
    #[inline]
    pub fn stream(&mut self, py: Python) -> PyResult<Streamer> {
        self.0.stream(py)
    }

    /// Closes the response connection.
    #[inline]
    pub fn close(&mut self, py: Python) {
        py.allow_threads(|| {
            let _ = self.0.inner().map(drop);
        })
    }
}

#[pymethods]
impl BlockingResponse {
    #[inline]
    fn __enter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[inline]
    fn __exit__<'py>(
        &mut self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) {
        self.close(py)
    }
}

impl From<Response> for BlockingResponse {
    #[inline]
    fn from(response: Response) -> Self {
        Self(response)
    }
}
