use crate::error::wrap_rquest_error;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::header::HeaderValue;

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
    /// * `username` - Optional username for proxy authentication.
    /// * `password` - Optional password for proxy authentication.
    /// * `custom_http_auth` - Optional custom HTTP authentication header value.
    /// * `exclusion` - Optional list of domains to exclude from proxying.
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
    #[pyo3(signature = (url, username = None, password = None, custom_http_auth = None, exclusion = None))]
    fn http(
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        rquest::Proxy::http(url)
            .map(|proxy| Proxy::apply_proxy(proxy, username, password, custom_http_auth, exclusion))
            .map_err(wrap_rquest_error)
    }

    /// Creates a new HTTPS proxy.
    ///
    /// This method sets up a proxy server for HTTPS requests.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the proxy server.
    /// * `username` - Optional username for proxy authentication.
    /// * `password` - Optional password for proxy authentication.
    /// * `custom_http_auth` - Optional custom HTTP authentication header value.
    /// * `exclusion` - Optional list of domains to exclude from proxying.
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
    #[pyo3(signature = (url, username = None, password = None, custom_http_auth = None, exclusion = None))]
    fn https(
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        rquest::Proxy::https(url)
            .map(|proxy| Proxy::apply_proxy(proxy, username, password, custom_http_auth, exclusion))
            .map_err(wrap_rquest_error)
    }

    /// Creates a new proxy for all protocols.
    ///
    /// This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the proxy server.
    /// * `username` - Optional username for proxy authentication.
    /// * `password` - Optional password for proxy authentication.
    /// * `custom_http_auth` - Optional custom HTTP authentication header value.
    /// * `exclusion` - Optional list of domains to exclude from proxying.
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
    #[pyo3(signature = (url, username = None, password = None, custom_http_auth = None, exclusion = None))]
    fn all(
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        rquest::Proxy::all(url)
            .map(|proxy| Proxy::apply_proxy(proxy, username, password, custom_http_auth, exclusion))
            .map_err(wrap_rquest_error)
    }
}

impl Proxy {
    fn apply_proxy(
        mut proxy: rquest::Proxy,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        exclusion: Option<&str>,
    ) -> Self {
        // Convert the username and password to a basic auth header value.
        if let (Some(username), Some(password)) = (username, password) {
            proxy = proxy.basic_auth(username, password);
        }

        // Convert the custom HTTP auth string to a header value.
        if let Some(Ok(custom_http_auth)) = custom_http_auth.map(HeaderValue::from_str) {
            proxy = proxy.custom_http_auth(custom_http_auth);
        }

        // Convert the exclusion list to a NoProxy instance.
        if let Some(exclusion) = exclusion {
            proxy = proxy.no_proxy(rquest::NoProxy::from_string(exclusion));
        }

        Proxy(proxy)
    }
}
