use pyo3::FromPyObject;
use std::path::PathBuf;

#[derive(FromPyObject)]
pub enum SslVerify {
    DisableSslVerification(bool),
    RootCertificateFilepath(PathBuf),
}
