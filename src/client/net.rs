use pyo3::{IntoPyObjectExt, prelude::*};

/// A IP socket address.
#[pyclass(eq)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SocketAddr(pub std::net::SocketAddr);

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

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}
