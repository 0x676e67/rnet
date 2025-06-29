from typing import Unpack
from rnet import Message, Proxy, RequestParams, WebSocketParams
import ipaddress
from typing import (
    Optional,
    Union,
    Any,
    Dict,
    List,
    Unpack,
)
from pathlib import Path

from rnet import LookupIpStrategy, TlsVersion, Version, Method, SocketAddr, StatusCode
from rnet.header import HeaderMap
from rnet.cookie import Cookie
from rnet.impersonate import ImpersonateOption, Impersonate

class BlockingClient:
    r"""
    A blocking client for making HTTP requests.
    """

    user_agent: Optional[str]
    r"""
    Returns the user agent of the client.
    """
    headers: HeaderMap
    r"""
    Returns the headers of the client.
    """
    def __new__(
        cls,
        impersonate: Optional[Union[Impersonate, ImpersonateOption]] = None,
        user_agent: Optional[str] = None,
        default_headers: Optional[Union[Dict[str, str], HeaderMap]] = None,
        headers_order: Optional[List[str]] = None,
        referer: Optional[bool] = None,
        allow_redirects: Optional[bool] = None,
        max_redirects: Optional[int] = None,
        cookie_store: Optional[bool] = None,
        lookup_ip_strategy: Optional[LookupIpStrategy] = None,
        timeout: Optional[int] = None,
        connect_timeout: Optional[int] = None,
        read_timeout: Optional[int] = None,
        no_keepalive: Optional[bool] = None,
        tcp_keepalive: Optional[int] = None,
        pool_idle_timeout: Optional[int] = None,
        pool_max_idle_per_host: Optional[int] = None,
        pool_max_size: Optional[int] = None,
        http1_only: Optional[bool] = None,
        http2_only: Optional[bool] = None,
        https_only: Optional[bool] = None,
        tcp_nodelay: Optional[bool] = None,
        http2_max_retry_count: Optional[int] = None,
        verify: Optional[Union[bool, Path]] = None,
        tls_info: Optional[bool] = None,
        min_tls_version: Optional[TlsVersion] = None,
        max_tls_version: Optional[TlsVersion] = None,
        no_proxy: Optional[bool] = None,
        proxies: Optional[List[Proxy]] = None,
        local_address: Optional[
            Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]
        ] = None,
        interface: Optional[str] = None,
        gzip: Optional[bool] = None,
        brotli: Optional[bool] = None,
        deflate: Optional[bool] = None,
        zstd: Optional[bool] = None,
    ) -> BlockingClient:
        r"""
        Creates a new BlockingClient instance.

        # Examples

        ```python
        import asyncio
        import rnet

        client = rnet.BlockingClient(
            user_agent="my-app/0.0.1",
            timeout=10,
        )
        response = client.get('https://httpbin.org/get')
        print(response.text())
        ```
        """

    def get_cookies(self, url: str) -> Optional[bytes]:
        r"""
        Returns the cookies for the given URL.

        # Arguments

        * `url` - The URL to get the cookies for.
        """

    def set_cookie(self, url: str, cookie: Cookie) -> None:
        r"""
        Sets the cookies for the given URL.

        # Arguments
        * `url` - The URL to set the cookies for.
        * `cookie` - The cookie to set.

        # Examples

        ```python
        import rnet

        client = rnet.Client(cookie_store=True)
        client.set_cookie("https://example.com", rnet.Cookie(name="foo", value="bar"))
        ```
        """

    def remove_cookie(self, url: str, name: str) -> None:
        r"""
        Removes the cookie with the given name for the given URL.

        # Arguments
        * `url` - The URL to remove the cookie from.
        * `name` - The name of the cookie to remove.

        # Examples

        ```python
        import rnet

        client = rnet.Client(cookie_store=True)
        client.remove_cookie("https://example.com", "foo")
        """

    def clear_cookies(self) -> None:
        r"""
        Clears the cookies for the given URL.
        """

    def update(
        self,
        impersonate: Optional[Union[Impersonate, ImpersonateOption]] = None,
        headers: Optional[Union[Dict[str, str], HeaderMap]] = None,
        headers_order: Optional[List[str]] = None,
        proxies: Optional[List[Proxy]] = None,
        local_address: Optional[
            Union[ipaddress.IPv4Address, ipaddress.IPv6Address]
        ] = None,
        interface: Optional[str] = None,
    ) -> None:
        r"""
        Updates the client with the given parameters.

        # Examples

        ```python
        import rnet

        client = rnet.BlockingClient()
        client.update(
           impersonate=rnet.Impersonate.Firefox135,
           headers={"X-My-Header": "value"},
           proxies=[rnet.Proxy.all("http://proxy.example.com:8080")],
        )
        ```
        """

    def request(
        self,
        method: Method,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given method and URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.request(Method.GET, "https://httpbin.org/anything")
            print(response.text())

        asyncio.run(main())
        ```
        """

    def websocket(
        self, url: str, **kwargs: Unpack[WebSocketParams]
    ) -> BlockingWebSocket:
        r"""
        Sends a WebSocket request.

        # Examples

        ```python
        import rnet
        import asyncio

        async def main():
            client = rnet.BlockingClient()
            ws = client.websocket("wss://echo.websocket.org")
            ws.send(rnet.Message.from_text("Hello, WebSocket!"))
            message = ws.recv()
            print("Received:", message.data)
            ws.close()

        asyncio.run(main())
        ```
        """

    def trace(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.trace("https://httpbin.org/anything")
            print(response.text())

        asyncio.run(main())
        ```
        """

    def options(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.options("https://httpbin.org/anything")
            print(response.text())

        asyncio.run(main())
        ```
        """

    def head(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.head("https://httpbin.org/anything")
            print(response.text())

        asyncio.run(main())
        ```
        """

    def delete(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.delete("https://httpbin.org/anything")
            print(response.text())

        asyncio.run(main())
        ```
        """

    def patch(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.patch("https://httpbin.org/anything", json={"key": "value"})
            print(response.text())

        asyncio.run(main())
        ```
        """

    def put(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.put("https://httpbin.org/anything", json={"key": "value"})
            print(response.text())

        asyncio.run(main())
        ```
        """

    def post(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.post("https://httpbin.org/anything", json={"key": "value"})
            print(response.text())

        asyncio.run(main())
        ```
        """

    def get(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> BlockingResponse:
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.BlockingClient()
            response = client.get("https://httpbin.org/anything")
            print(response.text())

        asyncio.run(main())
        ```
        """

class BlockingResponse:
    r"""
    A blocking response from a request.
    """

    url: str
    r"""
    Returns the URL of the response.
    """
    ok: bool
    r"""
    Returns whether the response is successful.
    """
    status: int
    r"""
    Returns the status code as integer of the response.
    """
    status_code: StatusCode
    r"""
    Returns the status code of the response.
    """
    version: Version
    r"""
    Returns the HTTP version of the response.
    """
    headers: HeaderMap
    r"""
    Returns the headers of the response.
    """
    cookies: List[Cookie]
    r"""
    Returns the cookies of the response.
    """
    content_length: int
    r"""
    Returns the content length of the response.
    """
    remote_addr: Optional[SocketAddr]
    r"""
    Returns the remote address of the response.
    """
    encoding: str
    r"""
    Encoding to decode with when accessing text.
    """
    def __enter__(self) -> BlockingResponse: ...
    def __exit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> None: ...
    def peer_certificate(self) -> Optional[bytes]:
        r"""
        Returns the TLS peer certificate of the response.
        """

    def text(self) -> str:
        r"""
        Returns the text content of the response.
        """

    def text_with_charset(self, encoding: str) -> str:
        r"""
        Returns the text content of the response with a specific charset.

        # Arguments

        * `encoding` - The default encoding to use if the charset is not specified.
        """

    def json(self) -> Dict[str, Any]:
        r"""
        Returns the JSON content of the response.
        """

    def bytes(self) -> bytes:
        r"""
        Returns the bytes content of the response.
        """

    def stream(self) -> BlockingStreamer:
        r"""
        Convert the response into a `Stream` of `Bytes` from the body.
        """

    def close(self) -> None:
        r"""
        Closes the response connection.
        """

class BlockingStreamer:
    r"""
    A blocking byte stream response.
    An asynchronous iterator yielding data chunks from the response stream.
    Used for streaming response content.
    Employed in the `stream` method of the `Response` class.
    Utilized in an asynchronous for loop in Python.
    """

    def __iter__(self) -> BlockingStreamer: ...
    def __next__(self) -> Any: ...
    def __enter__(self) -> BlockingStreamer: ...
    def __exit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> None: ...

class BlockingWebSocket:
    r"""
    A blocking WebSocket response.
    """

    ok: bool
    r"""
    Returns whether the response is successful.
    """
    status: int
    r"""
    Returns the status code as integer of the response.
    """
    status_code: StatusCode
    r"""
    Returns the status code of the response.
    """
    version: Version
    r"""
    Returns the HTTP version of the response.
    """
    headers: HeaderMap
    r"""
    Returns the headers of the response.
    """
    cookies: List[Cookie]
    r"""
    Returns the cookies of the response.
    """
    remote_addr: Optional[SocketAddr]
    r"""
    Returns the remote address of the response.
    """
    protocol: Optional[str]
    r"""
    Returns the WebSocket protocol.
    """
    def __iter__(self) -> BlockingWebSocket: ...
    def __next__(self) -> Message: ...
    def __enter__(self) -> BlockingWebSocket: ...
    def __exit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> None: ...
    def recv(self) -> Optional[Message]:
        r"""
        Receives a message from the WebSocket.
        """

    def send(self, message: Message) -> None:
        r"""
        Sends a message to the WebSocket.

        # Arguments

        * `message` - The message to send.
        """

    def close(
        self,
        code: Optional[int] = None,
        reason: Optional[str] = None,
    ) -> None:
        r"""
        Closes the WebSocket connection.

        # Arguments

        * `code` - An optional close code.
        * `reason` - An optional reason for closing.
        """
