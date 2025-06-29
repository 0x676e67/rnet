import asyncio
import rnet


async def main():
    resp = await rnet.get(
        "https://tls.peet.ws/api/all",
        timeout=10,
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
