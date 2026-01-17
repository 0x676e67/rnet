# Authentication Examples

## Basic Authentication

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    resp = await client.get(
        "https://httpbin.org/basic-auth/user/pass",
        auth=("user", "pass")
    )
    print(await resp.json())

asyncio.run(main())
```

## Bearer Token

```python
import asyncio
from rnet import Client
from rnet.header import HeaderMap

async def main():
    client = Client()
    headers = HeaderMap()
    headers["Authorization"] = "Bearer YOUR_TOKEN_HERE"
    
    resp = await client.get(
        "https://api.example.com/user",
        headers=headers
    )
    print(await resp.json())

asyncio.run(main())
```

## API Key Authentication

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    headers = {"X-API-Key": "your-api-key"}
    
    resp = await client.get(
        "https://api.example.com/data",
        headers=headers
    )
    print(await resp.json())

asyncio.run(main())
```

## Session with Cookies

```python
import asyncio
from rnet import Client
from rnet.cookie import Jar

async def main():
    jar = Jar()
    client = Client(cookie_store=jar)
    
    # Login request - cookies will be stored
    login_data = {"username": "user", "password": "pass"}
    await client.post("https://example.com/login", json=login_data)
    
    # Subsequent requests will use stored cookies
    resp = await client.get("https://example.com/protected")
    print(await resp.text())

asyncio.run(main())
```
