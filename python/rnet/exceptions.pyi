class DNSResolverError(RuntimeError):
    r"""
    An error occurred while resolving a DNS name.
    """

class BodyError(Exception):
    r"""
    An error occurred while processing the body of a request or response.
    """

class BuilderError(Exception):
    r"""
    An error occurred while building a request or response.
    """

class ConnectionError(Exception):
    r"""
    An error occurred while establishing a connection.
    """

class ConnectionResetError(Exception):
    r"""
    The connection was reset.
    """

class DecodingError(Exception):
    r"""
    An error occurred while decoding a response.
    """

class RedirectError(Exception):
    r"""
    An error occurred while following a redirect.
    """

class TimeoutError(Exception):
    r"""
    A timeout occurred while waiting for a response.
    """

class StatusError(Exception):
    r"""
    An error occurred while processing the status code of a response.
    """

class RequestError(Exception):
    r"""
    An error occurred while making a request.
    """

class UpgradeError(Exception):
    r"""
    An error occurred while upgrading a connection.
    """

class URLParseError(Exception):
    r"""
    An error occurred while parsing a URL.
    """

class MIMEParseError(Exception):
    r"""
    An error occurred while parsing a MIME type.
    """
