import asyncio
import wreq


async def main():
    resp = await wreq.get(
        "https://httpbin.io/anything",
        basic_auth=("username", "password"),
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
