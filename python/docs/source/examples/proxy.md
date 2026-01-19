# Proxy Examples

## HTTP/HTTPS Proxy

Using proxies with authentication:

```python
import asyncio
import rnet
from rnet import Client, Proxy


async def main():
    # Create a client with multiple proxies
    client = Client(
        proxies=[Proxy.http("socks5h://abc:def@127.0.0.1:6152")],
    )

    # Send request via the client proxy
    resp = await client.get("https://httpbin.io/anything")
    print(await resp.text())

    # Send request via custom proxy
    resp = await rnet.get(
        "https://httpbin.io/anything",
        proxy=Proxy.all(
            url="http://127.0.0.1:6152",
            custom_http_headers={
                "user-agent": "rnet",
                "accept": "*/*",
                "accept-encoding": "gzip, deflate, br",
                "x-proxy": "rnet",
            },
        ),
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
```

## Unix Socket Proxy

Using Unix sockets for local services like Docker:

```python
import asyncio
import rnet
from rnet import Proxy


async def main():
    # Send request via Unix socket proxy
    resp = await rnet.get(
        "http://localhost/v1.41/containers/json",
        proxy=Proxy.unix("/var/run/docker.sock"),
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
```
