import asyncio
import rnet


async def main():
    resp = await rnet.get(
        "https://httpbin.org/anything",
        auth="token",
    )
    print("Status Code: ", resp.status)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    print("Text: ", await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
