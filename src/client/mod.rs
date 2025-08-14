pub mod body;
pub mod multipart;
pub mod request;
pub mod response;

mod dns;
mod future;

use std::{fmt, net::IpAddr, sync::Arc, time::Duration};

use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use pyo3_async_runtimes::tokio::future_into_py;
use request::{Request, WebSocketRequest};
use wreq::{
    Proxy,
    header::{self, HeaderMap},
    redirect::Policy,
    tls::CertStore,
};
use wreq_util::EmulationOption;

use self::{
    dns::HickoryDnsResolver,
    response::{BlockingResponse, BlockingWebSocket},
};
use crate::{
    client::{
        future::AllowThreads,
        response::{Response, WebSocket},
    },
    error::Error,
    extractor::Extractor,
    http::{Method, Version, cookie::Jar},
    tls::{SslVerify, TlsVersion},
};

/// A IP socket address.
#[derive(Clone, Copy, PartialEq, Eq)]
#[pyclass(eq, str)]
pub struct SocketAddr(pub std::net::SocketAddr);

#[pymethods]
impl SocketAddr {
    /// Returns the IP address of the socket address.
    fn ip<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.0.ip().into_bound_py_any(py)
    }

    /// Returns the port number of the socket address.
    fn port(&self) -> u16 {
        self.0.port()
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A builder for `Client`.
#[derive(Default)]
pub struct Builder {
    /// The Emulation settings for the client.
    pub emulation: Option<Extractor<EmulationOption>>,

    /// The user agent to use for the client.
    pub user_agent: Option<PyBackedStr>,

    /// The headers to use for the client.
    pub default_headers: Option<Extractor<HeaderMap>>,

    /// Whether to use referer.
    pub referer: Option<bool>,

    /// Whether to allow redirects.
    pub allow_redirects: Option<bool>,

    /// The maximum number of redirects to follow.
    pub max_redirects: Option<usize>,

    // ========= Cookie options =========
    /// Whether to use cookie store.
    pub cookie_store: Option<bool>,

    /// Whether to use cookie store provider.
    pub cookie_provider: Option<Jar>,

    // ========= Timeout options =========
    /// The timeout to use for the client. (in seconds)
    pub timeout: Option<u64>,

    /// The connect timeout to use for the client. (in seconds)
    pub connect_timeout: Option<u64>,

    /// The read timeout to use for the client. (in seconds)
    pub read_timeout: Option<u64>,

    // ========= TCP options =========
    /// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration. (in seconds)
    pub tcp_keepalive: Option<u64>,

    /// Set the interval between TCP keepalive probes. (in seconds)
    pub tcp_keepalive_interval: Option<u64>,

    /// Set the number of retries for TCP keepalive.
    pub tcp_keepalive_retries: Option<u32>,

    /// Set an optional user timeout for TCP sockets. (in seconds)    
    pub tcp_user_timeout: Option<u64>,

    /// Set that all sockets have `NO_DELAY` set.
    pub tcp_nodelay: Option<bool>,

    /// Set that all sockets have `SO_REUSEADDR` set.
    pub tcp_reuse_address: Option<bool>,

    // ========= Connection pool options =========
    /// Set an optional timeout for idle sockets being kept-alive. (in seconds)
    pub pool_idle_timeout: Option<u64>,

    /// Sets the maximum idle connection per host allowed in the pool.
    pub pool_max_idle_per_host: Option<usize>,

    /// Sets the maximum number of connections in the pool.
    pub pool_max_size: Option<u32>,

    /// Disable keep-alive for the client.
    pub no_keepalive: Option<bool>,

    // ========= Protocol options =========
    /// Whether to use the HTTP/1 protocol only.
    pub http1_only: Option<bool>,

    /// Whether to use the HTTP/2 protocol only.
    pub http2_only: Option<bool>,

    /// Whether to use HTTPS only.
    pub https_only: Option<bool>,

    /// The maximum number of times to retry a client.
    pub http2_max_retry_count: Option<usize>,

    // ========= TLS options =========
    /// Whether to verify the SSL certificate or root certificate file path.
    pub verify: Option<SslVerify>,

    /// Add TLS information as `TlsInfo` extension to responses.
    pub tls_info: Option<bool>,

    /// The minimum TLS version to use for the client.
    pub min_tls_version: Option<TlsVersion>,

    /// The maximum TLS version to use for the client.
    pub max_tls_version: Option<TlsVersion>,

    // ========= Network options =========
    /// Whether to disable the proxy for the client.
    pub no_proxy: Option<bool>,

    /// The proxy to use for the client.
    pub proxies: Option<Extractor<Vec<Proxy>>>,

    /// Bind to a local IP Address.
    pub local_address: Option<Extractor<IpAddr>>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    pub interface: Option<String>,

    // ========= Compression options =========
    /// Sets gzip as an accepted encoding.
    pub gzip: Option<bool>,

    /// Sets brotli as an accepted encoding.
    pub brotli: Option<bool>,

    /// Sets deflate as an accepted encoding.
    pub deflate: Option<bool>,

    /// Sets zstd as an accepted encoding.
    pub zstd: Option<bool>,
}

impl<'py> FromPyObject<'py> for Builder {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, emulation);
        extract_option!(ob, params, user_agent);
        extract_option!(ob, params, default_headers);
        extract_option!(ob, params, referer);
        extract_option!(ob, params, allow_redirects);

        extract_option!(ob, params, cookie_store);
        extract_option!(ob, params, cookie_provider);

        extract_option!(ob, params, timeout);
        extract_option!(ob, params, connect_timeout);
        extract_option!(ob, params, read_timeout);

        extract_option!(ob, params, tcp_keepalive);
        extract_option!(ob, params, tcp_keepalive_interval);
        extract_option!(ob, params, tcp_keepalive_retries);
        extract_option!(ob, params, tcp_user_timeout);
        extract_option!(ob, params, tcp_nodelay);
        extract_option!(ob, params, tcp_reuse_address);

        extract_option!(ob, params, pool_idle_timeout);
        extract_option!(ob, params, pool_max_idle_per_host);
        extract_option!(ob, params, pool_max_size);
        extract_option!(ob, params, no_keepalive);

        extract_option!(ob, params, no_proxy);
        extract_option!(ob, params, proxies);
        extract_option!(ob, params, local_address);
        extract_option!(ob, params, interface);

        extract_option!(ob, params, http1_only);
        extract_option!(ob, params, http2_only);
        extract_option!(ob, params, https_only);
        extract_option!(ob, params, verify);
        extract_option!(ob, params, http2_max_retry_count);
        extract_option!(ob, params, tls_info);
        extract_option!(ob, params, min_tls_version);
        extract_option!(ob, params, max_tls_version);

        extract_option!(ob, params, gzip);
        extract_option!(ob, params, brotli);
        extract_option!(ob, params, deflate);
        extract_option!(ob, params, zstd);
        Ok(params)
    }
}

