use crate::{define_into_pyobject_todo, define_py_stub_gen};
use pyo3::{IntoPyObjectExt, prelude::*};
#[cfg(feature = "docs")]
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// An IP address.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpAddrExtractor(pub std::net::IpAddr);

impl FromPyObject<'_> for IpAddrExtractor {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(IpAddrExtractor)
    }
}

define_into_pyobject_todo!(IpAddrExtractor);

define_py_stub_gen!(
    IpAddrExtractor,
    "typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]",
    "ipaddress"
);

/// A IP socket address.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass(eq)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SocketAddr(std::net::SocketAddr);

impl From<std::net::SocketAddr> for SocketAddr {
    fn from(ip: std::net::SocketAddr) -> Self {
        SocketAddr(ip)
    }
}

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl SocketAddr {
    /// Returns the IP address of the socket address.
    fn ip<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.0.ip().into_bound_py_any(py)
    }

    /// Returns the port number of the socket address.
    fn port(&self) -> u16 {
        self.0.port()
    }
}

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl SocketAddr {
    #[inline(always)]
    fn __str__(&self) -> String {
        self.0.to_string()
    }
}
