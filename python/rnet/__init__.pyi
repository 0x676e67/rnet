import ipaddress
import typing
from typing import (
    Optional,
    Tuple,
    Union,
    Any,
    Dict,
    List,
    TypedDict,
)
from pathlib import Path
from enum import Enum, auto

from .blocking import *
from .cookie import *
from .exceptions import *
from .header import *
from .impersonate import *

try:
    from typing import Unpack, NotRequired
except ImportError:
    from typing_extensions import Unpack, NotRequired

class RequestParams(TypedDict, closed=True):
    proxy: NotRequired[Union[str, Proxy]]
    local_address: NotRequired[Union[ipaddress.IPv4Address, ipaddress.IPv6Address]]
    interface: NotRequired[str]
    timeout: NotRequired[int]
    read_timeout: NotRequired[int]
    version: NotRequired[Version]
    headers: NotRequired[Union[Dict[str, str], HeaderMap]]
    cookies: NotRequired[Dict[str, str]]
    allow_redirects: NotRequired[bool]
    max_redirects: NotRequired[int]
    auth: NotRequired[str]
    bearer_auth: NotRequired[str]
    basic_auth: NotRequired[Tuple[str, Optional[str]]]
    query: NotRequired[List[Tuple[str, str]]]
    form: NotRequired[List[Tuple[str, str]]]
    json: NotRequired[Dict[str, Any]]
    body: NotRequired[
        Union[
            str,
            bytes,
            typing.AsyncGenerator[bytes, str],
            typing.Generator[bytes, str],
        ]
    ]
    multipart: NotRequired[Multipart]

class WebSocketParams(TypedDict, closed=True):
    proxy: NotRequired[Union[str, Proxy]]
    local_address: NotRequired[Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]
    interface: NotRequired[str]
    headers: NotRequired[Union[Dict[str, str], HeaderMap]]
    cookies: NotRequired[Dict[str, str]]
    protocols: NotRequired[List[str]]
    use_http2: NotRequired[bool]
    auth: NotRequired[str]
    bearer_auth: NotRequired[str]
    basic_auth: NotRequired[Tuple[str, Optional[str]]]
    query: NotRequired[List[Tuple[str, str]]]
    read_buffer_size: NotRequired[int]
    write_buffer_size: NotRequired[int]
    max_write_buffer_size: NotRequired[int]
    max_message_size: NotRequired[int]
    max_frame_size: NotRequired[int]
    accept_unmasked_frames: NotRequired[bool]

class ProxyParams(TypedDict, closed=True):
    username: NotRequired[str]
    password: NotRequired[str]
    custom_http_auth: NotRequired[str]
    custom_http_headers: NotRequired[Union[Dict[str, str], HeaderMap]]
    exclusion: NotRequired[str]

