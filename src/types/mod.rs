mod body;
mod cookie;
mod enums;
mod headers;
mod ipaddr;
mod json;
mod multipart;
mod proxy;
mod status;

#[allow(unused_imports)]
pub use self::{
    body::Body,
    cookie::CookieMap,
    enums::{Impersonate, ImpersonateOS, LookupIpStrategy, Method, TlsVersion, Version},
    headers::HeaderMap,
    ipaddr::{IpAddr, SocketAddr},
    json::Json,
    multipart::{Multipart, Part},
    proxy::Proxy,
    status::StatusCode,
};
use pyo3::{PyObject, Python};
use std::{
    hash::RandomState,
    ops::{Deref, DerefMut},
};

struct PyIterator {
    iter: PyObject,
}

impl PyIterator {
    fn new(iter: PyObject) -> Self {
        PyIterator { iter }
    }
}

impl Iterator for PyIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        Python::with_gil(|py| {
            self.iter
                .call_method0(py, "__next__")
                .ok()
                .and_then(|item| item.extract::<Vec<u8>>(py).ok())
        })
    }
}

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
