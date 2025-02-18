import asyncio
from rnet import Impersonate, Client


async def main():
    client = Client(
        impersonate=Impersonate.Firefox133,
        user_agent="rnet",
        async_dns=True,
    )
    resp = await client.get("https://httpbin.org/stream/20")
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers.to_dict())
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    streamer = resp.stream()
    async for chunk in streamer:
        print("Chunk: ", chunk)
        await asyncio.sleep(0.1)


if __name__ == "__main__":
    asyncio.run(main())
