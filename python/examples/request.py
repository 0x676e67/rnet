import asyncio
import rnet
from rnet import Method


async def main():
    resp: rnet.Response = await rnet.request(Method.GET, url="https://www.google.com/")
    print("Status Code: ", resp.status)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    print("Headers set-cookie: ", resp.headers["set-cookie"])

    for key in resp.headers:
        print(key)

    for key, value in resp.headers.items():
        print(f"{key}: {value}")

    for cookie in resp.cookies:
        print(cookie)


if __name__ == "__main__":
    asyncio.run(main())
