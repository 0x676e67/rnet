mod dns;
mod headers;
mod impersonate;
mod ipaddr;
mod json;
mod method;
mod multipart;
mod proxy;
mod status;
mod version;

pub use self::{
    dns::LookupIpStrategy,
    headers::HeaderMap,
    impersonate::{Impersonate, ImpersonateOS},
    ipaddr::SocketAddr,
    json::Json,
    method::Method,
    multipart::{Multipart, Part},
    proxy::Proxy,
    status::StatusCode,
    version::Version,
};
