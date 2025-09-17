use std::borrow::Cow;

use pyo3::prelude::*;

use crate::tls::{
    AlpnProtocol, AlpsProtocol, CertificateCompressionAlgorithm, ExtensionType, TlsVersion,
};

#[derive(Default)]
pub struct Builder {
    /// Application-Layer Protocol Negotiation ([RFC 7301](https://datatracker.ietf.org/doc/html/rfc7301)).
    ///
    /// Specifies which application protocols (e.g., HTTP/2, HTTP/1.1) may be negotiated
    /// over a single TLS connection.
    ///
    /// **Default:** `Some([HTTP/2, HTTP/1.1])`
    pub alpn_protocols: Option<Vec<AlpnProtocol>>,

    /// Application-Layer Protocol Settings (ALPS).
    ///
    /// Enables exchanging application-layer settings during the handshake
    /// for protocols negotiated via ALPN.
    ///
    /// **Default:** `None`
    pub alps_protocols: Option<Vec<AlpsProtocol>>,

    /// Whether to use an alternative ALPS codepoint for compatibility.
    ///
    /// Useful when larger ALPS payloads are required.
    ///
    /// **Default:** `false`
    pub alps_use_new_codepoint: Option<bool>,

    /// Enables TLS Session Tickets ([RFC 5077](https://tools.ietf.org/html/rfc5077)).
    ///
    /// Allows session resumption without requiring server-side state.
    ///
    /// **Default:** `true`
    pub session_ticket: Option<bool>,

    /// Minimum TLS version allowed for the connection.
    ///
    /// **Default:** `None` (library default applied)
    pub min_tls_version: Option<TlsVersion>,

    /// Maximum TLS version allowed for the connection.
    ///
    /// **Default:** `None` (library default applied)
    pub max_tls_version: Option<TlsVersion>,

    /// Enables Pre-Shared Key (PSK) cipher suites ([RFC 4279](https://datatracker.ietf.org/doc/html/rfc4279)).
    ///
    /// Authentication relies on out-of-band pre-shared keys instead of certificates.
    ///
    /// **Default:** `false`
    pub pre_shared_key: Option<bool>,

    /// Controls whether to send a GREASE Encrypted ClientHello (ECH) extension
    /// when no supported ECH configuration is available.
    ///
    /// GREASE prevents protocol ossification by sending unknown extensions.
    ///
    /// **Default:** `false`
    pub enable_ech_grease: Option<bool>,

    /// Controls whether ClientHello extensions should be permuted.
    ///
    /// **Default:** `None` (implementation default)
    pub permute_extensions: Option<bool>,

    /// Controls whether GREASE extensions ([RFC 8701](https://datatracker.ietf.org/doc/html/rfc8701))
    /// are enabled in general.
    ///
    /// **Default:** `None` (implementation default)
    pub grease_enabled: Option<bool>,

    /// Enables OCSP stapling for the connection.
    ///
    /// **Default:** `false`
    pub enable_ocsp_stapling: Option<bool>,

    /// Enables Signed Certificate Timestamps (SCT).
    ///
    /// **Default:** `false`
    pub enable_signed_cert_timestamps: Option<bool>,

    /// Sets the maximum TLS record size.
    ///
    /// **Default:** `None`
    pub record_size_limit: Option<u16>,

    /// Whether to skip session tickets when using PSK.
    ///
    /// **Default:** `false`
    pub psk_skip_session_ticket: Option<bool>,

    /// Maximum number of key shares to include in ClientHello.
    ///
    /// **Default:** `None`
    pub key_shares_limit: Option<u8>,

    /// Enables PSK with (EC)DHE key establishment (`psk_dhe_ke`).
    ///
    /// **Default:** `true`
    pub psk_dhe_ke: Option<bool>,

    /// Enables TLS renegotiation by sending the `renegotiation_info` extension.
    ///
    /// **Default:** `true`
    pub renegotiation: Option<bool>,

    /// Delegated Credentials ([RFC 9345](https://datatracker.ietf.org/doc/html/rfc9345)).
    ///
    /// Allows TLS 1.3 endpoints to use temporary delegated credentials
    /// for authentication with reduced long-term key exposure.
    ///
    /// **Default:** `None`
    pub delegated_credentials: Option<String>,

    /// List of supported elliptic curves.
    ///
    /// **Default:** `None`
    pub curves_list: Option<String>,

    /// Cipher suite configuration string.
    ///
    /// Uses BoringSSL's mini-language to select, enable, and prioritize ciphers.
    ///
    /// **Default:** `None`
    pub cipher_list: Option<String>,

    /// List of supported signature algorithms.
    ///
    /// **Default:** `None`
    pub sigalgs_list: Option<String>,

    /// Supported certificate compression algorithms ([RFC 8879](https://datatracker.ietf.org/doc/html/rfc8879)).
    ///
    /// **Default:** `None`
    pub certificate_compression_algorithms: Option<Vec<CertificateCompressionAlgorithm>>,

    /// Supported TLS extensions, used for extension ordering/permutation.
    ///
    /// **Default:** `None`
    pub extension_permutation: Option<Vec<ExtensionType>>,

    /// Overrides AES hardware acceleration.
    ///
    /// **Default:** `None`
    pub aes_hw_override: Option<bool>,

    /// Preference for ChaCha20 over AES in TLS 1.3.
    ///
    /// When set, the order of preference is:
    /// - `AES_128_GCM`
    /// - `CHACHA20_POLY1305`
    /// - `AES_256_GCM`
    ///
    /// **Default:** `None` (implementation default)
    pub prefer_chacha20: Option<bool>,

