use crate::types::{Impersonate, Version};
use indexmap::IndexMap;
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
///     version=Version.HTTP_2,
///     user_agent="Mozilla/5.0",
///     headers={"Content-Type": "application/json"},
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
#[derive(Default, Debug)]
pub struct RequestParams {
    /// The impersonation settings for the request.
    pub impersonate: Option<Impersonate>,

    /// The HTTP version to use for the request.
    pub version: Option<Version>,

    /// The user agent to use for the request.
    pub user_agent: Option<String>,

    /// The headers to use for the request.
    pub headers: Option<IndexMap<String, String>>,

    /// The timeout to use for the request.
    pub timeout: Option<u64>,

    /// The connect timeout to use for the request.
    pub connect_timeout: Option<u64>,

    /// The read timeout to use for the request.
    pub read_timeout: Option<u64>,

    /// Disable keep-alive for the client.
    pub no_keepalive: Option<bool>,

    /// Whether to disable the proxy for the request.
    pub no_proxy: Option<bool>,

    /// Whether to use the HTTP/1 protocol only.
    pub http1_only: Option<bool>,

    /// Whether to use the HTTP/2 protocol only.
    pub http2_only: Option<bool>,

    /// Whether to use referer.
    pub referer: Option<bool>,
}

macro_rules! extract_option {
    ($ob:expr, $params:expr, $field:ident) => {
        if let Ok(value) = $ob.get_item(stringify!($field)) {
            $params.$field = value.extract()?;
        }
    };
}

impl<'py> FromPyObject<'py> for RequestParams {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, impersonate);
        extract_option!(ob, params, version);
        extract_option!(ob, params, user_agent);
        extract_option!(ob, params, headers);
        extract_option!(ob, params, timeout);
        extract_option!(ob, params, connect_timeout);
        extract_option!(ob, params, read_timeout);
        extract_option!(ob, params, no_keepalive);
        extract_option!(ob, params, no_proxy);
        extract_option!(ob, params, http1_only);
        extract_option!(ob, params, http2_only);
        extract_option!(ob, params, referer);
        Ok(params)
    }
}
