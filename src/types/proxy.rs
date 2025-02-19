use crate::error::wrap_rquest_error;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A proxy server for a request.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug)]
pub struct Proxy(rquest::Proxy);

impl From<Proxy> for rquest::Proxy {
    fn from(proxy: Proxy) -> rquest::Proxy {
        proxy.0
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Proxy {
    /// Creates a new HTTP proxy.
    ///
    /// This method sets up a proxy server for HTTP requests.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the proxy server.
    ///
    /// # Returns
    ///
    /// A new `Proxy` instance.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// proxy = rnet.Proxy.http("http://proxy.example.com")
    /// ```
    #[staticmethod]
    fn http(url: &str) -> PyResult<Self> {
        rquest::Proxy::http(url)
            .map(Proxy)
            .map_err(wrap_rquest_error)
    }

    /// Creates a new HTTPS proxy.
    ///
    /// This method sets up a proxy server for HTTPS requests.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the proxy server.
    ///
    /// # Returns
    ///
    /// A new `Proxy` instance.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// proxy = rnet.Proxy.https("https://proxy.example.com")
    /// ```
    #[staticmethod]
    fn https(url: &str) -> PyResult<Self> {
        rquest::Proxy::https(url)
            .map(Proxy)
            .map_err(wrap_rquest_error)
    }

    /// Creates a new proxy for all protocols.
    ///
    /// This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the proxy server.
    ///
    /// # Returns
    ///
    /// A new `Proxy` instance.
    ///
    /// # Examples
    ///
    /// ```python
    /// import rnet
    ///
    /// proxy = rnet.Proxy.all("https://proxy.example.com")
    /// ```
    #[staticmethod]
    fn all(url: &str) -> PyResult<Self> {
        rquest::Proxy::all(url)
            .map(Proxy)
            .map_err(wrap_rquest_error)
    }
}
