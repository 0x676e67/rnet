import datetime
from . import (
    Message,
    Streamer,
    Version,
    Method,
    SocketAddr,
    StatusCode,
    ClientConfig,
    Request,
    WebSocketRequest,
)
from typing import (
    Any,
    Sequence,
    Unpack,
)
from .redirect import History
from .header import HeaderMap
from .cookie import Cookie


class Response:
    r"""
    A blocking response from a request.
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

    cookies: Sequence[Cookie]
    r"""
    Get the cookies of the response.
    """

    content_length: int | None
    r"""
    Get the content length of the response.
    """

    remote_addr: SocketAddr | None
    r"""
    Get the remote address of the response.
    """

    local_addr: SocketAddr | None
    r"""
    Get the local address of the response.
    """

    history: Sequence[History]
    r"""
    Get the redirect history of the Response.
    """

    peer_certificate: bytes | None
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
        ...

    def text(self) -> str:
        r"""
        Get the text content of the response.
        """
        ...

    def text_with_charset(self, encoding: str) -> str:
        r"""
        Get the full response text given a specific encoding.
        """
        ...

    def json(self) -> Any:
        r"""
        Get the JSON content of the response.
        """

    def bytes(self) -> bytes:
        r"""
        Get the bytes content of the response.
        """
        ...

    def close(self) -> None:
        r"""
        Close the response connection.
        """

    def __enter__(self) -> "Response": ...
    def __exit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> None: ...
    def __str__(self) -> str: ...


class WebSocket:
    r"""
    A blocking WebSocket response.
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

    cookies: Sequence[Cookie]
    r"""
    Get the cookies of the response.
    """

    remote_addr: SocketAddr | None
    r"""
    Get the remote address of the response.
    """

    protocol: str | None
    r"""
    Get the WebSocket protocol.
    """

    def recv(self, timeout: datetime.timedelta | None = None) -> Message | None:
        r"""
        Receive a message from the WebSocket.
        """

    def send(self, message: Message) -> None:
        r"""
        Send a message to the WebSocket.

        # Arguments

        * `message` - The message to send.
        """

    def send_all(self, messages: Sequence[Message]) -> None:
        r"""
        Send multiple messages to the WebSocket.

        # Arguments

        * `messages` - The sequence of messages to send.
        """

    def close(
        self,
        code: int | None = None,
        reason: str | None = None,
    ) -> None:
        r"""
        Close the WebSocket connection.

        # Arguments

        * `code` - An optional close code.
        * `reason` - An optional reason for closing.
        """

    def __enter__(self) -> "WebSocket": ...
    def __exit__(self, _exc_type: Any, _exc_value: Any, _traceback: Any) -> None: ...
    def __str__(self) -> str: ...


class Client:
    r"""
    A blocking client for making HTTP requests.
    """

    def __init__(
        self,
        **kwargs: Unpack[ClientConfig],
    ) -> None:
        r"""
        Creates a new blocking Client instance.

        # Examples

        ```python
        import asyncio
        import rnet

        client = rnet.blocking.Client(
            user_agent="Mozilla/5.0",
            timeout=10,
        )
        response = client.get('https://httpbin.io/get')
        print(response.text())
        ```
        """
        ...

    def request(
        self,
        method: Method,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given method and URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        client = rnet.blocking.Client()
        response = client.request(Method.GET, "https://httpbin.io/anything")
        ```
        """
        ...

    def websocket(self, url: str, **kwargs: Unpack[WebSocketRequest]) -> "WebSocket":
        r"""
        Sends a WebSocket request.

        # Examples

        ```python
        import rnet

        client = rnet.blocking.Client()
        ws = client.websocket("wss://echo.websocket.org")
        ws.send(rnet.Message.from_text("Hello, WebSocket!"))
        message = ws.recv()
        print("Received:", message.data)
        ws.close()
        ```
        """
        ...

    def trace(
        self,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        client = rnet.blocking.Client()
        response = client.trace("https://httpbin.io/anything")
        print(response.text())
        ```
        """
        ...

    def options(
        self,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        client = rnet.blocking.Client()
        response = client.options("https://httpbin.io/anything")
        print(response.text())
        ```
        """
        ...

    def head(
        self,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        def main():
        client = rnet.blocking.Client()
        response = client.head("https://httpbin.io/anything")
        print(response.text())
        ```
        """
        ...

    def delete(
        self,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        client = rnet.blocking.Client()
        response = client.delete("https://httpbin.io/anything")
        print(response.text())
        ```
        """
        ...

    def patch(
        self,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        client = rnet.blocking.Client()
        response = client.patch("https://httpbin.io/anything", json={"key": "value"})
        print(response.text())
        ```
        """
        ...

    def put(
        self,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        client = rnet.blocking.Client()
        response = client.put("https://httpbin.io/anything", json={"key": "value"})
        print(response.text())
        ```
        """
        ...

    def post(
        self,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        client = rnet.blocking.Client()
        response = client.post("https://httpbin.io/anything", json={"key": "value"})
        print(response.text())
        ```
        """
        ...

    def get(
        self,
        url: str,
        **kwargs: Unpack[Request],
    ) -> "Response":
        r"""
        Sends a request with the given URL.

        # Examples

        ```python
        import rnet
        from rnet import Method

        client = rnet.blocking.Client()
        response = client.get("https://httpbin.io/anything")
        print(response.text())
        ```
        """
        ...
