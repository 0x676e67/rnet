use crate::{
    error::{memory_error, wrap_rquest_error, wrap_serde_error},
    headers::HeaderMap,
    json::PyJson,
    version::Version,
};
use arc_swap::ArcSwapOption;
use pyo3::{prelude::*, IntoPyObjectExt};
use rquest::{StatusCode, Url};
use serde_json::Value;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

#[pyclass]
pub struct Response {
    url: Url,
    version: Version,
    headers: HeaderMap,
    status_code: StatusCode,
    remote_addr: Option<SocketAddr>,
    content_length: Option<u64>,
    response: ArcSwapOption<rquest::Response>,
}

impl From<rquest::Response> for Response {
    fn from(mut response: rquest::Response) -> Self {
        Response {
            url: response.url().clone(),
            version: Version::from(response.version()),
            status_code: response.status(),
            remote_addr: response.remote_addr(),
            content_length: response.content_length(),
            headers: HeaderMap::from(std::mem::take(response.headers_mut())),
            response: ArcSwapOption::from_pointee(response),
        }
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

    /// Returns the status code of the response.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A Python object representing the HTTP status code.
    #[getter]
    pub fn status_code<'rt>(&'rt self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let http_module = py.import("http")?;
        let http_enum = http_module.getattr("HTTPStatus")?;
        http_enum.call1((self.status_code.as_u16(),))
    }

    /// Returns the HTTP version of the response.
    #[getter]
    pub fn version(&self) -> Version {
        self.version
    }

    /// Returns the headers of the response.
    #[getter]
    pub fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    /// Returns the content length of the response.
    #[getter]
    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// Returns the remote address of the response.
    #[getter]
    pub fn remote_addr(&self) -> Option<IpAddr> {
        self.remote_addr.map(|addr| addr.ip())
    }

    /// Returns the text content of the response.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A Python object representing the text content of the response.
    pub fn text<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            resp.text().await.map_err(wrap_rquest_error)
        })
    }

    /// Returns the text content of the response with a specific charset.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
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
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            resp.text_with_charset(&encoding)
                .await
                .map_err(wrap_rquest_error)
        })
    }

    /// Returns the JSON content of the response.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let json = resp.json::<PyJson>().await.map_err(wrap_rquest_error)?;
            Python::with_gil(|py| json.into_bound_py_any(py).map(|obj| obj.unbind()))
        })
    }

    /// Returns the JSON string content of the response.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json_str<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let json = resp.json::<Value>().await.map_err(wrap_rquest_error)?;
            serde_json::to_string(&json).map_err(wrap_serde_error)
        })
    }

    /// Returns the JSON pretty string content of the response.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A Python object representing the JSON content of the response.
    pub fn json_str_pretty<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let json = resp.json::<Value>().await.map_err(wrap_rquest_error)?;
            serde_json::to_string_pretty(&json).map_err(wrap_serde_error)
        })
    }

    /// Returns the bytes content of the response.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A Python object representing the bytes content of the response.
    pub fn bytes<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let bytes = resp.bytes().await.map_err(wrap_rquest_error)?;
            Python::with_gil(|py| bytes.into_bound_py_any(py).map(|obj| obj.unbind()))
        })
    }

    /// Closes the response connection.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    pub fn close<'rt>(&self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            drop(resp);
            Ok(())
        })
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
    pub fn into_inner(&self) -> PyResult<rquest::Response> {
        self.response
            .swap(None)
            .take()
            .ok_or_else(memory_error)
            .and_then(|arc| Arc::try_unwrap(arc).map_err(|_| memory_error()))
    }
}
