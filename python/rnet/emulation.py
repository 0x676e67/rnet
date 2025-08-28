"""
Browser and Client Emulation

This module provides functionality for emulating various browsers and HTTP clients
to bypass detection and fingerprinting. It supports emulating Chrome, Firefox, Edge,
Safari, Opera, and OkHttp clients across different operating systems and versions.

The emulation system modifies HTTP/2 settings, TLS fingerprints, and request headers
to match the behavior of real browsers and clients, making requests appear more
authentic and less likely to be blocked by anti-bot systems.
"""

from enum import Enum, auto
from typing import Optional


class Emulation(Enum):
    r"""
    An emulation.
    """

    # Chrome versions
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

    # Microsoft Edge versions
    Edge101 = auto()
    Edge122 = auto()
    Edge127 = auto()
    Edge131 = auto()
    Edge134 = auto()

    # Firefox versions
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

    # Safari versions
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
    Safari18_5 = auto()

    # OkHttp versions
    OkHttp3_9 = auto()
    OkHttp3_11 = auto()
    OkHttp3_13 = auto()
    OkHttp3_14 = auto()
    OkHttp4_9 = auto()
    OkHttp4_10 = auto()
    OkHttp4_12 = auto()
    OkHttp5 = auto()

    # Opera versions
    Opera116 = auto()
    Opera117 = auto()
    Opera118 = auto()
    Opera119 = auto()


class EmulationOS(Enum):
    """
    Operating systems that can be emulated.

    This enum defines the operating systems that can be combined with
    browser emulations to create more specific fingerprints.
    """

    Windows = auto()  # Windows (any version)
    MacOS = auto()  # macOS (any version)
    Linux = auto()  # Linux (any distribution)
    Android = auto()  # Android (mobile)
    IOS = auto()  # iOS (iPhone/iPad)


class EmulationOption:
    """
    Configuration options for browser and client emulation.

    This class allows fine-grained control over emulation behavior,
    including the ability to disable specific features or combine
    browser types with specific operating systems.
    """

    def __init__(
        cls,
        emulation: Emulation,
        emulation_os: EmulationOS | None = None,
        skip_http2: bool | None = None,
        skip_headers: bool | None = None,
    ) -> "EmulationOption":
        """
        Create a new emulation configuration.

        Args:
            emulation: The browser/client type to emulate
            emulation_os: The operating system to emulate (optional)
            skip_http2: Whether to disable HTTP/2 emulation (default: False)
            skip_headers: Whether to skip default browser headers (default: False)

        Returns:
            A configured EmulationOption instance

        Example:
            ```python
            # Basic Chrome emulation
            option = EmulationOption(Emulation.Chrome137)

            # Chrome on Windows with HTTP/2 disabled
            option = EmulationOption(
                emulation=Emulation.Chrome137,
                emulation_os=EmulationOS.Windows,
                skip_http2=True
            )
            ```
        """

    @staticmethod
    def random() -> "EmulationOption":
        """
        Generate a random emulation configuration.

        This method creates a randomized emulation setup using a random
        browser/client type and operating system combination. Useful for
        scenarios where you want to vary your fingerprint across requests.

        Returns:
            A randomly configured EmulationOption instance

        Example:
            ```python
            # Use different random emulation for each client
            client1 = rnet.Client(emulation=EmulationOption.random())
            client2 = rnet.Client(emulation=EmulationOption.random())
            ```
        """
