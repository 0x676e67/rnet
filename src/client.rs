pub mod body;
pub mod req;
pub mod resp;

mod param;
mod query;

use std::{
    fmt,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    sync::Arc,
    time::Duration,
};

use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use req::{Request, WebSocketRequest};
use wreq::{Proxy, tls::CertStore};
use wreq_util::EmulationOption;

use self::{
    req::{execute_request, execute_websocket_request},
    resp::{BlockingResponse, BlockingWebSocket},
};
use crate::{
    cookie::Jar,
    dns::{HickoryDnsResolver, LookupIpStrategy, ResolverOptions},
    error::Error,
    extractor::Extractor,
    header::{HeaderMap, OrigHeaderMap},
    http::Method,
    http1::Http1Options,
    http2::Http2Options,
    redirect,
    tls::{Identity, KeyLog, TlsOptions, TlsVerify, TlsVersion},
};

/// A IP socket address.
#[derive(Clone, Copy, PartialEq, Eq)]
#[pyclass(eq, str, frozen)]
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
        self.0.fmt(f)
    }
}

/// A builder for `Client`.
#[derive(Default)]
struct Builder {
    /// The Emulation settings for the client.
    emulation: Option<Extractor<EmulationOption>>,
    /// The user agent to use for the client.
    user_agent: Option<PyBackedStr>,
    /// The headers to use for the client.
    headers: Option<HeaderMap>,
    /// The original headers to use for the client.
    orig_headers: Option<OrigHeaderMap>,
    /// Whether to use referer.
    referer: Option<bool>,
    /// Whether to redirect policy.
    redirect: Option<redirect::Policy>,

    // ========= Cookie options =========
    /// Whether to use cookie store.
    cookie_store: Option<bool>,
    /// Whether to use cookie store provider.
    cookie_provider: Option<Jar>,

    // ========= Timeout options =========
    /// The timeout to use for the client.
    timeout: Option<Duration>,
    /// The connect timeout to use for the client.
    connect_timeout: Option<Duration>,
    /// The read timeout to use for the client.
    read_timeout: Option<Duration>,

    // ========= TCP options =========
    /// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration.
    tcp_keepalive: Option<Duration>,
    /// Set the interval between TCP keepalive probes.
    tcp_keepalive_interval: Option<Duration>,
    /// Set the number of retries for TCP keepalive.
    tcp_keepalive_retries: Option<u32>,
    /// Set an optional user timeout for TCP sockets.
    tcp_user_timeout: Option<Duration>,
    /// Set that all sockets have `NO_DELAY` set.
    tcp_nodelay: Option<bool>,
    /// Set that all sockets have `SO_REUSEADDR` set.
    tcp_reuse_address: Option<bool>,

    // ========= Connection pool options =========
    /// Set an optional timeout for idle sockets being kept-alive.
    pool_idle_timeout: Option<Duration>,
    /// Sets the maximum idle connection per host allowed in the pool.
    pool_max_idle_per_host: Option<usize>,
    /// Sets the maximum number of connections in the pool.
    pool_max_size: Option<u32>,

    // ========= Protocol options =========
    /// Whether to use the HTTP/1 protocol only.
    http1_only: Option<bool>,
    /// Whether to use the HTTP/2 protocol only.
    http2_only: Option<bool>,
    /// Whether to use HTTPS only.
    https_only: Option<bool>,
    /// Sets the HTTP/1 options for the client.
    http1_options: Option<Http1Options>,
    /// sets the HTTP/2 options for the client.
    http2_options: Option<Http2Options>,

    // ========= TLS options =========
    /// Whether to verify the SSL certificate or root certificate file path.
    verify: Option<TlsVerify>,
    /// Whether to verify the hostname in the SSL certificate.
    verify_hostname: Option<bool>,
    /// Represents a private key and X509 cert as a client certificate.
    identity: Option<Identity>,
    /// Key logging policy for TLS session keys.
    keylog: Option<KeyLog>,
    /// Add TLS information as `TlsInfo` extension to responses.
    tls_info: Option<bool>,
    /// The minimum TLS version to use for the client.
    min_tls_version: Option<TlsVersion>,
    /// The maximum TLS version to use for the client.
    max_tls_version: Option<TlsVersion>,
    /// Sets the TLS options for the client.
    tls_options: Option<TlsOptions>,

    // ========= Network options =========
    /// Whether to disable the proxy for the client.
    no_proxy: Option<bool>,
    /// The proxy to use for the client.
    proxies: Option<Extractor<Vec<Proxy>>>,
    /// Bind to a local IP Address.
    local_address: Option<IpAddr>,
    /// Bind to local IP Addresses (IPv4, IPv6).
    local_addresses: Option<Extractor<(Option<Ipv4Addr>, Option<Ipv6Addr>)>>,
    /// Bind to an interface by `SO_BINDTODEVICE`.
    interface: Option<String>,

