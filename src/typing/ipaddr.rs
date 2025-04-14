use pyo3::{IntoPyObjectExt, prelude::*};
#[cfg(feature = "docs")]
use pyo3_stub_gen::{
    PyStubType, TypeInfo,
    derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// An IP address.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpAddrExtractor(pub std::net::IpAddr);

impl FromPyObject<'_> for IpAddrExtractor {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(IpAddrExtractor)
    }
}

impl<'py> IntoPyObject<'py> for IpAddrExtractor {
    type Target = IpAddrExtractor;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, _: Python<'py>) -> Result<Self::Output, Self::Error> {
        todo!("IpAddrExtractor::into_pyobject is not implemented yet");
    }
}

#[cfg(feature = "docs")]
impl PyStubType for IpAddrExtractor {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module(
            "typing.Optional[typing.Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]",
            "ipaddress".into(),
        )
    }
}

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
