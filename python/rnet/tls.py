"""
TLS Utilities and Types

This module provides types and utilities for configuring TLS (Transport Layer Security) in HTTP clients.

These types are typically used to configure client-side TLS authentication and certificate verification in HTTP requests.
"""

from enum import Enum, auto
from pathlib import Path
from typing import List, NotRequired, TypedDict, Unpack

__all__ = [
    "TlsVersion",
    "Identity",
    "CertStore",
    "KeyLog",
    "AlpnProtocol",
    "AlpsProtocol",
    "CertificateCompressionAlgorithm",
    "ExtensionType",
    "TlsOptions",
    "Params",
]


class TlsVersion(Enum):
    r"""
    The TLS version.
    """

    TLS_1_0 = auto()
    TLS_1_1 = auto()
    TLS_1_2 = auto()
    TLS_1_3 = auto()


class AlpnProtocol(Enum):
    """
    A TLS ALPN protocol.
    """

    HTTP1 = auto()
    HTTP2 = auto()
    HTTP3 = auto()


class AlpsProtocol(Enum):
    """
    Application-layer protocol settings for HTTP/1.1 and HTTP/2.
    """

    HTTP1 = auto()
    HTTP2 = auto()
    HTTP3 = auto()


class CertificateCompressionAlgorithm(Enum):
    """
    IANA assigned identifier of compression algorithm.
    See https://www.rfc-editor.org/rfc/rfc8879.html#name-compression-algorithms
    """

    ZLIB = auto()
    BROTLI = auto()
    ZSTD = auto()


class ExtensionType(Enum):
    """
    A TLS extension type.
    """

    SERVER_NAME = auto()
    """Server Name Indication (SNI) extension"""

    STATUS_REQUEST = auto()
    """Certificate Status Request extension (OCSP stapling)"""

    EC_POINT_FORMATS = auto()
    """Elliptic Curve Point Formats extension"""

    SIGNATURE_ALGORITHMS = auto()
    """Signature Algorithms extension"""

    SRTP = auto()
    """Secure Real-time Transport Protocol extension"""

    APPLICATION_LAYER_PROTOCOL_NEGOTIATION = auto()
    """Application-Layer Protocol Negotiation (ALPN) extension"""

    PADDING = auto()
    """Padding extension"""

    EXTENDED_MASTER_SECRET = auto()
    """Extended Master Secret extension"""

    QUIC_TRANSPORT_PARAMETERS_LEGACY = auto()
    """QUIC Transport Parameters (legacy) extension"""

    QUIC_TRANSPORT_PARAMETERS_STANDARD = auto()
    """QUIC Transport Parameters (standard) extension"""

    CERT_COMPRESSION = auto()
    """Certificate Compression extension"""

    SESSION_TICKET = auto()
    """Session Ticket extension"""

    SUPPORTED_GROUPS = auto()
    """Supported Groups extension (formerly Supported Elliptic Curves)"""

    PRE_SHARED_KEY = auto()
    """Pre-Shared Key extension"""

    EARLY_DATA = auto()
    """Early Data extension (0-RTT)"""

    SUPPORTED_VERSIONS = auto()
    """Supported Versions extension"""

    COOKIE = auto()
    """Cookie extension"""

    PSK_KEY_EXCHANGE_MODES = auto()
    """PSK Key Exchange Modes extension"""

    CERTIFICATE_AUTHORITIES = auto()
    """Certificate Authorities extension"""

    SIGNATURE_ALGORITHMS_CERT = auto()
    """Signature Algorithms for Certificates extension"""

    KEY_SHARE = auto()
    """Key Share extension"""

    RENEGOTIATE = auto()
    """Renegotiation Indication extension"""

    DELEGATED_CREDENTIAL = auto()
    """Delegated Credentials extension"""

    APPLICATION_SETTINGS = auto()
    """Application-Layer Protocol Settings (ALPS) extension"""

    APPLICATION_SETTINGS_NEW = auto()
    """Application-Layer Protocol Settings (new codepoint) extension"""

    ENCRYPTED_CLIENT_HELLO = auto()
    """Encrypted Client Hello extension"""

    CERTIFICATE_TIMESTAMP = auto()
    """Certificate Transparency SCT extension"""

    NEXT_PROTO_NEG = auto()
    """Next Protocol Negotiation extension"""

    CHANNEL_ID = auto()
    """Channel ID extension"""

    RECORD_SIZE_LIMIT = auto()
    """Record Size Limit extension"""