    /// Overrides the random AES hardware acceleration.
    ///
    /// **Default:** `false`
    pub random_aes_hw_override: Option<bool>,
}

impl<'py> FromPyObject<'py> for Builder {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let mut params = Self::default();
        extract_option!(ob, params, alpn_protocols);
        extract_option!(ob, params, alps_protocols);
        extract_option!(ob, params, alps_use_new_codepoint);
        extract_option!(ob, params, session_ticket);
        extract_option!(ob, params, min_tls_version);
        extract_option!(ob, params, max_tls_version);
        extract_option!(ob, params, pre_shared_key);
        extract_option!(ob, params, enable_ech_grease);
        extract_option!(ob, params, permute_extensions);
        extract_option!(ob, params, grease_enabled);
        extract_option!(ob, params, enable_ocsp_stapling);
        extract_option!(ob, params, enable_signed_cert_timestamps);
        extract_option!(ob, params, record_size_limit);
        extract_option!(ob, params, psk_skip_session_ticket);
        extract_option!(ob, params, key_shares_limit);
        extract_option!(ob, params, psk_dhe_ke);
        extract_option!(ob, params, renegotiation);
        extract_option!(ob, params, delegated_credentials);
        extract_option!(ob, params, curves_list);
        extract_option!(ob, params, cipher_list);
        extract_option!(ob, params, sigalgs_list);
        extract_option!(ob, params, certificate_compression_algorithms);
        extract_option!(ob, params, extension_permutation);
        extract_option!(ob, params, aes_hw_override);
        extract_option!(ob, params, prefer_chacha20);
        extract_option!(ob, params, random_aes_hw_override);
        Ok(params)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[pyclass(subclass, frozen)]
pub struct TlsOptions(pub wreq::tls::TlsOptions);

#[pymethods]
impl TlsOptions {
    #[new]
    #[pyo3(signature = (**kwds))]
    fn new(py: Python, mut kwds: Option<Builder>) -> Self {
        py.detach(|| {
            let params = kwds.get_or_insert_default();
            let mut builder = wreq::tls::TlsOptions::builder();

            apply_option!(
                set_if_some_map,
                builder,
                params.alpn_protocols,
                alpn_protocols,
                |v: Vec<_>| v.into_iter().map(AlpnProtocol::into_ffi)
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.alps_protocols,
                alps_protocols,
                |v: Vec<_>| v.into_iter().map(AlpsProtocol::into_ffi)
            );
            apply_option!(
                set_if_some,
                builder,
                params.alps_use_new_codepoint,
                alps_use_new_codepoint
            );
            apply_option!(set_if_some, builder, params.session_ticket, session_ticket);
            apply_option!(
                set_if_some_map,
                builder,
                params.min_tls_version,
                min_tls_version,
                TlsVersion::into_ffi
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.max_tls_version,
                max_tls_version,
                TlsVersion::into_ffi
            );
            apply_option!(set_if_some, builder, params.pre_shared_key, pre_shared_key);
            apply_option!(
                set_if_some,
                builder,
                params.enable_ech_grease,
                enable_ech_grease
            );
            apply_option!(
                set_if_some,
                builder,
                params.permute_extensions,
                permute_extensions
            );
            apply_option!(set_if_some, builder, params.grease_enabled, grease_enabled);
            apply_option!(
                set_if_some,
                builder,
                params.enable_ocsp_stapling,
                enable_ocsp_stapling
            );
            apply_option!(
                set_if_some,
                builder,
                params.enable_signed_cert_timestamps,
                enable_signed_cert_timestamps
            );
            apply_option!(
                set_if_some,
                builder,
                params.record_size_limit,
                record_size_limit
            );
            apply_option!(
                set_if_some,
                builder,
                params.psk_skip_session_ticket,
                psk_skip_session_ticket
            );
            apply_option!(
                set_if_some,
                builder,
                params.key_shares_limit,
                key_shares_limit
            );
            apply_option!(set_if_some, builder, params.psk_dhe_ke, psk_dhe_ke);
            apply_option!(set_if_some, builder, params.renegotiation, renegotiation);
            apply_option!(
                set_if_some,
                builder,
                params.delegated_credentials,
                delegated_credentials
            );
            apply_option!(set_if_some, builder, params.curves_list, curves_list);
            apply_option!(set_if_some, builder, params.cipher_list, cipher_list);
            apply_option!(set_if_some, builder, params.sigalgs_list, sigalgs_list);
            apply_option!(
                set_if_some_map,
                builder,
                params.certificate_compression_algorithms,
                certificate_compression_algorithms,
                |v: Vec<_>| Cow::Owned(
                    v.into_iter()
                        .map(CertificateCompressionAlgorithm::into_ffi)
                        .collect()
                )
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.extension_permutation,
                extension_permutation,
                |v: Vec<_>| Cow::Owned(v.into_iter().map(ExtensionType::into_ffi).collect())
            );
            apply_option!(
                set_if_some,
                builder,
                params.aes_hw_override,
                aes_hw_override
            );
            apply_option!(
                set_if_some,
                builder,
                params.prefer_chacha20,
                prefer_chacha20
            );
            apply_option!(
                set_if_some,
                builder,
                params.random_aes_hw_override,
                random_aes_hw_override
            );

            Self(builder.build())
        })
    }
}
