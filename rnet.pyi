# This change addresses better typing support for unpacking **dict into keyword arguments.
# References for future readers:
# - Discussion: https://discuss.python.org/t/proposal-support-unpack-dict-for-typing-keyword-arguments/53380
# - Spec: httpso//typing.python.org/en/latest/spec/callables.html#unpack-for-keyword-arguments

import datetime
import ipaddress
from enum import Enum, auto
from pathlib import Path
from typing import (
    Any,
    AsyncGenerator,
    Dict,
    Generator,
    List,
    NotRequired,
    Optional,
    Tuple,
    TypedDict,
    Unpack,
)

class BaseParams(TypedDict):
    auth: NotRequired[str]
    basic_auth: NotRequired[Tuple[str, Optional[str]]]
    proxy: NotRequired[str | Proxy]
    cookies: NotRequired[Dict[str, str]]
    interface: NotRequired[str]
    local_address: NotRequired[str | ipaddress.IPv4Address | ipaddress.IPv6Address]
    query: NotRequired[List[Tuple[str, str]]]

class WebSocketParams(BaseParams):
    accept_unmasked_frames: NotRequired[bool]
    bearer_auth: NotRequired[str]
    headers: NotRequired[Dict[str, bytes]]
    max_frame_size: NotRequired[int]
    max_message_size: NotRequired[int]
    max_write_buffer_size: NotRequired[int]
    protocols: NotRequired[List[str]]
    read_buffer_size: NotRequired[int]
    use_http2: NotRequired[bool]
    write_buffer_size: NotRequired[int]

class RequestParams(BaseParams):
    allow_redirects: NotRequired[bool]
    bearer_auth: NotRequired[str]
    body: NotRequired[str | bytes | AsyncGenerator[bytes, str] | Generator[bytes, str]]
    form: NotRequired[List[Tuple[str, str]]]
    headers: NotRequired[Dict[str, str] | HeaderMap]
    json: NotRequired[Dict[str, Any]]
    max_redirects: NotRequired[int]
    multipart: NotRequired[Multipart]
    read_timeout: NotRequired[int]
    timeout: NotRequired[int]
    version: NotRequired[Version]

class ProxyParams(TypedDict):
    custom_http_auth: NotRequired[str]
    custom_http_headers: NotRequired[Dict[str, str] | HeaderMap]
    exclusion: NotRequired[str]
    password: NotRequired[str]
    username: NotRequired[str]

ImpersonateType = Optional[Impersonate | ImpersonateOption]