    // ========= DNS options =========
    dns_options: Option<ResolverOptions>,

    // ========= Compression options =========
    /// Sets gzip as an accepted encoding.
    gzip: Option<bool>,
    /// Sets brotli as an accepted encoding.
    brotli: Option<bool>,
    /// Sets deflate as an accepted encoding.
    deflate: Option<bool>,
    /// Sets zstd as an accepted encoding.
    zstd: Option<bool>,
}

impl FromPyObject<'_, '_> for Builder {
    type Error = PyErr;

    fn extract(ob: Borrowed<PyAny>) -> PyResult<Self> {
        let mut builder = Self::default();
        extract_option!(ob, builder, emulation);
        extract_option!(ob, builder, user_agent);
        extract_option!(ob, builder, headers);
        extract_option!(ob, builder, orig_headers);
        extract_option!(ob, builder, referer);
        extract_option!(ob, builder, redirect);

        extract_option!(ob, builder, cookie_store);
        extract_option!(ob, builder, cookie_provider);

        extract_option!(ob, builder, timeout);
        extract_option!(ob, builder, connect_timeout);
        extract_option!(ob, builder, read_timeout);

        extract_option!(ob, builder, tcp_keepalive);
        extract_option!(ob, builder, tcp_keepalive_interval);
        extract_option!(ob, builder, tcp_keepalive_retries);
        extract_option!(ob, builder, tcp_user_timeout);
        extract_option!(ob, builder, tcp_nodelay);
        extract_option!(ob, builder, tcp_reuse_address);

        extract_option!(ob, builder, pool_idle_timeout);
        extract_option!(ob, builder, pool_max_idle_per_host);
        extract_option!(ob, builder, pool_max_size);

        extract_option!(ob, builder, no_proxy);
        extract_option!(ob, builder, proxies);
        extract_option!(ob, builder, local_address);
        extract_option!(ob, builder, local_addresses);
        extract_option!(ob, builder, interface);

        extract_option!(ob, builder, https_only);
        extract_option!(ob, builder, http1_only);
        extract_option!(ob, builder, http2_only);
        extract_option!(ob, builder, http1_options);
        extract_option!(ob, builder, http2_options);

        extract_option!(ob, builder, verify);
        extract_option!(ob, builder, verify_hostname);
        extract_option!(ob, builder, identity);
        extract_option!(ob, builder, keylog);
        extract_option!(ob, builder, tls_info);
        extract_option!(ob, builder, min_tls_version);
        extract_option!(ob, builder, max_tls_version);
        extract_option!(ob, builder, tls_options);

        extract_option!(ob, builder, dns_options);

        extract_option!(ob, builder, gzip);
        extract_option!(ob, builder, brotli);
        extract_option!(ob, builder, deflate);
        extract_option!(ob, builder, zstd);
        Ok(builder)
    }
}

/// A client for making HTTP requests.
#[derive(Clone, Default)]
#[pyclass(subclass, frozen)]
pub struct Client(wreq::Client);

/// A blocking client for making HTTP requests.
#[pyclass(name = "Client", subclass, frozen)]
pub struct BlockingClient(Client);

// ====== Client =====

