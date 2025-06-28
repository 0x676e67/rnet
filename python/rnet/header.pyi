from typing import Optional, Tuple

class HeaderMap:
    r"""
    A HTTP header map.
    """

    def __getitem__(self, key: str) -> Optional[bytes]: ...
    def __setitem__(self, key: str, value: str) -> None: ...
    def __delitem__(self, key: str) -> None: ...
    def __contains__(self, key: str) -> bool: ...
    def __len__(self) -> int: ...
    def __iter__(self) -> HeaderMapKeysIter: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __new__(
        cls, init: Optional[dict] = None, capacity: Optional[int] = None
    ) -> HeaderMap: ...
    def contains_key(self, key: str) -> bool:
        r"""
        Returns true if the header map contains the given key.
        """

    def insert(self, key: str, value: str) -> None:
        r"""
        Insert a key-value pair into the header map.
        """

    def append(self, key: str, value: str) -> None:
        r"""
        Append a key-value pair to the header map.
        """

    def remove(self, key: str) -> None:
        r"""
        Remove a key-value pair from the header map.
        """

    def get(self, key: str) -> Optional[bytes]:
        r"""
        Returns a reference to the value associated with the key.

        If there are multiple values associated with the key, then the first one
        is returned. Use `get_all` to get all values associated with a given
        key. Returns `None` if there are no values associated with the key.
        """

    def get_all(self, key: str) -> HeaderMapValuesIter:
        r"""
        Returns a view of all values associated with a key.
        """

    def len(self) -> int:
        """
        Returns the number of values stored in the map.
        This number can be greater than or equal to the number of keys.
        """

    def keys_len(self) -> int:
        """
        Returns the number of unique keys stored in the map.
        """

    def is_empty(self) -> bool:
        """
        Returns True if the map contains no elements.
        """

    def clear(self) -> None:
        """
        Clears the map, removing all key-value pairs.
        """

    def items(self) -> HeaderMapItemsIter:
        r"""
        Returns key-value pairs in the order they were added.
        """

class HeaderMapItemsIter:
    r"""
    An iterator over the items in a HeaderMap.
    """

    def __iter__(self) -> HeaderMapItemsIter: ...
    def __next__(
        self,
    ) -> Optional[Tuple[bytes, Optional[bytes]]]: ...

class HeaderMapKeysIter:
    r"""
    An iterator over the keys in a HeaderMap.
    """

    def __iter__(self) -> HeaderMapKeysIter: ...
    def __next__(self) -> Optional[bytes]: ...

class HeaderMapValuesIter:
    r"""
    An iterator over the values in a HeaderMap.
    """

    def __iter__(self) -> HeaderMapValuesIter: ...
    def __next__(self) -> Optional[bytes]: ...
