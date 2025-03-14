use pyo3::FromPyObject;
use std::path::PathBuf;

#[derive(FromPyObject)]
pub enum Verify {
    DisableSslVerification(bool),
    RootCertificateFilepath(PathBuf),
}
