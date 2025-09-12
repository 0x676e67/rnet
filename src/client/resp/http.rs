use std::sync::Arc;

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http::{Extensions, response::Response as HttpResponse};
use http_body_util::BodyExt;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use wreq::{self, ResponseBuilderExt, Uri, tls::TlsInfo};

use super::Streamer;
use crate::{
    buffer::PyBuffer,
    client::{SocketAddr, body::Json, nogil::NoGIL, resp::history::History},
    error::Error,
    http::{Version, cookie::Cookie, header::HeaderMap, status::StatusCode},
    rt::Runtime,
};

/// A response from a request.
#[pyclass(subclass, frozen)]
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
    body: ArcSwapOption<Body>,
    extensions: Extensions,
}

/// Represents the state of the HTTP response body.
enum Body {
    /// The body can be streamed once (not yet buffered).
    Streamable(wreq::Body),
    /// The body has been fully read into memory and can be reused.
    Reusable(Bytes),
}

/// A blocking response from a request.
#[pyclass(name = "Response", subclass, frozen)]
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
            local_addr,
            remote_addr,
            content_length,
            extensions: parts.extensions,
            version: Version::from_ffi(parts.version),
            status: StatusCode::from(parts.status),
            headers: HeaderMap(parts.headers),
            body: ArcSwapOption::from_pointee(Body::Streamable(body)),
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

    fn reuse_response(&self, py: Python, stream: bool) -> PyResult<wreq::Response> {
        py.detach(|| {
            let build_response = |body: wreq::Body| -> PyResult<wreq::Response> {
                HttpResponse::builder()
                    .uri(self.uri.clone())
                    .body(body)
                    .map(wreq::Response::from)
                    .map_err(Error::Builder)
                    .map_err(Into::into)
            };

            if let Some(arc) = self.body.swap(None) {
                return match Arc::try_unwrap(arc) {
                    Ok(Body::Streamable(body)) if stream => build_response(body),
                    Ok(Body::Streamable(body)) if !stream => {
                        let bytes = Runtime::block_on(BodyExt::collect(body))
                            .map(|buf| buf.to_bytes())
                            .map_err(Error::Library)?;

                        self.body
                            .store(Some(Arc::new(Body::Reusable(bytes.clone()))));
                        build_response(wreq::Body::from(bytes))
                    }
                    Ok(Body::Reusable(bytes)) if !stream => {
                        self.body
                            .store(Some(Arc::new(Body::Reusable(bytes.clone()))));
                        build_response(wreq::Body::from(bytes))
                    }
                    _ => Err(Error::Memory.into()),
                };
            }

            Err(Error::Memory.into())
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

    /// Get the response into a `Stream` of `Bytes` from the body.
    pub fn stream(&self, py: Python) -> PyResult<Streamer> {
        self.reuse_response(py, true)
            .map(wreq::Response::bytes_stream)
            .map(Streamer::new)
    }

    /// Get the text content of the response.
    pub fn text<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .reuse_response(py, false)?
            .text()
            .map_err(Error::Library)
            .map_err(Into::into);
        NoGIL::future(py, fut)
    }

    /// Get the full response text given a specific encoding.
    #[pyo3(signature = (encoding))]
    pub fn text_with_charset<'py>(
        &self,
        py: Python<'py>,
        encoding: PyBackedStr,
    ) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .reuse_response(py, false)?
            .text_with_charset(encoding)
            .map_err(Error::Library)
            .map_err(Into::into);
        NoGIL::future(py, fut)
    }

    /// Get the JSON content of the response.
    pub fn json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .reuse_response(py, false)?
            .json::<Json>()
            .map_err(Error::Library)
            .map_err(Into::into);
        NoGIL::future(py, fut)
    }

    /// Get the bytes content of the response.
    pub fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .reuse_response(py, false)?
            .bytes()
            .map_ok(PyBuffer::from)
            .map_err(Error::Library)
            .map_err(Into::into);
        NoGIL::future(py, fut)
    }

    /// Close the response connection.
    pub fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        py.detach(|| self.body.swap(None));
        NoGIL::closure(py, || Ok(()))
    }
}

#[pymethods]
impl Response {
    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        NoGIL::closure(py, || Ok(slf))
    }

    #[inline]
    fn __aexit__<'py>(
        &self,
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

    /// Get the response into a `Stream` of `Bytes` from the body.
    #[inline]
    pub fn stream(&self, py: Python) -> PyResult<Streamer> {
        self.0.stream(py)
    }

    /// Get the text content of the response.
    pub fn text(&self, py: Python) -> PyResult<String> {
        let resp = self.0.reuse_response(py, false)?;
        py.detach(|| {
            Runtime::block_on(resp.text())
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Get the full response text given a specific encoding.
    #[pyo3(signature = (encoding))]
    pub fn text_with_charset(&self, py: Python, encoding: PyBackedStr) -> PyResult<String> {
        let resp = self.0.reuse_response(py, false)?;
        py.detach(|| {
            Runtime::block_on(resp.text_with_charset(&encoding))
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Get the JSON content of the response.
    pub fn json(&self, py: Python) -> PyResult<Json> {
        let resp = self.0.reuse_response(py, false)?;
        py.detach(|| {
            Runtime::block_on(resp.json::<Json>())
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Get the bytes content of the response.
    pub fn bytes(&self, py: Python) -> PyResult<PyBuffer> {
        let resp = self.0.reuse_response(py, false)?;
        py.detach(|| {
            Runtime::block_on(resp.bytes())
                .map(PyBuffer::from)
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

    /// Close the response connection.
    #[inline]
    pub fn close(&self, py: Python) {
        py.detach(|| self.0.body.swap(None));
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
        &self,
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
