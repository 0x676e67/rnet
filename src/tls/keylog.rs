use std::path::PathBuf;

use pyo3::{pyclass, pymethods};

/// Specifies the intent for a (TLS) keylogger to be used in a client or server configuration.
///
/// This type allows you to control how TLS session keys are logged for debugging or analysis.
/// You can either use the default environment variable (`SSLKEYLOGFILE`) or specify a file path
/// directly. This is useful for tools like Wireshark that can decrypt TLS traffic if provided
/// with the correct session keys.
#[derive(Clone)]
#[pyclass]
pub struct KeyLogPolicy(pub wreq::tls::KeyLogPolicy);

#[pymethods]
impl KeyLogPolicy {
    /// Use the environment variable SSLKEYLOGFILE.
    #[staticmethod]
    pub fn environment() -> Self {
        KeyLogPolicy(wreq::tls::KeyLogPolicy::Environment)
    }

    /// Log keys to the specified file path.
    #[staticmethod]
    pub fn file(path: PathBuf) -> Self {
        KeyLogPolicy(wreq::tls::KeyLogPolicy::File(path))
    }

    /// Returns True if this is the Environment variant.
    pub fn is_environment(&self) -> bool {
        matches!(self.0, wreq::tls::KeyLogPolicy::Environment)
    }

    /// Returns True if this is the File variant.
    pub fn is_file(&self) -> bool {
        matches!(self.0, wreq::tls::KeyLogPolicy::File(_))
    }
}
