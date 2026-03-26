import asyncio
import wreq


async def main():
    resp = await wreq.post(
        "https://httpbin.io/anything",
        json={"key": "value"},
    )
    print(await resp.json())


if __name__ == "__main__":
    asyncio.run(main())
