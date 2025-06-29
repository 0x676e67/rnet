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
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
