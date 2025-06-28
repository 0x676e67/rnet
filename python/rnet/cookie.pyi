import datetime
from enum import Enum, auto
from typing import Optional

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
    def __new__(
        cls,
        name: str,
        value: str,
        domain: Optional[str] = None,
        path: Optional[str] = None,
        max_age: Optional[datetime.timedelta] = None,
        expires: Optional[datetime.datetime] = None,
        http_only: bool = False,
        secure: bool = False,
        same_site: Optional[SameSite] = None,
    ) -> Cookie:
        r"""
        Create a new cookie.
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
