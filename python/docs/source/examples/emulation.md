# Browser Emulation Examples

## Chrome Emulation

```python
import asyncio
from rnet import Client, Emulation

async def main():
    # Emulate Chrome 120
    client = Client(emulation=Emulation.Chrome120)
    resp = await client.get("https://httpbin.org/headers")
    print(await resp.json())

asyncio.run(main())
```

## Safari Emulation

```python
import asyncio
from rnet import Client, Emulation

async def main():
    # Emulate Safari 17 on macOS
    client = Client(emulation=Emulation.Safari17_0)
    resp = await client.get("https://httpbin.org/user-agent")
    print(await resp.text())

asyncio.run(main())
```

## Firefox Emulation

```python
import asyncio
from rnet import Client, Emulation

async def main():
    # Emulate Firefox 120
    client = Client(emulation=Emulation.Firefox120)
    resp = await client.get("https://httpbin.org/get")
    print(await resp.json())

asyncio.run(main())
```

## Edge Emulation

```python
import asyncio
from rnet import Client, Emulation

async def main():
    # Emulate Microsoft Edge
    client = Client(emulation=Emulation.Edge120)
    resp = await client.get("https://httpbin.org/headers")
    print(await resp.json())

asyncio.run(main())
```

## Mobile Browser Emulation

```python
import asyncio
from rnet import Client, Emulation

async def main():
    # Emulate Chrome on Android
    client = Client(emulation=Emulation.Chrome120)
    # Note: Use appropriate user-agent for mobile
    resp = await client.get("https://httpbin.org/user-agent")
    print(await resp.text())

asyncio.run(main())
```
