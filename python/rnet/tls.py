"""
TLS Utilities and Types

This module provides types and utilities for configuring TLS (Transport Layer Security) in HTTP clients.

These types are typically used to configure client-side TLS authentication and certificate verification in HTTP requests.
"""

from enum import Enum, auto
from pathlib import Path
from typing import List

__all__ = [
    "TlsVersion",
    "Identity",
    "CertStore",
    "KeyLog",
    "AlpnProtocol",
    "AlpsProtocol",
    "CertificateCompressionAlgorithm",
    "ExtensionType"
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
    Application-Layer Protocol Negotiation (ALPN) protocol identifiers.

    ALPN allows the client and server to negotiate which application protocol
    to use over a secure connection during the TLS handshake.
    """

    HTTP1 = auto()  # HTTP/1.1
    HTTP2 = auto()  # HTTP/2
    HTTP3 = auto()  # HTTP/3


class AlpsProtocol(Enum):
    """
    Application-Layer Protocol Settings (ALPS) protocol identifiers.

    ALPS allows exchanging application-layer settings during the TLS handshake
    for protocols negotiated via ALPN.
    """

    HTTP1 = auto()  # HTTP/1.1
    HTTP2 = auto()  # HTTP/2
    HTTP3 = auto()  # HTTP/3


class CertificateCompressionAlgorithm(Enum):
    """
    Certificate compression algorithms for TLS certificate compression (RFC 8879).

    These algorithms can be used to compress X.509 certificates during the TLS handshake
    to reduce bandwidth usage.
    """

    ZLIB = auto()    # ZLIB compression
    BROTLI = auto()  # Brotli compression
    ZSTD = auto()    # Zstandard compression


class ExtensionType(Enum):
    """
    TLS extension type identifiers.

    These represent the various TLS extensions that can be included in ClientHello
    and ServerHello messages to negotiate additional features and capabilities.
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
