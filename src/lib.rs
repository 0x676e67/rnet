mod client;
mod dns;
mod error;
mod param;
mod response;
mod types;

use client::Client;
#[cfg(feature = "logging")]
use log::LevelFilter;
use param::{ClientParams, RequestParams, UpdateClientParams, WebSocketParams};
use pyo3::prelude::*;
#[cfg(feature = "logging")]
use pyo3_log::{Caching, Logger};
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};
use response::{Message, Response, Streamer, WebSocket};
use types::{
    HeaderMap, Impersonate, ImpersonateOS, Method, Multipart, Part, Proxy, SocketAddr, StatusCode,
    Version,
};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

type Result<T> = std::result::Result<T, PyErr>;

/// Shortcut method to quickly make a `GET` request.
///
/// See also the methods on the [`rquest::Response`](./struct.Response.html)
/// type.
///
/// **NOTE**: This function creates a new internal `Client` on each call,
/// and so should not be used if making many requests. Create a
/// [`Client`](./struct.Client.html) instead.
///
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
///
/// # Errors
///
/// This function fails if:
///
/// - native TLS backend cannot be initialized
/// - supplied `Url` cannot be parsed
/// - there was an error while sending request
/// - redirect limit was exhausted
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn get(py: Python<'_>, url: String, kwds: Option<RequestParams>) -> PyResult<Bound<'_, PyAny>> {
    Client::default().get(py, url, kwds)
}

/// Shortcut method to quickly make a `POST` request.
///
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
///
/// async def run():
///     response = await rnet.post("https://httpbin.org/anything", json={"key": "value"})
///     body = await response.text()
///     print(body)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn post(py: Python<'_>, url: String, kwds: Option<RequestParams>) -> PyResult<Bound<'_, PyAny>> {
    Client::default().post(py, url, kwds)
}

/// Shortcut method to quickly make a `PUT` request.
///
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
///
/// async def run():
///     response = await rnet.put("https://httpbin.org/anything", json={"key": "value"})
///     body = await response.text()
///     print(body)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn put(py: Python<'_>, url: String, kwds: Option<RequestParams>) -> PyResult<Bound<'_, PyAny>> {
    Client::default().put(py, url, kwds)
}

/// Shortcut method to quickly make a `PATCH` request.
///
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
///
/// async def run():
///     response = await rnet.patch("https://httpbin.org/anything", json={"key": "value"})
///     body = await response.text()
///     print(body)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn patch(py: Python<'_>, url: String, kwds: Option<RequestParams>) -> PyResult<Bound<'_, PyAny>> {
    Client::default().patch(py, url, kwds)
}

/// Shortcut method to quickly make a `DELETE` request.
///
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
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn delete(py: Python<'_>, url: String, kwds: Option<RequestParams>) -> PyResult<Bound<'_, PyAny>> {
    Client::default().delete(py, url, kwds)
}

/// Shortcut method to quickly make a `HEAD` request.
///
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
///
/// async def run():
///     response = await rnet.head("https://httpbin.org/anything")
///     print(response.headers)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn head(py: Python<'_>, url: String, kwds: Option<RequestParams>) -> PyResult<Bound<'_, PyAny>> {
    Client::default().head(py, url, kwds)
}

/// Shortcut method to quickly make an `OPTIONS` request.
///
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
///
/// async def run():
///     response = await rnet.options("https://httpbin.org/anything")
///     print(response.headers)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn options(py: Python<'_>, url: String, kwds: Option<RequestParams>) -> PyResult<Bound<'_, PyAny>> {
    Client::default().options(py, url, kwds)
}

/// Shortcut method to quickly make a `TRACE` request.
///
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
///
/// async def run():
///     response = await rnet.trace("https://www.rust-lang.org")
///     print(response.headers)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn trace(py: Python<'_>, url: String, kwds: Option<RequestParams>) -> PyResult<Bound<'_, PyAny>> {
    Client::default().trace(py, url, kwds)
}

/// Make a request with the given parameters.
///
/// This function allows you to make a request with the specified parameters encapsulated in a `Request` object.
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
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    Client::default().request(py, method, url, kwds)
}

/// Make a WebSocket connection with the given parameters.
///
/// This function allows you to make a WebSocket connection with the specified parameters encapsulated in a `WebSocket` object.
///
/// # Examples
///
/// ```python
/// import rnet
/// import asyncio
///
/// async def run():
///    async with rnet.websocket("wss://echo.websocket.org") as ws:
///       await ws.send("Hello, World!")
///      message = await ws.recv()
///     print(message)
///
/// asyncio.run(run())
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
#[inline(always)]
fn websocket(
    py: Python<'_>,
    url: String,
    kwds: Option<WebSocketParams>,
) -> PyResult<Bound<'_, PyAny>> {
    Client::default().websocket(py, url, kwds)
}

#[pymodule(gil_used = false)]
fn rnet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // A good place to install the Rust -> Python logger.
    #[cfg(feature = "logging")]
    {
        let handle = Logger::new(m.py(), Caching::LoggersAndLevels)?
            .filter(LevelFilter::Trace)
            .install()
            .expect("Someone installed a logger before rnet.");

        // Some time in the future when logging changes, reset the caches:
        handle.reset();
    }

    m.add_class::<Method>()?;
    m.add_class::<Version>()?;
    m.add_class::<HeaderMap>()?;
    m.add_class::<Impersonate>()?;
    m.add_class::<ImpersonateOS>()?;
    m.add_class::<SocketAddr>()?;
    m.add_class::<Proxy>()?;
    m.add_class::<ClientParams>()?;
    m.add_class::<UpdateClientParams>()?;
    m.add_class::<RequestParams>()?;
    m.add_class::<WebSocketParams>()?;
    m.add_class::<WebSocket>()?;
    m.add_class::<Message>()?;
    m.add_class::<StatusCode>()?;
    m.add_class::<Response>()?;
    m.add_class::<Streamer>()?;
    m.add_class::<Part>()?;
    m.add_class::<Multipart>()?;
    m.add_class::<Client>()?;

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
