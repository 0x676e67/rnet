# Blocking/Synchronous API Examples

The blocking API provides synchronous methods for environments where async/await is not needed.

## Simple GET Request

```python
import datetime
from rnet.blocking import Client
from rnet.emulation import Emulation


def main():
    client = Client()
    resp = client.get(
        "https://tls.peet.ws/api/all",
        timeout=datetime.timedelta(seconds=10),
        emulation=Emulation.Firefox139,
    )
    print(resp.text())


if __name__ == "__main__":
    main()
```

## Client Configuration

```python
from rnet import Proxy
from rnet.blocking import Client
from rnet.emulation import Emulation


def main():
    client = Client(
        emulation=Emulation.Firefox133,
        user_agent="rnet",
        proxies=[
            Proxy.http("socks5h://abc:def@127.0.0.1:1080"),
            Proxy.https(url="socks5h://127.0.0.1:1080", username="abc", password="def"),
            Proxy.http(url="http://abc:def@127.0.0.1:1080", custom_http_auth="abcedf"),
            Proxy.all(
                url="socks5h://abc:def@127.0.0.1:1080",
                exclusion="google.com, facebook.com, twitter.com",
            ),
        ],
    )
    resp = client.get("https://api.ip.sb/ip")
    print("Status Code: ", resp.status)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Content-Length: ", resp.content_length)
    print("Remote Address: ", resp.remote_addr)
    print("Text: ", resp.text())


if __name__ == "__main__":
    main()
```

## Cookies

```python
from rnet.blocking import Client, Method


def main():
    client = Client()
    resp = client.request(Method.GET, "https://www.google.com/")
    for resp in resp.cookies:
        print(f"{resp.name}: {resp.value}")


if __name__ == "__main__":
    main()
```

## Authentication

```python
from rnet.blocking import Client


def main():
    # Basic auth
    resp = Client().get(
        "https://httpbin.io/anything",
        basic_auth=("username", "password"),
    )
    print(resp.text())

    # Bearer token
    resp = Client().get(
        "https://httpbin.io/anything",
        bearer_auth="token",
    )
    print(resp.text())


if __name__ == "__main__":
    main()
```

## JSON and Form Data

```python
from rnet.blocking import Client


def main():
    client = Client()

    # JSON request
    resp = client.post(
        "https://httpbin.io/anything",
        json={"key": "value"},
    )
    print(resp.json())

    # Form data
    resp = client.post(
        "https://httpbin.io/anything",
        form={
            "keyA": "valueA",
            "keyB": "valueB",
            "number": 789,
        },
    )
    print(resp.text())


if __name__ == "__main__":
    main()
```

## Query Parameters

```python
from rnet.blocking import Client


def main():
    client = Client()
    resp = client.get(
        "https://httpbin.io/anything",
        query={
            "keyA": "valueA",
            "keyB": "valueB",
            "number": 789,
        },
    )
    print(resp.text())


if __name__ == "__main__":
    main()
```

## Streaming Response

```python
from rnet.blocking import Client


def main():
    client = Client()
    resp = client.get("https://httpbin.io/stream/20")
    with resp:
        with resp.stream() as streamer:
            for chunk in streamer:
                print(chunk)


if __name__ == "__main__":
    main()
```