#[pymethods]
impl Client {
    /// Creates a new Client instance.
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new(py: Python, kwds: Option<Builder>) -> PyResult<Client> {
        py.detach(|| {
            // Create the client builder.
            let mut builder = wreq::Client::builder();
            if let Some(mut config) = kwds {
                // Emulation options.
                apply_option!(set_if_some_inner, builder, config.emulation, emulation);

                // User agent options.
                apply_option!(
                    set_if_some_map_ref,
                    builder,
                    config.user_agent,
                    user_agent,
                    AsRef::<str>::as_ref
                );

                // Default headers options.
                apply_option!(set_if_some_inner, builder, config.headers, default_headers);
                apply_option!(
                    set_if_some_inner,
                    builder,
                    config.orig_headers,
                    orig_headers
                );

                // Allow redirects options.
                apply_option!(set_if_some, builder, config.referer, referer);
                apply_option!(set_if_some_inner, builder, config.redirect, redirect);

                // Cookie options.
                apply_option!(set_if_some, builder, config.cookie_store, cookie_store);
                apply_option!(
                    set_if_some_inner,
                    builder,
                    config.cookie_provider,
                    cookie_provider
                );

                // TCP options.
                apply_option!(set_if_some, builder, config.tcp_keepalive, tcp_keepalive);
                apply_option!(
                    set_if_some,
                    builder,
                    config.tcp_keepalive_interval,
                    tcp_keepalive_interval
                );
                apply_option!(
                    set_if_some,
                    builder,
                    config.tcp_keepalive_retries,
                    tcp_keepalive_retries
                );
                #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
                apply_option!(
                    set_if_some,
                    builder,
                    config.tcp_user_timeout,
                    tcp_user_timeout
                );
                apply_option!(set_if_some, builder, config.tcp_nodelay, tcp_nodelay);
                apply_option!(
                    set_if_some,
                    builder,
                    config.tcp_reuse_address,
                    tcp_reuse_address
                );

                // Timeout options.
                apply_option!(set_if_some, builder, config.timeout, timeout);
                apply_option!(
                    set_if_some,
                    builder,
                    config.connect_timeout,
                    connect_timeout
                );
                apply_option!(set_if_some, builder, config.read_timeout, read_timeout);

                // Pool options.
                apply_option!(
                    set_if_some,
                    builder,
                    config.pool_idle_timeout,
                    pool_idle_timeout
                );
                apply_option!(
                    set_if_some,
                    builder,
                    config.pool_max_idle_per_host,
                    pool_max_idle_per_host
                );
                apply_option!(set_if_some, builder, config.pool_max_size, pool_max_size);

                // Protocol options.
                apply_option!(set_if_true, builder, config.http1_only, http1_only, false);
                apply_option!(set_if_true, builder, config.http2_only, http2_only, false);
                apply_option!(set_if_some, builder, config.https_only, https_only);
                apply_option!(
                    set_if_some_inner,
                    builder,
                    config.http1_options,
                    http1_options
                );
                apply_option!(
                    set_if_some_inner,
                    builder,
                    config.http2_options,
                    http2_options
                );

                // TLS options.
                apply_option!(
                    set_if_some_map,
                    builder,
                    config.min_tls_version,
                    min_tls_version,
                    TlsVersion::into_ffi
                );
                apply_option!(
                    set_if_some_map,
                    builder,
                    config.max_tls_version,
                    max_tls_version,
                    TlsVersion::into_ffi
                );
                apply_option!(set_if_some, builder, config.tls_info, tls_info);

                // TLS Verification options.
                apply_option!(
                    set_if_some,
                    builder,
                    config.verify_hostname,
                    verify_hostname
                );
                apply_option!(set_if_some_inner, builder, config.identity, identity);
                apply_option!(set_if_some_inner, builder, config.keylog, keylog);
                apply_option!(set_if_some_inner, builder, config.tls_options, tls_options);
                if let Some(verify) = config.verify.take() {
                    builder = match verify {
                        TlsVerify::Verification(verify) => builder.cert_verification(verify),
                        TlsVerify::CertificatePath(path_buf) => {
                            let pem_data = std::fs::read(path_buf)?;
                            let store =
                                CertStore::from_pem_stack(pem_data).map_err(Error::Library)?;
                            builder.cert_store(store)
                        }
                        TlsVerify::CertificateStore(cert_store) => builder.cert_store(cert_store.0),
                    }
                }

                // Network options.
                apply_option!(set_if_some_iter_inner, builder, config.proxies, proxy);
                apply_option!(set_if_true, builder, config.no_proxy, no_proxy, false);
                apply_option!(set_if_some, builder, config.local_address, local_address);
                apply_option!(
                    set_if_some_tuple_inner,
                    builder,
                    config.local_addresses,
                    local_addresses
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
                apply_option!(set_if_some, builder, config.interface, interface);

                // DNS options.
                builder = {
                    let dns_resolver = if let Some(options) = config.dns_options.take() {
                        for (domain, addrs) in options.resolve_to_addrs {
                            builder = builder.resolve_to_addrs(domain.as_ref().to_string(), addrs);
                        }
                        HickoryDnsResolver::new(options.lookup_ip_strategy)
                    } else {
                        HickoryDnsResolver::new(LookupIpStrategy::default())
                    };
                    builder.dns_resolver(Arc::new(dns_resolver))
                };

                // Compression options.
                apply_option!(set_if_some, builder, config.gzip, gzip);
                apply_option!(set_if_some, builder, config.brotli, brotli);
                apply_option!(set_if_some, builder, config.deflate, deflate);
                apply_option!(set_if_some, builder, config.zstd, zstd);
            }

            builder
                .build()
                .map(Client)
                .map_err(Error::Library)
                .map_err(Into::into)
        })
    }

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
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_request(self.clone().0, method, url, kwds),
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
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            execute_websocket_request(self.clone().0, url, kwds),
        )
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
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(execute_request(self.0.clone().0, method, url, kwds))
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
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(execute_websocket_request(self.0.clone().0, url, kwds))
                .map(Into::into)
        })
    }
}
