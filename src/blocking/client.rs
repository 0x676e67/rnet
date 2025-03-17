use super::{http::BlockingResponse, ws::BlockingWebSocket};
use crate::{
    async_impl::{self, execute_request, execute_websocket_request},
    param::{ClientParams, RequestParams, UpdateClientParams, WebSocketParams},
    typing::{Cookie, HeaderMap, Method},
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A blocking client for making HTTP requests.
#[gen_stub_pyclass]
#[pyclass]
pub struct BlockingClient(async_impl::Client);

macro_rules! define_http_method {
    ($(#[$meta:meta])* $name:ident, $method:expr) => {
        #[gen_stub_pymethods]
        #[pymethods]
        impl BlockingClient {
            /// Sends a request with the given URL.
            ///
            /// # Arguments
            ///
            /// * `url` - The URL to send the request to.
            /// * `**kwds` - Additional request parameters.
            ///
            ///     proxy: typing.Optional[builtins.str]
            ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
            ///     interface: typing.Optional[builtins.str]
            ///     timeout: typing.Optional[builtins.int]
            ///     read_timeout: typing.Optional[builtins.int]
            ///     version: typing.Optional[Version]
            ///     headers: typing.Optional[typing.Dict[str, bytes]]
            ///     cookies: typing.Optional[typing.Dict[str, str]]
            ///     allow_redirects: typing.Optional[builtins.bool]
            ///     max_redirects: typing.Optional[builtins.int]
            ///     auth: typing.Optional[str]
            ///     bearer_auth: typing.Optional[str]
            ///     basic_auth: typing.Optional[tuple[str, typing.Optional[str]]]
            ///     query: typing.Optional[typing.List[typing.Tuple[str, str]]]
            ///     form: typing.Optional[typing.List[typing.Tuple[str, str]]]
            ///     json: typing.Optional[typing.Any]
            ///     body: typing.Optional[typing.Any]
            ///     multipart: typing.Optional[Multipart]
            ///
            /// # Returns
            ///
            /// A `Response` object.
            ///
            $(#[$meta])*
            #[pyo3(signature = (url, **kwds))]
            #[inline(always)]
            pub fn $name<'rt>(
                &self,
                py: Python<'rt>,
                url: PyBackedStr,
                kwds: Option<RequestParams>,
            ) -> PyResult<BlockingResponse> {
                self.request(py, $method, url, kwds)
            }
        }
    };
}

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.get("https://httpbin.org/anything")
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    get,
    Method::GET
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.post("https://httpbin.org/anything", json={"key": "value"})
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    post,
    Method::POST
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.put("https://httpbin.org/anything", json={"key": "value"})
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    put,
    Method::PUT
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.patch("https://httpbin.org/anything", json={"key": "value"})
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    patch,
    Method::PATCH
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.delete("https://httpbin.org/anything")
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    delete,
    Method::DELETE
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.head("https://httpbin.org/anything")
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    head,
    Method::HEAD
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.options("https://httpbin.org/anything")
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    options,
    Method::OPTIONS
);

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.trace("https://httpbin.org/anything")
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    trace,
    Method::TRACE
);

