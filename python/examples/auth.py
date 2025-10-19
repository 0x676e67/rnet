import asyncio
import rnet


async def main():
    resp = await rnet.get(
        "https://httpbin.org/anything",
        auth="token",
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
