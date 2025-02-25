mod http;
mod ws;

pub use self::{
    http::{BlockingResponse, BlockingStreamer},
    ws::BlockingWebSocket,
};
use crate::{
    async_impl::{self, execute_request2, execute_websocket_request2},
    param::{ClientParams, RequestParams, UpdateClientParams, WebSocketParams},
    types::Method,
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A blocking client for making HTTP requests.
#[gen_stub_pyclass]
#[pyclass]
pub struct BlockingClient {
    inner: async_impl::Client,
}

#[gen_stub_pymethods]
#[pymethods]
impl BlockingClient {
    /// Sends a GET request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn get(&self, url: String, kwds: Option<RequestParams>) -> PyResult<BlockingResponse> {
        self.request(Method::GET, url, kwds)
    }

    /// Sends a POST request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn post(&self, url: String, kwds: Option<RequestParams>) -> PyResult<BlockingResponse> {
        self.request(Method::POST, url, kwds)
    }

    /// Sends a PUT request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn put(&self, url: String, kwds: Option<RequestParams>) -> PyResult<BlockingResponse> {
        self.request(Method::PUT, url, kwds)
    }

    /// Sends a PATCH request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn patch(&self, url: String, kwds: Option<RequestParams>) -> PyResult<BlockingResponse> {
        self.request(Method::PATCH, url, kwds)
    }

    /// Sends a DELETE request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn delete(&self, url: String, kwds: Option<RequestParams>) -> PyResult<BlockingResponse> {
        self.request(Method::DELETE, url, kwds)
    }

    /// Sends a HEAD request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn head(&self, url: String, kwds: Option<RequestParams>) -> PyResult<BlockingResponse> {
        self.request(Method::HEAD, url, kwds)
    }

    /// Sends an OPTIONS request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (url, **kwds))]
    pub fn options(&self, url: String, kwds: Option<RequestParams>) -> PyResult<BlockingResponse> {
        self.request(Method::OPTIONS, url, kwds)
    }

    /// Sends a TRACE request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn trace(&self, url: String, kwds: Option<RequestParams>) -> PyResult<BlockingResponse> {
        self.request(Method::TRACE, url, kwds)
    }

    /// Sends a request with the given method and URL.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use.
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    /// # Returns
    ///
    /// A `Response` object.
    #[pyo3(signature = (method, url, **kwds))]
    #[inline(always)]
    pub fn request(
        &self,
        method: Method,
        url: String,
        kwds: Option<RequestParams>,
    ) -> PyResult<BlockingResponse> {
        let client = self.inner.0.load();
        pyo3_async_runtimes::tokio::get_runtime()
            .block_on(execute_request2(client, method, url, kwds))
            .map(Into::into)
    }

    /// Sends a WebSocket request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the WebSocket request to.
    /// * `**kwds` - Additional WebSocket request parameters.
    ///
    /// # Returns
    ///
    /// A `WebSocket` object representing the WebSocket connection.
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
    pub fn websocket(
        &self,
        url: String,
        kwds: Option<WebSocketParams>,
    ) -> PyResult<BlockingWebSocket> {
        let client = self.inner.0.load();
        pyo3_async_runtimes::tokio::get_runtime()
            .block_on(execute_websocket_request2(client, url, kwds))
            .map(Into::into)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BlockingClient {
    /// Creates a new Client instance.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional request parameters as a dictionary.
    ///
    /// # Returns
    ///
    /// A new `Client` instance.
    #[new]
    #[pyo3(signature = (**kwds))]
    #[inline(always)]
    fn new(py: Python, kwds: Option<ClientParams>) -> PyResult<BlockingClient> {
        async_impl::Client::new(py, kwds).map(|inner| BlockingClient { inner })
    }

    /// Returns the user agent of the client.
    ///
    /// # Returns
    ///
    /// An optional string containing the user agent of the client.
    #[getter]
    #[inline(always)]
    fn user_agent(&self, py: Python) -> Option<String> {
        self.inner.user_agent(py)
    }

    /// Returns the headers of the client.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` object containing the headers of the client.
    #[getter]
    #[inline(always)]
    fn headers(&self, py: Python) -> crate::HeaderMap {
        self.inner.headers(py)
    }

    /// Returns the cookies for the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to get the cookies for.
    ///
    /// # Returns
    ///
    /// A list of cookie strings.
    #[pyo3(signature = (url))]
    #[inline(always)]
    fn get_cookies(&self, py: Python, url: &str) -> PyResult<Vec<String>> {
        self.inner.get_cookies(py, url)
    }

    /// Sets cookies for the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to set the cookies for.
    /// * `value` - A list of cookie strings to set.
    ///
    /// # Returns
    ///
    /// A `PyResult` indicating success or failure.
    #[pyo3(signature = (url, value))]
    #[inline(always)]
    fn set_cookies(&self, py: Python, url: &str, value: Vec<String>) -> PyResult<()> {
        self.inner.set_cookies(py, url, value)
    }

    /// Updates the client with the given parameters.
    ///
    /// # Arguments
    /// * `params` - The parameters to update the client with.
    #[pyo3(signature = (**kwds))]
    #[inline(always)]
    fn update(&self, py: Python, kwds: Option<UpdateClientParams>) {
        self.inner.update(py, kwds);
    }
}