#[gen_stub_pymethods]
#[pymethods]
impl BlockingClient {
    /// Sends a request with the given method and URL.
    ///
    /// # Arguments
    ///
    /// * `method` - The method to use for the request.
    /// * `url` - The URL to send the request to.
    /// * `**kwds` - Additional request parameters.
    ///
    ///     proxy: typing.Optional[builtins.str]
    ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
    ///     interface: typing.Optional[builtins.str]
    ///     timeout: typing.Optional[builtins.int]
    ///     read_timeout: typing.Optional[builtins.int]
    ///     version: typing.Optional[Version]
    ///     headers: typing.Optional[typing.Dict[str, bytes]]
    ///     cookies: typing.Optional[typing.Dict[str, str]]
    ///     allow_redirects: typing.Optional[builtins.bool]
    ///     max_redirects: typing.Optional[builtins.int]
    ///     auth: typing.Optional[str]
    ///     bearer_auth: typing.Optional[str]
    ///     basic_auth: typing.Optional[tuple[str, typing.Optional[str]]]
    ///     query: typing.Optional[typing.List[typing.Tuple[str, str]]]
    ///     form: typing.Optional[typing.List[typing.Tuple[str, str]]]
    ///     json: typing.Optional[typing.Any]
    ///     body: typing.Optional[typing.Any]
    ///     multipart: typing.Optional[Multipart]
    ///
    /// # Returns
    ///
    /// A `Response` object.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    /// from rnet import Method
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     response = client.request(Method.GET, "https://httpbin.org/anything")
    ///     print(response.text())
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (method, url, kwds))]
    #[inline(always)]
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

    /// Sends a WebSocket request.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the WebSocket request to.
    /// * `**kwds` - Additional WebSocket request parameters.
    ///
    ///     proxy: typing.Optional[builtins.str]
    ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
    ///     interface: typing.Optional[builtins.str]
    ///     headers: typing.Optional[typing.Dict[str, bytes]]
    ///     cookies: typing.Optional[typing.Dict[str, str]]
    ///     protocols: typing.Optional[builtins.list[builtins.str]]
    ///     use_http2: typing.Optional[builtins.bool]
    ///     auth: typing.Optional[builtins.str]
    ///     bearer_auth: typing.Optional[builtins.str]
    ///     basic_auth: typing.Optional[tuple[builtins.str, typing.Optional[builtins.str]]]
    ///     query: typing.Optional[builtins.list[tuple[builtins.str, builtins.str]]]
    ///     read_buffer_size: typing.Optional[builtins.int]
    ///     write_buffer_size: typing.Optional[builtins.int]
    ///     max_write_buffer_size: typing.Optional[builtins.int]
    ///     max_message_size: typing.Optional[builtins.int]
    ///     max_frame_size: typing.Optional[builtins.int]
    ///     accept_unmasked_frames: typing.Optional[builtins.bool]
    ///
    /// # Returns
    ///
    /// A `WebSocket` object representing the WebSocket connection.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def main():
    ///     client = rnet.BlockingClient()
    ///     ws = client.websocket("wss://echo.websocket.org")
    ///     ws.send(rnet.Message.from_text("Hello, WebSocket!"))
    ///     message = ws.recv()
    ///     print("Received:", message.data)
    ///     ws.close()
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(signature = (url, **kwds))]
    #[inline(always)]
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

#[gen_stub_pymethods]
#[pymethods]
impl BlockingClient {
    /// Creates a new BlockingClient instance.
    ///
    /// # Arguments
    ///
    /// * `**kwds` - Optional request parameters as a dictionary.
    ///
    ///     impersonate: typing.Optional[Impersonate]
    ///     impersonate_os: typing.Optional[ImpersonateOS]
    ///     impersonate_skip_http2: typing.Optional[builtins.bool]
    ///     impersonate_skip_headers: typing.Optional[builtins.bool]
    ///     user_agent: typing.Optional[str]
    ///     default_headers: typing.Optional[typing.Dict[str, bytes]]
    ///     headers_order: typing.Optional[typing.List[str]]
    ///     referer: typing.Optional[builtins.bool]
    ///     allow_redirects: typing.Optional[builtins.bool]
    ///     max_redirects: typing.Optional[builtins.int]
    ///     cookie_store: typing.Optional[builtins.bool]
    ///     lookup_ip_strategy: typing.Optional[LookupIpStrategy]
    ///     timeout: typing.Optional[builtins.int]
    ///     connect_timeout: typing.Optional[builtins.int]
    ///     read_timeout: typing.Optional[builtins.int]
    ///     no_keepalive: typing.Optional[builtins.bool]
    ///     tcp_keepalive: typing.Optional[builtins.int]
    ///     pool_idle_timeout: typing.Optional[builtins.int]
    ///     pool_max_idle_per_host: typing.Optional[builtins.int]
    ///     pool_max_size: typing.Optional[builtins.int]
    ///     http1_only: typing.Optional[builtins.bool]
    ///     http2_only: typing.Optional[builtins.bool]
    ///     https_only: typing.Optional[builtins.bool]
    ///     tcp_nodelay: typing.Optional[builtins.bool]
    ///     http2_max_retry_count: typing.Optional[builtins.int]
    ///     verify: Optional[Union[bool, Path]]
    ///     tls_info: typing.Optional[builtins.bool]
    ///     min_tls_version: typing.Optional[TlsVersion]
    ///     max_tls_version: typing.Optional[TlsVersion]
    ///     no_proxy: typing.Optional[builtins.bool]
    ///     proxies: typing.Optional[builtins.list[Proxy]]
    ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
    ///     interface: typing.Optional[builtins.str]
    ///     gzip: typing.Optional[builtins.bool]
    ///     brotli: typing.Optional[builtins.bool]
    ///     deflate: typing.Optional[builtins.bool]
    ///     zstd: typing.Optional[builtins.bool]
    ///
    /// # Returns
    ///
    /// A new `BlockingClient` instance.
    ///
    /// # Examples
    ///
    /// ```python
    /// import asyncio
    /// import rnet
    ///
    /// client = rnet.BlockingClient(
    ///     user_agent="my-app/0.0.1",
    ///     timeout=10,
    /// )
    /// response = client.get('https://httpbin.org/get')
    /// print(response.text())
    /// ```
    #[new]
    #[pyo3(signature = (**kwds))]
    #[inline(always)]
    fn new(py: Python, kwds: Option<ClientParams>) -> PyResult<BlockingClient> {
        async_impl::Client::new(py, kwds).map(BlockingClient)
    }

