from enum import Enum, auto
from typing import Optional

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

class ImpersonateOption:
    r"""
    A struct to represent the `ImpersonateOption` class.
    """

    def __new__(
        cls,
        impersonate: Impersonate,
        impersonate_os: Optional[ImpersonateOS] = None,
        skip_http2: Optional[bool] = None,
        skip_headers: Optional[bool] = None,
    ) -> ImpersonateOption:
        r"""
        Create a new impersonation option instance.

        This class allows you to configure browser/client impersonation settings
        including the browser type, operating system, and HTTP protocol options.

        Args:
            impersonate (Impersonate): The browser/client type to impersonate
            impersonate_os (Optional[ImpersonateOS]): The operating system to impersonate, defaults to None
            skip_http2 (Optional[bool]): Whether to disable HTTP/2 support, defaults to False
            skip_headers (Optional[bool]): Whether to skip default request headers, defaults to False

        Returns:
            ImpersonateOption: A new impersonation option instance

        Examples:
            ```python
            from rnet import ImpersonateOption, Impersonate, ImpersonateOS

            # Basic Chrome 120 impersonation
            option = ImpersonateOption(Impersonate.Chrome120)

            # Firefox 136 on Windows with custom options
            option = ImpersonateOption(
                impersonate=Impersonate.Firefox136,
                impersonate_os=ImpersonateOS.Windows,
                skip_http2=False,
                skip_headers=True
            )
            ```
        """

    @staticmethod
    def random() -> ImpersonateOption:
        r"""
        Creates a new random impersonation option instance.

        This method generates a random browser/client impersonation option
        with random settings for browser type and operating system options.
        """
