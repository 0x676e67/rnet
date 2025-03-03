use crate::error::wrap_rquest_error;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::header::HeaderValue;

/// A proxy server for a request.
#[gen_stub_pyclass]
#[pyclass]
pub struct Proxy(pub Option<rquest::Proxy>);

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
    #[pyo3(signature = (
        url,
        username = None,
        password = None,
        custom_http_auth = None,
        exclusion = None,
    ))]
    #[inline]
    fn http(
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        Self::create_proxy(
            rquest::Proxy::http,
            url,
            username,
            password,
            custom_http_auth,
            exclusion,
        )
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
    #[pyo3(signature = (
        url,
        username = None,
        password = None,
        custom_http_auth = None,
        exclusion = None,
    ))]
    #[inline]
    fn https(
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        Self::create_proxy(
            rquest::Proxy::https,
            url,
            username,
            password,
            custom_http_auth,
            exclusion,
        )
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
    #[pyo3(signature = (
        url,
        username = None,
        password = None,
        custom_http_auth = None,
        exclusion = None,
    ))]
    #[inline]
    fn all(
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        Self::create_proxy(
            rquest::Proxy::all,
            url,
            username,
            password,
            custom_http_auth,
            exclusion,
        )
    }
}

impl Proxy {
    fn create_proxy<'a>(
        proxy_fn: impl Fn(&'a str) -> Result<rquest::Proxy, rquest::Error>,
        url: &'a str,
        username: Option<&'a str>,
        password: Option<&str>,
        custom_http_auth: Option<&'a str>,
        exclusion: Option<&'a str>,
    ) -> PyResult<Self> {
        let mut proxy = proxy_fn(url).map_err(wrap_rquest_error)?;
        // Convert the username and password to a basic auth header value.
        if let (Some(username), Some(password)) = (username, password) {
            proxy = proxy.basic_auth(username, password)
        }

        // Convert the custom HTTP auth string to a header value.
        if let Some(Ok(custom_http_auth)) = custom_http_auth.map(HeaderValue::from_str) {
            proxy = proxy.custom_http_auth(custom_http_auth)
        }

        // Convert the exclusion list to a NoProxy instance.
        if let Some(exclusion) = exclusion {
            proxy = proxy.no_proxy(rquest::NoProxy::from_string(exclusion))
        }

        Ok(Proxy(Some(proxy)))
    }
}
