import datetime
import ipaddress
from typing import (
    AsyncGenerator,
    Generator,
    Optional,
    Tuple,
    Union,
    Any,
    Dict,
    List,
    TypedDict,
    Unpack,
    NotRequired,
)
from pathlib import Path
from enum import Enum, auto

from .http1 import Http1Options
from .http2 import Http2Options
from .cookie import *
from .header import *
from .emulation import *
from .tls import *

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

class Version(Enum):
    r"""
    An HTTP version.
    """

    HTTP_09 = auto()
    HTTP_10 = auto()
    HTTP_11 = auto()
    HTTP_2 = auto()
    HTTP_3 = auto()

class StatusCode:
    r"""
    HTTP status code.
    """

    def __str__(self) -> str: ...
    def as_int(self) -> int:
        r"""
        Return the status code as an integer.
        """
        ...

    def is_informational(self) -> bool:
        r"""
        Check if status is within 100-199.
        """
        ...

    def is_success(self) -> bool:
        r"""
        Check if status is within 200-299.
        """
        ...

    def is_redirection(self) -> bool:
        r"""
        Check if status is within 300-399.
        """
        ...

    def is_client_error(self) -> bool:
        r"""
        Check if status is within 400-499.
        """
        ...

    def is_server_error(self) -> bool:
        r"""
        Check if status is within 500-599.
        """
        ...

class SocketAddr:
    r"""
    A IP socket address.
    """

    def __str__(self) -> str: ...
    def ip(self) -> Union[ipaddress.IPv4Address, ipaddress.IPv6Address]:
        r"""
        Returns the IP address of the socket address.
        """

    def port(self) -> int:
        r"""
        Returns the port number of the socket address.
        """

class Multipart:
    r"""
    A multipart form for a request.
    """

    def __init__(self, *parts: Part) -> None:
        r"""
        Creates a new multipart form.
        """
        ...

class Part:
    r"""
    A part of a multipart form.
    """

    def __init__(
        self,
        name: str,
        value: Union[
            str,
            bytes,
            Path,
            Generator[bytes, str, None],
            AsyncGenerator[bytes, str],
        ],
        filename: str | None = None,
        mime: str | None = None,
        length: int | None = None,
        headers: HeaderMap | None = None,
    ) -> None:
        r"""
        Creates a new part.

        # Arguments
        - `name` - The name of the part.
        - `value` - The value of the part, either text, bytes, a file path, or a async or sync stream.
        - `filename` - The filename of the part.
        - `mime` - The MIME type of the part.
        - `length` - The length of the part when value is a stream (e.g., for file uploads).
        - `headers` - The custom headers for the part.
        """
        ...

class ProxyParams(TypedDict):
    username: NotRequired[str]
    r"""Username for proxy authentication."""

    password: NotRequired[str]
    r"""Password for proxy authentication."""

    custom_http_auth: NotRequired[str]
    r"""Custom HTTP proxy authentication header value."""

    custom_http_headers: NotRequired[Union[Dict[str, str], HeaderMap]]
    r"""Custom HTTP proxy headers."""

    exclusion: NotRequired[str]
    r"""List of domains to exclude from proxying."""

