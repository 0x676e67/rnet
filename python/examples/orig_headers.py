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
    print(msg.json())
    await ws.close()


if __name__ == "__main__":
    asyncio.run(main())
