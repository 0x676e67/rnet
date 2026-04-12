"""
HTTP Cookie Management

This module provides classes for creating, managing, and storing HTTP cookies
in a thread-safe manner. It includes support for all standard cookie attributes
and provides a cookie jar for automatic cookie handling during HTTP requests.
"""

import datetime
from enum import Enum, auto
from typing import Sequence, final

__all__ = ["SameSite", "Cookie", "Jar"]


@final
class SameSite(Enum):
    r"""
    The Cookie SameSite attribute.
    """

    Strict = auto()
    Lax = auto()
    Empty = auto()


class Cookie:
    r"""
    A cookie.
    """

    name: str
    r"""
    The name of the cookie.
    """
    value: str
    r"""
    The value of the cookie.
    """
    http_only: bool
    r"""
    Returns true if the 'HttpOnly' directive is enabled.
    """
    secure: bool
    r"""
    Returns true if the 'Secure' directive is enabled.
    """
    same_site_lax: bool
    r"""
    Returns true if  'SameSite' directive is 'Lax'.
    """
    same_site_strict: bool
    r"""
    Returns true if  'SameSite' directive is 'Strict'.
    """
    path: str | None
    r"""
    Returns the path directive of the cookie, if set.
    """
    domain: str | None
    r"""
    Returns the domain directive of the cookie, if set.
    """
    max_age: datetime.timedelta | None
    r"""
    Get the Max-Age information.
    """
    expires: datetime.datetime | None
    r"""
    The cookie expiration time.
    """

    def __init__(
        self,
        name: str,
        value: str,
        domain: str | None = None,
        path: str | None = None,
        max_age: datetime.timedelta | None = None,
        expires: datetime.datetime | None = None,
        http_only: bool | None = None,
        secure: bool | None = None,
        same_site: SameSite | None = None,
    ) -> None:
        r"""
        Create a new cookie.
        """
        ...

    def __str__(self) -> str: ...


class Jar:
    r"""
    A thread-safe cookie jar for storing and managing HTTP cookies.

    `Jar` can be shared across multiple threads and tasks. When passed to a
    client, it is used to automatically persist and send cookies across
    requests and responses.

    **Protocol-level behaviour**

    - **HTTP/1.1** — all cookies are folded into a single `Cookie` header,
      as required by [RFC 9112 §5.6.3].
    - **HTTP/2 and above** — each cookie is sent as an individual header
      field per [RFC 9113 §8.1.2.5].

    [RFC 9112 §5.6.3]: https://www.rfc-editor.org/rfc/rfc9112#section-5.6.3
    [RFC 9113 §8.1.2.5]: https://www.rfc-editor.org/rfc/rfc9113#section-8.1.2.5
    """

    def __init__(self) -> None:
        r"""
        Create a new empty cookie jar.
        """
        ...

    def get(self, name: str, url: str) -> Cookie | None:
        r"""
        Look up a cookie by name scoped to the given URL.

        Returns `None` if no matching cookie is found.

        Args:
            name: The cookie name to look up.
            url: The URL the cookie is scoped to (used for domain / path matching).
        """
        ...

    def get_all(self) -> Sequence[Cookie]:
        r"""
        Return all cookies currently stored in the jar.
        """
        ...

    def add(self, cookie: Cookie | str, url: str) -> None:
        r"""
        Insert a cookie into the jar, scoped to the given URL.

        Args:
            cookie: A `Cookie` object or a raw `Set-Cookie` header string.
            url: The URL the cookie originates from (used for domain / path scoping).

        Example:
            ```python
            jar.add("session=abc123; Path=/; HttpOnly", "https://example.com")
            ```
        """
        ...

    def remove(self, name: str, url: str) -> None:
        r"""
        Remove a cookie by name, scoped to the given URL.

        Args:
            name: The cookie name to remove.
            url: The URL the cookie is scoped to.
        """
        ...

    def clear(self) -> None:
        r"""
        Remove all cookies from the jar.
        """
        ...
