pub mod body;
pub mod req;
pub mod resp;

mod dns;
mod nogil;

use std::{fmt, net::IpAddr, sync::Arc, time::Duration};

use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use req::{Request, WebSocketRequest};
use wreq::{
    Proxy,
    header::{self, HeaderMap, OrigHeaderMap},
    redirect::Policy,
    tls::CertStore,
};
use wreq_util::EmulationOption;

use self::{
    dns::HickoryDnsResolver,
    resp::{BlockingResponse, BlockingWebSocket},
};
use crate::{
    bridge::Runtime,
    client::resp::{Response, WebSocket},
    error::Error,
    extractor::Extractor,
    http::{Method, cookie::Jar},
    http1::Http1Options,
    http2::Http2Options,
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
        write!(f, "{}", self.0)
    }
}

/// A builder for `Client`.
#[derive(Default)]
pub struct Builder {
    /// The Emulation settings for the client.
    emulation: Option<Extractor<EmulationOption>>,
    /// The user agent to use for the client.
    user_agent: Option<PyBackedStr>,
    /// The headers to use for the client.
    headers: Option<Extractor<HeaderMap>>,
    /// The original headers to use for the client.
    orig_headers: Option<Extractor<OrigHeaderMap>>,
    /// Whether to use referer.
    referer: Option<bool>,
    /// Whether to keep track of request history.
    history: Option<bool>,
    /// Whether to allow redirects.
    allow_redirects: Option<bool>,
    /// The maximum number of redirects to follow.
    max_redirects: Option<usize>,

    // ========= Cookie options =========
    /// Whether to use cookie store.
    cookie_store: Option<bool>,
    /// Whether to use cookie store provider.
    cookie_provider: Option<Jar>,

    // ========= Timeout options =========
    /// The timeout to use for the client. (in seconds)
    timeout: Option<u64>,
    /// The connect timeout to use for the client. (in seconds)
    connect_timeout: Option<u64>,
    /// The read timeout to use for the client. (in seconds)
    read_timeout: Option<u64>,

    // ========= TCP options =========
    /// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration. (in seconds)
    tcp_keepalive: Option<u64>,
    /// Set the interval between TCP keepalive probes. (in seconds)
    tcp_keepalive_interval: Option<u64>,
    /// Set the number of retries for TCP keepalive.
    tcp_keepalive_retries: Option<u32>,
    /// Set an optional user timeout for TCP sockets. (in seconds)
    tcp_user_timeout: Option<u64>,
    /// Set that all sockets have `NO_DELAY` set.
    tcp_nodelay: Option<bool>,
    /// Set that all sockets have `SO_REUSEADDR` set.
    tcp_reuse_address: Option<bool>,

    // ========= Connection pool options =========
    /// Set an optional timeout for idle sockets being kept-alive. (in seconds)
    pool_idle_timeout: Option<u64>,
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
    local_address: Option<Extractor<IpAddr>>,
    /// Bind to an interface by `SO_BINDTODEVICE`.
    interface: Option<String>,

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

impl<'py> FromPyObject<'py> for Builder {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, emulation);
        extract_option!(ob, params, user_agent);
        extract_option!(ob, params, headers);
        extract_option!(ob, params, orig_headers);
        extract_option!(ob, params, referer);
        extract_option!(ob, params, history);
        extract_option!(ob, params, allow_redirects);
        extract_option!(ob, params, max_redirects);

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

        extract_option!(ob, params, no_proxy);
        extract_option!(ob, params, proxies);
        extract_option!(ob, params, local_address);
        extract_option!(ob, params, interface);

        extract_option!(ob, params, https_only);
        extract_option!(ob, params, http1_only);
        extract_option!(ob, params, http2_only);
        extract_option!(ob, params, http1_options);
        extract_option!(ob, params, http2_options);

        extract_option!(ob, params, verify);
        extract_option!(ob, params, verify_hostname);
        extract_option!(ob, params, identity);
        extract_option!(ob, params, keylog);
        extract_option!(ob, params, tls_info);
        extract_option!(ob, params, min_tls_version);
        extract_option!(ob, params, max_tls_version);
        extract_option!(ob, params, tls_options);

        extract_option!(ob, params, gzip);
        extract_option!(ob, params, brotli);
        extract_option!(ob, params, deflate);
        extract_option!(ob, params, zstd);
        Ok(params)
    }
}

/// A client for making HTTP requests.
#[derive(Clone)]
#[pyclass(subclass, frozen)]
pub struct Client(wreq::Client);

/// A blocking client for making HTTP requests.
#[pyclass(name = "Client", subclass, frozen)]
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
        Runtime::future_into_py(py, execute_request(self.clone().0, method, url, kwds))
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
        Runtime::future_into_py(py, execute_websocket_request(self.clone().0, url, kwds))
    }
}