class BaseClient:
    def __new__(
        cls,
        allow_redirects: Optional[bool] = None,
        brotli: Optional[bool] = None,
        connect_timeout: Optional[int] = None,
        cookie_store: Optional[bool] = None,
        default_headers: Optional[Dict[str, bytes]] = None,
        deflate: Optional[bool] = None,
        gzip: Optional[bool] = None,
        impersonate: ImpersonateType | None = None,
        interface: Optional[str] = None,
        headers_order: Optional[List[str]] = None,
        http1_only: Optional[bool] = None,
        http2_max_retry_count: Optional[int] = None,
        http2_only: Optional[bool] = None,
        https_only: Optional[bool] = None,
        local_address: Optional[
            str | ipaddress.IPv4Address | ipaddress.IPv6Address
        ] = None,
        lookup_ip_strategy: Optional[LookupIpStrategy] = None,
        max_redirects: Optional[int] = None,
        max_tls_version: Optional[TlsVersion] = None,
        min_tls_version: Optional[TlsVersion] = None,
        no_keepalive: Optional[bool] = None,
        no_proxy: Optional[bool] = None,
        pool_idle_timeout: Optional[int] = None,
        pool_max_idle_per_host: Optional[int] = None,
        pool_max_size: Optional[int] = None,
        proxies: Optional[List[Proxy]] = None,
        read_timeout: Optional[int] = None,
        referer: Optional[bool] = None,
        tcp_keepalive: Optional[int] = None,
        tcp_nodelay: Optional[bool] = None,
        timeout: Optional[int] = None,
        tls_info: Optional[bool] = None,
        user_agent: Optional[str] | None = None,
        verify: Optional[bool | Path] = None,
        zstd: Optional[bool] = None,
    ):
        """"""

    def get_cookies(self, url: str) -> Optional[bytes]:
        """
        Returns the cookies for the given URL.

        **Arguments**

        * `url` - The target to acquire the cookie.
        """

    def set_cookie(self, url: str, cookie: Cookie) -> None:
        """
        Sets the cookies for the given URL.

        **Arguments**
        * `url` - The URL to set the cookies for.
        * `cookie` - The cookie to set.

        **Examples**

        ```python
        import rnet

        client = rnet.Client(cookie_store=True)
        client.set_cookie("https://example.com", rnet.Cookie(name="foo", value="bar"))
        ```
        """

    def remove_cookie(self, url: str, name: str) -> None:
        """
        Removes the cookie with the given name for the given URL.

        **Arguments**
        * `url` - The URL to remove the cookie from.
        * `name` - The name of the cookie to remove.

        **Examples**

        ```python
        import rnet

        client = rnet.Client(cookie_store=True)
        client.remove_cookie("https://example.com", "foo")
        """

    def clear_cookies(self) -> None:
        """
        Clears the cookies for the given URL.
        """

    def update(
        self,
        impersonate: ImpersonateType | None = None,
        headers: Optional[Dict[str, str] | HeaderMap] = None,
        headers_order: Optional[List[str]] = None,
        proxies: Optional[List[Proxy]] = None,
        local_address: Optional[ipaddress.IPv4Address | ipaddress.IPv6Address] = None,
        interface: Optional[str] = None,
    ) -> None: ...

