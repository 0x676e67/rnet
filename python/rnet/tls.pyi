from enum import Enum, auto

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
    def from_pkcs12_der(buf: bytes, pass_: str) -> Identity:
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
    def from_pkcs8_pem(buf: bytes, key: bytes) -> Identity:
        """
        Parses a chain of PEM encoded X509 certificates, with the leaf certificate first.
        `key` is a PEM encoded PKCS #8 formatted private key for the leaf certificate.

        The certificate chain should contain any intermediate certificates that should be sent to
        clients to allow them to build a chain to a trusted root.

        A certificate chain here means a series of PEM encoded certificates concatenated together.
        """
        ...