/// A client for making HTTP requests.
#[derive(Clone)]
#[pyclass(subclass)]
pub struct Client(wreq::Client);

/// A blocking client for making HTTP requests.
#[pyclass(name = "Client", subclass)]
pub struct BlockingClient(Client);

// ====== Client =====

#[pymethods]
impl Client {
    /// Make a GET request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn get<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::GET, url, kwds)
    }

    /// Make a HEAD request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn head<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::HEAD, url, kwds)
    }

    /// Make a POST request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn post<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::POST, url, kwds)
    }

    /// Make a PUT request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn put<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::PUT, url, kwds)
    }

    /// Make a DELETE request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn delete<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::DELETE, url, kwds)
    }

    /// Make a PATCH request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn patch<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::PATCH, url, kwds)
    }

    /// Make a OPTIONS request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn options<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::OPTIONS, url, kwds)
    }

    /// Make a TRACE request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn trace<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, Method::TRACE, url, kwds)
    }

    /// Make a request with the given method and URL.
    #[inline]
    #[pyo3(signature = (method, url, **kwds))]
    pub fn request<'py>(
        &self,
        py: Python<'py>,
        method: Method,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.clone();
        future_into_py(
            py,
            AllowThreads::new_future(py, client.execute_request(method, url, kwds)),
        )
    }

    /// Make a WebSocket request to the given URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn websocket<'py>(
        &self,
        py: Python<'py>,
        url: PyBackedStr,
        kwds: Option<WebSocketRequest>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.clone();
        future_into_py(
            py,
            AllowThreads::new_future(py, client.execute_websocket_request(url, kwds)),
        )
    }
}

#[pymethods]
impl Client {
    /// Creates a new Client instance.
    #[new]
    #[pyo3(signature = (**kwds))]
    pub fn new(py: Python, mut kwds: Option<Builder>) -> PyResult<Client> {
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
            apply_option!(apply_if_some, builder, params.tcp_nodelay, tcp_nodelay);
            apply_option!(
                apply_if_some,
                builder,
                params.tcp_reuse_address,
                tcp_reuse_address
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
                .dns_resolver(HickoryDnsResolver::new())
                .build()
                .map(Client)
                .map_err(Error::Request)
                .map_err(Into::into)
        })
    }
}

