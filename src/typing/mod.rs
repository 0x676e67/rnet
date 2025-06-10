mod body;
mod cookie;
mod enums;
mod header;
mod ipaddr;
mod json;
mod multipart;
pub mod param;
mod proxy;
mod ssl;
mod status;

pub use self::{
    body::BodyExtractor,
    cookie::{Cookie, CookieExtractor},
    enums::{Impersonate, ImpersonateOS, LookupIpStrategy, Method, SameSite, TlsVersion, Version},
    header::{
        HeaderMap, HeaderMapExtractor, HeaderMapItemsIter, HeaderMapKeysIter, HeaderMapValuesIter,
        HeadersOrderExtractor,
    },
    ipaddr::{IpAddrExtractor, SocketAddr},
    json::Json,
    multipart::{Multipart, Part},
    proxy::{Proxy, ProxyExtractor},
    ssl::SslVerify,
    status::StatusCode,
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use wreq_util::EmulationOption;

/// A struct to represent the `ImpersonateOption` class.
#[pyclass(subclass)]
pub struct ImpersonateOption(EmulationOption);

#[pymethods]
impl ImpersonateOption {
    /// Create a new impersonation option instance.
    #[new]
    #[pyo3(signature = (
        impersonate,
        impersonate_os = None,
        skip_http2 = None,
        skip_headers = None
    ))]
    fn new(
        impersonate: Impersonate,
        impersonate_os: Option<ImpersonateOS>,
        skip_http2: Option<bool>,
        skip_headers: Option<bool>,
    ) -> Self {
        let emulation = EmulationOption::builder()
            .emulation(impersonate.into_ffi())
            .emulation_os(impersonate_os.map(|os| os.into_ffi()).unwrap_or_default())
            .skip_http2(skip_http2.unwrap_or(false))
            .skip_headers(skip_headers.unwrap_or(false))
            .build();

        Self(emulation)
    }

    /// Creates a new random impersonation option instance.
    #[staticmethod]
    fn random() -> Self {
        Self(wreq_util::Emulation::random())
    }
}

pub struct UrlEncodedValuesExtractor(Vec<(PyBackedStr, PyBackedStr)>);

impl Serialize for UrlEncodedValuesExtractor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (key, value) in &self.0 {
            seq.serialize_element::<(&str, &str)>(&(key.as_ref(), value.as_ref()))?;
        }
        seq.end()
    }
}

impl FromPyObject<'_> for UrlEncodedValuesExtractor {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(Self)
    }
}

pub struct ImpersonateExtractor(pub EmulationOption);

impl FromPyObject<'_> for ImpersonateExtractor {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(impersonate) = ob.downcast::<Impersonate>() {
            let emulation = EmulationOption::builder()
                .emulation(impersonate.borrow().into_ffi())
                .build();

            return Ok(Self(emulation));
        }

        let option = ob.downcast::<ImpersonateOption>()?.borrow();
        Ok(Self(option.0.clone()))
    }
}