class Proxy:
    r"""
    A proxy server for a request.
    Supports HTTP, HTTPS, SOCKS4, SOCKS4a, SOCKS5, and SOCKS5h protocols.
    """

    @staticmethod
    def http(url: str, **kwargs: Unpack[ProxyParams]) -> "Proxy":
        r"""
        Creates a new HTTP proxy.

        This method sets up a proxy server for HTTP requests.

        # Examples

        ```python
        import rnet

        proxy = rnet.Proxy.http("http://proxy.example.com")
        ```
        """

    @staticmethod
    def https(url: str, **kwargs: Unpack[ProxyParams]) -> "Proxy":
        r"""
        Creates a new HTTPS proxy.

        This method sets up a proxy server for HTTPS requests.

        # Examples

        ```python
        import rnet

        proxy = rnet.Proxy.https("https://proxy.example.com")
        ```
        """

    @staticmethod
    def all(url: str, **kwargs: Unpack[ProxyParams]) -> "Proxy":
        r"""
        Creates a new proxy for all protocols.

        This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).

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

    @staticmethod
    def text_from_json(json: Dict[str, Any]) -> "Message":
        r"""
        Creates a new text message from the JSON representation.

        # Arguments
        * `json` - The JSON representation of the message.
        """
    ...

    @staticmethod
    def binary_from_json(json: Dict[str, Any]) -> "Message":
        r"""
        Creates a new binary message from the JSON representation.

        # Arguments
        * `json` - The JSON representation of the message.
        """

    @staticmethod
    def from_text(text: str) -> "Message":
        r"""
        Creates a new text message.

        # Arguments

        * `text` - The text content of the message.
        """

    @staticmethod
    def from_binary(data: bytes) -> "Message":
        r"""
        Creates a new binary message.

        # Arguments

        * `data` - The binary data of the message.
        """

    @staticmethod
    def from_ping(data: bytes) -> "Message":
        r"""
        Creates a new ping message.

        # Arguments

        * `data` - The ping data of the message.
        """

    @staticmethod
    def from_pong(data: bytes) -> "Message":
        r"""
        Creates a new pong message.

        # Arguments

        * `data` - The pong data of the message.
        """

    @staticmethod
    def from_close(code: int, reason: str | None = None) -> "Message":
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

    def __str__(self) -> str: ...

class History:
    """
    An entry in the redirect history.
    """

    status: int
    """Get the status code of the redirect response."""

    url: str
    """Get the URL of the redirect response."""

    previous: str
    """Get the previous URL before the redirect response."""

    headers: HeaderMap
    """Get the headers of the redirect response."""

    def __str__(self) -> str: ...

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
    from rnet import Method, Emulation

    async def main():
        resp = await rnet.get("https://httpbin.org/stream/20")
        async with resp.stream() as streamer:
            async for chunk in streamer:
                print("Chunk: ", chunk)
                await asyncio.sleep(0.1)

    if __name__ == "__main__":
        asyncio.run(main())
    ```
    """

    async def __aiter__(self) -> "Streamer": ...
    async def __anext__(self) -> Optional[bytes]: ...
    async def __aenter__(self) -> Any: ...
    async def __aexit__(
        self, _exc_type: Any, _exc_value: Any, _traceback: Any
    ) -> Any: ...
    def __iter__(self) -> "Streamer": ...
    def __next__(self) -> bytes: ...
    def __enter__(self) -> "Streamer": ...
    def __exit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> None: ...

class Response:
    r"""
    A response from a request.

    # Examples

    ```python
    import asyncio
    import rnet

    async def main():
        response = await rnet.get("https://www.rust-lang.org")
        print("Status Code: ", response.status)
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
    Get the URL of the response.
    """

    status: StatusCode
    r"""
    Get the status code of the response.
    """

    version: Version
    r"""
    Get the HTTP version of the response.
    """

    headers: HeaderMap
    r"""
    Get the headers of the response.
    """

    cookies: List[Cookie]
    r"""
    Get the cookies of the response.
    """

    content_length: Optional[int]
    r"""
    Get the content length of the response.
    """

    remote_addr: Optional[SocketAddr]
    r"""
    Get the remote address of the response.
    """

    local_addr: Optional[SocketAddr]
    r"""
    Get the local address of the response.
    """

    history: List[History]
    r"""
    Get the redirect history of the Response.
    """

    peer_certificate: Optional[bytes]
    r"""
    Get the DER encoded leaf certificate of the response.
    """

    def raise_for_status(self) -> None:
        r"""
        Turn a response into an error if the server returned an error.
        """

    def stream(self) -> Streamer:
        r"""
        Get the response into a `Streamer` of `bytes` from the body.
        """

    async def text(self) -> str:
        r"""
        Get the text content of the response.
        """

    async def text_with_charset(self, encoding: str) -> str:
        r"""
        Get the full response text given a specific encoding.
        """

    async def json(self) -> Any:
        r"""
        Get the JSON content of the response.
        """

    async def bytes(self) -> bytes:
        r"""
        Get the bytes content of the response.
        """

    async def close(self) -> None:
        r"""
        Close the response connection.
        """

    async def __aenter__(self) -> Any: ...
    async def __aexit__(
        self, _exc_type: Any, _exc_value: Any, _traceback: Any
    ) -> Any: ...

