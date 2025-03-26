import asyncio
import rnet
from rnet import Proxy


async def main():
    resp = await rnet.post(
        "https://httpbin.org/anything",
        proxy=Proxy.all(
            url="http://127.0.0.1:6152",
            custom_http_headers={
                "user-agent": "rnet",
                "accept": "*/*",
                "accept-encoding": "gzip, deflate, br",
                "x-proxy": "rnet",
            },
        ),
    )
    print("Status Code: ", resp.status_code)
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
