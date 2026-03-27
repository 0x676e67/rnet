# :zap: Quick Start

This page will help you get up and running with wreq in minutes.

---

## Basic GET Request

=== "Async"
    ```python
    import asyncio
    from wreq import Client

    async def main():
        client = Client()
        resp = await client.get("https://httpbin.org/get")
        print(resp.status_code)
        print(await resp.text())

    asyncio.run(main())
    ```
=== "Blocking"
    ```python
    from wreq.blocking import Client

    client = Client()
    resp = client.get("https://httpbin.org/get")
    print(resp.status_code)
    print(resp.text())
    ```

---

## POST with JSON

```python
import asyncio
from wreq import Client

async def main():
    client = Client()
    data = {"name": "John", "age": 30}
    resp = await client.post("https://httpbin.org/post", json=data)
    result = await resp.json()
    print(result)

asyncio.run(main())
```

---

## Emulation

Emulate different browsers to bypass detection:

```python
import asyncio
from wreq import Client, Emulation

async def main():
    client = Client(emulation=Emulation.Safari26)
    resp = await client.get("https://tls.peet.ws/api/all")
    print(await resp.text())

asyncio.run(main())
```

---

## Using Proxies

```python
import asyncio
from wreq import Client
from wreq.proxy import Proxy

async def main():
    client = Client(proxy=Proxy.all("http://proxy.example.com:8080"))
    resp = await client.get("https://httpbin.org/ip")
    print(await resp.text())

asyncio.run(main())
```

---

## Custom Headers

```python
import asyncio
from wreq import Client
from wreq.header import HeaderMap

async def main():
    client = Client()
    headers = HeaderMap()
    headers["User-Agent"] = "MyApp/1.0"
    headers["Custom-Header"] = "value"
    resp = await client.get("https://httpbin.org/headers", headers=headers)
    print(await resp.text())

asyncio.run(main())
```

---

## Next Steps

- See the [Examples](../guide/basic.md) for more code samples
- Explore the [API Reference](../api/wreq.md) for detailed documentation