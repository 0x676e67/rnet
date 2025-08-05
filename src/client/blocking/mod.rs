mod response;

use pyo3::{prelude::*, pybacked::PyBackedStr};

pub use self::response::{BlockingResponse, BlockingStreamer, BlockingWebSocket};
use super::{
    async_impl::Client,
    opts::{execute_request, execute_websocket_request},
    param::{ClientParams, RequestParams, WebSocketParams},
    typing::Method,
};

/// A blocking client for making HTTP requests.
#[pyclass(subclass)]
pub struct BlockingClient(Client);

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

    /// Make a OPTIONS request to the specified URL.
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
        Client::new(py, kwds).map(BlockingClient)
    }
}
