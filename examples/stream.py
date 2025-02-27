import asyncio
import rnet


async def main():
    async with await rnet.get("https://httpbin.org/stream/20") as resp:
        print("Status Code: ", resp.status_code)
        print("Version: ", resp.version)
        print("Response URL: ", resp.url)
        print("Headers: ", resp.headers)
        print("Content-Length: ", resp.content_length)
        print("Encoding: ", resp.encoding)
        print("Remote Address: ", resp.remote_addr)

        async with resp.stream() as streamer:
            async for chunk in streamer:
                print("Chunk: ", chunk)
                await asyncio.sleep(0.1)


if __name__ == "__main__":
    asyncio.run(main())
