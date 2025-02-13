mod client;
mod error;
mod param;
mod resp;
mod types;

use client::Client;
use param::{ClientParams, RequestParams};
use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};
use types::{HeaderMap, Impersonate, Method, Proxy, SocketAddr, Version};

#[macro_export]
macro_rules! define_constants {
    ($type:tt, $inner_type:ty, $($name:ident),*) => {
        #[allow(non_upper_case_globals)]
        #[gen_stub_pymethods]
        #[pymethods]
        impl $type {
            $(
                #[classattr]
                pub const $name: $type = $type(<$inner_type>::$name);
            )*
        }
    };
}

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
fn get<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::get(url, kwds).await })
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
fn post<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::post(url, kwds).await })
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
fn put<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::put(url, kwds).await })
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
fn patch<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::patch(url, kwds).await })
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
fn delete<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::delete(url, kwds).await })
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
fn head<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::head(url, kwds).await })
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
fn options<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::options(url, kwds).await })
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
fn trace<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::trace(url, kwds).await })
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
fn request<'rt>(
    py: Python<'rt>,
    method: Method,
    url: String,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        client::request(method, url, kwds).await
    })
}

#[pymodule]
fn rnet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Method>()?;
    m.add_class::<Version>()?;
    m.add_class::<HeaderMap>()?;
    m.add_class::<Impersonate>()?;
    m.add_class::<SocketAddr>()?;
    m.add_class::<Proxy>()?;
    m.add_class::<ClientParams>()?;
    m.add_class::<RequestParams>()?;
    m.add_class::<resp::Response>()?;
    m.add_class::<Client>()?;
    m.add_function(wrap_pyfunction!(request, m)?)?;
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(post, m)?)?;
    m.add_function(wrap_pyfunction!(put, m)?)?;
    m.add_function(wrap_pyfunction!(patch, m)?)?;
    m.add_function(wrap_pyfunction!(delete, m)?)?;
    m.add_function(wrap_pyfunction!(head, m)?)?;
    m.add_function(wrap_pyfunction!(options, m)?)?;
    m.add_function(wrap_pyfunction!(trace, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
