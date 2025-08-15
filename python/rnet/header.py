"""
HTTP Header Management

This module provides efficient storage and manipulation of HTTP headers with
support for multiple values per header name. The HeaderMap class is designed
to handle the complexities of HTTP header processing, including case-insensitive
header names and multiple header values.

The implementation follows HTTP specifications (RFC 7230) for header handling,
including proper support for headers that can have multiple values (like
Set-Cookie, Accept-Encoding, etc.).
"""

from typing import Optional, Tuple


class HeaderMap:
    r"""
    A case-insensitive HTTP header map supporting multiple values per header.

    This class provides efficient storage and retrieval of HTTP headers,
    automatically handling case-insensitive header names and supporting
    headers with multiple values (such as Set-Cookie or Accept-Encoding).

    The implementation follows HTTP/1.1 specifications for header handling
    and provides both dictionary-like access and specialized methods for
    HTTP header manipulation.
    """

    def __getitem__(self, key: str) -> Optional[bytes]:
        """Get the first value for a header name (case-insensitive)."""
        ...

    def __setitem__(self, key: str, value: str) -> None:
        """Set a header to a single value, replacing any existing values."""
        ...

    def __delitem__(self, key: str) -> None:
        """Remove all values for a header name."""
        ...

    def __contains__(self, key: str) -> bool:
        """Check if a header name exists (case-insensitive)."""
        ...

    def __len__(self) -> int:
        """Return the total number of header values (not unique names)."""
        ...

    def __iter__(self) -> "HeaderMapKeysIter":
        """Iterate over unique header names."""
        ...

    def __str__(self) -> str:
        """Return a string representation of all headers."""
        ...

    def __new__(
        cls, init: Optional[dict] = None, capacity: Optional[int] = None
    ) -> "HeaderMap":
        """
        Create a new HeaderMap.

        Args:
            init: Optional dictionary to initialize headers from
            capacity: Optional initial capacity hint for performance

        Returns:
            A new HeaderMap instance

        Example:
            ```python
            # Empty header map
            headers = HeaderMap()

            # Initialize from dictionary
            headers = HeaderMap({
                'Content-Type': 'text/html',
                'Cache-Control': 'no-cache'
            })

            # Pre-allocate capacity for performance
            headers = HeaderMap(capacity=50)
            ```
        """

    def contains_key(self, key: str) -> bool:
        r"""
        Check if the header map contains the given key.

        This is equivalent to using the 'in' operator but provides
        an explicit method name. Header name comparison is case-insensitive.

        Args:
            key: The header name to check

        Returns:
            True if the header exists, False otherwise
        """

    def insert(self, key: str, value: str) -> None:
        r"""
        Insert a header, replacing any existing values.

        This method replaces all existing values for the given header name
        with the new value. For adding additional values, use append() instead.

        Args:
            key: The header name (case-insensitive)
            value: The header value to set

        Example:
            ```python
            headers.insert('Content-Type', 'application/json')
            # Replaces any existing Content-Type header
            ```
        """

    def append(self, key: str, value: str) -> None:
        r"""
        Append a value to an existing header or create a new one.

        If the header already exists, this adds an additional value.
        If the header doesn't exist, it creates a new header with this value.
        This is useful for headers that can have multiple values.

        Args:
            key: The header name (case-insensitive)
            value: The header value to append

        Example:
            ```python
            headers.append('Accept-Encoding', 'gzip')
            headers.append('Accept-Encoding', 'deflate')
            # Results in: Accept-Encoding: gzip, deflate
            ```
        """

    def remove(self, key: str) -> None:
        r"""
        Remove all values for a header name.

        This removes the header entirely from the map. If the header
        doesn't exist, this method does nothing.

        Args:
            key: The header name to remove (case-insensitive)
        """

    def get(self, key: str, default: Optional[bytes] = None) -> Optional[bytes]:
        r"""
        Get the first value for a header name with optional default.

        Returns the first value associated with the header name, or the
        default value if the header doesn't exist. For headers with multiple
        values, use get_all() to retrieve all values.

        Args:
            key: The header name (case-insensitive)
            default: Value to return if header doesn't exist

        Returns:
            The first header value as bytes, or the default value

        Example:
            ```python
            content_type = headers.get('Content-Type', b'text/plain')
            auth = headers.get('Authorization')  # Returns None if missing
            ```
        """

    def get_all(self, key: str) -> "HeaderMapValuesIter":
        r"""
        Get all values for a header name.

        Returns an iterator over all values associated with the header name.
        This is useful for headers that can have multiple values, such as
        Set-Cookie, Accept-Encoding, or custom headers.

        Args:
            key: The header name (case-insensitive)

        Returns:
            An iterator over all header values

        Example:
            ```python
            # Get all Set-Cookie headers
            cookies = list(headers.get_all('Set-Cookie'))

            # Process multiple Accept-Encoding values
            for encoding in headers.get_all('Accept-Encoding'):
                print(f"Accepts: {encoding.decode()}")
            ```
        """

    def len(self) -> int:
        """
        Get the total number of header values.

        This returns the total count of header values, which can be greater
        than the number of unique header names if some headers have multiple
        values.

        Returns:
            Total number of header values stored
        """

    def keys_len(self) -> int:
        """
        Get the number of unique header names.

        This returns the count of unique header names, regardless of how
        many values each header has.

        Returns:
            Number of unique header names
        """

    def is_empty(self) -> bool:
        """
        Check if the header map is empty.

        Returns:
            True if no headers are stored, False otherwise
        """

    def clear(self) -> None:
        """
        Remove all headers from the map.

        After calling this method, the header map will be empty and
        is_empty() will return True.
        """

    def items(self) -> "HeaderMapItemsIter":
        r"""
        Get an iterator over all header name-value pairs.

        Returns an iterator that yields tuples of (name, value) for each
        header value. Headers with multiple values will appear multiple
        times with different values.

        Returns:
            Iterator over (name, value) tuples

        Example:
            ```python
            for name, value in headers.items():
                print(f"{name.decode()}: {value.decode()}")
            ```
        """


class HeaderMapItemsIter:
    r"""
    Iterator over header name-value pairs in a HeaderMap.

    Yields tuples of (header_name, header_value) where both are bytes.
    Headers with multiple values will appear as separate tuples.
    """

    def __iter__(self) -> "HeaderMapItemsIter":
        """Return self as iterator."""
        ...

    def __next__(self) -> Optional[Tuple[bytes, Optional[bytes]]]:
        """
        Get the next header name-value pair.

        Returns:
            Tuple of (header_name, header_value) or None when exhausted
        """
        ...


class HeaderMapKeysIter:
    r"""
    Iterator over unique header names in a HeaderMap.

    Yields each unique header name as bytes, regardless of how many
    values each header has.
    """

    def __iter__(self) -> "HeaderMapKeysIter":
        """Return self as iterator."""
        ...

    def __next__(self) -> Optional[bytes]:
        """
        Get the next unique header name.

        Returns:
            Header name as bytes, or None when exhausted
        """
        ...


class HeaderMapValuesIter:
    r"""
    Iterator over header values in a HeaderMap.

    Yields header values as bytes. When used with get_all(), yields
    all values for a specific header name. When used independently,
    yields all values in the entire map.
    """

    def __iter__(self) -> "HeaderMapValuesIter":
        """Return self as iterator."""
        ...

    def __next__(self) -> Optional[bytes]:
        """
        Get the next header value.

        Returns:
            Header value as bytes, or None when exhausted
        """
        ...
