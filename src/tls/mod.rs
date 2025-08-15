mod identity;
mod keylog;
mod store;

use pyo3::prelude::*;

pub use self::{identity::Identity, keylog::KeyLogPolicy, store::CertStore};

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
pub enum TlsVerify {
    Verification(bool),
    CertificatePath(std::path::PathBuf),
    CertificateStore(CertStore),
}
