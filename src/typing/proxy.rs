use crate::{define_into_pyobject_todo, define_py_stub_gen, error::Error};

use super::HeaderMapExtractor;
use pyo3::{prelude::*, pybacked::PyBackedStr, types::PyList};
#[cfg(feature = "docs")]
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::header::HeaderValue;

macro_rules! proxy_method {
    ( $( { $(#[$meta:meta])* $name:ident, $proxy_fn:path} ),* ) => {
        #[cfg_attr(feature = "docs", gen_stub_pymethods)]
        #[pymethods]
        impl Proxy {
            $(
                $(#[$meta])*
                #[staticmethod]
                #[pyo3(signature = (
                    url,
                    username = None,
                    password = None,
                    custom_http_auth = None,
                    custom_http_headers = None,
                    exclusion = None,
                ))]
                #[inline]
                fn $name(
                    py: Python,
                    url: &str,
                    username: Option<&str>,
                    password: Option<&str>,
                    custom_http_auth: Option<&str>,
                    custom_http_headers: Option<HeaderMapExtractor>,
                    exclusion: Option<&str>,
                ) -> PyResult<Self> {
                    py.allow_threads(|| {
                        Self::create_proxy(
                            $proxy_fn,
                            url,
                            username,
                            password,
                            custom_http_auth,
                            custom_http_headers,
                            exclusion,
                        )
                    })
                }
            )*
        }
    };
}

/// A proxy server for a request.
/// Supports HTTP, HTTPS, SOCKS4, SOCKS4a, SOCKS5, and SOCKS5h protocols.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass]
pub struct Proxy(pub rquest::Proxy);

proxy_method! {
    {
        /// Creates a new HTTP proxy.
        ///
        /// This method sets up a proxy server for HTTP requests.
        ///
        /// # Arguments
        ///
        /// * `url` - The URL of the proxy server.
        /// * `username` - Optional username for proxy authentication.
        /// * `password` - Optional password for proxy authentication.
        /// * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        /// * `custom_http_headers` - Optional custom HTTP proxy headers.
        /// * `exclusion` - Optional list of domains to exclude from proxying.
        ///
        /// # Examples
        ///
        /// ```python
        /// import rnet
        ///
        /// proxy = rnet.Proxy.http("http://proxy.example.com")
        /// ```
        http,
        rquest::Proxy::http
    },
    {
        /// Creates a new HTTPS proxy.
        ///
        /// This method sets up a proxy server for HTTPS requests.
        ///
        /// # Arguments
        ///
        /// * `url` - The URL of the proxy server.
        /// * `username` - Optional username for proxy authentication.
        /// * `password` - Optional password for proxy authentication.
        /// * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        /// * `custom_http_headers` - Optional custom HTTP proxy headers.
        /// * `exclusion` - Optional list of domains to exclude from proxying.
        ///
        /// # Examples
        ///
        /// ```python
        /// import rnet
        ///
        /// proxy = rnet.Proxy.https("https://proxy.example.com")
        /// ```
        https,
        rquest::Proxy::https
    },
    {
        /// Creates a new proxy for all protocols.
        ///
        /// This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).
        ///
        /// # Arguments
        ///
        /// * `url` - The URL of the proxy server.
        /// * `username` - Optional username for proxy authentication.
        /// * `password` - Optional password for proxy authentication.
        /// * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        /// * `custom_http_headers` - Optional custom HTTP proxy headers.
        /// * `exclusion` - Optional list of domains to exclude from proxying.
        ///
        /// # Examples
        ///
        /// ```python
        /// import rnet
        ///
        /// proxy = rnet.Proxy.all("https://proxy.example.com")
        /// ```
        all,
        rquest::Proxy::all
    }
}

impl Proxy {
    fn create_proxy<'a>(
        proxy_fn: impl Fn(&'a str) -> Result<rquest::Proxy, rquest::Error>,
        url: &'a str,
        username: Option<&'a str>,
        password: Option<&str>,
        custom_http_auth: Option<&'a str>,
        custom_http_headers: Option<HeaderMapExtractor>,
        exclusion: Option<&'a str>,
    ) -> PyResult<Self> {
        let mut proxy = proxy_fn(url).map_err(Error::RquestError)?;
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
            proxy = proxy.no_proxy(rquest::NoProxy::from_string(exclusion))
        }

        Ok(Proxy(proxy))
    }
}

pub struct ProxyExtractor(pub rquest::Proxy);

impl FromPyObject<'_> for ProxyExtractor {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(proxy_str) = ob.extract::<PyBackedStr>() {
            let proxy = rquest::Proxy::all(proxy_str.as_ref() as &str)
                .map(Self)
                .map_err(Error::RquestError)?;

            return Ok(proxy);
        }

        let proxy = ob.downcast::<Proxy>()?;
        let proxy = proxy.borrow().0.clone();
        Ok(Self(proxy))
    }
}

pub struct ProxyListExtractor(pub Vec<rquest::Proxy>);

impl FromPyObject<'_> for ProxyListExtractor {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let proxies = ob.downcast::<PyList>()?;
        let len = proxies.len();
        proxies
            .into_iter()
            .try_fold(Vec::with_capacity(len), |mut list, proxy| {
                let proxy = proxy.downcast::<Proxy>()?;
                list.push(proxy.borrow().0.clone());
                Ok::<_, PyErr>(list)
            })
            .map(Self)
    }
}

define_into_pyobject_todo!(ProxyExtractor);

define_into_pyobject_todo!(ProxyListExtractor);

define_py_stub_gen!(ProxyExtractor, "typing.Union[Proxy, str]", "typing");

define_py_stub_gen!(ProxyListExtractor, "typing.List[Proxy]", "typing");