#[pymethods]
impl Client {
    /// Creates a new Client instance.
    #[new]
    #[pyo3(signature = (**kwds))]
    pub fn new(py: Python, mut kwds: Option<Builder>) -> PyResult<Client> {
        py.detach(|| {
            let params = kwds.get_or_insert_default();
            let mut builder = wreq::Client::builder();

            // Emulation options.
            apply_option!(set_if_some_inner, builder, params.emulation, emulation);

            // User agent options.
            apply_option!(
                set_if_some_map_ref,
                builder,
                params.user_agent,
                user_agent,
                AsRef::<str>::as_ref
            );

            // Default headers options.
            apply_option!(set_if_some_inner, builder, params.headers, default_headers);
            apply_option!(
                set_if_some_inner,
                builder,
                params.orig_headers,
                orig_headers
            );

            // Allow redirects options.
            apply_option!(set_if_some, builder, params.referer, referer);
            apply_option!(set_if_some, builder, params.history, history);
            apply_option!(
                set_if_true_with,
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
                apply_option!(set_if_some, builder, params.cookie_store, cookie_store);
            }

            // TCP options.
            apply_option!(
                set_if_some_map,
                builder,
                params.tcp_keepalive,
                tcp_keepalive,
                Duration::from_secs
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.tcp_keepalive_interval,
                tcp_keepalive_interval,
                Duration::from_secs
            );
            apply_option!(
                set_if_some,
                builder,
                params.tcp_keepalive_retries,
                tcp_keepalive_retries
            );
            #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
            apply_option!(
                set_if_some_map,
                builder,
                params.tcp_user_timeout,
                tcp_user_timeout,
                Duration::from_secs
            );
            apply_option!(set_if_some, builder, params.tcp_nodelay, tcp_nodelay);
            apply_option!(
                set_if_some,
                builder,
                params.tcp_reuse_address,
                tcp_reuse_address
            );

            // Timeout options.
            apply_option!(
                set_if_some_map,
                builder,
                params.timeout,
                timeout,
                Duration::from_secs
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.connect_timeout,
                connect_timeout,
                Duration::from_secs
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.read_timeout,
                read_timeout,
                Duration::from_secs
            );

            // Pool options.
            apply_option!(
                set_if_some_map,
                builder,
                params.pool_idle_timeout,
                pool_idle_timeout,
                Duration::from_secs
            );
            apply_option!(
                set_if_some,
                builder,
                params.pool_max_idle_per_host,
                pool_max_idle_per_host
            );
            apply_option!(set_if_some, builder, params.pool_max_size, pool_max_size);

            // Protocol options.
            apply_option!(set_if_true, builder, params.http1_only, http1_only, false);
            apply_option!(set_if_true, builder, params.http2_only, http2_only, false);
            apply_option!(set_if_some, builder, params.https_only, https_only);
            apply_option!(
                set_if_some_inner,
                builder,
                params.http1_options,
                http1_options
            );
            apply_option!(
                set_if_some_inner,
                builder,
                params.http2_options,
                http2_options
            );

            // TLS options.
            apply_option!(
                set_if_some_map,
                builder,
                params.min_tls_version,
                min_tls_version,
                TlsVersion::into_ffi
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.max_tls_version,
                max_tls_version,
                TlsVersion::into_ffi
            );
            apply_option!(set_if_some, builder, params.tls_info, tls_info);

            // TLS Verification options.
            if let Some(verify) = params.verify.take() {
                builder = match verify {
                    TlsVerify::Verification(verify) => builder.cert_verification(verify),
                    TlsVerify::CertificatePath(path_buf) => {
                        let pem_data = std::fs::read(path_buf)?;
                        let store = CertStore::from_pem_stack(pem_data).map_err(Error::Library)?;
                        builder.cert_store(store)
                    }
                    TlsVerify::CertificateStore(cert_store) => builder.cert_store(cert_store.0),
                }
            }
            apply_option!(
                set_if_some,
                builder,
                params.verify_hostname,
                verify_hostname
            );
            apply_option!(set_if_some_inner, builder, params.identity, identity);
            apply_option!(set_if_some_inner, builder, params.keylog, keylog);
            apply_option!(set_if_some_inner, builder, params.tls_options, tls_options);

            // Network options.
            if let Some(proxies) = params.proxies.take() {
                for proxy in proxies.0 {
                    builder = builder.proxy(proxy);
                }
            }
            apply_option!(set_if_true, builder, params.no_proxy, no_proxy, false);
            apply_option!(
                set_if_some_inner,
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
            apply_option!(set_if_some, builder, params.interface, interface);

            // Compression options.
            apply_option!(set_if_some, builder, params.gzip, gzip);
            apply_option!(set_if_some, builder, params.brotli, brotli);
            apply_option!(set_if_some, builder, params.deflate, deflate);
            apply_option!(set_if_some, builder, params.zstd, zstd);

            builder
                .dns_resolver(HickoryDnsResolver::new())
                .build()
                .map(Client)
                .map_err(Error::Library)
                .map_err(Into::into)
        })
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
        py.detach(|| {
            Runtime::block_on(execute_request(self.0.clone().0, method, url, kwds)).map(Into::into)
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
            Runtime::block_on(execute_websocket_request(self.0.clone().0, url, kwds))
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

pub async fn execute_request<C, U>(
    client: C,
    method: Method,
    url: U,
    mut params: Option<Request>,
) -> PyResult<Response>
where
    C: Into<Option<wreq::Client>>,
    U: AsRef<str>,
{
    let params = params.get_or_insert_default();
    let mut builder = match client.into() {
        Some(client) => client.request(method.into_ffi(), url.as_ref()),
        None => wreq::request(method.into_ffi(), url.as_ref()),
    };

    // Emulation options.
    apply_option!(set_if_some_inner, builder, params.emulation, emulation);

    // Version options.
    apply_option!(set_if_some_inner, builder, params.version, version);

    // Timeout options.
    apply_option!(
        set_if_some_map,
        builder,
        params.timeout,
        timeout,
        Duration::from_secs
    );
    apply_option!(
        set_if_some_map,
        builder,
        params.read_timeout,
        read_timeout,
        Duration::from_secs
    );

    // Network options.
    apply_option!(set_if_some_inner, builder, params.proxy, proxy);
    apply_option!(
        set_if_some_inner,
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
    apply_option!(set_if_some, builder, params.interface, interface);

    // Headers options.
    apply_option!(set_if_some_inner, builder, params.headers, headers);
    apply_option!(
        set_if_some_inner,
        builder,
        params.orig_headers,
        orig_headers
    );
    apply_option!(
        set_if_some,
        builder,
        params.default_headers,
        default_headers
    );

    // Authentication options.
    apply_option!(
        set_if_some_map_ref,
        builder,
        params.auth,
        auth,
        AsRef::<str>::as_ref
    );
    apply_option!(set_if_some, builder, params.bearer_auth, bearer_auth);
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
    apply_option!(set_if_some, builder, params.gzip, gzip);
    apply_option!(set_if_some, builder, params.brotli, brotli);
    apply_option!(set_if_some, builder, params.deflate, deflate);
    apply_option!(set_if_some, builder, params.zstd, zstd);

    // Query options.
    apply_option!(set_if_some_ref, builder, params.query, query);

    // Form options.
    apply_option!(set_if_some_ref, builder, params.form, form);

    // JSON options.
    apply_option!(set_if_some_ref, builder, params.json, json);

    // Body options.
    apply_option!(set_if_some, builder, params.body, body);

    // Multipart options.
    apply_option!(set_if_some_inner, builder, params.multipart, multipart);

    // Send request.
    builder
        .send()
        .await
        .map(Response::new)
        .map_err(Error::Library)
        .map_err(Into::into)
}

pub async fn execute_websocket_request<C, U>(
    client: C,
    url: U,
    mut params: Option<WebSocketRequest>,
) -> PyResult<WebSocket>
where
    C: Into<Option<wreq::Client>>,
    U: AsRef<str>,
{
    let params = params.get_or_insert_default();
    let mut builder = match client.into() {
        Some(client) => client.websocket(url.as_ref()),
        None => wreq::websocket(url.as_ref()),
    };

    // The protocols to use for the request.
    apply_option!(set_if_some, builder, params.protocols, protocols);

    // The WebSocket config
    apply_option!(
        set_if_some,
        builder,
        params.read_buffer_size,
        read_buffer_size
    );
    apply_option!(
        set_if_some,
        builder,
        params.write_buffer_size,
        write_buffer_size
    );
    apply_option!(
        set_if_some,
        builder,
        params.max_write_buffer_size,
        max_write_buffer_size
    );
    apply_option!(set_if_some, builder, params.max_frame_size, max_frame_size);
    apply_option!(
        set_if_some,
        builder,
        params.max_message_size,
        max_message_size
    );
    apply_option!(
        set_if_some,
        builder,
        params.accept_unmasked_frames,
        accept_unmasked_frames
    );

    // Use http2 options.
    apply_option!(set_if_true, builder, params.force_http2, force_http2, false);

    // Network options.
    apply_option!(set_if_some_inner, builder, params.proxy, proxy);
    apply_option!(
        set_if_some_inner,
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
    apply_option!(set_if_some, builder, params.interface, interface);

    // Headers options.
    apply_option!(set_if_some_inner, builder, params.headers, headers);
    apply_option!(
        set_if_some_inner,
        builder,
        params.orig_headers,
        orig_headers
    );
    apply_option!(
        set_if_some,
        builder,
        params.default_headers,
        default_headers
    );

    // Authentication options.
    apply_option!(
        set_if_some_map_ref,
        builder,
        params.auth,
        auth,
        AsRef::<str>::as_ref
    );
    apply_option!(set_if_some, builder, params.bearer_auth, bearer_auth);
    if let Some(basic_auth) = params.basic_auth.take() {
        builder = builder.basic_auth(basic_auth.0, basic_auth.1);
    }

    // Cookies options.
    if let Some(cookies) = params.cookies.take() {
        for cookie in cookies.0 {
            builder = builder.header_append(header::COOKIE, cookie);
        }
    }

    // Query options.
    apply_option!(set_if_some_ref, builder, params.query, query);

    // Send the WebSocket request.
    let response = builder.send().await.map_err(Error::Library)?;
    WebSocket::new(response)
        .await
        .map_err(Error::Library)
        .map_err(Into::into)
}