class Identity:
    """
    Represents a private key and X509 cert as a client certificate.
    """

    @staticmethod
    def from_pkcs12_der(buf: bytes, pass_: str) -> "Identity":
        """
        Parses a DER-formatted PKCS #12 archive, using the specified password to decrypt the key.

        The archive should contain a leaf certificate and its private key, as well any intermediate
        certificates that allow clients to build a chain to a trusted root.
        The chain certificates should be in order from the leaf certificate towards the root.

        PKCS #12 archives typically have the file extension `.p12` or `.pfx`, and can be created
        with the OpenSSL `pkcs12` tool:

            openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem -certfile chain_certs.pem
        """
        ...

    @staticmethod
    def from_pkcs8_pem(buf: bytes, key: bytes) -> "Identity":
        """
        Parses a chain of PEM encoded X509 certificates, with the leaf certificate first.
        `key` is a PEM encoded PKCS #8 formatted private key for the leaf certificate.

        The certificate chain should contain any intermediate certificates that should be sent to
        clients to allow them to build a chain to a trusted root.

        A certificate chain here means a series of PEM encoded certificates concatenated together.
        """
        ...


class CertStore:
    """
    Represents a certificate store for verifying TLS connections.
    """

    def __init__(
        self,
        der_certs: List[bytes] | None = None,
        pem_certs: List[str] | None = None,
        default_paths: bool | None = None,
    ) -> None:
        """
        Creates a new CertStore.

        Args:
            der_certs: Optional list of DER-encoded certificates (as bytes).
            pem_certs: Optional list of PEM-encoded certificates (as str).
            default_paths: If True, use system default certificate paths.
        """
        ...

    @staticmethod
    def from_der_certs(certs: List[bytes]) -> "CertStore":
        """
        Creates a CertStore from a collection of DER-encoded certificates.

        Args:
            certs: List of DER-encoded certificates (as bytes).
        """
        ...

    @staticmethod
    def from_pem_certs(certs: List[str]) -> "CertStore":
        """
        Creates a CertStore from a collection of PEM-encoded certificates.

        Args:
            certs: List of PEM-encoded certificates (as str).
        """
        ...

    @staticmethod
    def from_pem_stack(certs: bytes) -> "CertStore":
        """
        Creates a CertStore from a PEM-encoded certificate stack.

        Args:
            certs: PEM-encoded certificate stack (as bytes).
        """
        ...


class KeyLog:
    """
    Specifies the intent for a (TLS) keylogger to be used in a client or server configuration.

    This type allows you to control how TLS session keys are logged for debugging or analysis.
    You can either use the default environment variable (SSLKEYLOGFILE) or specify a file path
    directly. This is useful for tools like Wireshark that can decrypt TLS traffic if provided
    with the correct session keys.

    Static Methods:
        environment() -> KeyLog
            Use the SSLKEYLOGFILE environment variable for key logging.
        file(path: Path) -> KeyLog
            Log keys to the specified file path.

    Methods:
        is_environment() -> bool
            Returns True if this policy uses the environment variable.
        is_file() -> bool
            Returns True if this policy logs to a specific file.
    """

    @staticmethod
    def environment() -> "KeyLog":
        """
        Use the SSLKEYLOGFILE environment variable for key logging.
        """
        ...

    @staticmethod
    def file(path: Path) -> "KeyLog":
        """
        Log keys to the specified file path.

        Args:
            path: The file path to log TLS keys to.
        """
        ...


