use std::path::PathBuf;

use pyo3::prelude::*;

define_enum!(
    /// The TLS version.
    const,
    TlsVersion,
    wreq::tls::TlsVersion,
    TLS_1_0,
    TLS_1_1,
    TLS_1_2,
    TLS_1_3,
);

#[derive(FromPyObject)]
pub enum SslVerify {
    DisableSslVerification(bool),
    RootCertificateFilepath(PathBuf),
}
