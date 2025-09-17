import asyncio
from rnet import Client
from rnet.tls import KeyLog
from rnet.browser import Browser


async def main():
    client = Client(
        emulation=Browser.Firefox139,
        keylog=KeyLog.file("keylog.log"),
    )

    resp = await client.get("https://www.google.com")
    async with resp:
        print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
