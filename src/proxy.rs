use pyo3::prelude::*;
use wreq::header::{HeaderMap, HeaderValue};

use crate::{error::Error, extractor::Extractor};

/// A proxy server for a request.
/// Supports HTTP, HTTPS, SOCKS4, SOCKS4a, SOCKS5, and SOCKS5h protocols.
#[pyclass(subclass)]
pub struct Proxy(pub wreq::Proxy);

proxy_method! {
    {
        /// Creates a new HTTP proxy.
        ///
        /// This method sets up a proxy server for HTTP requests.
        http,
        wreq::Proxy::http
    },
    {
        /// Creates a new HTTPS proxy.
        ///
        /// This method sets up a proxy server for HTTPS requests.
        https,
        wreq::Proxy::https
    },
    {
        /// Creates a new proxy for all protocols.
        ///
        /// This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).
        all,
        wreq::Proxy::all
    }
}

impl Proxy {
    fn create_proxy<'py>(
        proxy_fn: impl Fn(&'py str) -> wreq::Result<wreq::Proxy>,
        url: &'py str,
        username: Option<&'py str>,
        password: Option<&'py str>,
        custom_http_auth: Option<&'py str>,
        custom_http_headers: Option<Extractor<HeaderMap>>,
        exclusion: Option<&'py str>,
    ) -> PyResult<Self> {
        let mut proxy = proxy_fn(url).map_err(Error::Request)?;
        // Convert the username and password to a basic auth header value.
        if let (Some(username), Some(password)) = (username, password) {
            proxy = proxy.basic_auth(username, password)
        }

        // Convert the custom HTTP auth string to a header value.
        if let Some(Ok(custom_http_auth)) = custom_http_auth.map(HeaderValue::from_str) {
            proxy = proxy.custom_http_auth(custom_http_auth)
        }

        // Convert the custom HTTP headers to a HeaderMap instance.
        if let Some(custom_http_headers) = custom_http_headers {
            proxy = proxy.custom_http_headers(custom_http_headers.0)
        }

        // Convert the exclusion list to a NoProxy instance.
        if let Some(exclusion) = exclusion {
            proxy = proxy.no_proxy(wreq::NoProxy::from_string(exclusion))
        }

        Ok(Proxy(proxy))
    }
}
