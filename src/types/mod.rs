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
    headers::HeaderMap,
    ipaddr::{IpAddr, SocketAddr},
    json::Json,
    multipart::{Multipart, Part},
    proxy::Proxy,
    status::StatusCode,
};
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