    /// Returns the user agent of the client.
    ///
    /// # Returns
    ///
    /// An optional string containing the user agent of the client.
    #[getter]
    #[inline(always)]
    fn user_agent(&self, py: Python) -> Option<String> {
        self.0.user_agent(py)
    }

    /// Returns the headers of the client.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` object containing the headers of the client.
    #[getter]
    #[inline(always)]
    fn headers(&self) -> HeaderMap {
        self.0.headers()
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
    pub fn get_cookies<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        self.0.get_cookies(py, url)
    }

    /// Sets the cookies for the given URL.
    ///
    /// # Arguments
    /// * `url` - The URL to set the cookies for.
    /// * `cookie` - The cookie to set.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client(cookie_store=True)
    /// client.set_cookie("https://example.com", rnet.Cookie(name="foo", value="bar"))
    /// ```
    #[pyo3(signature = (url, cookie))]
    #[inline(always)]
    pub fn set_cookie(&self, py: Python, url: PyBackedStr, cookie: Cookie) -> PyResult<()> {
        self.0.set_cookie(py, url, cookie)
    }

    /// Removes the cookie with the given name for the given URL.
    ///
    /// # Arguments
    /// * `url` - The URL to remove the cookie from.
    /// * `name` - The name of the cookie to remove.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.Client(cookie_store=True)
    /// client.remove_cookie("https://example.com", "foo")
    #[pyo3(signature = (url, name))]
    #[inline(always)]
    pub fn remove_cookie(&self, py: Python, url: PyBackedStr, name: PyBackedStr) -> PyResult<()> {
        self.0.remove_cookie(py, url, name)
    }

    /// Clears the cookies for the given URL.
    #[inline(always)]
    pub fn clear_cookies(&self, py: Python) {
        self.0.clear_cookies(py);
    }

    /// Updates the client with the given parameters.
    ///
    /// # Arguments
    /// * `**kwds` - The parameters to update the client with.
    ///
    ///     impersonate: typing.Optional[Impersonate]
    ///     impersonate_os: typing.Optional[ImpersonateOS]
    ///     impersonate_skip_http2: typing.Optional[builtins.bool]
    ///     impersonate_skip_headers: typing.Optional[builtins.bool]
    ///     headers: typing.Optional[typing.Dict[str, bytes]]
    ///     headers_order: typing.Optional[typing.List[str]]
    ///     proxies: typing.Optional[builtins.list[Proxy]]
    ///     local_address: typing.Optional[typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]]
    ///     interface: typing.Optional[builtins.str]
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// client = rnet.BlockingClient()
    /// client.update(
    ///    impersonate=rnet.Impersonate.Firefox135,
    ///    headers={"X-My-Header": "value"},
    ///    proxies=[rnet.Proxy.all("http://proxy.example.com:8080")],
    /// )
    /// ```
    #[pyo3(signature = (**kwds))]
    #[inline(always)]
    fn update(&self, py: Python, kwds: Option<UpdateClientParams>) -> PyResult<()> {
        self.0.update(py, kwds)
    }
}
