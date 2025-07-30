use std::path::PathBuf;

use pyo3::FromPyObject;

#[derive(FromPyObject)]
pub enum SslVerify {
    DisableSslVerification(bool),
    RootCertificateFilepath(PathBuf),
}
