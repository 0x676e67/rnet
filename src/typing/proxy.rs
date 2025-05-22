use super::HeaderMapExtractor;
use crate::error::Error;
use pyo3::{prelude::*, pybacked::PyBackedStr, types::PyList};
use rquest::header::HeaderValue;

macro_rules! proxy_method {
    ( $( { $(#[$meta:meta])* $name:ident, $proxy_fn:path} ),* ) => {
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
#[pyclass]
pub struct Proxy(pub rquest::Proxy);

proxy_method! {
    {
        /// Creates a new HTTP proxy.
        ///
        /// This method sets up a proxy server for HTTP requests.
        http,
        rquest::Proxy::http
    },
    {
        /// Creates a new HTTPS proxy.
        ///
        /// This method sets up a proxy server for HTTPS requests.
        https,
        rquest::Proxy::https
    },
    {
        /// Creates a new proxy for all protocols.
        ///
        /// This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).
        all,
        rquest::Proxy::all
    }
}

impl Proxy {
    fn create_proxy<'py>(
        proxy_fn: impl Fn(&'py str) -> rquest::Result<rquest::Proxy>,
        url: &'py str,
        username: Option<&'py str>,
        password: Option<&'py str>,
        custom_http_auth: Option<&'py str>,
        custom_http_headers: Option<HeaderMapExtractor>,
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
                .map_err(Error::Request)?;

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