impl Client {
    async fn execute_request<U>(
        self,
        method: Method,
        url: U,
        mut params: Option<Request>,
    ) -> PyResult<Response>
    where
        U: AsRef<str>,
    {
        let params = params.get_or_insert_default();
        let mut builder = self.0.request(method.into_ffi(), url.as_ref());

        // Emulation options.
        apply_option!(apply_if_some_inner, builder, params.emulation, emulation);

        // Version options.
        apply_option!(
            apply_transformed_option,
            builder,
            params.version,
            version,
            Version::into_ffi
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
            params.read_timeout,
            read_timeout,
            Duration::from_secs
        );

        // Network options.
        apply_option!(apply_if_some_inner, builder, params.proxy, proxy);
        apply_option!(
            apply_if_some_inner,
            builder,
            params.local_address,
            local_address
        );
        #[cfg(any(
            target_os = "android",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "linux",
            target_os = "macos",
            target_os = "solaris",
            target_os = "tvos",
            target_os = "visionos",
            target_os = "watchos",
        ))]
        apply_option!(apply_if_some, builder, params.interface, interface);

        // Headers options.
        apply_option!(apply_if_some_inner, builder, params.headers, headers);
        apply_option!(
            apply_if_some,
            builder,
            params.default_headers,
            default_headers
        );

        // Authentication options.
        apply_option!(
            apply_transformed_option_ref,
            builder,
            params.auth,
            auth,
            AsRef::<str>::as_ref
        );

        // Bearer authentication options.
        apply_option!(apply_if_some, builder, params.bearer_auth, bearer_auth);

        // Basic authentication options.
        if let Some(basic_auth) = params.basic_auth.take() {
            builder = builder.basic_auth(basic_auth.0, basic_auth.1);
        }

        // Cookies options.
        if let Some(cookies) = params.cookies.take() {
            for cookie in cookies.0 {
                builder = builder.header_append(header::COOKIE, cookie);
            }
        }

        // Allow redirects options.
        match params.allow_redirects {
            Some(false) => {
                builder = builder.redirect(Policy::none());
            }
            Some(true) => {
                builder = builder.redirect(
                    params
                        .max_redirects
                        .take()
                        .map(Policy::limited)
                        .unwrap_or_default(),
                );
            }
            None => {}
        };

        // Compression options.
        apply_option!(apply_if_some, builder, params.gzip, gzip);
        apply_option!(apply_if_some, builder, params.brotli, brotli);
        apply_option!(apply_if_some, builder, params.deflate, deflate);
        apply_option!(apply_if_some, builder, params.zstd, zstd);

        // Query options.
        apply_option!(apply_if_some_ref, builder, params.query, query);

        // Form options.
        apply_option!(apply_if_some_ref, builder, params.form, form);

        // JSON options.
        apply_option!(apply_if_some_ref, builder, params.json, json);

        // Body options.
        apply_option!(apply_if_some, builder, params.body, body);

        // Multipart options.
        apply_option!(apply_if_some_inner, builder, params.multipart, multipart);

        // Send request.
        builder
            .send()
            .await
            .map(Response::new)
            .map_err(Error::Request)
            .map_err(Into::into)
    }

    async fn execute_websocket_request<U>(
        self,
        url: U,
        mut params: Option<WebSocketRequest>,
    ) -> PyResult<WebSocket>
    where
        U: AsRef<str>,
    {
        let params = params.get_or_insert_default();
        let mut builder = self.0.websocket(url.as_ref());

        // The protocols to use for the request.
        apply_option!(apply_if_some, builder, params.protocols, protocols);

        // The WebSocket config
        apply_option!(
            apply_if_some,
            builder,
            params.read_buffer_size,
            read_buffer_size
        );
        apply_option!(
            apply_if_some,
            builder,
            params.write_buffer_size,
            write_buffer_size
        );
        apply_option!(
            apply_if_some,
            builder,
            params.max_write_buffer_size,
            max_write_buffer_size
        );
        apply_option!(
            apply_if_some,
            builder,
            params.max_frame_size,
            max_frame_size
        );
        apply_option!(
            apply_if_some,
            builder,
            params.max_message_size,
            max_message_size
        );
        apply_option!(
            apply_if_some,
            builder,
            params.accept_unmasked_frames,
            accept_unmasked_frames
        );

        // Use http2 options.
        apply_option!(
            apply_option_or_default,
            builder,
            params.force_http2,
            force_http2,
            false
        );

        // Network options.
        apply_option!(apply_if_some_inner, builder, params.proxy, proxy);
        apply_option!(
            apply_if_some_inner,
            builder,
            params.local_address,
            local_address
        );
        #[cfg(any(
            target_os = "android",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "linux",
            target_os = "macos",
            target_os = "solaris",
            target_os = "tvos",
            target_os = "visionos",
            target_os = "watchos",
        ))]
        apply_option!(apply_if_some, builder, params.interface, interface);

        // Authentication options.
        apply_option!(
            apply_transformed_option_ref,
            builder,
            params.auth,
            auth,
            AsRef::<str>::as_ref
        );

        // Bearer authentication options.
        apply_option!(apply_if_some, builder, params.bearer_auth, bearer_auth);

        // Basic authentication options.
        if let Some(basic_auth) = params.basic_auth.take() {
            builder = builder.basic_auth(basic_auth.0, basic_auth.1);
        }

        // Headers options.
        apply_option!(apply_if_some_inner, builder, params.headers, headers);
        apply_option!(
            apply_if_some,
            builder,
            params.default_headers,
            default_headers
        );

        // Cookies options.
        if let Some(cookies) = params.cookies.take() {
            for cookie in cookies.0 {
                builder = builder.header_append(header::COOKIE, cookie);
            }
        }

        // Query options.
        apply_option!(apply_if_some_ref, builder, params.query, query);

        // Send the WebSocket request.
        let response = builder.send().await.map_err(Error::Request)?;
        WebSocket::new(response)
            .await
            .map_err(Error::Request)
            .map_err(Into::into)
    }
}

