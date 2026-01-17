# Quick Start

This guide will help you get started with rnet quickly.

## Basic GET Request

### Async

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    resp = await client.get("https://httpbin.org/get")
    print(resp.status_code)
    print(await resp.text())

asyncio.run(main())
```

### Blocking

```python
from rnet.blocking import Client

client = Client()
resp = client.get("https://httpbin.org/get")
print(resp.status_code)
print(resp.text())
```

## POST with JSON

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    data = {"name": "John", "age": 30}
    resp = await client.post("https://httpbin.org/post", json=data)
    result = await resp.json()
    print(result)

asyncio.run(main())
```

## Browser Emulation

Emulate different browsers to bypass detection:

```python
import asyncio
from rnet import Client, Emulation

async def main():
    # Emulate Chrome 120
    client = Client(emulation=Emulation.Chrome120)
    resp = await client.get("https://httpbin.org/get")
    print(await resp.text())

asyncio.run(main())
```

## Using Proxies

```python
import asyncio
from rnet import Client
from rnet.proxy import Proxy

async def main():
    proxy = Proxy.http("http://proxy.example.com:8080")
    client = Client(proxy=proxy)
    resp = await client.get("https://httpbin.org/ip")
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
    headers["Custom-Header"] = "value"
    
    resp = await client.get("https://httpbin.org/headers", headers=headers)
    print(await resp.text())

asyncio.run(main())
```

## Next Steps

- Explore the [API Reference](../api/core.md) for detailed documentation
- Check out [Examples](../examples/basic.md) for more code samples
- Learn about [Browser Emulation](../examples/emulation.md) for advanced use cases
