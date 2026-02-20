use std::{fmt::Display, sync::Arc};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http::response::{Parts, Response as HttpResponse};
use http_body_util::BodyExt;
use pyo3::{coroutine::CancelHandle, prelude::*, pybacked::PyBackedStr};
use wreq::{self, Uri};

use crate::{
    buffer::PyBuffer,
    client::{
        SocketAddr,
        body::{Json, Streamer},
        nogil::NoGIL,
        resp::ext::ResponseExt,
    },
    cookie::Cookie,
    error::Error,
    header::HeaderMap,
    http::{StatusCode, Version},
    redirect::History,
    tls::TlsInfo,
};

/// A response from a request.
#[derive(Clone)]
#[pyclass(subclass, frozen, str, skip_from_py_object)]
pub struct Response {
    uri: Uri,
    parts: Parts,
    body: Arc<ArcSwapOption<Body>>,
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
        let response = HttpResponse::from(response);
        let (parts, body) = response.into_parts();
        Response {
            uri,
            parts,
            body: Arc::new(ArcSwapOption::from_pointee(Body::Streamable(body))),
        }
    }

    /// Builds a `wreq::Response` from the current response metadata and the given body.
    ///
    /// This creates a new HTTP response with the same version, status, headers, and extensions
    /// as the current response, but with the provided body.
    fn build_response(self, body: wreq::Body) -> wreq::Response {
        let mut response = HttpResponse::new(body);
        *response.version_mut() = self.parts.version;
        *response.status_mut() = self.parts.status;
        *response.headers_mut() = self.parts.headers;
        *response.extensions_mut() = self.parts.extensions;
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

    /// Get the status code of the response.
    #[getter]
    pub fn status(&self) -> StatusCode {
        StatusCode(self.parts.status)
    }

    /// Get the HTTP version of the response.
    #[getter]
    pub fn version(&self) -> Version {
        Version::from_ffi(self.parts.version)
    }

    /// Get the headers of the response.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        HeaderMap(self.parts.headers.clone())
    }

    /// Get the cookies of the response.
    #[getter]
    pub fn cookies(&self) -> Vec<Cookie> {
        Cookie::extract_headers_cookies(&self.parts.headers)
    }

    /// Get the content length of the response.
    #[getter]
    pub fn content_length(&self, py: Python) -> Option<u64> {
        py.detach(|| self.clone().empty_response().content_length())
    }

    /// Get the remote address of the response.
    #[getter]
    pub fn remote_addr(&self, py: Python) -> Option<SocketAddr> {
        py.detach(|| self.clone().empty_response().remote_addr().map(SocketAddr))
    }

    /// Get the local address of the response.
    #[getter]
    pub fn local_addr(&self, py: Python) -> Option<SocketAddr> {
        py.detach(|| self.clone().empty_response().local_addr().map(SocketAddr))
    }

    /// Get the redirect history of the Response.
    #[getter]
    pub fn history(&self, py: Python) -> Vec<History> {
        py.detach(|| {
            self.clone()
                .empty_response()
                .extensions()
                .get::<wreq::redirect::History>()
                .map_or_else(Vec::new, |history| {
                    history.into_iter().cloned().map(History::from).collect()
                })
        })
    }

    /// Get the TLS information of the response.
    #[getter]
    pub fn tls_info(&self, py: Python) -> Option<TlsInfo> {
        py.detach(|| {
            self.clone()
                .empty_response()
                .extensions()
                .get::<wreq::tls::TlsInfo>()
                .cloned()
                .map(TlsInfo)
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

    /// Get the text content with the response encoding, defaulting to utf-8 when unspecified.
    #[pyo3(signature = (encoding = None))]
    pub async fn text(
        &self,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
        encoding: Option<PyBackedStr>,
    ) -> PyResult<String> {
        let this = self.clone();
        let fut = async move {
            match encoding {
                Some(encoding) => this
                    .cache_response()
                    .and_then(|resp| ResponseExt::text_with_charset(resp, encoding))
                    .await
                    .map_err(Into::into),
                None => this
                    .cache_response()
                    .and_then(ResponseExt::text)
                    .await
                    .map_err(Into::into),
            }
        };
        NoGIL::new(fut, cancel).await
    }

    /// Get the JSON content of the response.
    pub async fn json(&self, #[pyo3(cancel_handle)] cancel: CancelHandle) -> PyResult<Json> {
        let fut = self
            .clone()
            .cache_response()
            .and_then(ResponseExt::json::<Json>)
            .map_err(Into::into);
        NoGIL::new(fut, cancel).await
    }

    /// Get the bytes content of the response.
    pub async fn bytes(&self, #[pyo3(cancel_handle)] cancel: CancelHandle) -> PyResult<PyBuffer> {
        let fut = self
            .clone()
            .cache_response()
            .and_then(ResponseExt::bytes)
            .map_ok(PyBuffer::from)
            .map_err(Into::into);
        NoGIL::new(fut, cancel).await
    }

    /// Close the response.
    ///
    /// **Current behavior:**
    /// - When connection pooling is **disabled**: This method closes the network connection.
    /// - When connection pooling is **enabled**: This method closes the response, prevents further body reads,
    ///   and returns the connection to the pool for reuse.
    ///
    /// **Future changes:**
    /// In future versions, this method will be changed to always close the network connection regardless of
    /// whether connection pooling is enabled or not.
    ///
    /// **Recommendation:**
    /// It is **not recommended** to manually call this method at present. Instead, use context managers
    /// (async with statement) to properly manage response lifecycle. Wait for the improved implementation
    /// in future versions.
    pub async fn close(&self) -> PyResult<()> {
        self.body.swap(None);
        Ok(())
    }
}

#[pymethods]
impl Response {
    #[inline]
    async fn __aenter__(slf: Py<Self>) -> PyResult<Py<Self>> {
        Ok(slf)
    }

    #[inline]
    async fn __aexit__(
        &self,
        _exc_type: Py<PyAny>,
        _exc_val: Py<PyAny>,
        _traceback: Py<PyAny>,
    ) -> PyResult<()> {
        self.close().await
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}({}) [{}] >",
            stringify!(Response),
            self.uri,
            self.parts.status,
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
        self.0.status()
    }

    /// Get the HTTP version of the response.
    #[getter]
    pub fn version(&self) -> Version {
        self.0.version()
    }

    /// Get the headers of the response.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        self.0.headers()
    }

    /// Get the cookies of the response.
    #[getter]
    pub fn cookies(&self) -> Vec<Cookie> {
        self.0.cookies()
    }

    /// Get the content length of the response.
    #[getter]
    pub fn content_length(&self, py: Python) -> Option<u64> {
        self.0.content_length(py)
    }

    /// Get the remote address of the response.
    #[getter]
    pub fn remote_addr(&self, py: Python) -> Option<SocketAddr> {
        self.0.remote_addr(py)
    }

    /// Get the local address of the response.
    #[getter]
    pub fn local_addr(&self, py: Python) -> Option<SocketAddr> {
        self.0.local_addr(py)
    }

    /// Get the redirect history of the Response.
    #[getter]
    pub fn history(&self, py: Python) -> Vec<History> {
        self.0.history(py)
    }

    /// Get the TLS information of the response.
    #[getter]
    pub fn tls_info(&self, py: Python) -> Option<TlsInfo> {
        self.0.tls_info(py)
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

    /// Get the text content with the response encoding, defaulting to utf-8 when unspecified.
    #[pyo3(signature = (encoding = None))]
    pub fn text(&self, py: Python, encoding: Option<PyBackedStr>) -> PyResult<String> {
        py.detach(|| match encoding {
            Some(encoding) => {
                let fut = self
                    .0
                    .clone()
                    .cache_response()
                    .and_then(|resp| ResponseExt::text_with_charset(resp, encoding))
                    .map_err(Into::into);
                pyo3_async_runtimes::tokio::get_runtime().block_on(fut)
            }
            None => {
                let fut = self
                    .0
                    .clone()
                    .cache_response()
                    .and_then(ResponseExt::text)
                    .map_err(Into::into);
                pyo3_async_runtimes::tokio::get_runtime().block_on(fut)
            }
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

    /// Close the response.
    ///
    /// **Current behavior:**
    /// - When connection pooling is **disabled**: This method closes the network connection.
    /// - When connection pooling is **enabled**: This method closes the response, prevents further body reads,
    ///   and returns the connection to the pool for reuse.
    ///
    /// **Future changes:**
    /// In future versions, this method will be changed to always close the network connection regardless of
    /// whether connection pooling is enabled or not.
    ///
    /// **Recommendation:**
    /// It is **not recommended** to manually call this method at present. Instead, use context managers
    /// (with statement) to properly manage response lifecycle. Wait for the improved implementation
    /// in future versions.
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
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
