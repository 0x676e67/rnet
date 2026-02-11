# Basic Usage Examples

## Simple GET Request

```python
import asyncio
import rnet
from rnet import Method


async def main():
    resp: rnet.Response = await rnet.request(Method.GET, url="https://www.google.com/")
    print("Status Code: ", resp.status)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Remote Address: ", resp.remote_addr)


if __name__ == "__main__":
    asyncio.run(main())
```

## POST Request with JSON

```python
import asyncio
import rnet


async def main():
    resp = await rnet.post(
        "https://httpbin.io/anything",
        json={"key": "value"},
    )
    print(await resp.json())


if __name__ == "__main__":
    asyncio.run(main())
```

## Form Data

```python
import asyncio
import rnet


async def main():
    client = rnet.Client()

    # use a list of tuples
    resp = await client.post(
        "https://httpbin.io/anything",
        form=[
            ("key1", "value1"),
            ("key2", "value2"),
            ("number", 123),
            ("flag", True),
            ("float", 45.67),
        ],
    )
    print(await resp.text())

    # OR use a dictionary
    resp = await client.post(
        "https://httpbin.io/anything",
        form={
            "keyA": "valueA",
            "keyB": "valueB",
            "number": 789,
            "flag": False,
            "float": 12.34,
        },
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
```

## Custom Headers

```python
from rnet.header import HeaderMap


if __name__ == "__main__":
    headers = HeaderMap()
    # Add Content-Type header
    headers.insert("Content-Type", "application/json")
    # Add Accept header (first value)
    headers.insert("Accept", "application/json")
    # Add Accept header (second value)
    headers.insert("Accept", "text/html")
    # Get all values for 'Accept' header
    print("All Accept:", list(headers.get_all("Accept")))
    # Get the value for 'Content-Type' header
    print("Content-Type:", headers.get("Content-Type"))
```

## Query Parameters

```python
import asyncio
import rnet


async def main():
    # Send list of tuples as query parameters
    resp = await rnet.get(
        "https://httpbin.io/anything",
        query=[
            ("key1", "value1"),
            ("key2", "value2"),
            ("number", 123),
            ("flag", True),
            ("float", 45.67),
        ],
    )
    print(await resp.text())

    # OR send dictionary as query parameters
    resp = await rnet.get(
        "https://httpbin.io/anything",
        query={
            "keyA": "valueA",
            "keyB": "valueB",
            "number": 789,
            "flag": False,
            "float": 12.34,
        },
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
```

## Streaming Response

```python
import asyncio
import rnet
from rnet import Response


async def main():
    resp: Response = await rnet.get("https://httpbin.io/stream/20")
    async with resp:
        async with resp.stream() as streamer:
            async for chunk in streamer:
                print(chunk)
                await asyncio.sleep(0.1)


if __name__ == "__main__":
    asyncio.run(main())
```

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    resp = await client.get("https://httpbin.org/stream/10")
    
    async for chunk in resp.stream():
        print(chunk.decode('utf-8'))

asyncio.run(main())
```
