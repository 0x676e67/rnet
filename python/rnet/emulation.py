# rnet/emulation.py

"""
HTTP and TLS emulation options for advanced network fingerprinting.

This module provides classes for fine-grained control over HTTP/1, HTTP/2,
and TLS protocol behavior to emulate specific browser fingerprints.
"""

from typing import List, Dict, TypedDict, Unpack, NotRequired, ClassVar, Self, Union
from enum import Enum, auto
import datetime

from .header import HeaderMap, OrigHeaderMap
from .tls import TlsVersion, AlpnProtocol, AlpsProtocol, CertificateCompressionAlgorithm, ExtensionType


__all__ = [
    "StreamId",
    "StreamDependency",
    "Priority",
    "PseudoId",
    "SettingId",
    "Http1Options",
    "Http2Options",
    "TlsOptions",
    "Emulation",
]


class StreamId:
    """
    HTTP/2 stream identifier.

    Stream IDs are used to identify individual streams within an HTTP/2 connection.
    Client-initiated streams use odd numbers, server-initiated streams use even numbers.
    """

    ZERO: ClassVar[Self]
    """Stream ID 0, reserved for connection-level frames."""

    MAX: ClassVar[Self]
    """Maximum allowed stream ID value."""

    def __init__(self, src: int) -> None:
        """
        Create a new stream ID.

        Args:
            src: The stream ID value (must be a valid 31-bit unsigned integer).
        """
        ...


class StreamDependency:
    """
    HTTP/2 stream dependency specification.

    Defines dependency relationships between streams for prioritization.
    """

    def __init__(
        self,
        dependency_id: Union[StreamId, int],
        weight: int,
        is_exclusive: bool
    ) -> None:
        """
        Create a new stream dependency.

        Args:
            dependency_id: The stream this stream depends on.
            weight: Priority weight (1-256, where 256 is highest priority).
            is_exclusive: Whether this dependency is exclusive.
        """
        ...


class Priority:
    """
    HTTP/2 stream priority specification.

    Combines stream ID and dependency information for priority frames.
    """

    def __init__(self, stream_id: Union[StreamId, int], dependency: StreamDependency) -> None:
        """
        Create a new priority specification.

        Args:
            stream_id: The stream ID this priority applies to.
            dependency: The dependency specification.
        """
        ...


class PseudoId(Enum):
    """
    HTTP/2 pseudo-header field identifiers.

    These are special headers that start with ':' in HTTP/2.
    """

    Method = auto()      # :method
    Scheme = auto()      # :scheme
    Authority = auto()   # :authority
    Path = auto()        # :path
    Protocol = auto()    # :protocol
    Status = auto()      # :status


class SettingId:
    """
    HTTP/2 settings frame parameter identifiers.

    These control various aspects of HTTP/2 connection behavior.
    """

    HeaderTableSize: ClassVar[Self]         # SETTINGS_HEADER_TABLE_SIZE
    EnablePush: ClassVar[Self]              # SETTINGS_ENABLE_PUSH
    MaxConcurrentStreams: ClassVar[Self]    # SETTINGS_MAX_CONCURRENT_STREAMS
    InitialWindowSize: ClassVar[Self]       # SETTINGS_INITIAL_WINDOW_SIZE
    MaxFrameSize: ClassVar[Self]            # SETTINGS_MAX_FRAME_SIZE
    MaxHeaderListSize: ClassVar[Self]       # SETTINGS_MAX_HEADER_LIST_SIZE
    EnableConnectProtocol: ClassVar[Self]   # SETTINGS_ENABLE_CONNECT_PROTOCOL
    NoRfc7540Priorities: ClassVar[Self]     # SETTINGS_NO_RFC7540_PRIORITIES

    def __init__(self, value: int) -> None:
        """
        Create a new setting ID.

        Args:
            value: The setting ID value.
        """
        ...


class Http1OptionsParams(TypedDict):
    """
    Parameters for HTTP/1 options configuration.

    All parameters are optional and can be used to customize HTTP/1 behavior.
    """

    http09_responses: NotRequired[bool]
    """Enable support for HTTP/0.9 responses."""

    writev: NotRequired[bool]
    """Whether to use vectored writes for HTTP/1 connections."""

    max_headers: NotRequired[int]
    """Maximum number of headers allowed in HTTP/1 responses."""

    read_buf_exact_size: NotRequired[int]
    """Exact size of the read buffer to use."""

    max_buf_size: NotRequired[int]
    """Maximum buffer size for HTTP/1 connections."""

    allow_spaces_after_header_name_in_responses: NotRequired[bool]
    """Allow spaces after header names."""

    ignore_invalid_headers_in_responses: NotRequired[bool]
    """Ignore invalid headers in responses."""

    allow_obsolete_multiline_headers_in_responses: NotRequired[bool]
    """Allow obsolete multiline headers."""


