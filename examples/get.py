import asyncio
import rnet


async def main():
    resp = await rnet.get(
        "https://tls.peet.ws/api/all",
        timeout=10,
    )
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)

    text_content = await resp.text()
    print("Text: ", text_content)


if __name__ == "__main__":
    asyncio.run(main())
