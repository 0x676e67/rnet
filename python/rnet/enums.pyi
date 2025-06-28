from enum import Enum, auto

class LookupIpStrategy(Enum):
    r"""
    The lookup ip strategy.
    """

    Ipv4Only = auto()
    Ipv6Only = auto()
    Ipv4AndIpv6 = auto()
    Ipv6thenIpv4 = auto()
    Ipv4thenIpv6 = auto()

class Method(Enum):
    r"""
    An HTTP method.
    """

    GET = auto()
    HEAD = auto()
    POST = auto()
    PUT = auto()
    DELETE = auto()
    OPTIONS = auto()
    TRACE = auto()
    PATCH = auto()

class TlsVersion(Enum):
    r"""
    The TLS version.
    """

    TLS_1_0 = auto()
    TLS_1_1 = auto()
    TLS_1_2 = auto()
    TLS_1_3 = auto()

class Version(Enum):
    r"""
    An HTTP version.
    """

    HTTP_09 = auto()
    HTTP_10 = auto()
    HTTP_11 = auto()
    HTTP_2 = auto()
    HTTP_3 = auto()