class WebSocket:
    r"""
    A WebSocket response.
    """

    status: StatusCode
    r"""
    Get the status code of the response.
    """

    version: Version
    r"""
    Get the HTTP version of the response.
    """

    headers: HeaderMap
    r"""
    Get the headers of the response.
    """

    cookies: List[Cookie]
    r"""
    Get the cookies of the response.
    """

    remote_addr: Optional[SocketAddr]
    r"""
    Get the remote address of the response.
    """

    protocol: Optional[str]
    r"""
    Get the WebSocket protocol.
    """

    async def recv(
        self, timeout: datetime.timedelta | None = None
    ) -> Optional[Message]:
        r"""
        Receive a message from the WebSocket.
        """

    async def send(self, message: Message) -> None:
        r"""
        Send a message to the WebSocket.
        """

    async def send_all(self, messages: List[Message]) -> None:
        r"""
        Send multiple messages to the WebSocket.
        """

    async def close(
        self,
        code: int | None = None,
        reason: str | None = None,
    ) -> None:
        r"""
        Close the WebSocket connection.
        """

    def __aenter__(self) -> Any: ...
    def __aexit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> Any: ...

class ClientParams(TypedDict):
    emulation: NotRequired[Union[Emulation, EmulationOption]]
    """Emulation config."""

    user_agent: NotRequired[str]
    """
    Default User-Agent string.
    """

    headers: NotRequired[Union[Dict[str, str], HeaderMap]]
    """
    Default request headers.
    """

    orig_headers: NotRequired[Union[List[str], OrigHeaderMap]]
    """
    Original request headers (case-sensitive and order).
    """

    referer: NotRequired[bool]
    """
    Automatically set Referer.
    """

    history: NotRequired[bool]
    """
    Store redirect history.
    """

    allow_redirects: NotRequired[bool]
    """
    Allow automatic redirects.
    """

    max_redirects: NotRequired[int]
    """
    Maximum number of redirects.
    """

    cookie_store: NotRequired[bool]
    """
    Enable cookie store.
    """

    cookie_provider: NotRequired[Jar]
    """
    Custom cookie provider.
    """

    lookup_ip_strategy: NotRequired[str]
    """
    IP lookup strategy.
    """

    timeout: NotRequired[int]
    """
    Total timeout (seconds).
    """

    connect_timeout: NotRequired[int]
    """
    Connection timeout (seconds).
    """

    read_timeout: NotRequired[int]
    """
    Read timeout (seconds).
    """

    tcp_keepalive: NotRequired[int]
    """
    TCP keepalive time (seconds).
    """

    tcp_keepalive_interval: NotRequired[int]
    """
    TCP keepalive interval (seconds).
    """

    tcp_keepalive_retries: NotRequired[int]
    """
    TCP keepalive retry count.
    """

    tcp_user_timeout: NotRequired[int]
    """
    TCP user timeout (seconds).
    """

    tcp_nodelay: NotRequired[bool]
    """
    Enable TCP_NODELAY.
    """

    tcp_reuse_address: NotRequired[bool]
    """
    Enable SO_REUSEADDR.
    """

    pool_idle_timeout: NotRequired[int]
    """
    Connection pool idle timeout (seconds).
    """

    pool_max_idle_per_host: NotRequired[int]
    """
    Max idle connections per host.
    """

    pool_max_size: NotRequired[int]
    """
    Max total connections in pool.
    """

    http1_only: NotRequired[bool]
    """
    Enable HTTP/1.1 only.
    """

    http2_only: NotRequired[bool]
    """
    Enable HTTP/2 only.
    """

    https_only: NotRequired[bool]
    """
    Enable HTTPS only.
    """

    http1_options: NotRequired[Http1Options]
    """
    Sets the HTTP/1 options.
    """

    http2_options: NotRequired[Http2Options]
    """
    Sets the HTTP/2 options.
    """

    verify: NotRequired[Union[bool, Path, CertStore]]
    """
    Verify SSL or specify CA path.
    """

    verify_hostname: NotRequired[bool]
    """
    Configures the use of hostname verification when connecting.
    """

    identity: NotRequired[Identity]
    """
    Represents a private key and X509 cert as a client certificate.
    """

    keylog: NotRequired[KeyLog]
    """
    Key logging policy (environment or file).
    """

    tls_info: NotRequired[bool]
    """
    Return TLS info.
    """

    min_tls_version: NotRequired[TlsVersion]
    """
    Minimum TLS version.
    """

    max_tls_version: NotRequired[TlsVersion]
    """
    Maximum TLS version.
    """

    tls_options: NotRequired[TlsOptions]
    """
    Sets the TLS options.
    """

    no_proxy: NotRequired[bool]
    """
    Disable proxy.
    """

    proxies: NotRequired[List[Proxy]]
    """
    Proxy server list.
    """

    local_address: NotRequired[Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]
    """
    Local bind address.
    """

    interface: NotRequired[str]
    """
    Local network interface.
    """

    gzip: NotRequired[bool]
    """
    Enable gzip decompression.
    """

    brotli: NotRequired[bool]
    """
    Enable brotli decompression.
    """

    deflate: NotRequired[bool]
    """
    Enable deflate decompression.
    """

    zstd: NotRequired[bool]
    """
    Enable zstd decompression.
    """

