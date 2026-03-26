import asyncio
import wreq


async def main():
    resp = await wreq.get(
        "https://httpbin.io/anything",
        bearer_auth="token",
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
