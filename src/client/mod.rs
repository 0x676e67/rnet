pub mod async_impl;
pub mod blocking;
pub mod typing;

mod dns;
mod opts;
mod param;

use pyo3::{PyResult, prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;

use self::{
    opts::{shortcut_request, shortcut_websocket_request},
    param::{RequestParams, WebSocketParams},
    typing::Method,
};

/// Make a GET request with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn get(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, Method::GET, kwds))
}

/// Make a POST request with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn post(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, Method::POST, kwds))
}

/// Make a PUT request with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn put(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, Method::PUT, kwds))
}

/// Make a PATCH request with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn patch(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, Method::PATCH, kwds))
}

/// Make a DELETE request with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn delete(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, Method::DELETE, kwds))
}

/// Make a HEAD request with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn head(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, Method::HEAD, kwds))
}

/// Make a OPTIONS request with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn options(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, Method::OPTIONS, kwds))
}

/// Make a TRACE request with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn trace(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, Method::TRACE, kwds))
}

/// Make a request with the given parameters.
#[pyfunction]
#[pyo3(signature = (method, url, **kwds))]
pub fn request(
    py: Python<'_>,
    method: Method,
    url: PyBackedStr,
    kwds: Option<RequestParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_request(url, method, kwds))
}

/// Make a WebSocket connection with the given parameters.
#[pyfunction]
#[pyo3(signature = (url, **kwds))]
pub fn websocket(
    py: Python<'_>,
    url: PyBackedStr,
    kwds: Option<WebSocketParams>,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, shortcut_websocket_request(url, kwds))
}
