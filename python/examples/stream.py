import asyncio
import rnet
from rnet import Response


async def main():
    resp: Response = await rnet.get("https://httpbin.org/stream/20")
    async with resp:
        async with resp.stream() as streamer:
            async for chunk in streamer:
                print("Chunk: ", chunk)
                await asyncio.sleep(0.1)


if __name__ == "__main__":
    asyncio.run(main())
