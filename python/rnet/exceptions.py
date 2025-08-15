"""
HTTP Client Exceptions

This module defines all exceptions that can be raised by the rnet HTTP client.
The exceptions are organized into logical categories based on their cause and
severity, making it easier to handle specific types of errors appropriately.
"""

# ========================================
# Network and System-Level Errors
# ========================================


class DNSResolverError(RuntimeError):
    r"""
    An error occurred while resolving a DNS name.

    This exception is raised when the DNS resolver fails to resolve
    a hostname to an IP address. This is typically due to network
    issues, invalid hostnames, or DNS server problems.

    Since this is a system-level issue that usually cannot be resolved
    by retrying the request, it inherits from RuntimeError.
    """


class TlsError(Exception):
    r"""
    An error occurred in the TLS security layer.

    This exception covers TLS/SSL related issues such as:
    - Certificate verification failures
    - TLS handshake failures
    - Protocol version mismatches
    - Cipher suite negotiations
    """


class ConnectionError(Exception):
    r"""
    An error occurred while establishing a connection.

    This exception is raised when the client cannot establish a
    TCP connection to the remote server. Common causes include:
    - Server is unreachable
    - Port is closed or blocked
    - Network connectivity issues
    - Firewall blocking the connection
    """


class ConnectionResetError(Exception):
    r"""
    The connection was reset by the remote peer.

    This exception occurs when an established connection is
    unexpectedly closed by the remote server. This can happen
    due to server overload, network issues, or server-side
    connection limits.
    """


# ========================================
# Request/Response Processing Errors
# ========================================


class BodyError(Exception):
    r"""
    An error occurred while processing the body of a request or response.

    This exception covers issues with reading, writing, or processing
    HTTP message bodies, including:
    - Invalid content encoding
    - Incomplete body data
    - Body size limit exceeded
    """


class BuilderError(Exception):
    r"""
    An error occurred while building a request or response.

    This exception is raised when there are issues constructing
    HTTP requests or responses, such as:
    - Invalid header combinations
    - Malformed request parameters
    - Configuration conflicts
    """


class DecodingError(Exception):
    r"""
    An error occurred while decoding a response.

    This exception covers failures in decoding response content,
    including:
    - Character encoding issues (UTF-8, Latin-1, etc.)
    - Compression decompression failures (gzip, deflate, etc.)
    - Content format parsing errors
    """


class StatusError(Exception):
    r"""
    An error occurred while processing the status code of a response.

    This exception is typically raised for HTTP error status codes
    (4xx, 5xx) when automatic error handling is enabled, or when
    there are issues interpreting the status line.
    """


class RequestError(Exception):
    r"""
    An error occurred while making a request.

    This is a general exception for request-related issues that
    don't fit into more specific categories. It covers various
    problems during the request lifecycle.
    """


# ========================================
# HTTP Protocol and Navigation Errors
# ========================================


class RedirectError(Exception):
    r"""
    An error occurred while following a redirect.

    This exception is raised when there are issues with HTTP
    redirects, such as:
    - Too many redirects (redirect loop)
    - Invalid redirect location
    - Cross-protocol redirects when not allowed
    - Redirect limit exceeded
    """


class UpgradeError(Exception):
    r"""
    An error occurred while upgrading a connection.

    This exception covers failures when upgrading HTTP connections
    to other protocols, such as:
    - WebSocket upgrade failures
    - HTTP/2 upgrade issues
    - Protocol negotiation errors
    """


class WebSocketError(Exception):
    r"""
    An error occurred while handling a WebSocket connection.

    This exception covers WebSocket-specific issues including:
    - WebSocket handshake failures
    - Frame parsing errors
    - Connection state violations
    - Message sending/receiving errors
    """


# ========================================
# Parsing and Validation Errors
# ========================================


class URLParseError(Exception):
    r"""
    An error occurred while parsing a URL.

    This exception is raised when a provided URL cannot be parsed
    or is malformed. Common issues include:
    - Invalid URL schemes
    - Malformed hostnames or ports
    - Invalid characters in URL components
    - Missing required URL components
    """


class MIMEParseError(Exception):
    r"""
    An error occurred while parsing a MIME type.

    This exception covers failures in parsing or processing
    MIME types and related content type information, such as:
    - Invalid MIME type format
    - Unsupported content types
    - Malformed content-type headers
    - Character set parsing issues
    """


# ========================================
# Timeout Errors
# ========================================


class TimeoutError(Exception):
    r"""
    A timeout occurred while waiting for a response.

    This exception is raised when operations exceed their configured
    time limits, including:
    - Connection timeout (time to establish connection)
    - Read timeout (time to receive response)
    - Total request timeout (entire request lifecycle)

    Timeouts can often be resolved by increasing timeout values
    or retrying the request.
    """
