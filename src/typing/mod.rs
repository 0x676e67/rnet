mod body;
mod cookie;
mod enums;
mod headers;
mod ipaddr;
mod json;
mod macros;
mod multipart;
pub mod param;
mod proxy;
mod ssl;
mod status;

use crate::{define_into_pyobject_todo, define_py_stub_gen};

pub use self::{
    body::BodyExtractor,
    cookie::{Cookie, CookieExtractor},
    enums::{Impersonate, ImpersonateOS, LookupIpStrategy, Method, SameSite, TlsVersion, Version},
    headers::{
        HeaderMap, HeaderMapExtractor, HeaderMapItemsIter, HeaderMapKeysIter, HeaderMapValuesIter,
        HeadersOrderExtractor,
    },
    ipaddr::{IpAddrExtractor, SocketAddr},
    json::Json,
    multipart::{Multipart, Part},
    proxy::{Proxy, ProxyExtractor, ProxyListExtractor},
    ssl::SslVerify,
    status::StatusCode,
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
#[cfg(feature = "docs")]
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest_util::EmulationOption;
use serde::ser::{Serialize, SerializeSeq, Serializer};

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
                .emulation(impersonate.borrow().clone().into_ffi())
                .build();

            return Ok(Self(emulation));
        }

        let option = ob.downcast::<ImpersonateOption>()?.borrow();

        Ok(Self(
            EmulationOption::builder()
                .emulation(option.impersonate.into_ffi())
                .emulation_os(
                    option
                        .impersonate_os
                        .map(|os| os.into_ffi())
                        .unwrap_or_default(),
                )
                .skip_http2(option.skip_http2.unwrap_or(false))
                .skip_headers(option.skip_headers.unwrap_or(false))
                .build(),
        ))
    }
}

define_into_pyobject_todo!(ImpersonateExtractor);

define_py_stub_gen!(
    ImpersonateExtractor,
    "typing.Union[Impersonate, ImpersonateOption]",
    "typing"
);

/// A struct to represent the `ImpersonateOption` class.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass]
pub struct ImpersonateOption {
    /// The browser version to impersonate.
    impersonate: Impersonate,

    /// The operating system.
    impersonate_os: Option<ImpersonateOS>,

    /// Whether to skip HTTP/2.
    skip_http2: Option<bool>,

    /// Whether to skip headers.
    skip_headers: Option<bool>,
}

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl ImpersonateOption {
    /// Create a new impersonation option instance.
    ///
    /// This class allows you to configure browser/client impersonation settings
    /// including the browser type, operating system, and HTTP protocol options.
    ///
    /// Args:
    ///     impersonate (Impersonate): The browser/client type to impersonate
    ///     impersonate_os (Optional[ImpersonateOS]): The operating system to impersonate, defaults to None
    ///     skip_http2 (Optional[bool]): Whether to disable HTTP/2 support, defaults to False
    ///     skip_headers (Optional[bool]): Whether to skip default request headers, defaults to False
    ///
    /// Returns:
    ///     ImpersonateOption: A new impersonation option instance
    ///
    /// Examples:
    ///     ```python
    ///     from rnet import ImpersonateOption, Impersonate, ImpersonateOS
    ///
    ///     # Basic Chrome 120 impersonation
    ///     option = ImpersonateOption(Impersonate.Chrome120)
    ///
    ///     # Firefox 136 on Windows with custom options
    ///     option = ImpersonateOption(
    ///         impersonate=Impersonate.Firefox136,
    ///         impersonate_os=ImpersonateOS.Windows,
    ///         skip_http2=False,
    ///         skip_headers=True
    ///     )
    ///     ```
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
        Self {
            impersonate,
            impersonate_os,
            skip_http2,
            skip_headers,
        }
    }
}
