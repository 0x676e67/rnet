"""
TLS Utilities and Types

This module provides types and utilities for configuring TLS (Transport Layer Security) in HTTP clients.

These types are typically used to configure client-side TLS authentication and certificate verification in HTTP requests.
"""

from enum import Enum, auto
from pathlib import Path
from typing import List, Optional


class TlsVersion(Enum):
    r"""
    The TLS version.
    """

    TLS_1_0 = auto()
    TLS_1_1 = auto()
    TLS_1_2 = auto()
    TLS_1_3 = auto()


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
    def from_pkcs8_pem(buf: bytes, key: bytes) -> "CertStore":
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
        cls,
        der_certs: List[bytes] | None = None,
        pem_certs: List[str] | None = None,
        default_paths: bool | None = None,
    ) -> "CertStore":
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


class KeyLogPolicy:
    """
    Specifies the intent for a (TLS) keylogger to be used in a client or server configuration.

    This type allows you to control how TLS session keys are logged for debugging or analysis.
    You can either use the default environment variable (SSLKEYLOGFILE) or specify a file path
    directly. This is useful for tools like Wireshark that can decrypt TLS traffic if provided
    with the correct session keys.

    Static Methods:
        environment() -> KeyLogPolicy
            Use the SSLKEYLOGFILE environment variable for key logging.
        file(path: Path) -> KeyLogPolicy
            Log keys to the specified file path.

    Methods:
        is_environment() -> bool
            Returns True if this policy uses the environment variable.
        is_file() -> bool
            Returns True if this policy logs to a specific file.
    """

    @staticmethod
    def environment() -> "KeyLogPolicy":
        """
        Use the SSLKEYLOGFILE environment variable for key logging.
        """
        ...

    @staticmethod
    def file(path: Path) -> "KeyLogPolicy":
        """
        Log keys to the specified file path.

        Args:
            path: The file path to log TLS keys to.
        """
        ...

    def is_environment(self) -> bool:
        """
        Returns True if this policy uses the environment variable.
        """
        ...

    def is_file(self) -> bool:
        """
        Returns True if this policy logs to a specific file.
        """
        ...
