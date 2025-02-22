mod stream;

pub use self::stream::Streamer;
use crate::{
    error::{memory_error, wrap_rquest_error, wrap_serde_error},
    types::{HeaderMap, Json, SocketAddr, StatusCode, Version},
};
use arc_swap::ArcSwapOption;
use indexmap::IndexMap;
use mime::Mime;
use pyo3::{prelude::*, IntoPyObjectExt};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::{header, TlsInfo, Url};
use serde_json::Value;
use std::sync::Arc;

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
///     print("Headers: ", response.headers.to_dict())
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
    headers: HeaderMap,
    response: ArcSwapOption<rquest::Response>,
}

impl From<rquest::Response> for Response {
    fn from(mut response: rquest::Response) -> Self {
        Response {
            url: response.url().clone(),
            version: Version::from_ffi(response.version()),
            status_code: StatusCode::from(response.status()),
            remote_addr: response.remote_addr().map(SocketAddr::from),
            content_length: response.content_length(),
            headers: HeaderMap::from(std::mem::take(response.headers_mut())),
            response: ArcSwapOption::from_pointee(response),
        }
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
        self.headers.clone()
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
    pub fn encoding(&self) -> PyResult<String> {
        let content_type = self
            .headers
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<Mime>().ok());

        let encoding = content_type
            .as_ref()
            .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
            .unwrap_or("utf-8")
            .to_owned();

        Ok(encoding)
    }

    /// Returns the TLS peer certificate of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the TLS peer certificate of the response.
    pub fn peer_certificate(&self) -> PyResult<Option<Vec<u8>>> {
        let resp_ref = self.response.load();
        let resp = resp_ref.as_ref().ok_or_else(memory_error)?;
        if let Some(val) = resp.extensions().get::<TlsInfo>() {
            return Ok(val.peer_certificate().map(ToOwned::to_owned));
        }

        Ok(None)
    }

    /// Returns the text content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the text content of the response.
    pub fn text<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
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
        let resp = self.into_inner()?;
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
        let resp = self.into_inner()?;
        future_into_py(py, async move {
            resp.json::<Json>().await.map_err(wrap_rquest_error)
        })
    }

    /// Returns the JSON string content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json_str<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        future_into_py(py, async move {
            let json = resp.json::<Value>().await.map_err(wrap_rquest_error)?;
            serde_json::to_string(&json).map_err(wrap_serde_error)
        })
    }

    /// Returns the JSON pretty string content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json_str_pretty<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        future_into_py(py, async move {
            let json = resp.json::<Value>().await.map_err(wrap_rquest_error)?;
            serde_json::to_string_pretty(&json).map_err(wrap_serde_error)
        })
    }

    /// Returns the bytes content of the response.
    ///
    /// # Returns
    ///
    /// A Python object representing the bytes content of the response.
    pub fn bytes<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        future_into_py(py, async move {
            let bytes = resp.bytes().await.map_err(wrap_rquest_error)?;
            Python::with_gil(|py| bytes.into_bound_py_any(py).map(|obj| obj.unbind()))
        })
    }

    /// Convert the response into a `Stream` of `Bytes` from the body.
    ///
    /// # Returns
    ///
    /// A Python object representing the stream content of the response.
    pub fn stream(&self) -> PyResult<Streamer> {
        self.into_inner()
            .map(rquest::Response::bytes_stream)
            .map(Streamer::new)
    }

    /// Closes the response connection.
    pub fn close(&self) {
        let _ = self.into_inner().map(drop);
    }
}

#[pymethods]
impl Response {
    /// Returns the cookies of the response.
    ///
    /// # Returns
    ///
    /// A Python dictionary representing the cookies of the response.
    #[getter]
    pub fn cookies(&self) -> IndexMap<String, String> {
        self.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .map(|value| {
                std::str::from_utf8(value.as_bytes())
                    .map_err(cookie::ParseError::from)
                    .and_then(cookie::Cookie::parse)
            })
            .filter_map(Result::ok)
            .map(|cookie| (cookie.name().to_owned(), cookie.value().to_owned()))
            .collect()
    }
}

impl Response {
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
        self.response
            .swap(None)
            .and_then(Arc::into_inner)
            .ok_or_else(memory_error)
    }
}