class Http1Options:
    """
    HTTP/1 protocol configuration options.

    Controls various aspects of HTTP/1.0 and HTTP/1.1 behavior for fingerprinting.
    """

    def __init__(self, **kwargs: Unpack[Http1OptionsParams]) -> None:
        """
        Create HTTP/1 options configuration.

        Args:
            **kwargs: HTTP/1 configuration parameters. See Http1OptionsParams for details.
        """
        ...


class Http2OptionsParams(TypedDict):
    """
    Parameters for HTTP/2 options configuration.

    All parameters are optional and can be used to customize HTTP/2 behavior.
    """

    initial_window_size: NotRequired[int]
    """Initial window size for HTTP/2 streams."""

    initial_connection_window_size: NotRequired[int]
    """Initial connection-level window size."""

    initial_max_send_streams: NotRequired[int]
    """Initial maximum number of send streams."""

    initial_stream_id: NotRequired[int]
    """Initial stream ID for the connection."""

    adaptive_window: NotRequired[bool]
    """Whether to use adaptive flow control."""

    max_frame_size: NotRequired[int]
    """Maximum frame size to use for HTTP/2."""

    max_header_list_size: NotRequired[int]
    """Maximum size of the header list."""

    header_table_size: NotRequired[int]
    """Header table size for HPACK compression."""

    max_concurrent_streams: NotRequired[int]
    """Maximum concurrent streams from remote peer."""

    keep_alive_interval: NotRequired[datetime.timedelta]
    """Interval for HTTP/2 keep-alive ping frames."""

    keep_alive_timeout: NotRequired[datetime.timedelta]
    """Timeout for keep-alive ping acknowledgements."""

    keep_alive_while_idle: NotRequired[bool]
    """Whether keep-alive applies while idle."""

    enable_push: NotRequired[bool]
    """Whether to enable push promises."""

    enable_connect_protocol: NotRequired[bool]
    """Whether to enable the CONNECT protocol."""

    no_rfc7540_priorities: NotRequired[bool]
    """Whether to disable RFC 7540 Stream Priorities."""

    max_concurrent_reset_streams: NotRequired[int]
    """Max concurrent locally reset streams."""

    max_send_buf_size: NotRequired[int]
    """Maximum send buffer size for streams."""

    max_pending_accept_reset_streams: NotRequired[int]
    """Max pending accept reset streams."""

    headers_stream_dependency: NotRequired[StreamDependency]
    """Stream dependency for outgoing HEADERS."""

    headers_pseudo_order: NotRequired[List[PseudoId]]
    """Order of pseudo-header fields in HEADERS."""

    experimental_settings: NotRequired[Dict[Union[SettingId, int], int]]
    """Custom experimental HTTP/2 settings."""

    settings_order: NotRequired[List[Union[SettingId, int]]]
    """Order of settings parameters in SETTINGS frame."""

    priorities: NotRequired[List[Priority]]
    """List of PRIORITY frames to send after connection."""


class Http2Options:
    """
    HTTP/2 protocol configuration options.

    Provides detailed control over HTTP/2 connection and stream behavior.
    """

    def __init__(self, **kwargs: Unpack[Http2OptionsParams]) -> None:
        """
        Create HTTP/2 options configuration.

        Args:
            **kwargs: HTTP/2 configuration parameters. See Http2OptionsParams for details.
        """
        ...


