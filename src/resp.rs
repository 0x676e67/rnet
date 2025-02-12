use crate::{
    error::{memory_error, wrap_rquest_error, wrap_serde_error},
    headers::HeaderMap,
    json::PyJson,
    version::Version,
};
use mime::Mime;
use pyo3::{exceptions::PyStopAsyncIteration, prelude::*, types::PyDict, IntoPyObjectExt};
use rquest::{header, StatusCode};
use serde_json::Value;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::sync::Mutex;

#[pyclass]
pub struct Response {
    version: Version,
    status_code: StatusCode,
    remote_addr: Option<SocketAddr>,
    content_length: Option<u64>,
    response: Option<rquest::Response>,
}

impl From<rquest::Response> for Response {
    fn from(response: rquest::Response) -> Self {
        Response {
            version: Version::from(response.version()),
            status_code: response.status(),
            remote_addr: response.remote_addr(),
            content_length: response.content_length(),
            response: Some(response),
        }
    }
}

#[pymethods]
impl Response {
    /// Returns the URL of the response.
    #[getter]
    pub fn url(&self) -> PyResult<&str> {
        self.inner().map(|resp| resp.url().as_str())
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
    pub fn headers(&self) -> PyResult<HeaderMap> {
        self.inner()
            .map(|resp| HeaderMap::from(resp.headers().clone()))
    }

    /// Returns the content length of the response.
    #[getter]
    pub fn content_length(&self) -> u64 {
        self.content_length.unwrap_or_default()
    }

    /// Returns the remote address of the response.
    #[getter]
    pub fn remote_addr(&self) -> Option<IpAddr> {
        self.remote_addr.map(|addr| addr.ip())
    }

    /// Encoding to decode with when accessing text.
    #[getter]
    pub fn encoding(&mut self) -> PyResult<String> {
        let resp = self.inner()?;
        let content_type = resp
            .headers()
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

    /// Returns the cookies of the response.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A Python dictionary representing the cookies of the response.
    #[getter]
    pub fn cookies<'rt>(&'rt self, py: Python<'rt>) -> PyResult<Bound<'rt, PyDict>> {
        let resp = self.inner()?;

        let py_dict = PyDict::new(py);
        for cookie in resp.cookies() {
            py_dict.set_item(cookie.name(), cookie.value())?;
        }

        Ok(py_dict)
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
    pub fn text<'rt>(&mut self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
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
        &mut self,
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
    pub fn json<'rt>(&mut self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
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
    pub fn json_str<'rt>(&mut self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
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
    pub fn json_str_pretty<'rt>(&mut self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
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
    pub fn bytes<'rt>(&mut self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let bytes = resp.bytes().await.map_err(wrap_rquest_error)?;
            Python::with_gil(|py| bytes.into_bound_py_any(py).map(|obj| obj.unbind()))
        })
    }

    /// Returns the stream content of the response.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A Python object representing the stream content of the response.
    pub fn stream<'rt>(&mut self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let streamer = Streamer::new(self.into_inner()?);
        streamer.into_bound_py_any(py)
    }

    /// Closes the response connection.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    pub fn close<'rt>(&mut self, py: Python<'rt>) -> PyResult<Bound<'rt, PyAny>> {
        let resp = self.into_inner()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            drop(resp);
            Ok(())
        })
    }
}

impl Response {
    /// Returns the inner `Arc<rquest::Response>`.
    ///
    /// # Returns
    ///
    /// An `PyResult<Arc<rquest::Response>>` containing the inner `rquest::Response` if it exists.
    ///
    /// This method provides access to the inner `rquest::Response` wrapped in an `Arc`. It can be
    /// useful for cases where you need to access the original response object directly.
    #[inline(always)]
    fn inner(&self) -> PyResult<&rquest::Response> {
        self.response.as_ref().ok_or_else(memory_error)
    }

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
    fn into_inner(&mut self) -> PyResult<rquest::Response> {
        self.response.take().ok_or_else(memory_error)
    }
}

/// A streaming response.
/// This is an asynchronous iterator that yields chunks of data from the response stream.
/// This is used to stream the response content.
/// This is used in the `stream` method of the `Response` class.
/// This is used in an asynchronous for loop in Python.
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
///     print("Headers: ", resp.headers.to_dict())
///     print("Content-Length: ", resp.content_length)
///     print("Encoding: ", resp.encoding)
///     print("Remote Address: ", resp.remote_addr)
///
///     streamer = resp.stream()
///     async for chunk in streamer:
///         print("Chunk: ", chunk)
///         await asyncio.sleep(0.1)
///
/// if __name__ == "__main__":
///     asyncio.run(main())
/// ```
#[pyclass]
struct Streamer(Arc<Mutex<rquest::Response>>);

impl Streamer {
    /// Creates a new `Streamer` instance.
    ///
    /// # Arguments
    ///
    /// * `response` - The `rquest::Response` to be streamed.
    ///
    /// # Returns
    ///
    /// A new `Streamer` instance.
    fn new(response: rquest::Response) -> Self {
        Streamer(Arc::new(Mutex::new(response)))
    }
}

#[pymethods]
impl Streamer {
    /// Returns the `Streamer` instance itself to be used as an asynchronous iterator.
    ///
    /// This method allows the `Streamer` to be used in an asynchronous for loop in Python.
    ///
    /// # Arguments
    ///
    /// * `slf` - A reference to the `Streamer` instance.
    ///
    /// # Returns
    ///
    /// The `Streamer` instance itself.
    #[inline(always)]
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Returns the next chunk of the response as an asynchronous iterator.
    ///
    /// This method implements the `__anext__` method for the `Streamer` class, allowing it to be
    /// used as an asynchronous iterator in Python. It returns the next chunk of the response or
    /// raises `PyStopAsyncIteration` if the iterator is exhausted.
    ///
    /// # Arguments
    ///
    /// * `py` - The Python interpreter.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing an `Option<PyObject>`. If there is a next chunk, it returns `Some(PyObject)`.
    /// If the iterator is exhausted, it raises `PyStopAsyncIteration`.
    fn __anext__<'a>(&self, py: Python<'a>) -> PyResult<Option<PyObject>> {
        // Here we clone the inner field, so we can use it
        // in our future.
        let streamer = self.0.clone();
        let future = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Here we lock the mutex to access the data inside
            // and call chunk() method to get the next value.
            let val = streamer
                .lock()
                .await
                .chunk()
                .await
                .map_err(wrap_rquest_error)?;

            match val {
                Some(val) => {
                    // If we have a value, we return it as a PyObject.
                    Python::with_gil(|py| Ok(Some(val.into_bound_py_any(py)?.unbind())))
                }
                // Here we return PyStopAsyncIteration error,
                // because python needs exceptions to tell that iterator
                // has ended.
                None => Err(PyStopAsyncIteration::new_err("The iterator is exhausted")),
            }
        });
        Ok(Some(future?.into()))
    }

    /// This is a helper method to get the next chunk from the stream.
    /// It is used to get the next chunk from the stream.
    /// This method is used in __anext__ method.
    #[inline(always)]
    pub fn chunk<'a>(&self, py: Python<'a>) -> PyResult<Option<PyObject>> {
        self.__anext__(py)
    }
}
