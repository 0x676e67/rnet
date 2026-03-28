import asyncio
from wreq import Client
from wreq.tls import KeyLog


async def main():
    client = Client(tls_keylog=KeyLog.file("keylog.log"))
    resp = await client.get("https://www.google.com")
    async with resp:
        print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
