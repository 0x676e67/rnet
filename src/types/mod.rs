mod body;
mod enums;
mod headers;
mod ipaddr;
mod json;
mod multipart;
mod proxy;
mod status;

pub use self::{
    body::Body,
    enums::{Impersonate, ImpersonateOS, LookupIpStrategy, Method, Version},
    headers::HeaderMap,
    ipaddr::SocketAddr,
    json::Json,
    multipart::{Multipart, Part},
    proxy::Proxy,
    status::StatusCode,
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