// ====== BlockingClient ======

#[pymethods]
impl BlockingClient {
    /// Make a GET request to the specified URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn get(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::GET, url, kwds)
    }

    /// Make a POST request to the specified URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn post(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::POST, url, kwds)
    }

    /// Make a PUT request to the specified URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn put(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::PUT, url, kwds)
    }

    /// Make a PATCH request to the specified URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn patch(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::PATCH, url, kwds)
    }

    /// Make a DELETE request to the specified URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn delete(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::DELETE, url, kwds)
    }

    /// Make a HEAD request to the specified URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn head(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::HEAD, url, kwds)
    }

    /// Make a OPTIONS request to the specified URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn options(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::OPTIONS, url, kwds)
    }

    /// Make a TRACE request to the specified URL.
    #[inline]
    #[pyo3(signature = (url, **kwds))]
    pub fn trace(
        &self,
        py: Python<'_>,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        self.request(py, Method::TRACE, url, kwds)
    }

    /// Make a rqeuest with the specified method and URL.
    #[pyo3(signature = (method, url, **kwds))]
    pub fn request(
        &self,
        py: Python,
        method: Method,
        url: PyBackedStr,
        kwds: Option<Request>,
    ) -> PyResult<BlockingResponse> {
        py.allow_threads(|| {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(client.execute_request(method, url, kwds))
                .map(Into::into)
        })
    }

    /// Make a WebSocket request to the specified URL.
    #[pyo3(signature = (url, **kwds))]
    pub fn websocket(
        &self,
        py: Python,
        url: PyBackedStr,
        kwds: Option<WebSocketRequest>,
    ) -> PyResult<BlockingWebSocket> {
        py.allow_threads(|| {
            let client = self.0.clone();
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(client.execute_websocket_request(url, kwds))
                .map(Into::into)
        })
    }
}

#[pymethods]
impl BlockingClient {
    /// Creates a new blocking Client instance.
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new(py: Python, kwds: Option<Builder>) -> PyResult<BlockingClient> {
        Client::new(py, kwds).map(BlockingClient)
    }
}

pub mod short {
    use std::sync::LazyLock;

    use super::{
        Client, HickoryDnsResolver, Method, PyResult, Request, Response, WebSocket,
        WebSocketRequest,
    };

    static DEFAULT_CLIENT: LazyLock<wreq::Client> = LazyLock::new(|| {
        let builder = wreq::Client::builder();
        builder
            .dns_resolver(HickoryDnsResolver::new())
            .pool_max_idle_per_host(0)
            .build()
            .expect("Failed to build the default client.")
    });

    #[inline]
    pub async fn shortcut_request<U>(
        method: Method,
        url: U,
        params: Option<Request>,
    ) -> PyResult<Response>
    where
        U: AsRef<str>,
    {
        Client(DEFAULT_CLIENT.clone())
            .execute_request(method, url, params)
            .await
    }

    #[inline]
    pub async fn shortcut_websocket_request<U>(
        url: U,
        params: Option<WebSocketRequest>,
    ) -> PyResult<WebSocket>
    where
        U: AsRef<str>,
    {
        Client(DEFAULT_CLIENT.clone())
            .execute_websocket_request(url, params)
            .await
    }
}
