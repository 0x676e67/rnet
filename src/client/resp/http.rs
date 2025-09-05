use futures_util::TryFutureExt;
use http::{Extensions, response::Response as HttpResponse};
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use wreq::{self, ResponseBuilderExt, Uri, tls::TlsInfo};

use super::Streamer;
use crate::{
    buffer::PyBuffer,
    client::{
        SocketAddr,
        body::Json,
        resp::{future::AllowThreads, history::History},
    },
    error::Error,
    http::{Version, cookie::Cookie, header::HeaderMap, status::StatusCode},
};

/// A response from a request.
#[pyclass(subclass)]
pub struct Response {
    /// Get the status code of the response.
    #[pyo3(get)]
    version: Version,

    /// Get the HTTP version of the response.
    #[pyo3(get)]
    status: StatusCode,

    /// Get the content length of the response.
    #[pyo3(get)]
    content_length: Option<u64>,

    /// Get the headers of the response.
    #[pyo3(get)]
    headers: HeaderMap,

    /// Get the local address of the response.
    #[pyo3(get)]
    local_addr: Option<SocketAddr>,

    /// Get the content length of the response.
    #[pyo3(get)]
    remote_addr: Option<SocketAddr>,

    uri: Uri,

    body: Body,

    extensions: Extensions,
}

/// Represents the state of the HTTP response body.
enum Body {
    /// The body can be streamed once (not yet buffered).
    Streamable(wreq::Body),
    /// The body has been fully read into memory and can be reused.
    Reusable(wreq::Body),
    /// The body has already been consumed and is no longer available.
    Consumed,
}

/// A blocking response from a request.
#[pyclass(name = "Response", subclass)]
pub struct BlockingResponse(Response);

// ===== impl Response =====

impl Response {
    /// Create a new [`Response`] instance.
    pub fn new(response: wreq::Response) -> Self {
        let uri = response.uri().clone();
        let content_length = response.content_length();
        let local_addr = response.local_addr().map(SocketAddr);
        let remote_addr = response.remote_addr().map(SocketAddr);
        let response = HttpResponse::from(response);
        let (parts, body) = response.into_parts();

        Response {
            uri,
            content_length,
            local_addr,
            remote_addr,
            version: Version::from_ffi(parts.version),
            status: StatusCode::from(parts.status),
            headers: HeaderMap(parts.headers),
            body: Body::Streamable(body),
            extensions: parts.extensions,
        }
    }

    fn ext_response(&self) -> wreq::Response {
        let mut response = HttpResponse::builder()
            .uri(self.uri.clone())
            .body(wreq::Body::default())
            .map(wreq::Response::from)
            .expect("build response from parts should not fail");
        *response.extensions_mut() = self.extensions.clone();
        response
    }

    fn reuse_response(&mut self, py: Python, stream: bool) -> PyResult<wreq::Response> {
        use http_body_util::BodyExt;

        // Helper to build a response from a body
        let build_response = |body: wreq::Body| -> PyResult<wreq::Response> {
            HttpResponse::builder()
                .uri(self.uri.clone())
                .body(body)
                .map(wreq::Response::from)
                .map_err(Error::Builder)
                .map_err(Into::into)
        };

        py.detach(|| {
            if stream {
                // Only allow streaming if the body is in MayStream state
                match std::mem::replace(&mut self.body, Body::Consumed) {
                    Body::Streamable(body) => build_response(body),
                    _ => Err(Error::Memory.into()),
                }
            } else {
                // For non-streaming, allow reuse if possible
                match &mut self.body {
                    Body::Streamable(body) | Body::Reusable(body) => {
                        let bytes = pyo3_async_runtimes::tokio::get_runtime()
                            .block_on(BodyExt::collect(body))
                            .map(|buf| buf.to_bytes())
                            .map_err(Error::Library)?;

                        self.body = Body::Reusable(wreq::Body::from(bytes.clone()));
                        build_response(wreq::Body::from(bytes))
                    }
                    Body::Consumed => Err(Error::Memory.into()),
                }
            }
        })
    }
}

#[pymethods]
impl Response {
    /// Get the URL of the response.
    #[getter]
    pub fn url(&self) -> String {
        self.uri.to_string()
    }

    /// Get the cookies of the response.
    #[getter]
    pub fn cookies(&self) -> Vec<Cookie> {
        Cookie::extract_headers_cookies(&self.headers.0)
    }

    /// Get the redirect history of the Response.
    #[getter]
    pub fn history(&self, py: Python) -> Vec<History> {
        py.detach(|| {
            self.ext_response()
                .history()
                .cloned()
                .map(History::from)
                .collect()
        })
    }

