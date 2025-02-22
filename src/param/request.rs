use crate::types::{Json, Multipart, Version};
use indexmap::IndexMap;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use std::net::IpAddr;

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
#[pyclass]
#[derive(Default, Debug)]
pub struct RequestParams {
    /// The proxy to use for the request.
    #[pyo3(get)]
    pub proxy: Option<String>,

    /// Bind to a local IP Address.
    pub local_address: Option<IpAddr>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    #[pyo3(get)]
    pub interface: Option<String>,

    /// The timeout to use for the request.
    #[pyo3(get)]
    pub timeout: Option<u64>,

    /// The read timeout to use for the request.
    #[pyo3(get)]
    pub read_timeout: Option<u64>,

    /// The HTTP version to use for the request.
    #[pyo3(get)]
    pub version: Option<Version>,

    /// The headers to use for the request.
    pub headers: Option<IndexMap<String, String>>,

    /// Whether to allow redirects.
    #[pyo3(get)]
    pub allow_redirects: Option<bool>,

    /// The authentication to use for the request.
    #[pyo3(get)]
    pub auth: Option<String>,

    /// The bearer authentication to use for the request.
    #[pyo3(get)]
    pub bearer_auth: Option<String>,

    /// The basic authentication to use for the request.
    #[pyo3(get)]
    pub basic_auth: Option<(String, Option<String>)>,

    /// The query parameters to use for the request.
    #[pyo3(get)]
    pub query: Option<Vec<(String, String)>>,

    /// The form parameters to use for the request.
    #[pyo3(get)]
    pub form: Option<Vec<(String, String)>>,

    /// The JSON body to use for the request.
    #[pyo3(get)]
    pub json: Option<Json>,

    /// The body to use for the request.
    #[pyo3(get)]
    pub body: Option<Vec<u8>>,

    /// The multipart form to use for the request.
    #[pyo3(get)]
    pub multipart: Option<Py<Multipart>>,
}

macro_rules! extract_option {
    ($ob:expr, $params:expr, $field:ident) => {
        if let Ok(value) = $ob.get_item(stringify!($field)) {
            $params.$field = value.extract()?;
        }
    };
}

impl<'py> FromPyObject<'py> for RequestParams {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<RequestParams> {
        let mut params = Self::default();
        extract_option!(ob, params, proxy);
        extract_option!(ob, params, local_address);
        extract_option!(ob, params, interface);
        extract_option!(ob, params, timeout);
        extract_option!(ob, params, read_timeout);

        extract_option!(ob, params, version);
        extract_option!(ob, params, headers);
        extract_option!(ob, params, allow_redirects);
        extract_option!(ob, params, auth);
        extract_option!(ob, params, bearer_auth);
        extract_option!(ob, params, basic_auth);
        extract_option!(ob, params, query);
        extract_option!(ob, params, form);
        extract_option!(ob, params, json);
        extract_option!(ob, params, body);
        extract_option!(ob, params, multipart);

        Ok(params)
    }
}
