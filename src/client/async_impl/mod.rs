pub mod response;

use std::{ops::Deref, sync::Arc, time::Duration};

use pyo3::{prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use wreq::{redirect::Policy, tls::CertStore};

use super::{
    opts::{execute_request, execute_websocket_request},
    param::{ClientParams, RequestParams, WebSocketParams},
};
use crate::{
    client::dns::HickoryDnsResolver,
    error::Error,
    http::Method,
    tls::{SslVerify, TlsVersion},
};

/// A client for making HTTP requests.
#[pyclass(subclass)]
pub struct Client(wreq::Client);

impl Deref for Client {
    type Target = wreq::Client;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[pymethods]
impl Client {
    /// Make a GET request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn get<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::GET, url, kwds)
    }

    /// Make a HEAD request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn head<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::HEAD, url, kwds)
    }

    /// Make a POST request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn post<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::POST, url, kwds)
    }

    /// Make a PUT request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn put<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::PUT, url, kwds)
    }

    /// Make a DELETE request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn delete<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::DELETE, url, kwds)
    }

    /// Make a PATCH request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn patch<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::PATCH, url, kwds)
    }

    /// Make a OPTIONS request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn options<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::OPTIONS, url, kwds)
    }

    /// Make a TRACE request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn trace<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::TRACE, url, kwds)
    }

    /// Make a request with the given method and URL.
    #[pyo3(signature = (method, url, **kwds))]
    pub fn request<'py>(
        &self,
        py: Python<'py>,
        method: Method,
        url: PyBackedStr,
        kwds: Option<RequestParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.0.clone();
        future_into_py(py, execute_request(client, method, url, kwds))
    }

    /// Make a WebSocket request to the given URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn websocket<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<WebSocketParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.0.clone();
        future_into_py(py, execute_websocket_request(client, url, kwds))
    }
}

#[pymethods]
impl Client {
    /// Creates a new Client instance.
    #[new]
    #[pyo3(signature = (**kwds))]
    pub fn new(py: Python, mut kwds: Option<ClientParams>) -> PyResult<Client> {
        py.allow_threads(|| {
            let params = kwds.get_or_insert_default();
            let mut builder = wreq::Client::builder();

            // Emulation options.
            apply_option!(apply_if_some_inner, builder, params.emulation, emulation);

            // User agent options.
            apply_option!(
                apply_transformed_option_ref,
                builder,
                params.user_agent,
                user_agent,
                AsRef::<str>::as_ref
            );

            // Default headers options.
            apply_option!(
                apply_if_some_inner,
                builder,
                params.default_headers,
                default_headers
            );

            // Referer options.
            apply_option!(apply_if_some, builder, params.referer, referer);

            // Allow redirects options.
            apply_option!(
                apply_option_or_default_with_value,
                builder,
                params.allow_redirects,
                redirect,
                false,
                params
                    .max_redirects
                    .take()
                    .map(Policy::limited)
                    .unwrap_or_default()
            );

            // Cookie options.
            if let Some(cookie_provider) = params.cookie_provider.take() {
                builder = builder.cookie_provider(Arc::new(cookie_provider));
            } else {
                apply_option!(apply_if_some, builder, params.cookie_store, cookie_store);
            }

            // TCP options.
            apply_option!(
                apply_option_or_default,
                builder,
                params.no_keepalive,
                no_keepalive,
                false
            );
            apply_option!(
                apply_transformed_option,
                builder,
                params.tcp_keepalive,
                tcp_keepalive,
                Duration::from_secs
            );
            apply_option!(
                apply_transformed_option,
                builder,
                params.tcp_keepalive_interval,
                tcp_keepalive_interval,
                Duration::from_secs
            );
            apply_option!(
                apply_if_some,
                builder,
                params.tcp_keepalive_retries,
                tcp_keepalive_retries
            );
            #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
            apply_option!(
                apply_transformed_option,
                builder,
                params.tcp_user_timeout,
                tcp_user_timeout,
                Duration::from_secs
            );

            // Timeout options.
            apply_option!(
                apply_transformed_option,
                builder,
                params.timeout,
                timeout,
                Duration::from_secs
            );
            apply_option!(
                apply_transformed_option,
                builder,
                params.connect_timeout,
                connect_timeout,
                Duration::from_secs
            );
            apply_option!(
                apply_transformed_option,
                builder,
                params.read_timeout,
                read_timeout,
                Duration::from_secs
            );

            // Pool options.
            apply_option!(
                apply_transformed_option,
                builder,
                params.pool_idle_timeout,
                pool_idle_timeout,
                Duration::from_secs
            );
            apply_option!(
                apply_if_some,
                builder,
                params.pool_max_idle_per_host,
                pool_max_idle_per_host
            );
            apply_option!(apply_if_some, builder, params.pool_max_size, pool_max_size);

            // Protocol options.
            apply_option!(
                apply_option_or_default,
                builder,
                params.http1_only,
                http1_only,
                false
            );
            apply_option!(
                apply_option_or_default,
                builder,
                params.http2_only,
                http2_only,
                false
            );
            apply_option!(apply_if_some, builder, params.https_only, https_only);
            apply_option!(apply_if_some, builder, params.tcp_nodelay, tcp_nodelay);
            apply_option!(
                apply_if_some,
                builder,
                params.http2_max_retry_count,
                http2_max_retry
            );

            // TLS options.
            apply_option!(
                apply_transformed_option,
                builder,
                params.min_tls_version,
                min_tls_version,
                TlsVersion::into_ffi
            );
            apply_option!(
                apply_transformed_option,
                builder,
                params.max_tls_version,
                max_tls_version,
                TlsVersion::into_ffi
            );
            apply_option!(apply_if_some, builder, params.tls_info, tls_info);

            // SSL Verification options.
            if let Some(verify) = params.verify.take() {
                builder = match verify {
                    SslVerify::DisableSslVerification(verify) => builder.cert_verification(verify),
                    SslVerify::RootCertificateFilepath(path_buf) => {
                        let pem_data = std::fs::read(path_buf)?;
                        let store = CertStore::from_pem_stack(pem_data).map_err(Error::Request)?;
                        builder.cert_store(store)
                    }
                }
            }

            // Network options.
            if let Some(proxies) = params.proxies.take() {
                for proxy in proxies.0 {
                    builder = builder.proxy(proxy);
                }
            }
            apply_option!(
                apply_option_or_default,
                builder,
                params.no_proxy,
                no_proxy,
                false
            );
            apply_option!(
                apply_if_some_inner,
                builder,
                params.local_address,
                local_address
            );
            #[cfg(any(
                target_os = "android",
                target_os = "fuchsia",
                target_os = "linux",
                target_os = "ios",
                target_os = "visionos",
                target_os = "macos",
                target_os = "tvos",
                target_os = "watchos"
            ))]
            apply_option!(apply_if_some, builder, params.interface, interface);

            // Compression options.
            apply_option!(apply_if_some, builder, params.gzip, gzip);
            apply_option!(apply_if_some, builder, params.brotli, brotli);
            apply_option!(apply_if_some, builder, params.deflate, deflate);
            apply_option!(apply_if_some, builder, params.zstd, zstd);

            builder
                .dns_resolver(Arc::new(HickoryDnsResolver::new()))
                .build()
                .map(Client)
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }
}