    /// Get the DER encoded leaf certificate of the response.
    #[getter]
    pub fn peer_certificate(&self, py: Python) -> Option<PyBuffer> {
        py.detach(|| {
            self.extensions
                .get::<TlsInfo>()?
                .peer_certificate()
                .map(ToOwned::to_owned)
                .map(PyBuffer::from)
        })
    }

    /// Get the text content of the response.
    pub fn text<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .reuse_response(py, false)?
            .text()
            .map_err(Error::Library)
            .map_err(Into::into);
        AllowThreads::future(fut).future_into_py(py)
    }

    /// Get the full response text given a specific encoding.
    #[pyo3(signature = (encoding))]
    pub fn text_with_charset<'py>(
        &mut self,
        py: Python<'py>,
        encoding: PyBackedStr,
    ) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .reuse_response(py, false)?
            .text_with_charset(encoding)
            .map_err(Error::Library)
            .map_err(Into::into);
        AllowThreads::future(fut).future_into_py(py)
    }

    /// Get the JSON content of the response.
    pub fn json<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .reuse_response(py, false)?
            .json::<Json>()
            .map_err(Error::Library)
            .map_err(Into::into);
        AllowThreads::future(fut).future_into_py(py)
    }

    /// Get the bytes content of the response.
    pub fn bytes<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .reuse_response(py, false)?
            .bytes()
            .map_ok(PyBuffer::from)
            .map_err(Error::Library)
            .map_err(Into::into);
        AllowThreads::future(fut).future_into_py(py)
    }

    /// Get the response into a `Stream` of `Bytes` from the body.
    pub fn stream(&mut self, py: Python) -> PyResult<Streamer> {
        self.reuse_response(py, true)
            .map(wreq::Response::bytes_stream)
            .map(Streamer::new)
    }

    /// Close the response connection.
    pub fn close<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.body = Body::Consumed;
        AllowThreads::closure(|| Ok(())).future_into_py(py)
    }
}

#[pymethods]
impl Response {
    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        AllowThreads::closure(|| Ok(slf)).future_into_py(py)
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
    /// Get the URL of the response.
    #[getter]
    pub fn url(&self) -> String {
        self.0.url()
    }

    /// Get the status code of the response.
    #[getter]
    pub fn status(&self) -> StatusCode {
        self.0.status
    }

    /// Get the HTTP version of the response.
    #[getter]
    pub fn version(&self) -> Version {
        self.0.version
    }

    /// Get the headers of the response.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        self.0.headers.clone()
    }

    /// Get the cookies of the response.
    #[getter]
    pub fn cookies(&self) -> Vec<Cookie> {
        self.0.cookies()
    }

    /// Get the content length of the response.
    #[getter]
    pub fn content_length(&self) -> Option<u64> {
        self.0.content_length
    }

    /// Get the remote address of the response.
    #[getter]
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.0.remote_addr
    }

    /// Get the local address of the response.
    #[getter]
    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.0.local_addr
    }

    /// Get the redirect history of the Response.
    #[getter]
    pub fn history(&self, py: Python) -> Vec<History> {
        self.0.history(py)
    }

    /// Get the DER encoded leaf certificate of the response.
    #[getter]
    pub fn peer_certificate(&self, py: Python) -> Option<PyBuffer> {
        self.0.peer_certificate(py)
    }

    /// Get the text content of the response.
    pub fn text(&mut self, py: Python) -> PyResult<String> {
        let resp = self.0.reuse_response(py, false)?;
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.text())
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Get the full response text given a specific encoding.
    #[pyo3(signature = (encoding))]
    pub fn text_with_charset(&mut self, py: Python, encoding: PyBackedStr) -> PyResult<String> {
        let resp = self.0.reuse_response(py, false)?;
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.text_with_charset(&encoding))
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Get the JSON content of the response.
    pub fn json(&mut self, py: Python) -> PyResult<Json> {
        let resp = self.0.reuse_response(py, false)?;
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.json::<Json>())
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Get the bytes content of the response.
    pub fn bytes(&mut self, py: Python) -> PyResult<PyBuffer> {
        let resp = self.0.reuse_response(py, false)?;
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(resp.bytes())
                .map(PyBuffer::from)
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Get the response into a `Stream` of `Bytes` from the body.
    #[inline]
    pub fn stream(&mut self, py: Python) -> PyResult<Streamer> {
        self.0.stream(py)
    }

    /// Close the response connection.
    #[inline]
    pub fn close(&mut self) {
        self.0.body = Body::Consumed;
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
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) {
        self.close()
    }
}

impl From<Response> for BlockingResponse {
    #[inline]
    fn from(response: Response) -> Self {
        Self(response)
    }
}