class Request(TypedDict):
    emulation: NotRequired[Union[Emulation, EmulationOption]]
    """
    The Emulation settings for the request.
    """

    proxy: NotRequired[Proxy]
    """
    The proxy to use for the request.
    """

    local_address: NotRequired[Union[ipaddress.IPv4Address, ipaddress.IPv6Address]]
    """
    Bind to a local IP Address.
    """

    interface: NotRequired[str]
    """
    Bind to an interface by SO_BINDTODEVICE.
    """

    timeout: NotRequired[int]
    """
    The timeout to use for the request.
    """

    read_timeout: NotRequired[int]
    """
    The read timeout to use for the request.
    """

    version: NotRequired[Version]
    """
    The HTTP version to use for the request.
    """

    headers: NotRequired[Union[Dict[str, str], HeaderMap]]
    """
    The headers to use for the request.
    """

    orig_headers: NotRequired[Union[List[str], OrigHeaderMap]]
    """
    The original headers to use for the request.
    """

    default_headers: NotRequired[bool]
    """
    The option enables default headers.
    """

    cookies: NotRequired[Dict[str, str]]
    """
    The cookies to use for the request.
    """

    allow_redirects: NotRequired[bool]
    """
    Whether to allow redirects.
    """

    max_redirects: NotRequired[int]
    """
    The maximum number of redirects to follow.
    """

    gzip: NotRequired[bool]
    """
    Sets gzip as an accepted encoding.
    """

    brotli: NotRequired[bool]
    """
    Sets brotli as an accepted encoding.
    """

    deflate: NotRequired[bool]
    """
    Sets deflate as an accepted encoding.
    """

    zstd: NotRequired[bool]
    """
    Sets zstd as an accepted encoding.
    """

    auth: NotRequired[str]
    """
    The authentication to use for the request.
    """

    bearer_auth: NotRequired[str]
    """
    The bearer authentication to use for the request.
    """

    basic_auth: NotRequired[Tuple[str, Optional[str]]]
    """
    The basic authentication to use for the request.
    """

    query: NotRequired[List[Tuple[str, str]]]
    """
    The query parameters to use for the request.
    """

    form: NotRequired[List[Tuple[str, str]]]
    """
    The form parameters to use for the request.
    """

    json: NotRequired[Dict[str, Any]]
    """
    The JSON body to use for the request.
    """

    body: NotRequired[
        Union[
            str,
            bytes,
            Generator[bytes, str, None],
            AsyncGenerator[bytes, str],
        ]
    ]
    """
    The body to use for the request.
    """

    multipart: NotRequired[Multipart]
    """
    The multipart form to use for the request.
    """

