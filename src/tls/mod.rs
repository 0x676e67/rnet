mod identity;
mod keylog;
mod store;

use pyo3::prelude::*;

pub use self::{identity::Identity, keylog::KeyLog, store::CertStore};

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

define_enum!(
    /// The ALPN protocol.
    const,
    AlpnProtocol,
    wreq::tls::AlpnProtocol,
    HTTP1,
    HTTP2,
    HTTP3,
);

define_enum!(
    /// The ALPS protocol.
    const,
    AlpsProtocol,
    wreq::tls::AlpsProtocol,
    HTTP1,
    HTTP2,
    HTTP3,
);

define_enum!(
    /// The certificate compression algorithm.
    const,
    CertificateCompressionAlgorithm,
    wreq::tls::CertificateCompressionAlgorithm,
    ZLIB,
    BROTLI,
    ZSTD,
);

define_enum!(
    /// The extension type.
    const,
    ExtensionType,
    wreq::tls::ExtensionType,
    SERVER_NAME,
    STATUS_REQUEST,
    EC_POINT_FORMATS,
    SIGNATURE_ALGORITHMS,
    SRTP,
    APPLICATION_LAYER_PROTOCOL_NEGOTIATION,
    PADDING,
    EXTENDED_MASTER_SECRET,
    QUIC_TRANSPORT_PARAMETERS_LEGACY,
    QUIC_TRANSPORT_PARAMETERS_STANDARD,
    CERT_COMPRESSION,
    SESSION_TICKET,
    SUPPORTED_GROUPS,
    PRE_SHARED_KEY,
    EARLY_DATA,
    SUPPORTED_VERSIONS,
    COOKIE,
    PSK_KEY_EXCHANGE_MODES,
    CERTIFICATE_AUTHORITIES,
    SIGNATURE_ALGORITHMS_CERT,
    KEY_SHARE,
    RENEGOTIATE,
    DELEGATED_CREDENTIAL,
    APPLICATION_SETTINGS,
    APPLICATION_SETTINGS_NEW,
    ENCRYPTED_CLIENT_HELLO,
    CERTIFICATE_TIMESTAMP,
    NEXT_PROTO_NEG,
    CHANNEL_ID,
    RECORD_SIZE_LIMIT,
);
