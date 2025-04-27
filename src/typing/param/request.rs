use crate::{
    extract_option,
    typing::{
        BodyExtractor, CookieExtractor, HeaderMapExtractor, IpAddrExtractor, Json, ProxyExtractor,
        UrlEncodedValuesExtractor, Version, multipart::MultipartExtractor,
    },
};
use pyo3::{prelude::*, pybacked::PyBackedStr};

/// The parameters for a request.
#[derive(Default)]
pub struct RequestParams {
    /// The proxy to use for the request.
    pub proxy: Option<ProxyExtractor>,

    /// Bind to a local IP Address.
    pub local_address: Option<IpAddrExtractor>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    pub interface: Option<String>,

    /// The timeout to use for the request.
    pub timeout: Option<u64>,

    /// The read timeout to use for the request.
    pub read_timeout: Option<u64>,

    /// The HTTP version to use for the request.
    pub version: Option<Version>,

    /// The headers to use for the request.
    pub headers: Option<HeaderMapExtractor>,

    /// The cookies to use for the request.
    pub cookies: Option<CookieExtractor>,

    /// Whether to allow redirects.
    pub allow_redirects: Option<bool>,

    /// The maximum number of redirects to follow.
    pub max_redirects: Option<usize>,

    /// The authentication to use for the request.
    pub auth: Option<PyBackedStr>,

    /// The bearer authentication to use for the request.
    pub bearer_auth: Option<PyBackedStr>,

    /// The basic authentication to use for the request.
    pub basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,

    /// The query parameters to use for the request.
    pub query: Option<UrlEncodedValuesExtractor>,

    /// The form parameters to use for the request.
    pub form: Option<UrlEncodedValuesExtractor>,

    /// The JSON body to use for the request.
    pub json: Option<Json>,

    /// The body to use for the request.
    pub body: Option<BodyExtractor>,

    /// The multipart form to use for the request.
    pub multipart: Option<MultipartExtractor>,
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
        extract_option!(ob, params, cookies);
        extract_option!(ob, params, allow_redirects);
        extract_option!(ob, params, max_redirects);
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
