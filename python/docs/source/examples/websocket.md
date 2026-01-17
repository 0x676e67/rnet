# WebSocket Examples

## Basic WebSocket Connection

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    
    # Upgrade to WebSocket
    ws = await client.websocket("wss://echo.websocket.org")
    
    # Send message
    await ws.send_text("Hello WebSocket!")
    
    # Receive message
    msg = await ws.recv()
    print(f"Received: {msg}")
    
    # Close connection
    await ws.close()

asyncio.run(main())
```

## WebSocket with Headers

```python
import asyncio
from rnet import Client
from rnet.header import HeaderMap

async def main():
    client = Client()
    
    headers = HeaderMap()
    headers["Authorization"] = "Bearer token"
    
    ws = await client.websocket(
        "wss://example.com/ws",
        headers=headers
    )
    
    await ws.send_text("Authenticated message")
    msg = await ws.recv()
    print(msg)
    
    await ws.close()

asyncio.run(main())
```

## WebSocket Message Loop

```python
import asyncio
from rnet import Client

async def main():
    client = Client()
    ws = await client.websocket("wss://echo.websocket.org")
    
    try:
        # Send multiple messages
        for i in range(5):
            await ws.send_text(f"Message {i}")
            response = await ws.recv()
            print(f"Echo: {response}")
    finally:
        await ws.close()

asyncio.run(main())
```
