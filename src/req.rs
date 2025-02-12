use crate::{impersonate::Impersonate, version::Version};
use indexmap::IndexMap;
use pyo3::{prelude::*, types::PyDict};

#[derive(FromPyObject, Default)]
pub struct RequestParams {
    /// The impersonation settings for the request.
    #[pyo3(item)]
    pub impersonate: Option<Impersonate>,

    /// The HTTP version to use for the request.
    #[pyo3(item)]
    pub version: Option<Version>,

    /// The user agent to use for the request.
    #[pyo3(item)]
    pub user_agent: Option<String>,

    /// The headers to use for the request.
    #[pyo3(item)]
    pub headers: Option<IndexMap<String, String>>,

    /// The timeout to use for the request.
    #[pyo3(item)]
    pub timeout: Option<u64>,

    /// The connect timeout to use for the request.
    #[pyo3(item)]
    pub connect_timeout: Option<u64>,

    /// The read timeout to use for the request.
    #[pyo3(item)]
    pub read_timeout: Option<u64>,

    /// Disable keep-alive for the client.
    #[pyo3(item)]
    pub no_keepalive: Option<bool>,

    /// Whether to disable the proxy for the request.
    #[pyo3(item)]
    pub no_proxy: Option<bool>,

    /// Whether to use the HTTP/1 protocol only.
    #[pyo3(item)]
    pub http1_only: Option<bool>,

    /// Whether to use the HTTP/2 protocol only.
    #[pyo3(item)]
    pub http2_only: Option<bool>,

    /// Whether to use referer.
    #[pyo3(item)]
    pub referer: Option<bool>,
}

impl From<Option<&Bound<'_, PyDict>>> for RequestParams {
    fn from(kwds: Option<&Bound<'_, PyDict>>) -> Self {
        kwds.map(|kwds| kwds.extract::<RequestParams>())
            .into_iter()
            .flatten()
            .next()
            .unwrap_or_default()
    }
}