class Client:
    r"""
    A client for making HTTP requests.
    """

    user_agent: Optional[str]
    r"""
    Returns the user agent of the client.
    
    # Examples
    
    ```python
    import rnet
    
    client = rnet.Client()
    user_agent = client.user_agent()
    print(user_agent)
    ```
    """
    headers: HeaderMap
    r"""
    Returns the headers of the client.
    
    # Examples
    
    ```python
    import rnet
    
    client = rnet.Client()
    headers = client.headers()
    print(headers)
    ```
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
    ) -> Client:
        r"""
        Creates a new Client instance.

        # Examples

        ```python
        import asyncio
        import rnet

        client = rnet.Client(
            user_agent="my-app/0.0.1",
            timeout=10,
        )
        response = await client.get('https://httpbin.org/get')
        print(response.text)
        ```
        """

    def get_cookies(self, url: str) -> Optional[bytes]:
        r"""
        Returns the cookies for the given URL.

        # Arguments

        * `url` - The URL to get the cookies for.

        # Returns

        A List of cookie strings.

        # Examples

        ```python
        import rnet

        client = rnet.Client(cookie_store=True)
        cookies = client.get_cookies("https://example.com")
        print(cookies)
        ```
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

        # Arguments

        * `impersonate` - The impersonation settings for the request.
        * `headers` - The headers to use for the request.
        * `headers_order` - The order of the headers to use for the request.
        * `proxies` - The proxy to use for the request.
        * `local_address` - The local IP address to bind to.
        * `interface` - The interface to bind to.

        # Examples

        ```python
        import rnet

        client = rnet.Client()
        client.update(
           impersonate=rnet.Impersonate.Firefox135,
           headers={"X-My-Header": "value"},
           proxies=[rnet.Proxy.all("http://proxy.example.com:8080")],
        )
        ```
        """

    async def request(
        self,
        method: Method,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given method and URL.

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.request(Method.GET, "https://httpbin.org/anything")
            print(await response.text())

        asyncio.run(main())
        ```
        """

    async def websocket(
        self,
        url: str,
        **kwargs: Unpack[WebSocketParams],
    ) -> WebSocket:
        r"""
        Sends a WebSocket request.

        # Examples

        ```python
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            ws = await client.websocket("wss://echo.websocket.org")
            await ws.send(rnet.Message.from_text("Hello, WebSocket!"))
            message = await ws.recv()
            print("Received:", message.data)
            await ws.close()

        asyncio.run(main())
        ```
        """

    async def trace(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given URL

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.trace("https://httpbin.org/anything")
            print(await response.text())

        asyncio.run(main())
        ```
        """

    async def options(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given URL

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.options("https://httpbin.org/anything")
            print(await response.text())

        asyncio.run(main())
        ```
        """

    async def patch(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given URL

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.patch("https://httpbin.org/anything", json={"key": "value"})
            print(await response.text())

        asyncio.run(main())
        ```
        """

    async def delete(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given URL

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.delete("https://httpbin.org/anything")
            print(await response.text())

        asyncio.run(main())
        ```
        """

    async def put(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given URL

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.put("https://httpbin.org/anything", json={"key": "value"})
            print(await response.text())

        asyncio.run(main())
        ```
        """

    async def post(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given URL

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.post("https://httpbin.org/anything", json={"key": "value"})
            print(await response.text())

        asyncio.run(main())
        ```
        """

    async def head(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given URL

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.head("https://httpbin.org/anything")
            print(response.status)

        asyncio.run(main())
        ```
        """

    async def get(
        self,
        url: str,
        **kwargs: Unpack[RequestParams],
    ) -> Response:
        r"""
        Sends a request with the given URL

        # Examples

        ```python
        import rnet
        import asyncio
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.get("https://httpbin.org/anything")
            print(await response.text())

        asyncio.run(main())
        ```
        """

class Multipart:
    r"""
    A multipart form for a request.
    """

    def __new__(cls, *parts) -> Multipart:
        r"""
        Creates a new multipart form.
        """

class Part:
    r"""
    A part of a multipart form.
    """

    def __new__(
        cls,
        name: str,
        value: Union[
            str,
            bytes,
            Path,
            typing.AsyncGenerator[bytes, str],
            typing.Generator[bytes, str],
        ],
        filename: Optional[str] = None,
        mime: Optional[str] = None,
    ) -> Part:
        r"""
        Creates a new part.

        # Arguments
        - `name` - The name of the part.
        - `value` - The value of the part, either text, bytes, a file path, or a async or sync stream.
        - `filename` - The filename of the part.
        - `mime` - The MIME type of the part.
        """

class Response:
    r"""
    A response from a request.

    # Examples

    ```python
    import asyncio
    import rnet

    async def main():
        response = await rnet.get("https://www.rust-lang.org")
        print("Status Code: ", response.status_code)
        print("Version: ", response.version)
        print("Response URL: ", response.url)
        print("Headers: ", response.headers)
        print("Content-Length: ", response.content_length)
        print("Encoding: ", response.encoding)
        print("Remote Address: ", response.remote_addr)

        text_content = await response.text()
        print("Text: ", text_content)

    if __name__ == "__main__":
        asyncio.run(main())
    ```
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
    def __aenter__(self) -> Any: ...
    def __aexit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> Any: ...
    def peer_certificate(self) -> Optional[bytes]:
        r"""
        Returns the TLS peer certificate of the response.
        """

    async def text(self) -> str:
        r"""
        Returns the text content of the response.
        """

    async def text_with_charset(self, encoding: str) -> str:
        r"""
        Returns the text content of the response with a specific charset.

        # Arguments

        * `encoding` - The default encoding to use if the charset is not specified.
        """

    async def json(self) -> Any:
        r"""
        Returns the JSON content of the response.
        """

    async def bytes(self) -> bytes:
        r"""
        Returns the bytes content of the response.
        """

    def stream(self) -> Streamer:
        r"""
        Convert the response into a `Stream` of `Bytes` from the body.
        """

    async def close(self) -> None:
        r"""
        Closes the response connection.
        """

class SocketAddr:
    r"""
    A IP socket address.
    """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def ip(self) -> Union[ipaddress.IPv4Address, ipaddress.IPv6Address]:
        r"""
        Returns the IP address of the socket address.
        """

    def port(self) -> int:
        r"""
        Returns the port number of the socket address.
        """

class StatusCode:
    r"""
    HTTP status code.
    """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def as_int(self) -> int:
        r"""
        Return the status code as an integer.
        """

    def is_informational(self) -> bool:
        r"""
        Check if status is within 100-199.
        """

    def is_success(self) -> bool:
        r"""
        Check if status is within 200-299.
        """

    def is_redirection(self) -> bool:
        r"""
        Check if status is within 300-399.
        """

    def is_client_error(self) -> bool:
        r"""
        Check if status is within 400-499.
        """

    def is_server_error(self) -> bool:
        r"""
        Check if status is within 500-599.
        """

class Streamer:
    r"""
    A byte stream response.
    An asynchronous iterator yielding data chunks from the response stream.
    Used to stream response content.
    Implemented in the `stream` method of the `Response` class.
    Can be used in an asynchronous for loop in Python.

    # Examples

    ```python
    import asyncio
    import rnet
    from rnet import Method, Impersonate

    async def main():
        resp = await rnet.get("https://httpbin.org/stream/20")
        print("Status Code: ", resp.status_code)
        print("Version: ", resp.version)
        print("Response URL: ", resp.url)
        print("Headers: ", resp.headers)
        print("Content-Length: ", resp.content_length)
        print("Encoding: ", resp.encoding)
        print("Remote Address: ", resp.remote_addr)

        async with resp.stream() as streamer:
            async for chunk in streamer:
                print("Chunk: ", chunk)
                await asyncio.sleep(0.1)

    if __name__ == "__main__":
        asyncio.run(main())
    ```
    """

    def __aiter__(self) -> Streamer: ...
    def __anext__(self) -> Any: ...
    def __aenter__(self) -> Any: ...
    def __aexit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> Any: ...

async def delete(
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Shortcut method to quickly make a request.

    # Examples

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.delete("https://httpbin.org/anything")
        body = await response.text()
        print(body)

    asyncio.run(run())
    ```
    """

async def get(
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Shortcut method to quickly make a request.

    # Examples

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.get("https://httpbin.org/anything")
        body = await response.text()
        print(body)

    asyncio.run(run())
    ```
    """

async def head(
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Shortcut method to quickly make a request.

    # Examples

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.head("https://httpbin.org/anything")
        print(response.status)

    asyncio.run(run())
    ```
    """

async def options(
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Shortcut method to quickly make a request.

    # Examples

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.options("https://httpbin.org/anything")
        print(response.status)

    asyncio.run(run())
    ```
    """

async def patch(
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Shortcut method to quickly make a request.

    # Examples

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.patch("https://httpbin.org/anything")
        body = await response.text()
        print(body)

    asyncio.run(run())
    ```
    """

async def post(
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Shortcut method to quickly make a request.

    # Examples

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.post("https://httpbin.org/anything")
        body = await response.text()
        print(body)

    asyncio.run(run())
    ```
    """

async def put(
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Shortcut method to quickly make a request.

    # Examples

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.put("https://httpbin.org/anything")
        body = await response.text()
        print(body)

    asyncio.run(run())
    ```
    """

async def request(
    method: Method,
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Make a request with the given parameters.

    # Arguments

    * `method` - The method to use for the request.
    * `url` - The URL to send the request to.
    * `**kwargs` - Additional request parameters.

    # Examples

    ```python
    import rnet
    import asyncio
    from rnet import Method

    async def run():
        response = await rnet.request(Method.GET, "https://www.rust-lang.org")
        body = await response.text()
        print(body)

    asyncio.run(run())
    ```
    """

async def trace(
    url: str,
    **kwargs: Unpack[RequestParams],
) -> Response:
    r"""
    Shortcut method to quickly make a request.

    # Examples

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.trace("https://httpbin.org/anything")
        print(response.status)

    asyncio.run(run())
    ```
    """

async def websocket(
    url: str,
    **kwargs: Unpack[WebSocketParams],
) -> WebSocket:
    r"""
    Make a WebSocket connection with the given parameters.

    # Examples

    ```python
    import rnet
    import asyncio
    from rnet import Message

    async def run():
        ws = await rnet.websocket("wss://echo.websocket.org")
        await ws.send(Message.from_text("Hello, World!"))
        message = await ws.recv()
        print("Received:", message.data)
        await ws.close()

    asyncio.run(run())
    ```
    """

class Proxy:
    r"""
    A proxy server for a request.
    Supports HTTP, HTTPS, SOCKS4, SOCKS4a, SOCKS5, and SOCKS5h protocols.
    """

    @staticmethod
    def http(url: str, **kwargs: Unpack[ProxyParams]) -> Proxy:
        r"""
        Creates a new HTTP proxy.

        This method sets up a proxy server for HTTP requests.

        # Arguments

        * `url` - The URL of the proxy server.
        * `username` - Optional username for proxy authentication.
        * `password` - Optional password for proxy authentication.
        * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        * `custom_http_headers` - Optional custom HTTP proxy headers.
        * `exclusion` - Optional List of domains to exclude from proxying.

        # Examples

        ```python
        import rnet

        proxy = rnet.Proxy.http("http://proxy.example.com")
        ```
        """

    @staticmethod
    def https(url: str, **kwargs: Unpack[ProxyParams]) -> Proxy:
        r"""
        Creates a new HTTPS proxy.

        This method sets up a proxy server for HTTPS requests.

        # Arguments

        * `url` - The URL of the proxy server.
        * `username` - Optional username for proxy authentication.
        * `password` - Optional password for proxy authentication.
        * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        * `custom_http_headers` - Optional custom HTTP proxy headers.
        * `exclusion` - Optional List of domains to exclude from proxying.

        # Examples

        ```python
        import rnet

        proxy = rnet.Proxy.https("https://proxy.example.com")
        ```
        """

    @staticmethod
    def all(url: str, **kwargs: Unpack[ProxyParams]) -> Proxy:
        r"""
        Creates a new proxy for all protocols.

        This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).

        # Arguments

        * `url` - The URL of the proxy server.
        * `username` - Optional username for proxy authentication.
        * `password` - Optional password for proxy authentication.
        * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        * `custom_http_headers` - Optional custom HTTP proxy headers.
        * `exclusion` - Optional List of domains to exclude from proxying.

        # Examples

        ```python
        import rnet

        proxy = rnet.Proxy.all("https://proxy.example.com")
        ```
        """

class Message:
    r"""
    A WebSocket message.
    """

    data: Optional[bytes]
    r"""
    Returns the data of the message as bytes.
    """
    text: Optional[str]
    r"""
    Returns the text content of the message if it is a text message.
    """
    binary: Optional[bytes]
    r"""
    Returns the binary data of the message if it is a binary message.
    """
    ping: Optional[bytes]
    r"""
    Returns the ping data of the message if it is a ping message.
    """
    pong: Optional[bytes]
    r"""
    Returns the pong data of the message if it is a pong message.
    """
    close: Optional[Tuple[int, Optional[str]]]
    r"""
    Returns the close code and reason of the message if it is a close message.
    """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    @staticmethod
    def text_from_json(json: Dict[str, Any]) -> Message:
        r"""
        Creates a new text message from the JSON representation.

        # Arguments
        * `json` - The JSON representation of the message.
        """

    @staticmethod
    def binary_from_json(json: Dict[str, Any]) -> Message:
        r"""
        Creates a new binary message from the JSON representation.

        # Arguments
        * `json` - The JSON representation of the message.
        """

    @staticmethod
    def from_text(text: str) -> Message:
        r"""
        Creates a new text message.

        # Arguments

        * `text` - The text content of the message.
        """

    @staticmethod
    def from_binary(data: bytes) -> Message:
        r"""
        Creates a new binary message.

        # Arguments

        * `data` - The binary data of the message.
        """

    @staticmethod
    def from_ping(data: bytes) -> Message:
        r"""
        Creates a new ping message.

        # Arguments

        * `data` - The ping data of the message.
        """

    @staticmethod
    def from_pong(data: bytes) -> Message:
        r"""
        Creates a new pong message.

        # Arguments

        * `data` - The pong data of the message.
        """

    @staticmethod
    def from_close(code: int, reason: Optional[str] = None) -> Message:
        r"""
        Creates a new close message.

        # Arguments

        * `code` - The close code.
        * `reason` - An optional reason for closing.
        """

    def json(self) -> Dict[str, Any]:
        r"""
        Returns the JSON representation of the message.
        """

class WebSocket:
    r"""
    A WebSocket response.
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
    def __aiter__(self) -> WebSocket: ...
    def __anext__(self) -> Any: ...
    def __aenter__(self) -> Any: ...
    def __aexit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> Any: ...
    async def recv(self) -> Optional[Message]:
        r"""
        Receives a message from the WebSocket.
        """

    async def send(self, message: Message) -> None:
        r"""
        Sends a message to the WebSocket.

        # Arguments

        * `message` - The message to send.
        """

    async def close(
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