class TlsOptionsParams(TypedDict):
    """
    Parameters for TLS options configuration.

    All parameters are optional and can be used to customize TLS behavior.
    """

    alpn_protocols: NotRequired[List[AlpnProtocol]]
    """Application-Layer Protocol Negotiation protocols."""

    alps_protocols: NotRequired[List[AlpsProtocol]]
    """Application-Layer Protocol Settings protocols."""

    alps_use_new_codepoint: NotRequired[bool]
    """Use alternative ALPS codepoint."""

    session_ticket: NotRequired[bool]
    """Enable TLS Session Tickets (RFC 5077)."""

    min_tls_version: NotRequired[TlsVersion]
    """Minimum TLS version allowed."""

    max_tls_version: NotRequired[TlsVersion]
    """Maximum TLS version allowed."""

    pre_shared_key: NotRequired[bool]
    """Enable Pre-Shared Key cipher suites."""

    enable_ech_grease: NotRequired[bool]
    """Send GREASE ECH extension when no ECH config."""

    permute_extensions: NotRequired[bool]
    """Whether ClientHello extensions should be permuted."""

    grease_enabled: NotRequired[bool]
    """Whether GREASE extensions are enabled."""

    enable_ocsp_stapling: NotRequired[bool]
    """Enable OCSP stapling."""

    enable_signed_cert_timestamps: NotRequired[bool]
    """Enable Signed Certificate Timestamps."""

    record_size_limit: NotRequired[int]
    """Maximum TLS record size."""

    psk_skip_session_ticket: NotRequired[bool]
    """Skip session tickets when using PSK."""

    key_shares_limit: NotRequired[int]
    """Maximum key shares in ClientHello."""

    psk_dhe_ke: NotRequired[bool]
    """Enable PSK with (EC)DHE key establishment."""

    renegotiation: NotRequired[bool]
    """Enable TLS renegotiation."""

    delegated_credentials: NotRequired[str]
    """Delegated Credentials configuration."""

    curves_list: NotRequired[str]
    """List of supported elliptic curves."""

    cipher_list: NotRequired[str]
    """Cipher suite configuration string."""

    sigalgs_list: NotRequired[str]
    """List of supported signature algorithms."""

    certificate_compression_algorithms: NotRequired[List[CertificateCompressionAlgorithm]]
    """Certificate compression algorithms."""

    extension_permutation: NotRequired[List[ExtensionType]]
    """TLS extensions for ordering/permutation."""

    aes_hw_override: NotRequired[bool]
    """Override AES hardware acceleration."""

    prefer_chacha20: NotRequired[bool]
    """Prefer ChaCha20 over AES in TLS 1.3."""

    random_aes_hw_override: NotRequired[bool]
    """Override random AES hardware acceleration."""


class TlsOptions:
    """
    TLS protocol configuration options.

    Controls TLS handshake behavior, cipher selection, and extensions for fingerprinting.
    """

    def __init__(self, **kwargs: Unpack[TlsOptionsParams]) -> None:
        """
        Create TLS options configuration.

        Args:
            **kwargs: TLS configuration parameters. See TlsOptionsParams for details.
        """
        ...


class EmulationParams(TypedDict):
    """
    Parameters for emulation configuration.

    All parameters are optional and can be used to customize emulation behavior.
    """

    http1_options: NotRequired[Http1Options]
    """HTTP/1 protocol configuration."""

    http2_options: NotRequired[Http2Options]
    """HTTP/2 protocol configuration."""

    tls_options: NotRequired[TlsOptions]
    """TLS protocol configuration."""

    headers: NotRequired[Union[Dict[str, str], HeaderMap]]
    """Default headers to include."""

    orig_headers: NotRequired[Union[List[str], OrigHeaderMap]]
    """Original headers (case-sensitive and ordered)."""


class Emulation:
    """
    Advanced network emulation configuration.

    Combines HTTP/1, HTTP/2, and TLS options to create comprehensive
    browser fingerprints for sophisticated network behavior emulation.
    """

    def __init__(self, **kwargs: Unpack[EmulationParams]) -> None:
        """
        Create a new emulation configuration.

        Args:
            **kwargs: Emulation configuration parameters. See EmulationParams for details.

        Examples:
            Advanced TLS/HTTP2 configuration:

            ```python
            import rnet
            from rnet.emulation import TlsOptions, Http2Options, Emulation
            from rnet.tls import TlsVersion, AlpnProtocol

            # Configure TLS options
            tls_opts = TlsOptions(
                min_tls_version=TlsVersion.TLS_1_2,
                max_tls_version=TlsVersion.TLS_1_3,
                cipher_list=":".join([
                    "TLS_AES_128_GCM_SHA256",
                    "TLS_AES_256_GCM_SHA384",
                    "TLS_CHACHA20_POLY1305_SHA256",
                    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
                    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
                    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
                    "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
                    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
                    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
                ]),
                alpn_protocols=[AlpnProtocol.HTTP2, AlpnProtocol.HTTP1],
                session_ticket=True,
                enable_ocsp_stapling=True,
                grease_enabled=False
            )

            # Configure HTTP/2 options
            http2_opts = Http2Options(
                initial_window_size=6291456,
                initial_connection_window_size=15728640,
                max_frame_size=16384,
                header_table_size=65536,
                max_concurrent_streams=1000,
                enable_push=False,
                adaptive_window=True
            )

            # Combine both in emulation
            emulation = Emulation(
                tls_options=tls_opts,
                http2_options=http2_opts
            )

            # Use with client
            client = rnet.Client(emulation=emulation)
            response = await client.get("https://example.com")
            print(f"Status: {response.status}")
            print(f"HTTP Version: {response.version}")
            ```
        """
        ...