class WebSocketRequest(TypedDict):
    emulation: NotRequired[Union[Emulation, EmulationOption]]
    """
    The Emulation settings for the request.
    """

    proxy: NotRequired[Proxy]
    """
    The proxy to use for the request.
    """

    local_address: NotRequired[Union[str, ipaddress.IPv4Address, ipaddress.IPv6Address]]
    """
    Bind to a local IP Address.
    """

    interface: NotRequired[str]
    """
    Bind to an interface by SO_BINDTODEVICE.
    """

    headers: NotRequired[Union[Dict[str, str], HeaderMap]]
    """
    The headers to use for the request.
    """

    orig_headers: NotRequired[Union[List[str], OrigHeaderMap]]
    """
    The original headers to use for the request.
    """

    default_headers: NotRequired[bool]
    """
    The option enables default headers.
    """

    cookies: NotRequired[Dict[str, str]]
    """
    The cookies to use for the request.
    """

    protocols: NotRequired[List[str]]
    """
    The protocols to use for the request.
    """

    force_http2: NotRequired[bool]
    """
    Whether to use HTTP/2 for the websocket.
    """

    auth: NotRequired[str]
    """
    The authentication to use for the request.
    """

    bearer_auth: NotRequired[str]
    """
    The bearer authentication to use for the request.
    """

    basic_auth: NotRequired[Tuple[str, Optional[str]]]
    """
    The basic authentication to use for the request.
    """

    query: NotRequired[List[Tuple[str, str]]]
    """
    The query parameters to use for the request.
    """

    read_buffer_size: NotRequired[int]
    """
    Read buffer capacity. This buffer is eagerly allocated and used for receiving messages.

    For high read load scenarios a larger buffer, e.g. 128 KiB, improves performance.

    For scenarios where you expect a lot of connections and don't need high read load
    performance a smaller buffer, e.g. 4 KiB, would be appropriate to lower total
    memory usage.

    The default value is 128 KiB.
    """

    write_buffer_size: NotRequired[int]
    """
    The target minimum size of the write buffer to reach before writing the data
    to the underlying stream. The default value is 128 KiB.

    If set to 0 each message will be eagerly written to the underlying stream.
    It is often more optimal to allow them to buffer a little, hence the default value.

    Note: flush() will always fully write the buffer regardless.
    """

    max_write_buffer_size: NotRequired[int]
    """
    The max size of the write buffer in bytes. Setting this can provide backpressure
    in the case the write buffer is filling up due to write errors.
    The default value is unlimited.

    Note: The write buffer only builds up past write_buffer_size when writes to the
    underlying stream are failing. So the write buffer can not fill up if you are not
    observing write errors even if not flushing.

    Note: Should always be at least write_buffer_size + 1 message and probably a little
    more depending on error handling strategy.
    """

    max_message_size: NotRequired[int]
    """
    The maximum size of an incoming message. None means no size limit.
    The default value is 64 MiB which should be reasonably big for all normal use-cases
    but small enough to prevent memory eating by a malicious user.
    """

    max_frame_size: NotRequired[int]
    """
    The maximum size of a single incoming message frame. None means no size limit.
    The limit is for frame payload NOT including the frame header.
    The default value is 16 MiB which should be reasonably big for all normal use-cases
    but small enough to prevent memory eating by a malicious user.
    """

    accept_unmasked_frames: NotRequired[bool]
    """
    When set to True, the server will accept and handle unmasked frames from the client.
    According to RFC 6455, the server must close the connection to the client in such cases,
    however it seems like there are some popular libraries that are sending unmasked frames,
    ignoring the RFC. By default this option is set to False, i.e. according to RFC6455.
    """

class Client:
    r"""
    A client for making HTTP requests.
    """

    def __init__(
        self,
        **kwargs: Unpack[ClientParams],
    ) -> None:
        r"""
        Creates a new Client instance.

        Examples:

        ```python
        import asyncio
        import rnet

        async def main():
            client = rnet.Client(
                user_agent="Mozilla/5.0",
                timeout=10,
            )
            response = await client.get('https://httpbin.org/get')
            print(await response.text())

        asyncio.run(main())
        ```
        """
        ...

    async def request(
        self,
        method: Method,
        url: str,
        **kwargs: Unpack[Request],
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
        **kwargs: Unpack[WebSocketRequest],
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
        **kwargs: Unpack[Request],
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
        **kwargs: Unpack[Request],
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
        **kwargs: Unpack[Request],
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
        **kwargs: Unpack[Request],
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
        **kwargs: Unpack[Request],
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
        **kwargs: Unpack[Request],
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
        **kwargs: Unpack[Request],
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
        **kwargs: Unpack[Request],
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

async def delete(
    url: str,
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[Request],
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
    **kwargs: Unpack[WebSocketRequest],
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
