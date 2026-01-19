# Advanced Examples

## Streaming Request Body

Send data using async generators for streaming uploads:

```python
import asyncio
import rnet


async def gen():
    for i in range(10):
        await asyncio.sleep(0.1)

        if i <= 5:
            # bytes chunk
            yield bytes(f"Hello {i}\n", "utf-8")
        else:
            # str chunk
            yield str("Hello {}\n".format(i)).encode("utf-8")


async def main():
    resp = await rnet.post(
        "https://httpbin.io/anything",
        body=gen(),
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
```

## Multipart File Upload

Upload multiple files and data parts:

```python
from pathlib import Path
import asyncio
import aiofiles
import rnet
from rnet import Multipart, Part


async def file_to_bytes_stream(file_path):
    async with aiofiles.open(file_path, "rb") as f:
        while chunk := await f.read(1024):
            yield chunk


async def main():
    resp = await rnet.post(
        "https://httpbin.io/anything",
        multipart=Multipart(
            # Upload text data
            Part(name="def", value="111", filename="def.txt", mime="text/plain"),
            # Upload binary data
            Part(name="abc", value=b"000", filename="abc.txt", mime="text/plain"),
            # Upload file data
            Part(
                name="LICENSE",
                value=Path("LICENSE"),
                filename="LICENSE",
                mime="text/plain",
            ),
            # Upload bytes stream file data
            Part(
                name="README",
                value=file_to_bytes_stream("README.md"),
                filename="README.md",
                mime="text/plain",
            ),
        ),
    )

    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
```

## TLS Key Logging

Capture TLS keys for debugging with tools like Wireshark:

```python
import asyncio
from rnet import Client
from rnet.tls import KeyLog


async def main():
    client = Client(keylog=KeyLog.file("keylog.log"))
    resp = await client.get("https://www.google.com")
    async with resp:
        print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
```

## Original Header Order Preservation

Preserve header case and order for specific sites:

```python
import asyncio
import rnet
from rnet.emulation import Emulation


async def main():
    ws = await rnet.websocket(
        "wss://gateway.discord.gg/",
        emulation=Emulation.Chrome137,
        headers={"Origin": "https://discord.com"},
        # Preserve HTTP/1 case and header order
        orig_headers=[
            "User-Agent",
            "Origin",
            "Host",
            "Accept",
            "Accept-Encoding",
            "Accept-Language",
        ],
    )

    msg = await ws.recv()
    if msg is not None:
        print(msg.json())
    await ws.close()


if __name__ == "__main__":
    asyncio.run(main())
```
