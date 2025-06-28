from enum import Enum, auto


class Impersonate(Enum):
    r"""
    An impersonate.
    """

    Chrome100 = auto()
    Chrome101 = auto()
    Chrome104 = auto()
    Chrome105 = auto()
    Chrome106 = auto()
    Chrome107 = auto()
    Chrome108 = auto()
    Chrome109 = auto()
    Chrome110 = auto()
    Chrome114 = auto()
    Chrome116 = auto()
    Chrome117 = auto()
    Chrome118 = auto()
    Chrome119 = auto()
    Chrome120 = auto()
    Chrome123 = auto()
    Chrome124 = auto()
    Chrome126 = auto()
    Chrome127 = auto()
    Chrome128 = auto()
    Chrome129 = auto()
    Chrome130 = auto()
    Chrome131 = auto()
    Chrome132 = auto()
    Chrome133 = auto()
    Chrome134 = auto()
    Chrome135 = auto()
    Chrome136 = auto()
    Chrome137 = auto()
    Edge101 = auto()
    Edge122 = auto()
    Edge127 = auto()
    Edge131 = auto()
    Edge134 = auto()
    Firefox109 = auto()
    Firefox117 = auto()
    Firefox128 = auto()
    Firefox133 = auto()
    Firefox135 = auto()
    FirefoxPrivate135 = auto()
    FirefoxAndroid135 = auto()
    Firefox136 = auto()
    FirefoxPrivate136 = auto()
    Firefox139 = auto()
    SafariIos17_2 = auto()
    SafariIos17_4_1 = auto()
    SafariIos16_5 = auto()
    Safari15_3 = auto()
    Safari15_5 = auto()
    Safari15_6_1 = auto()
    Safari16 = auto()
    Safari16_5 = auto()
    Safari17_0 = auto()
    Safari17_2_1 = auto()
    Safari17_4_1 = auto()
    Safari17_5 = auto()
    Safari18 = auto()
    SafariIPad18 = auto()
    Safari18_2 = auto()
    Safari18_3 = auto()
    Safari18_3_1 = auto()
    SafariIos18_1_1 = auto()
    OkHttp3_9 = auto()
    OkHttp3_11 = auto()
    OkHttp3_13 = auto()
    OkHttp3_14 = auto()
    OkHttp4_9 = auto()
    OkHttp4_10 = auto()
    OkHttp4_12 = auto()
    OkHttp5 = auto()
    Opera116 = auto()
    Opera117 = auto()
    Opera118 = auto()
    Opera119 = auto()

class ImpersonateOS(Enum):
    r"""
    An impersonate operating system.
    """

    Windows = auto()
    MacOS = auto()
    Linux = auto()
    Android = auto()
    IOS = auto()

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

class SameSite(Enum):
    r"""
    The Cookie SameSite attribute.
    """

    Strict = auto()
    Lax = auto()
    Empty = auto()

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