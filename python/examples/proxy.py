import asyncio
import rnet
from rnet import Client, Proxy
from rnet.emulation import Emulation


async def main():
    # Create a client with multiple proxies
    client = Client(
        proxies=[
            Proxy.http("socks5h://abc:def@127.0.0.1:6152"),
            Proxy.https(url="socks5h://127.0.0.1:6153", username="abc", password="def"),
            Proxy.http(
                url="http://abc:def@127.0.0.1:6152",
                custom_http_auth="abcedf",
                custom_http_headers={"User-Agent": "rnet", "x-custom-header": "value"},
            ),
            Proxy.all(
                url="socks5h://abc:def@127.0.0.1:6153",
                exclusion="google.com, facebook.com, twitter.com",
            ),
        ],
    )

    resp = await client.get("https://httpbin.org/anything")
    print(await resp.text())

    # OR use Proxy directly in request
    resp = await rnet.get(
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
