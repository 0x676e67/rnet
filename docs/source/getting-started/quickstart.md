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

The [Proxy](../guide/proxy.md) object defines how your HTTP client routes traffic through a proxy server.
You create one using a named constructor that specifies the scope:

- `Proxy.all(url)` — routes all traffic through the given proxy
- `Proxy.http(url)` - only intercepts HTTP requests
- `Proxy.https(url)` — only intercepts HTTPS requests

Once created, you pass it to the `Client` and all requests will go through it automatically.

```python
import asyncio
from wreq import Client, Proxy

async def main():
    client = Client(proxies=[Proxy.all("http://proxy.example.com:8080")])
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
