# Basic Usage Examples

## Simple GET Request

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    resp = await client.get("https://httpbin.org/get")
    print(f"Status: {resp.status_code}")
    print(f"Body: {await resp.text()}")

asyncio.run(main())
```

## POST Request with JSON

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    data = {"username": "john", "email": "john@example.com"}
    resp = await client.post("https://httpbin.org/post", json=data)
    result = await resp.json()
    print(result)

asyncio.run(main())
```

## Form Data

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    form_data = {"field1": "value1", "field2": "value2"}
    resp = await client.post("https://httpbin.org/post", data=form_data)
    print(await resp.text())

asyncio.run(main())
```

## Custom Headers

```python
import asyncio
from rnet import Client
from rnet.header import HeaderMap

async def main():
    client = Client()
    headers = HeaderMap()
    headers["User-Agent"] = "MyApp/1.0"
    headers["Accept"] = "application/json"
    
    resp = await client.get("https://httpbin.org/headers", headers=headers)
    print(await resp.json())

asyncio.run(main())
```

## Query Parameters

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    params = {"q": "search term", "page": "1"}
    resp = await client.get("https://httpbin.org/get", params=params)
    print(await resp.text())

asyncio.run(main())
```

## Timeout

```python
import asyncio
from rnet import Client

async def main():
    client = Client(timeout=5.0)  # 5 seconds timeout
    try:
        resp = await client.get("https://httpbin.org/delay/10")
        print(await resp.text())
    except Exception as e:
        print(f"Request timeout: {e}")

asyncio.run(main())
```

## Streaming Response

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
