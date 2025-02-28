use crate::types::{
    HeaderMap, Impersonate, ImpersonateOS, IpAddr, LookupIpStrategy, Proxy, TlsVersion,
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;

/// The parameters for a request.
///
/// # Examples
///
/// ```python
/// import rnet
/// from rnet import Impersonate, Version
///
/// params = rnet.RequestParams(
///     impersonate=Impersonate.Chrome100,
///     default_headers={"Content-Type": "application/json"},
///     user_agent="Mozilla/5.0",
///     timeout=10,
///     connect_timeout=5,
///     read_timeout=15,
///     no_keepalive=True,
///     no_proxy=False,
///     http1_only=False,
///     http2_only=True,
///     referer=True
/// )
///
/// response = await rnet.get("https://www.rust-lang.org", **params)
/// body = await response.text()
/// print(body)
/// ```
#[gen_stub_pyclass]
#[pyclass(get_all, set_all)]
#[derive(Default, Debug)]
pub struct ClientParams {
    /// The impersonation settings for the request.
    pub impersonate: Option<Impersonate>,

    /// The impersonation settings for the operating system.
    pub impersonate_os: Option<ImpersonateOS>,

    /// Whether to skip impersonate HTTP/2.
    pub impersonate_skip_http2: Option<bool>,

    /// Whether to skip impersonate headers.
    pub impersonate_skip_headers: Option<bool>,

    /// The base URL to use for the request.
    pub base_url: Option<String>,

    /// The user agent to use for the request.
    pub user_agent: Option<String>,

    /// The headers to use for the request.
    pub default_headers: Option<HeaderMap>,

    /// The order of the headers to use for the request.
    pub headers_order: Option<Vec<String>>,

    /// Whether to use referer.
    pub referer: Option<bool>,

    /// Whether to allow redirects.
    pub allow_redirects: Option<bool>,

    /// The maximum number of redirects to follow.
    pub max_redirects: Option<usize>,

    /// Whether to use cookie store.
    pub cookie_store: Option<bool>,

    /// The lookup ip strategy
    pub lookup_ip_strategy: Option<LookupIpStrategy>,

    // ========= Timeout options =========
    /// The timeout to use for the request. (in seconds)
    pub timeout: Option<u64>,

    /// The connect timeout to use for the request. (in seconds)
    pub connect_timeout: Option<u64>,

    /// The read timeout to use for the request. (in seconds)
    pub read_timeout: Option<u64>,

    /// Disable keep-alive for the client.
    pub no_keepalive: Option<bool>,

    /// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration. (in seconds)
    pub tcp_keepalive: Option<u64>,

    /// Set an optional timeout for idle sockets being kept-alive. (in seconds)
    pub pool_idle_timeout: Option<u64>,

    /// Sets the maximum idle connection per host allowed in the pool.
    pub pool_max_idle_per_host: Option<usize>,

    /// Sets the maximum number of connections in the pool.
    pub pool_max_size: Option<usize>,

    // ========= Protocol options =========
    /// Whether to use the HTTP/1 protocol only.
    pub http1_only: Option<bool>,

    /// Whether to use the HTTP/2 protocol only.
    pub http2_only: Option<bool>,

    /// Whether to use HTTPS only.
    pub https_only: Option<bool>,

    /// Set whether sockets have `TCP_NODELAY` enabled.
    pub tcp_nodelay: Option<bool>,

    /// The maximum number of times to retry a request.
    pub http2_max_retry_count: Option<usize>,

    // ========= TLS options =========
    /// Whether to verify the SSL certificate.
    pub danger_accept_invalid_certs: Option<bool>,

    /// Add TLS information as `TlsInfo` extension to responses.
    pub tls_info: Option<bool>,

    /// The minimum TLS version to use for the request.
    pub min_tls_version: Option<TlsVersion>,

    /// The maximum TLS version to use for the request.
    pub max_tls_version: Option<TlsVersion>,

    // ========= Network options =========
    /// Whether to disable the proxy for the request.
    pub no_proxy: Option<bool>,

    /// The proxy to use for the request.
    pub proxies: Option<Vec<Proxy>>,

    /// Bind to a local IP Address.
    pub local_address: Option<IpAddr>,

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

/// The parameters for updating a client.
///
/// This struct allows you to update various settings for an existing client instance.
///
/// # Examples
///
/// ```python
/// import rnet
/// from rnet import Impersonate, UpdateClientParams
///
/// params = UpdateClientParams(
///     impersonate=Impersonate.Chrome100,
///     headers={"Content-Type": "application/json"},
///     proxies=[rnet.Proxy.all("http://proxy.example.com:8080")]
/// )
///
/// client = rnet.Client()
/// client.update(**params)
/// ```
///
/// This will update the client with the specified impersonation settings, headers, and proxies.
#[gen_stub_pyclass]
#[pyclass(get_all, set_all)]
#[derive(Default, Debug)]
pub struct UpdateClientParams {
    /// The impersonation settings for the request.
    pub impersonate: Option<Impersonate>,

    /// The impersonation settings for the operating system.
    pub impersonate_os: Option<ImpersonateOS>,

    /// Whether to skip impersonate HTTP/2.
    pub impersonate_skip_http2: Option<bool>,

    /// Whether to skip impersonate headers.
    pub impersonate_skip_headers: Option<bool>,

    /// The headers to use for the request.
    pub headers: Option<HeaderMap>,

    /// The order of the headers to use for the request.
    pub headers_order: Option<Vec<String>>,

    // ========= Network options =========
    /// The proxy to use for the request.
    pub proxies: Option<Vec<Proxy>>,

    /// Bind to a local IP Address.
    pub local_address: Option<IpAddr>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    pub interface: Option<String>,
}

macro_rules! extract_option {
    ($ob:expr, $params:expr, $field:ident) => {
        if let Ok(value) = $ob.get_item(stringify!($field)) {
            $params.$field = value.extract()?;
        }
    };
}

impl<'py> FromPyObject<'py> for ClientParams {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, impersonate);
        extract_option!(ob, params, impersonate_os);
        extract_option!(ob, params, impersonate_skip_http2);
        extract_option!(ob, params, impersonate_skip_headers);

        extract_option!(ob, params, base_url);
        extract_option!(ob, params, user_agent);
        extract_option!(ob, params, default_headers);
        extract_option!(ob, params, headers_order);
        extract_option!(ob, params, referer);
        extract_option!(ob, params, allow_redirects);
        extract_option!(ob, params, cookie_store);
        extract_option!(ob, params, lookup_ip_strategy);

        extract_option!(ob, params, timeout);
        extract_option!(ob, params, connect_timeout);
        extract_option!(ob, params, read_timeout);
        extract_option!(ob, params, pool_idle_timeout);
        extract_option!(ob, params, pool_max_idle_per_host);
        extract_option!(ob, params, pool_max_size);
        extract_option!(ob, params, no_keepalive);
        extract_option!(ob, params, tcp_keepalive);

        extract_option!(ob, params, no_proxy);
        extract_option!(ob, params, proxies);
        extract_option!(ob, params, local_address);
        extract_option!(ob, params, interface);

        extract_option!(ob, params, http1_only);
        extract_option!(ob, params, http2_only);
        extract_option!(ob, params, https_only);
        extract_option!(ob, params, tcp_nodelay);
        extract_option!(ob, params, danger_accept_invalid_certs);
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

impl<'py> FromPyObject<'py> for UpdateClientParams {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, impersonate);
        extract_option!(ob, params, impersonate_os);
        extract_option!(ob, params, impersonate_skip_http2);
        extract_option!(ob, params, impersonate_skip_headers);
        extract_option!(ob, params, headers);
        extract_option!(ob, params, headers_order);
        extract_option!(ob, params, proxies);
        extract_option!(ob, params, local_address);
        extract_option!(ob, params, interface);
        Ok(params)
    }
}
