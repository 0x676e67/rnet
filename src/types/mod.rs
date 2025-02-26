mod body;
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
    body::Body,
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
use pyo3::{PyObject, Python};

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
