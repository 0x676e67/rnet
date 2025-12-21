use std::{fmt::Display, future, sync::Arc};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http::{Extensions, response::Response as HttpResponse};
use http_body_util::BodyExt;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use wreq::{self, Uri, tls::TlsInfo};

use crate::{
    buffer::PyBuffer,
    client::{
        SocketAddr,
        body::{Json, Streamer},
        resp::ext::ResponseExt,
    },
    cookie::Cookie,
    error::Error,
    header::HeaderMap,
    http::{StatusCode, Version},
    redirect::History,
};

/// A response from a request.
#[derive(Clone)]
#[pyclass(subclass, frozen, str)]
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
    body: Arc<ArcSwapOption<Body>>,
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
#[pyclass(name = "Response", subclass, frozen, str)]
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
            body: Arc::new(ArcSwapOption::from_pointee(Body::Streamable(body))),
        }
    }

    /// Builds a `wreq::Response` from the current response metadata and the given body.
    ///
    /// This creates a new HTTP response with the same version, status, headers, and extensions
    /// as the current response, but with the provided body.
    fn build_response(self, body: wreq::Body) -> wreq::Response {
        let mut response = HttpResponse::new(body);
        *response.version_mut() = self.version.into_ffi();
        *response.status_mut() = self.status.0;
        *response.headers_mut() = self.headers.0;
        *response.extensions_mut() = self.extensions;
        wreq::Response::from(response)
    }

    /// Creates an empty response with the same metadata but no body content.
    ///
    /// Useful for operations that only need response headers/metadata without consuming the body.
    fn empty_response(self) -> wreq::Response {
        self.build_response(wreq::Body::from(Bytes::new()))
    }

    /// Consumes the response body and caches it in memory for reuse.
    ///
    /// If the body is streamable, it will be fully read into memory and cached.
    /// If the body is already cached, it will be cloned and reused.
    /// Returns an error if the body has already been consumed or if reading fails.
    async fn cache_response(self) -> Result<wreq::Response, Error> {
        if let Some(arc) = self.body.swap(None) {
            match Arc::try_unwrap(arc) {
                Ok(Body::Streamable(body)) => {
                    let bytes = BodyExt::collect(body)
                        .await
                        .map(|buf| buf.to_bytes())
                        .map_err(Error::Library)?;

                    self.body
                        .store(Some(Arc::new(Body::Reusable(bytes.clone()))));
                    Ok(self.build_response(wreq::Body::from(bytes)))
                }
                Ok(Body::Reusable(bytes)) => {
                    self.body
                        .store(Some(Arc::new(Body::Reusable(bytes.clone()))));
                    Ok(self.build_response(wreq::Body::from(bytes)))
                }
                _ => Err(Error::Memory),
            }
        } else {
            Err(Error::Memory)
        }
    }

    /// Consumes the response body for streaming without caching.
    ///
    /// This method transfers ownership of the streamable body for one-time use.
    /// Returns an error if the body has already been consumed or is not streamable.
    fn stream_response(self) -> Result<wreq::Response, Error> {
        if let Some(arc) = self.body.swap(None) {
            if let Ok(Body::Streamable(body)) = Arc::try_unwrap(arc) {
                return Ok(self.build_response(body));
            }
        }
        Err(Error::Memory)
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
            self.extensions
                .get::<wreq::redirect::History>()
                .map_or_else(Vec::new, |history| {
                    history.into_iter().cloned().map(History::from).collect()
                })
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

    /// Turn a response into an error if the server returned an error.
    pub fn raise_for_status(&self) -> PyResult<()> {
        self.clone()
            .empty_response()
            .error_for_status()
            .map(|_| ())
            .map_err(Error::Library)
            .map_err(Into::into)
    }

    /// Get the response into a `Stream` of `Bytes` from the body.
    pub fn stream(&self) -> PyResult<Streamer> {
        self.clone()
            .stream_response()
            .map(Streamer::new)
            .map_err(Into::into)
    }

    /// Get the text content of the response.
    pub fn text<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .clone()
            .cache_response()
            .and_then(ResponseExt::text)
            .map_err(Into::into);
        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Get the full response text given a specific encoding.
    #[pyo3(signature = (encoding))]
    pub fn text_with_charset<'py>(
        &self,
        py: Python<'py>,
        encoding: PyBackedStr,
    ) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .clone()
            .cache_response()
            .and_then(|resp| ResponseExt::text_with_charset(resp, encoding))
            .map_err(Into::into);
        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Get the JSON content of the response.
    pub fn json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .clone()
            .cache_response()
            .and_then(ResponseExt::json::<Json>)
            .map_err(Into::into);
        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Get the bytes content of the response.
    pub fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fut = self
            .clone()
            .cache_response()
            .and_then(ResponseExt::bytes)
            .map_ok(PyBuffer::from)
            .map_err(Into::into);
        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Close the response connection.
    pub fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        py.detach(|| self.body.clone().swap(None));
        pyo3_async_runtimes::tokio::future_into_py(py, future::ready(Ok(())))
    }
}

#[pymethods]
impl Response {
    #[inline]
    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let slf = slf.into_py_any(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, future::ready(Ok(slf)))
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

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}({}) [{}] >",
            stringify!(Response),
            self.uri,
            self.status.0,
        )
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

    /// Turn a response into an error if the server returned an error.
    #[inline]
    pub fn raise_for_status(&self) -> PyResult<()> {
        self.0.raise_for_status()
    }

    /// Get the response into a `Stream` of `Bytes` from the body.
    #[inline]
    pub fn stream(&self) -> PyResult<Streamer> {
        self.0.stream()
    }

    /// Get the text content of the response.
    pub fn text(&self, py: Python) -> PyResult<String> {
        py.detach(|| {
            let fut = self
                .0
                .clone()
                .cache_response()
                .and_then(ResponseExt::text)
                .map_err(Into::into);
            pyo3_async_runtimes::tokio::get_runtime().block_on(fut)
        })
    }

    /// Get the full response text given a specific encoding.
    #[pyo3(signature = (encoding))]
    pub fn text_with_charset(&self, py: Python, encoding: PyBackedStr) -> PyResult<String> {
        py.detach(|| {
            let fut = self
                .0
                .clone()
                .cache_response()
                .and_then(|resp| ResponseExt::text_with_charset(resp, encoding))
                .map_err(Into::into);
            pyo3_async_runtimes::tokio::get_runtime().block_on(fut)
        })
    }

    /// Get the JSON content of the response.
    pub fn json(&self, py: Python) -> PyResult<Json> {
        py.detach(|| {
            let fut = self
                .0
                .clone()
                .cache_response()
                .and_then(ResponseExt::json::<Json>)
                .map_err(Into::into);
            pyo3_async_runtimes::tokio::get_runtime().block_on(fut)
        })
    }

    /// Get the bytes content of the response.
    pub fn bytes(&self, py: Python) -> PyResult<PyBuffer> {
        py.detach(|| {
            let fut = self
                .0
                .clone()
                .cache_response()
                .and_then(ResponseExt::bytes)
                .map_ok(PyBuffer::from)
                .map_err(Into::into);
            pyo3_async_runtimes::tokio::get_runtime().block_on(fut)
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

impl Display for BlockingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}({}) [{}] >",
            stringify!(Response),
            self.0.uri,
            self.0.status.0,
        )
    }
}
