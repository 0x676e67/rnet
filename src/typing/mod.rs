mod body;
mod cookie;
mod enums;
mod headers;
mod ipaddr;
mod json;
mod multipart;
mod proxy;
mod status;

pub use self::{
    body::Body,
    cookie::{FromPyCookieList, FromPyCookieMap, IntoPyCookieHeader, IntoPyCookieMapRef},
    enums::{Impersonate, ImpersonateOS, LookupIpStrategy, Method, TlsVersion, Version},
    headers::{HeaderMap, HeaderMapRef, HeaderNameOrder},
    ipaddr::{IpAddr, SocketAddr},
    json::Json,
    multipart::{Multipart, Part},
    proxy::Proxy,
    status::StatusCode,
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Debug)]
pub struct QueryOrForm(Vec<(PyBackedStr, PyBackedStr)>);

impl Serialize for QueryOrForm {
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

impl FromPyObject<'_> for QueryOrForm {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(Self)
    }
}
