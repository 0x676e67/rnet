import asyncio
import rnet
from rnet.emulation import Emulation


async def main():
    resp = await rnet.get(
        "https://tls.peet.ws/api/all",
        timeout=10,
        emulation=Emulation.Firefox139,
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
