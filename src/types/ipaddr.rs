use pyo3::{prelude::*, IntoPyObjectExt};
use pyo3_stub_gen::{
    derive::{gen_stub_pyclass, gen_stub_pymethods},
    PyStubType, TypeInfo,
};

/// An IP address.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

impl<'py> IntoPyObject<'py> for IpAddr {
    type Target = PyAny;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_bound_py_any(py)
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

    /// Returns the socket address as a string.
    fn __str__(&self) -> String {
        self.0.to_string()
    }
}
