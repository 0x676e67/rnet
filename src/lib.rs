mod client;
mod error;
mod headers;
mod impersonate;
mod json;
mod method;
mod request;
mod response;
mod version;

use error::wrap_rquest_error;
use pyo3::prelude::*;
use response::Response;

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
fn get<'rt>(py: Python<'rt>, url: String) -> PyResult<Bound<'rt, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        rquest::get(url)
            .await
            .map(Response::from)
            .map_err(wrap_rquest_error)
    })
}

#[pymodule]
fn rnet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<method::Method>()?;
    m.add_class::<version::Version>()?;
    m.add_class::<impersonate::Impersonate>()?;
    m.add_class::<response::Response>()?;
    m.add_class::<headers::HeaderMap>()?;
    m.add_function(wrap_pyfunction!(get, m)?)?;
    Ok(())
}
