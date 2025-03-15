mod async_impl;
mod blocking;
mod buffer;
mod dns;
mod error;
mod param;
mod stream;
mod typing;

use async_impl::{Client, Message, Response, Streamer, WebSocket};
use blocking::{BlockingClient, BlockingResponse, BlockingStreamer, BlockingWebSocket};
use param::{RequestParams, WebSocketParams};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};
use typing::{
    Cookie, HeaderMap, HeaderMapItemsIter, HeaderMapKeysIter, Impersonate, ImpersonateOS,
    LookupIpStrategy, Method, Multipart, Part, Proxy, SameSite, SocketAddr, StatusCode, TlsVersion,
    Version,
};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

type Result<T> = std::result::Result<T, PyErr>;

macro_rules! define_http_method {
    ($(#[$meta:meta])* $name:ident, $method:expr) => {
        /// Shortcut method to quickly make a request.
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
        $(#[$meta])*
        #[gen_stub_pyfunction]
        #[pyfunction]
        #[pyo3(signature = (url, **kwds))]
        #[inline(always)]
        fn $name(
            py: Python<'_>,
            url: PyBackedStr,
            kwds: Option<RequestParams>,
        ) -> PyResult<Bound<'_, PyAny>> {
            future_into_py(py, async_impl::shortcut_request(url, $method, kwds))
        }
    };
}

define_http_method!(
    /// # Examples
    ///
    /// ```python
    /// import rnet
    /// import asyncio
    ///
    /// async def run():
    ///     response = await rnet.get("https://httpbin.org/anything")
    ///     body = await response.text()
    ///     print(body)
    ///
    /// asyncio.run(run())
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
    ///
    /// async def run():
    ///     response = await rnet.post("https://httpbin.org/anything")
    ///     body = await response.text()
    ///     print(body)
    ///
    /// asyncio.run(run())
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
    ///
    /// async def run():
    ///     response = await rnet.put("https://httpbin.org/anything")
    ///     body = await response.text()
    ///     print(body)
    ///
    /// asyncio.run(run())
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
    ///
    /// async def run():
    ///     response = await rnet.patch("https://httpbin.org/anything")
    ///     body = await response.text()
    ///     print(body)
    ///
    /// asyncio.run(run())
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
    ///
    /// async def run():
    ///     response = await rnet.delete("https://httpbin.org/anything")
    ///     body = await response.text()
    ///     print(body)
    ///
    /// asyncio.run(run())
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
    ///
    /// async def run():
    ///     response = await rnet.head("https://httpbin.org/anything")
    ///     print(response.status)
    ///
    /// asyncio.run(run())
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
    ///
    /// async def run():
    ///     response = await rnet.options("https://httpbin.org/anything")
    ///     print(response.status)
    ///
    /// asyncio.run(run())
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
    ///
    /// async def run():
    ///     response = await rnet.trace("https://httpbin.org/anything")
    ///     print(response.status)
    ///
    /// asyncio.run(run())
    /// ```
    trace,
    Method::TRACE
);

/// Make a request with the given parameters.
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
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
/// from rnet import Method
///
/// async def run():
///     response = await rnet.request(Method.GET, "https://www.rust-lang.org")
///     body = await response.text()
///     print(body)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (method, url, **kwds))]
#[inline(always)]
fn request(
    py: Python<'_>,
    method: Method,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async_impl::shortcut_request(url, method, kwds))
}

/// Make a WebSocket connection with the given parameters.
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
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
/// from rnet import Message
///
/// async def run():
///     ws = await rnet.websocket("wss://echo.websocket.org")
///     await ws.send(Message.from_text("Hello, World!"))
///     message = await ws.recv()
///     print("Received:", message.data)
///     await ws.close()
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn websocket(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<WebSocketParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async_impl::shortcut_websocket_request(url, kwds))
}

#[pymodule(gil_used = false)]
fn rnet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    m.add_class::<Method>()?;
    m.add_class::<Version>()?;
    m.add_class::<HeaderMap>()?;
    m.add_class::<HeaderMapItemsIter>()?;
    m.add_class::<HeaderMapKeysIter>()?;
    m.add_class::<Impersonate>()?;
    m.add_class::<ImpersonateOS>()?;
    m.add_class::<TlsVersion>()?;
    m.add_class::<SocketAddr>()?;
    m.add_class::<Proxy>()?;
    m.add_class::<LookupIpStrategy>()?;
    m.add_class::<Message>()?;
    m.add_class::<StatusCode>()?;
    m.add_class::<Part>()?;
    m.add_class::<Multipart>()?;

    m.add_class::<SameSite>()?;
    m.add_class::<Cookie>()?;

    m.add_class::<Client>()?;
    m.add_class::<Response>()?;
    m.add_class::<WebSocket>()?;
    m.add_class::<Streamer>()?;
    m.add_class::<BlockingClient>()?;
    m.add_class::<BlockingResponse>()?;
    m.add_class::<BlockingWebSocket>()?;
    m.add_class::<BlockingStreamer>()?;

    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(post, m)?)?;
    m.add_function(wrap_pyfunction!(put, m)?)?;
    m.add_function(wrap_pyfunction!(patch, m)?)?;
    m.add_function(wrap_pyfunction!(delete, m)?)?;
    m.add_function(wrap_pyfunction!(head, m)?)?;
    m.add_function(wrap_pyfunction!(options, m)?)?;
    m.add_function(wrap_pyfunction!(trace, m)?)?;
    m.add_function(wrap_pyfunction!(request, m)?)?;
    m.add_function(wrap_pyfunction!(websocket, m)?)?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
