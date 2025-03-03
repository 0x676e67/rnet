use pyo3::{IntoPyObjectExt, prelude::*};
use pyo3_stub_gen::{
    PyStubType, TypeInfo,
    derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// An IP address.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpAddr(std::net::IpAddr);

impl From<IpAddr> for std::net::IpAddr {
    fn from(ip: IpAddr) -> Self {
        ip.0
    }
}

impl FromPyObject<'_> for IpAddr {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(IpAddr)
    }
}

impl PyStubType for IpAddr {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module(
            "typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]",
            "ipaddress".into(),
        )
    }
}

/// A IP socket address.
#[gen_stub_pyclass]
#[pyclass(eq)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SocketAddr(std::net::SocketAddr);

impl From<std::net::SocketAddr> for SocketAddr {
    fn from(ip: std::net::SocketAddr) -> Self {
        SocketAddr(ip)
    }
}

#[gen_stub_pymethods]
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

#[gen_stub_pymethods]
#[pymethods]
impl SocketAddr {
    fn __str__(&self) -> String {
        self.0.to_string()
    }
}
