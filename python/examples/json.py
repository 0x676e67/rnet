import asyncio
import rnet


async def main():
    resp = await rnet.post(
        "https://httpbin.org/anything",
        json={"key": "value"},
    )
    print(await resp.json())


if __name__ == "__main__":
    asyncio.run(main())
