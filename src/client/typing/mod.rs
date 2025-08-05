mod body;
mod enums;
mod ipaddr;
mod json;
mod multipart;
mod status;
mod stream;

use pyo3::{prelude::*, pybacked::PyBackedStr};
use serde::ser::{Serialize, SerializeSeq, Serializer};

pub use self::{
    body::BodyExtractor,
    enums::{LookupIpStrategy, Method, SameSite, TlsVersion, Version},
    ipaddr::{IpAddrExtractor, SocketAddr},
    json::Json,
    multipart::{Multipart, MultipartExtractor, Part},
    status::StatusCode,
};

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