class Params(TypedDict):
    """
    All parameters for TLS connections.
    """

    alpn_protocols: NotRequired[List[AlpnProtocol]]
    """
    Application-Layer Protocol Negotiation (RFC 7301).

    Specifies which application protocols (e.g., HTTP/2, HTTP/1.1) may be negotiated
    over a single TLS connection.
    """

    alps_protocols: NotRequired[List[AlpsProtocol]]
    """
    Application-Layer Protocol Settings (ALPS).

    Enables exchanging application-layer settings during the handshake
    for protocols negotiated via ALPN.
    """

    alps_use_new_codepoint: NotRequired[bool]
    """
    Whether to use an alternative ALPS codepoint for compatibility.

    Useful when larger ALPS payloads are required.
    """

    session_ticket: NotRequired[bool]
    """
    Enables TLS Session Tickets (RFC 5077).

    Allows session resumption without requiring server-side state.
    """

    min_tls_version: NotRequired[TlsVersion]
    """
    Minimum TLS version allowed for the connection.
    """

    max_tls_version: NotRequired[TlsVersion]
    """
    Maximum TLS version allowed for the connection.
    """

    pre_shared_key: NotRequired[bool]
    """
    Enables Pre-Shared Key (PSK) cipher suites (RFC 4279).

    Authentication relies on out-of-band pre-shared keys instead of certificates.
    """

    enable_ech_grease: NotRequired[bool]
    """
    Controls whether to send a GREASE Encrypted ClientHello (ECH) extension
    when no supported ECH configuration is available.

    GREASE prevents protocol ossification by sending unknown extensions.
    """

    permute_extensions: NotRequired[bool]
    """
    Controls whether ClientHello extensions should be permuted.
    """

    grease_enabled: NotRequired[bool]
    """
    Controls whether GREASE extensions (RFC 8701) are enabled in general.
    """

    enable_ocsp_stapling: NotRequired[bool]
    """
    Enables OCSP stapling for the connection.
    """

    enable_signed_cert_timestamps: NotRequired[bool]
    """
    Enables Signed Certificate Timestamps (SCT).
    """

    record_size_limit: NotRequired[int]
    """
    Sets the maximum TLS record size.
    """

    psk_skip_session_ticket: NotRequired[bool]
    """
    Whether to skip session tickets when using PSK.
    """

    key_shares_limit: NotRequired[int]
    """
    Maximum number of key shares to include in ClientHello.
    """

    psk_dhe_ke: NotRequired[bool]
    """
    Enables PSK with (EC)DHE key establishment (`psk_dhe_ke`).
    """

    renegotiation: NotRequired[bool]
    """
    Enables TLS renegotiation by sending the `renegotiation_info` extension.
    """

    delegated_credentials: NotRequired[str]
    """
    Delegated Credentials (RFC 9345).

    Allows TLS 1.3 endpoints to use temporary delegated credentials
    for authentication with reduced long-term key exposure.
    """

    curves_list: NotRequired[str]
    """
    List of supported elliptic curves.
    """

    cipher_list: NotRequired[str]
    """
    Cipher suite configuration string.

    Uses BoringSSL's mini-language to select, enable, and prioritize ciphers.
    """

    sigalgs_list: NotRequired[str]
    """
    List of supported signature algorithms.
    """

    certificate_compression_algorithms: NotRequired[
        List[CertificateCompressionAlgorithm]
    ]
    """
    Supported certificate compression algorithms (RFC 8879).
    """

    extension_permutation: NotRequired[List[ExtensionType]]
    """
    Supported TLS extensions, used for extension ordering/permutation.
    """

    aes_hw_override: NotRequired[bool]
    """
    Overrides AES hardware acceleration.
    """

    prefer_chacha20: NotRequired[bool]
    """
    Preference for ChaCha20 over AES in TLS 1.3.

    When set, the order of preference is:
    - AES_128_GCM
    - CHACHA20_POLY1305
    - AES_256_GCM
    """

    random_aes_hw_override: NotRequired[bool]
    """
    Overrides the random AES hardware acceleration.
    """


class TlsOptions:
    """
    TLS connection configuration options.

    This struct provides fine-grained control over the behavior of TLS
    connections, including:
     - **Protocol negotiation** (ALPN, ALPS, TLS versions)
     - **Session management** (tickets, PSK, key shares)
     - **Security & privacy** (OCSP, GREASE, ECH, delegated credentials)
     - **Performance tuning** (record size, cipher preferences, hardware overrides)

    All fields are optional or have defaults. See each field for details.
    """

    def __init__(self, **kwargs: Unpack[Params]) -> None:
        """
        Creates a new TlsOptions.
        """
        ...