class BlockingClient(BaseClientDS, BaseClient):
    """
    Creates a new BlockingClient instance.

    **Arguments:**

     * `**kwargs` - Optional request parameters as a dictionary.

    **Examples:**

    ```python
     import rnet

     client = rnet.BlockingClient(
         user_agent="my-app/0.0.1",
         timeout=10,
     )
     response = client.get('https://httpbin.org/get')
     print(response.text())

    # Updates the client's parameters.
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
        self, method: Method, url: str, **kwargs: Unpack[RequestParams]
    ) -> BlockingResponse: ...
    def websocket(
        self, wsoc: str, **kwargs: Unpack[WebSocketParams]
    ) -> BlockingWebSocket: ...
    def trace(self, url: str, **kwargs: Unpack[RequestParams]) -> BlockingResponse: ...
    def options(
        self, url: str, **kwargs: Unpack[RequestParams]
    ) -> BlockingResponse: ...
    def head(self, url: str, **kwargs: Unpack[RequestParams]) -> BlockingResponse: ...
    def delete(self, url: str, **kwargs: Unpack[RequestParams]) -> BlockingResponse: ...
    def patch(self, url: str, **kwargs: Unpack[RequestParams]) -> BlockingResponse: ...
    def put(self, url: str, **kwargs: Unpack[RequestParams]) -> BlockingResponse: ...
    def post(self, url: str, **kwargs: Unpack[RequestParams]) -> BlockingResponse: ...
    def get(self, url: str, **kwargs: Unpack[RequestParams]) -> BlockingResponse: ...

class Client(BaseClientDS, BaseClient):
    """
    A client for making HTTP requests.

    **Examples:**

    ```python
    # Sets the client's user agent.
    import rnet

    client = rnet.Client()
    client.update(impersonate=rnet.ImpersonateOption.random())
    user_agent = client.user_agent
    print(user_agent)

    # Sets the client's headers.
    import rnet

    client = rnet.Client()
    client.update(headers={'Content-Type': 'text/html'})
    headers = client.headers
    print(headers)
    ```
    """

    headers: HeaderMap

    user_agent: Optional[str]

    async def request(
        self, method: Method, url: str, **kwargs: Unpack[RequestParams]
    ) -> Response: ...
    async def websocket(
        self, wsoc: str, **kwargs: Unpack[WebSocketParams]
    ) -> WebSocket: ...
    async def trace(self, url: str, **kwargs: Unpack[RequestParams]) -> Response: ...
    async def options(self, url: str, **kwargs: Unpack[RequestParams]) -> Response: ...
    async def patch(self, url: str, **kwargs: Unpack[RequestParams]) -> Response: ...
    async def delete(self, url: str, **kwargs: Unpack[RequestParams]) -> Response: ...
    async def put(self, url: str, **kwargs: Unpack[RequestParams]) -> Response: ...
    async def post(self, url: str, **kwargs: Unpack[RequestParams]) -> Response: ...
    async def head(self, url: str, **kwargs: Unpack[RequestParams]) -> Response: ...
    async def get(self, url: str, **kwargs: Unpack[RequestParams]) -> Response: ...

class BaseResponse:
    url: str
    """
    Returns the URL of the response.
    """
    ok: bool
    """
    Returns whether the response is successful.
    """
    status: int
    """
    Returns the status code as integer of the response.
    """
    status_code: StatusCode
    """
    Returns the status code of the response.
    """
    version: Version
    """
    Returns the HTTP version of the response.
    """
    headers: HeaderMap
    """
    Returns the headers of the response.
    """
    cookies: List[Cookie]
    """
    Returns the cookies of the response.
    """
    content_length: int
    """
    Returns the content length of the response.
    """
    remote_addr: Optional[SocketAddr]
    """
    Returns the remote address of the response.
    """
    encoding: str
    """
    Encoding to decode with when accessing text.
    """

    def peer_certificate(self) -> Optional[bytes]:
        """
        Returns the TLS peer certificate of the response.
        """

class BlockingResponse(BaseResponseDS, BaseResponse):
    """
    A blocking response from a request.
    """

    def __enter__(self) -> BlockingResponse: ...
    def __exit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> None: ...
    def bytes(self) -> bytes: ...
    def close(self) -> None: ...
    def json(self) -> Dict[str, Any]: ...
    def stream(self) -> BlockingStreamer: ...
    def text(self) -> str: ...
    def text_with_charset(self, encoding: str) -> str: ...

class Response(BaseResponseDS, BaseResponse):
    """
    A response from a request.

    **Examples**

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

    def __aenter__(self) -> Any: ...
    def __aexit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> Any: ...
    async def bytes(self) -> bytes: ...
    async def close(self) -> None: ...
    async def json(self) -> Dict[str, Any]: ...
    def stream(self) -> Streamer: ...
    async def text(self) -> str: ...
    async def text_with_charset(self, encoding: str) -> str: ...

