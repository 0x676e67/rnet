"""
HTTP Cookie Management

This module provides classes for creating, managing, and storing HTTP cookies
in a thread-safe manner. It includes support for all standard cookie attributes
and provides a cookie jar for automatic cookie handling during HTTP requests.
"""

import datetime
from enum import Enum, auto
from typing import List, Optional


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
    path: Optional[str]
    r"""
    Returns the path directive of the cookie, if set.
    """
    domain: Optional[str]
    r"""
    Returns the domain directive of the cookie, if set.
    """
    max_age: Optional[datetime.timedelta]
    r"""
    Get the Max-Age information.
    """
    expires: Optional[datetime.datetime]
    r"""
    The cookie expiration time.
    """

    def __init__(
        cls,
        name: str,
        value: str,
        domain: str | None = None,
        path: str | None = None,
        max_age: datetime.timedelta | None = None,
        expires: datetime.datetime | None = None,
        http_only: bool = False,
        secure: bool = False,
        same_site: SameSite | None = None,
    ) -> "Cookie":
        r"""
        Create a new cookie.
        """

    def __str__(self) -> str: ...


class Jar:
    r"""
    A thread-safe cookie jar for storing and managing HTTP cookies.

    This cookie jar can be safely shared across multiple threads and is used
    to automatically handle cookies during HTTP requests and responses.
    """

    def __init__(cls) -> "Jar":
        r"""
        Create a new cookie jar.
        """

    def get(self, name: str, url: str) -> Optional[Cookie]:
        r"""
        Get a cookie by name and URL.
        """

    def get_all(self) -> List[Cookie]:
        r"""
        Get all cookies.
        """

    def add(self, cookie: Cookie, url: str) -> None:
        r"""
        Add a cookie to this jar.
        """

    def add_cookie_str(self, cookie: str, url: str) -> None:
        r"""
        Add a cookie str to this jar.
        """

    def remove(self, name: str, url: str) -> None:
        r"""
        Remove a cookie from this jar by name and URL.
        """

    def clear(self) -> None:
        r"""
        Clear all cookies in this jar.
        """
