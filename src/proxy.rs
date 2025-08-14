use pyo3::prelude::*;
use wreq::header::{HeaderMap, HeaderValue};

use crate::{error::Error, extractor::Extractor};

/// A proxy server for a request.
/// Supports HTTP, HTTPS, SOCKS4, SOCKS4a, SOCKS5, and SOCKS5h protocols.
#[pyclass(subclass)]
pub struct Proxy(pub wreq::Proxy);

#[pymethods]
impl Proxy {
    /// Creates a new HTTP proxy.
    ///
    /// This method sets up a proxy server for HTTP requests.
    #[staticmethod]
    #[pyo3(signature = (
        url,
        username = None,
        password = None,
        custom_http_auth = None,
        custom_http_headers = None,
        exclusion = None,
    ))]
    fn http(
        py: Python,
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        custom_http_headers: Option<Extractor<HeaderMap>>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        create_proxy(
            py,
            wreq::Proxy::http,
            url,
            username,
            password,
            custom_http_auth,
            custom_http_headers,
            exclusion,
        )
    }

    /// Creates a new HTTPS proxy.
    ///
    /// This method sets up a proxy server for HTTPS requests.
    #[staticmethod]
    #[pyo3(signature = (
        url,
        username = None,
        password = None,
        custom_http_auth = None,
        custom_http_headers = None,
        exclusion = None,
    ))]
    fn https(
        py: Python,
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        custom_http_headers: Option<Extractor<HeaderMap>>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        create_proxy(
            py,
            wreq::Proxy::https,
            url,
            username,
            password,
            custom_http_auth,
            custom_http_headers,
            exclusion,
        )
    }

    /// Creates a new proxy for all protocols.
    ///
    /// This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).
    #[staticmethod]
    #[pyo3(signature = (
        url,
        username = None,
        password = None,
        custom_http_auth = None,
        custom_http_headers = None,
        exclusion = None,
    ))]
    fn all(
        py: Python,
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
        custom_http_auth: Option<&str>,
        custom_http_headers: Option<Extractor<HeaderMap>>,
        exclusion: Option<&str>,
    ) -> PyResult<Self> {
        create_proxy(
            py,
            wreq::Proxy::all,
            url,
            username,
            password,
            custom_http_auth,
            custom_http_headers,
            exclusion,
        )
    }
}

/// Internal helper for creating a configured proxy.
/// Handles auth, custom headers, and exclusion rules.
#[allow(clippy::too_many_arguments)]
fn create_proxy<'py>(
    py: Python<'py>,
    proxy_fn: fn(&'py str) -> wreq::Result<wreq::Proxy>,
    url: &'py str,
    username: Option<&'py str>,
    password: Option<&'py str>,
    custom_http_auth: Option<&'py str>,
    custom_http_headers: Option<Extractor<HeaderMap>>,
    exclusion: Option<&'py str>,
) -> PyResult<Proxy> {
    py.allow_threads(|| {
        // Create base proxy using the provided constructor (http, https, all)
        let mut proxy = proxy_fn(url).map_err(Error::Library)?;

        // Convert the username and password to a basic auth header value.
        if let (Some(username), Some(password)) = (username, password) {
            proxy = proxy.basic_auth(username, password);
        }

        // Convert the custom HTTP auth string to a header value.
        if let Some(Ok(custom_http_auth)) = custom_http_auth.map(HeaderValue::from_str) {
            proxy = proxy.custom_http_auth(custom_http_auth);
        }

        // Convert the custom HTTP headers to a HeaderMap instance.
        if let Some(custom_http_headers) = custom_http_headers {
            proxy = proxy.custom_http_headers(custom_http_headers.0);
        }

        // Convert the exclusion list string to a NoProxy instance.
        if let Some(exclusion) = exclusion {
            proxy = proxy.no_proxy(wreq::NoProxy::from_string(exclusion));
        }

        Ok(Proxy(proxy))
    })
}
