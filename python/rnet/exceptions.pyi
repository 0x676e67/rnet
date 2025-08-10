# ========================================
# Network and System-Level Errors
# ========================================

class DNSResolverError(RuntimeError):
    r"""
    An error occurred while resolving a DNS name.
    """

class TlsError(Exception):
    r"""
    An error occurred in the TLS security layer.
    """

class ConnectionError(Exception):
    r"""
    An error occurred while establishing a connection.
    """

class ConnectionResetError(Exception):
    r"""
    The connection was reset by the remote peer.
    """

# ========================================
# Request/Response Processing Errors
# ========================================

class BodyError(Exception):
    r"""
    An error occurred while processing the body of a request or response.
    """

class BuilderError(Exception):
    r"""
    An error occurred while building a request or response.
    """

class DecodingError(Exception):
    r"""
    An error occurred while decoding a response.
    """

class StatusError(Exception):
    r"""
    An error occurred while processing the status code of a response.
    """

class RequestError(Exception):
    r"""
    An error occurred while making a request.
    """

# ========================================
# HTTP Protocol and Navigation Errors
# ========================================

class RedirectError(Exception):
    r"""
    An error occurred while following a redirect.
    """

class UpgradeError(Exception):
    r"""
    An error occurred while upgrading a connection.
    """

class WebSocketError(Exception):
    r"""
    An error occurred while handling a WebSocket connection.
    """

# ========================================
# Parsing and Validation Errors
# ========================================

class URLParseError(Exception):
    r"""
    An error occurred while parsing a URL.
    """

class MIMEParseError(Exception):
    r"""
    An error occurred while parsing a MIME type.
    """

# ========================================
# Timeout Errors
# ========================================

class TimeoutError(Exception):
    r"""
    A timeout occurred while waiting for a response.
    """
