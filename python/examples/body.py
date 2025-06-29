import asyncio
import rnet


async def gen():
    for i in range(10):
        await asyncio.sleep(0.1)
        # bytes chunk
        yield bytes(f"Hello {i}\n", "utf-8")
        # str chunk
        # yield str("Hello {}\n".format(i)).encode("utf-8")


async def main():
    resp = await rnet.post(
        "https://httpbin.org/anything",
        headers={"Content-Type": "application/x-www-form-urlencoded"},
        body=gen(),
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
