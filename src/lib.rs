mod client;
mod error;
mod headers;
mod impersonate;
mod json;
mod method;
mod req;
mod resp;
mod version;

use method::Method;
use pyo3::{prelude::*, types::PyDict};
use req::RequestParams;

#[macro_export]
macro_rules! define_constants {
    ($type:tt, $inner_type:ty, $($name:ident),*) => {
        #[allow(non_upper_case_globals)]
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
///     response = await rnet.get("https://www.rust-lang.org")
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
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
fn get<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::get(url, params).await })
}

/// Shortcut method to quickly make a `POST` request.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
fn post<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::post(url, params).await })
}

/// Shortcut method to quickly make a `PUT` request.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
fn put<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::put(url, params).await })
}

/// Shortcut method to quickly make a `PATCH` request.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
fn patch<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::patch(url, params).await })
}

/// Shortcut method to quickly make a `DELETE` request.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
fn delete<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::delete(url, params).await })
}

/// Shortcut method to quickly make a `HEAD` request.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
fn head<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::head(url, params).await })
}

/// Shortcut method to quickly make a `OPTIONS` request.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
fn options<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::options(url, params).await })
}

/// Shortcut method to quickly make a `TRACE` request.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
fn trace<'rt>(
    py: Python<'rt>,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move { client::trace(url, params).await })
}

/// Make a request with the given parameters.
///
/// This function allows you to make a request with the specified parameters encapsulated in a `Request` object.
#[pyfunction]
#[pyo3(signature = (method, url, **kwds))]
fn request<'rt>(
    py: Python<'rt>,
    method: Method,
    url: String,
    kwds: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'rt, PyAny>> {
    let params = RequestParams::from(kwds);
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        client::request(method, url, params).await
    })
}

#[pymodule]
fn rnet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<method::Method>()?;
    m.add_class::<version::Version>()?;
    m.add_class::<impersonate::Impersonate>()?;
    m.add_class::<resp::Response>()?;
    m.add_class::<headers::HeaderMap>()?;
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
