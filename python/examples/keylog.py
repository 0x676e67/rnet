import asyncio
from rnet import Client
from rnet.tls import KeyLogPolicy
from rnet.emulation import Emulation


async def main():
    client = Client(
        emulation=Emulation.Firefox139,
        keylog=KeyLogPolicy.file("keylog.log"),
    )

    resp = await client.get("https://www.google.com")
    async with resp:
        print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
