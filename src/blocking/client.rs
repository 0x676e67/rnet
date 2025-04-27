use super::{BlockingResponse, BlockingWebSocket};
use crate::{
    async_impl::{self, execute_request, execute_websocket_request},
    typing::{
        Cookie, HeaderMap, Method,
        param::{ClientParams, RequestParams, UpdateClientParams, WebSocketParams},
    },
};
use pyo3::{prelude::*, pybacked::PyBackedStr};

/// A blocking client for making HTTP requests.
#[pyclass(subclass)]
pub struct BlockingClient(async_impl::Client);

#[pymethods]
impl BlockingClient {
    /// Make a GET request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn get(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::GET, url, kwds)
    }

    /// Make a POST request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn post(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::POST, url, kwds)
    }

    /// Make a PUT request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn put(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::PUT, url, kwds)
    }

    /// Make a PATCH request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn patch(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::PATCH, url, kwds)
    }

    /// Make a DELETE request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn delete(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::DELETE, url, kwds)
    }

    /// Make a HEAD request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn head(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::HEAD, url, kwds)
    }

    /// Make an OPTIONS request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn options(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::OPTIONS, url, kwds)
    }

    /// Make a TRACE request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn trace(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::TRACE, url, kwds)
    }

    /// Make a rqeuest with the specified method and URL.
    #[pyo3(signature = (method, url, **kwds))]
    pub fn request(
        &self,
        py: Python,
        method: Method,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        py.allow_threads(|| {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(execute_request(client, method, url, kwds))
                .map(Into::into)
        })
    }

    /// Make a WebSocket request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn websocket(
        &self,
        py: Python,
        url: PyBackedStr,
        kwds: Option<WebSocketParams>,
    ) -> PyResult<BlockingWebSocket> {
        py.allow_threads(|| {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(execute_websocket_request(client, url, kwds))
                .map(Into::into)
        })
    }
}

#[pymethods]
impl BlockingClient {
    /// Creates a new BlockingClient instance.
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new(py: Python, kwds: Option<ClientParams>) -> PyResult<BlockingClient> {
        async_impl::Client::new(py, kwds).map(BlockingClient)
    }

    /// Returns the user agent of the client.
    #[getter]
    fn user_agent(&self, py: Python) -> Option<String> {
        self.0.user_agent(py)
    }

    /// Returns the headers of the client.
    #[getter]
    fn headers(&self) -> HeaderMap {
        self.0.headers()
    }

    /// Returns the cookies for the given URL.
    #[pyo3(signature = (url))]
    pub fn get_cookies<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        self.0.get_cookies(py, url)
    }

    /// Sets the cookies for the given URL.
    #[pyo3(signature = (url, cookie))]
    pub fn set_cookie(&self, py: Python, url: PyBackedStr, cookie: Cookie) -> PyResult<()> {
        self.0.set_cookie(py, url, cookie)
    }

    /// Removes the cookie with the given name for the given URL.
    #[pyo3(signature = (url, name))]
    pub fn remove_cookie(&self, py: Python, url: PyBackedStr, name: PyBackedStr) -> PyResult<()> {
        self.0.remove_cookie(py, url, name)
    }

    /// Clears the cookies for the given URL.
    pub fn clear_cookies(&self, py: Python) {
        self.0.clear_cookies(py);
    }

    /// Updates the client with the given parameters.
    #[pyo3(signature = (**kwds))]
    pub fn update(&self, py: Python, kwds: Option<UpdateClientParams>) -> PyResult<()> {
        self.0.update(py, kwds)
    }
}
