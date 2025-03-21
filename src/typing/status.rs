use pyo3::prelude::*;
#[cfg(feature = "docs")]
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// HTTP status code.
#[cfg_attr(feature = "docs", gen_stub_pyclass)]
#[pyclass(eq)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusCode(rquest::StatusCode);

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl StatusCode {
    /// Return the status code as an integer.
    #[inline]
    pub const fn as_int(&self) -> u16 {
        self.0.as_u16()
    }

    /// Check if status is within 100-199.
    #[inline]
    pub fn is_informational(&self) -> bool {
        self.0.is_informational()
    }

    /// Check if status is within 200-299.
    #[inline]
    pub fn is_success(&self) -> bool {
        self.0.is_success()
    }

    /// Check if status is within 300-399.
    #[inline]
    pub fn is_redirection(&self) -> bool {
        self.0.is_redirection()
    }

    /// Check if status is within 400-499.
    #[inline]
    pub fn is_client_error(&self) -> bool {
        self.0.is_client_error()
    }

    /// Check if status is within 500-599.
    #[inline]
    pub fn is_server_error(&self) -> bool {
        self.0.is_server_error()
    }
}

#[cfg_attr(feature = "docs", gen_stub_pymethods)]
#[pymethods]
impl StatusCode {
    #[inline(always)]
    fn __str__(&self) -> &str {
        self.0.as_str()
    }

    #[inline(always)]
    fn __repr__(&self) -> &str {
        self.__str__()
    }
}

impl From<rquest::StatusCode> for StatusCode {
    fn from(status_code: rquest::StatusCode) -> Self {
        Self(status_code)
    }
}