class BlockingStreamer:
    """
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

class Streamer:
    """
    A byte stream response.
    An asynchronous iterator yielding data chunks from the response stream.
    Used to stream response content.
    Implemented in the `stream` method of the `Response` class.
    Can be used in an asynchronous for loop in Python.

    **Examples**

    ```python
    import asyncio
    import rnet
    from rnet import Impersonate

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

class BaseWebSocket:
    ok: bool
    """
    Returns whether the response is successful.
    """
    status: int
    """
    Returns the status code as integer of the response.
    """
    status_code: StatusCode
    """
    Returns the status code of the response.
    """
    version: Version
    """
    Returns the HTTP version of the response.
    """
    headers: HeaderMap
    """
    Returns the headers of the response.
    """
    cookies: List[Cookie]
    """
    Returns the cookies of the response.
    """
    remote_addr: Optional[SocketAddr]
    """
    Returns the remote address of the response.
    """
    protocol: Optional[str]
    """
    Returns the WebSocket protocol.
    """

class BlockingWebSocket(BaseWebSocketDS, BaseWebSocket):
    """
    A blocking WebSocket response.
    """

    def __iter__(self) -> BlockingWebSocket: ...
    def __next__(self) -> Message: ...
    def __enter__(self) -> BlockingWebSocket: ...
    def __exit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> None: ...
    def recv(self) -> Optional[Message]: ...
    def send(self, message: Message) -> None: ...
    def close(
        self, code: Optional[int] = None, reason: Optional[str] = None
    ) -> None: ...

class WebSocket(BaseWebSocketDS, BaseWebSocket):
    """
    Async WebSocket response.
    """

    def __aiter__(self) -> WebSocket: ...
    def __anext__(self) -> Any: ...
    def __aenter__(self) -> Any: ...
    def __aexit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> Any: ...
    async def recv(self) -> Optional[Message]: ...
    async def send(self, message: Message) -> None: ...
    async def close(
        self, code: Optional[int] = None, reason: Optional[str] = None
    ) -> None: ...

class Cookie:
    """
    A cookie.
    """

    name: str
    """
    The name of the cookie.
    """
    value: str
    """
    The value of the cookie.
    """
    http_only: bool
    """
    Returns true if the 'HttpOnly' directive is enabled.
    """
    secure: bool
    """
    Returns true if the 'Secure' directive is enabled.
    """
    same_site_lax: bool
    """
    Returns true if  'SameSite' directive is 'Lax'.
    """
    same_site_strict: bool
    """
    Returns true if  'SameSite' directive is 'Strict'.
    """
    path: Optional[str]
    """
    Returns the path directive of the cookie, if set.
    """
    domain: Optional[str]
    """
    Returns the domain directive of the cookie, if set.
    """
    max_age: Optional[datetime.timedelta]
    """
    Get the Max-Age information.
    """
    expires: Optional[datetime.datetime]
    """
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
        """
        Create a new cookie.
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class HeaderMap:
    """
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
    def __new__(cls, init: Optional[dict]) -> HeaderMap: ...
    def contains_key(self, key: str) -> bool:
        """
        Returns true if the header map contains the given key.
        """

    def insert(self, key: str, value: str) -> None:
        """
        Insert a key-value pair into the header map.
        """

    def append(self, key: str, value: str) -> None:
        """
        Append a key-value pair to the header map.
        """

    def remove(self, key: str) -> None:
        """
        Remove a key-value pair from the header map.
        """

    def get(self, key: str) -> Optional[bytes]:
        """
        Returns a reference to the value associated with the key.

        If there are multiple values associated with the key, then the first one
        is returned. Use `get_all` to get all values associated with a given
        key. Returns `None` if there are no values associated with the key.
        """

    def get_all(self, key: str) -> HeaderMapValuesIter:
        """
        Returns a view of all values associated with a key.
        """

    def items(self) -> HeaderMapItemsIter:
        """
        Returns key-value pairs in the order they were added.
        """

class HeaderMapItemsIter:
    """
    An iterator over the items in a HeaderMap.
    """

    def __iter__(self) -> HeaderMapItemsIter: ...
    def __next__(
        self,
    ) -> Optional[Tuple[bytes, Optional[bytes]]]: ...

class HeaderMapKeysIter:
    """
    An iterator over the keys in a HeaderMap.
    """

    def __iter__(self) -> HeaderMapKeysIter: ...
    def __next__(self) -> Optional[bytes]: ...

class HeaderMapValuesIter:
    """
    An iterator over the values in a HeaderMap.
    """

    def __iter__(self) -> HeaderMapValuesIter: ...
    def __next__(self) -> Optional[bytes]: ...

class Message:
    """
    A WebSocket message.
    """

    data: Optional[bytes]
    """
    Returns the data of the message as bytes.
    """
    text: Optional[str]
    """
    Returns the text content of the message if it is a text message.
    """
    binary: Optional[bytes]
    """
    Returns the binary data of the message if it is a binary message.
    """
    ping: Optional[bytes]
    """
    Returns the ping data of the message if it is a ping message.
    """
    pong: Optional[bytes]
    """
    Returns the pong data of the message if it is a pong message.
    """
    close: Optional[Tuple[int, Optional[str]]]
    """
    Returns the close code and reason of the message if it is a close message.
    """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    @staticmethod
    def text_from_json(json: Dict[str, Any]) -> Message:
        """
        Creates a new text message from the JSON representation.

        **Arguments**
        * `json` - The JSON representation of the message.
        """

    @staticmethod
    def binary_from_json(json: Dict[str, Any]) -> Message:
        """
        Creates a new binary message from the JSON representation.

        **Arguments**
        * `json` - The JSON representation of the message.
        """

    @staticmethod
    def from_text(text: str) -> Message:
        """
        Creates a new text message.

        **Arguments**

        * `text` - The text content of the message.
        """

    @staticmethod
    def from_binary(data: bytes) -> Message:
        """
        Creates a new binary message.

        **Arguments**

        * `data` - The binary data of the message.
        """

    @staticmethod
    def from_ping(data: bytes) -> Message:
        """
        Creates a new ping message.

        **Arguments**

        * `data` - The ping data of the message.
        """

    @staticmethod
    def from_pong(data: bytes) -> Message:
        """
        Creates a new pong message.

        **Arguments**

        * `data` - The pong data of the message.
        """

    @staticmethod
    def from_close(code: int, reason: Optional[str] = None) -> Message:
        """
        Creates a new close message.

        **Arguments**

        * `code` - The close code.
        * `reason` - An optional reason for closing.
        """

    def json(self) -> Dict[str, Any]:
        """
        Returns the JSON representation of the message.
        """

class Multipart:
    """
    A multipart form for a request.
    """

    def __new__(cls, *part) -> Multipart:
        """
        Creates a new multipart form.
        """

class Part:
    """
    A part of a multipart form.
    """

    def __new__(
        cls,
        name: str,
        value: str | bytes | Path | AsyncGenerator[bytes, str] | Generator[bytes, str],
        filename: Optional[str] = None,
        mime: Optional[str] = None,
    ) -> Part:
        """
        Creates a new part.

        **Arguments**
        - `name` - The name of the part.
        - `value` - The value of the part, either text, bytes, a file path, or a async or sync stream.
        - `filename` - The filename of the part.
        - `mime` - The MIME type of the part.
        """

class Proxy:
    """
    A proxy server for a request.
    Supports HTTP, HTTPS, SOCKS4, SOCKS4a, SOCKS5, and SOCKS5h protocols.
    """

    @staticmethod
    def http(url: str, **kwargs: Unpack[ProxyParams]) -> Proxy:
        """
        Creates a new HTTP proxy.

        This method sets up a proxy server for HTTP requests.

        **Arguments**

        * `url` - The URL of the proxy server.
        * `username` - Optional username for proxy authentication.
        * `password` - Optional password for proxy authentication.
        * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        * `custom_http_headers` - Optional custom HTTP proxy headers.
        * `exclusion` - Optional List of domains to exclude from proxying.

        **Examples**

        ```python
        import rnet

        proxy = rnet.Proxy.http("http://proxy.example.com")
        ```
        """

    @staticmethod
    def https(url: str, **kwargs: Unpack[ProxyParams]) -> Proxy:
        """
        Creates a new HTTPS proxy.

        This method sets up a proxy server for HTTPS requests.

        **Arguments**

        * `url` - The URL of the proxy server.
        * `username` - Optional username for proxy authentication.
        * `password` - Optional password for proxy authentication.
        * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        * `custom_http_headers` - Optional custom HTTP proxy headers.
        * `exclusion` - Optional List of domains to exclude from proxying.

        **Examples**

        ```python
        import rnet

        proxy = rnet.Proxy.https("https://proxy.example.com")
        ```
        """

    @staticmethod
    def all(url: str, **kwargs: Unpack[ProxyParams]) -> Proxy:
        """
        Creates a new proxy for all protocols.

        This method sets up a proxy server for all types of requests (HTTP, HTTPS, etc.).

        **Arguments**

        * `url` - The URL of the proxy server.
        * `username` - Optional username for proxy authentication.
        * `password` - Optional password for proxy authentication.
        * `custom_http_auth` - Optional custom HTTP proxy authentication header value.
        * `custom_http_headers` - Optional custom HTTP proxy headers.
        * `exclusion` - Optional List of domains to exclude from proxying.

        **Examples**

        ```python
        import rnet

        proxy = rnet.Proxy.all("https://proxy.example.com")
        ```
        """

class SocketAddr:
    """
    A IP socket address.
    """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def ip(self) -> ipaddress.IPv4Address | ipaddress.IPv6Address:
        """
        Returns the IP address of the socket address.
        """

    def port(self) -> int:
        """
        Returns the port number of the socket address.
        """

class StatusCode:
    """
    HTTP status code.
    """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def as_int(self) -> int:
        """
        Return the status code as an integer.
        """

    def is_informational(self) -> bool:
        """
        Check if status is within 100-199.
        """

    def is_success(self) -> bool:
        """
        Check if status is within 200-299.
        """

    def is_redirection(self) -> bool:
        """
        Check if status is within 300-399.
        """

    def is_client_error(self) -> bool:
        """
        Check if status is within 400-499.
        """

    def is_server_error(self) -> bool:
        """
        Check if status is within 500-599.
        """

class ImpersonateOption:
    """
        Create a new impersonation option instance.

        This class allows you to configure browser/client impersonation settings
        including the browser type, operating system, and HTTP protocol options.

        **Args:**
            impersonate (Impersonate): The browser/client type to impersonate
            impersonate_os (Optional[ImpersonateOS]): The operating system to impersonate, defaults to None
            skip_http2 (Optional[bool]): Whether to disable HTTP/2 support, defaults to False
            skip_headers (Optional[bool]): Whether to skip default request headers, defaults to False

        **Returns:**
            ImpersonateOption: A new impersonation option instance

        **Examples:**
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
        """
    def __new__(
        cls,
        impersonate: Impersonate,
        impersonate_os: Optional[ImpersonateOS] = None,
        skip_http2: Optional[bool] = None,
        skip_headers: Optional[bool] = None,
    ) -> ImpersonateOption:...

    @staticmethod
    def random() -> ImpersonateOption:
        """
        Creates a new random impersonation option instance.

        This method generates a random browser/client impersonation option
        with random settings for browser type and operating system options.
        """

class Impersonate(Enum):
    """
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

class ImpersonateOS(Enum):
    """
    An impersonate operating system.
    """

    Windows = auto()
    MacOS = auto()
    Linux = auto()
    Android = auto()
    IOS = auto()

class LookupIpStrategy(Enum):
    """
    The lookup ip strategy.
    """

    Ipv4Only = auto()
    Ipv6Only = auto()
    Ipv4AndIpv6 = auto()
    Ipv6thenIpv4 = auto()
    Ipv4thenIpv6 = auto()

class Method(Enum):
    """
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

class SameSite(Enum):
    """
    The Cookie SameSite attribute.
    """

    Strict = auto()
    Lax = auto()
    Empty = auto()

class TlsVersion(Enum):
    """
    The TLS version.
    """

    TLS_1_0 = auto()
    TLS_1_1 = auto()
    TLS_1_2 = auto()
    TLS_1_3 = auto()

class Version(Enum):
    """
    An HTTP version.
    """

    HTTP_09 = auto()
    HTTP_10 = auto()
    HTTP_11 = auto()
    HTTP_2 = auto()
    HTTP_3 = auto()

class DNSResolverError(RuntimeError):
    """
    An error occurred while resolving a DNS name.
    """

class BodyError(Exception):
    """
    An error occurred while processing the body of a request or response.
    """

class BuilderError(Exception):
    """
    An error occurred while building a request or response.
    """

class ConnectionError(Exception):
    """
    An error occurred while establishing a connection.
    """

class ConnectionResetError(Exception):
    """
    The connection was reset.
    """

class DecodingError(Exception):
    """
    An error occurred while decoding a response.
    """

class RedirectError(Exception):
    """
    An error occurred while following a redirect.
    """

class TimeoutError(Exception):
    """
    A timeout occurred while waiting for a response.
    """

class StatusError(Exception):
    """
    An error occurred while processing the status code of a response.
    """

class RequestError(Exception):
    """
    An error occurred while making a request.
    """

class UpgradeError(Exception):
    """
    An error occurred while upgrading a connection.
    """

class URLParseError(Exception):
    """
    An error occurred while parsing a URL.
    """

class MIMEParseError(Exception):
    """
    An error occurred while parsing a MIME type.
    """

class BaseWebSocketDS:
    """Doc Strings"""

    def recv(self) -> Any:
        """
        Receives a message from the websocket.
        """

    def send(self, message: Message) -> Any:
        """
        Sends a message to the websocket.

        **Arguments**

        * `message` - The message to send.
        """

    def close(self, code: Optional[int] = None, reason: Optional[str] = None) -> Any:
        """
        Closes the websocket connection.

        **Arguments**

        * `code` - An optional close code.
        * `reason` - An optional reason for closing.
        """

class BaseResponseDS:
    """Doc Strings"""

    def text(self) -> Any:
        """
        Returns the text content of the response.
        """
    def text_with_charset(self, encoding: str) -> Any:
        """
        Returns the text content of the response with a specific charset.

        **Arguments**

        * `encoding` - The default encoding to use if the charset is not specified.
        """
    def json(self) -> Any:
        """
        Returns the JSON content of the response.
        """
    def bytes(self) -> Any:
        """
        Returns the bytes content of the response.
        """
    def stream(self) -> Any:
        """
        Convert the response into a `Stream` of `Bytes` from the body.
        """

    def close(self) -> Any:
        """
        Closes the response connection.
        """

class BaseClientDS:
    """Doc String"""
    def request(self, method: Method, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Sends a REQUEST request to the specified url.

        **Examples:**

        ```python
        # Blocking
        import rnet
        from rnet import Method

        def main():
            client = rnet.BlockingClient()
            response = client.request(Method.GET, "https://httpbin.org/anything")
            print(response.text())

        # Async
        import asyncio
        import rnet
        from rnet import Method

        async def main():
            client = rnet.Client()
            response = await client.request(Method.GET, "https://httpbin.org/anything")
            print(await response.text())

        asyncio.run(main())
        ```
        """

    def websocket(self, wsoc: str, **kwargs: Unpack[WebSocketParams]) -> Any:
        """
        Sends a WEBSOCKET request.

        **Examples**

        ```python
        # Blocking
        import asyncio

        def main():
            client = rnet.BlockingClient()
            ws = client.websocket("wss://echo.websocket.org")
            ws.send(rnet.Message.from_text("Hello, WebSocket!"))
            message = ws.recv()
            print("Received:", message.data)
            ws.close()

        # Async
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

    def trace(self, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Sends a TRACE request to the specified url.

        **Examples**

        ```python
        # Blocking
        import rnet

        def main():
            client = rnet.BlockingClient()
            response = client.trace("https://httpbin.org/anything")
            print(response.text())

        # Async
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            response = await client.trace("https://httpbin.org/anything")
        print(await response.text())

        asyncio.run(main())
        ```
        """

    def options(self, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Sends a OPTIONS request to the specified url.

        **Examples**

        ```python
        # Blocking
        import rnet

        def main():
            client = rnet.BlockingClient()
            response = client.options("https://httpbin.org/anything")
            print("Allow: ", response.headers["allow"])
            print("Access control: ", response.headers["access-control-allow-methods"])

        # Async
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            response = await client.options("https://httpbin.org/anything")
            print("Allow: ", response.headers["allow"])
            print("Access control: ", response.headers["access-control-allow-methods"])

        asyncio.run(main())
        ```
        """

    def head(self, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Sends a HEAD request to the specified url.

        **Examples**

        ```python
        # Blocking
        import rnet

        def main():
            client = rnet.BlockingClient()
            response = client.head("https://httpbin.org/anything")
            print(response.headers)

        # Async
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            response = await client.head("https://httpbin.org/anything")
            print(response.headers)

        asyncio.run(main())

        ```
        """

    def post(self, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Sends a POST request to the specified url.

        **Examples**

        ```python
        # Blocking
        import rnet

        async def main():
            client = rnet.BlockingClient()
            response = client.post("https://httpbin.org/post", json={"key": "value"})
            print(response.text())

        # Async
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            response = await client.post("https://httpbin.org/post", json={"key": "value"})
            print(await response.text())

        asyncio.run(main())
        ```
        """

    def put(self, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Sends a PUT request to the specified url.

        **Examples**

        ```python
        # Blocking
        import rnet

        def main():
            client = rnet.BlockingClient()
            response = client.put("https://httpbin.org/put", json={"key": "value"})
            print(response.text())

        # Async
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            response = await client.put("https://httpbin.org/put", json={"key": "value"})
            print(await response.text())

        asyncio.run(main())

        ```
        """

    def delete(self, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Sends a DELETE request to the specified url.

        **Examples**

        ```python
        # Blocking
        import rnet

        def main()
        client = rnet.BlockingClient
        response = client.delete("https://httpbin.org/delete")
        print(response.text())
        print("Status Code: ", response.status_code)

        # Async
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            response = await client.delete("https://httpbin.org/delete")
            print(await response.text())
            print("Status Code: ", response.status_code)

        asyncio.run(main())
        ```
        """

    def patch(self, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Send a PATCH  request to the specified url.

        **Examples**

        ```python
        # Blocking
        import rnet

        def main():
            client = rnet.BlockingClient()
            response = client.patch("https://httpbin.org/patch", json={"key": "value"})
            print(response.text())

        # Async
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            response = await client.patch("https://httpbin.org/patch", json={"key": "value"})
            print(await response.text())

        asyncio.run(main())

        ```
        """

    def get(self, url: str, **kwargs: Unpack[RequestParams]) -> Any:
        """
        Send a GET request to the specified url.

        **Example:**

        ```python
        # Blocking
        import rnet

        def main():
            client = rnet.BlockingClient()
            response = client.get("https://httpbin.org/get")
            print(response.text())

        # Async
        import rnet
        import asyncio

        async def main():
            client = rnet.Client()
            response = await client.get("https://httpbin.org/get")
            print(await response.text())

        asyncio.run(main())
        ```
        """

async def delete(url: str, **kwargs: Unpack[RequestParams]) -> Response:
    """
    Shortcut method to quickly make a request.

    **Examples**

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

async def get(url: str, **kwargs: Unpack[RequestParams]) -> Response:
    """
    Shortcut method to quickly make a request.

    **Examples**

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

async def head(url: str, **kwargs: Unpack[RequestParams]) -> Response:
    """
    Shortcut method to quickly make a request.

    **Examples**

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.head("https://httpbin.org/anything")
        print(response.status)

    asyncio.run(run())
    ```
    """

async def options(url: str, **kwargs: Unpack[RequestParams]) -> Response:
    """
    Shortcut method to quickly make a request.

    **Examples**

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.options("https://httpbin.org/anything")
        print(response.status)

    asyncio.run(run())
    ```
    """

async def patch(url: str, **kwargs: Unpack[RequestParams]) -> Response:
    """
    Shortcut method to quickly make a request.

    **Examples**

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

async def post(url: str, **kwargs: Unpack[RequestParams]) -> Response:
    """
    Shortcut method to quickly make a request.

    **Examples**

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

async def put(url: str, **kwargs: Unpack[RequestParams]) -> Response:
    """
    Shortcut method to quickly make a request.

    **Examples**

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
    method: Method, url: str, **kwargs: Unpack[RequestParams]
) -> Response:
    """
    Make a request with the given parameters.

    **Arguments**

    * `method` - The method to use for the request.
    * `url` - The URL to send the request to.
    * `**kwargs` - Additional request parameters.

    **Examples**

    ```python
    import rnet
    import asyncio


    async def run():
        response = await rnet.request(Method.GET, "https://www.rust-lang.org")
        body = await response.text()
        print(body)

    asyncio.run(run())
    ```
    """

async def trace(url: str, **kwargs: Unpack[RequestParams]) -> Response:
    """
    Shortcut method to quickly make a request.

    **Examples**

    ```python
    import rnet
    import asyncio

    async def run():
        response = await rnet.trace("https://httpbin.org/anything")
        print(response.status)

    asyncio.run(run())
    ```
    """

async def websocket(wsoc: str, **kwargs: Unpack[WebSocketParams]) -> WebSocket:
    """
    Make a WebSocket connection with the given parameters.

    **Examples**

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
