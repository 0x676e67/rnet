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
    cookie::CookieMap,
    enums::{Impersonate, ImpersonateOS, LookupIpStrategy, Method, TlsVersion, Version},
    headers::{HeaderMap, HeaderNameOrder},
    ipaddr::{IpAddr, SocketAddr},
    json::Json,
    multipart::{Multipart, Part},
    proxy::Proxy,
    status::StatusCode,
};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use pyo3_stub_gen::{PyStubType, TypeInfo};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::{
    hash::RandomState,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug)]
pub struct IndexMap<K, V, S = RandomState>(indexmap::IndexMap<K, V, S>);

impl<K, V, S> IndexMap<K, V, S>
where
    S: Default,
{
    pub fn new() -> IndexMap<K, V, S> {
        IndexMap(indexmap::IndexMap::with_hasher(S::default()))
    }
}

impl<K, V, S> Deref for IndexMap<K, V, S> {
    type Target = indexmap::IndexMap<K, V, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V, S> DerefMut for IndexMap<K, V, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

impl PyStubType for QueryOrForm {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.List[typing.Tuple[str, str]]", "typing".into())
    }
}

impl FromPyObject<'_> for QueryOrForm {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(Self)
    }
}
