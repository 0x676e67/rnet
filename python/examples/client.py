import asyncio
from rnet import Emulation, Client, Proxy


async def main():
    client = Client(
        emulation=Emulation.Firefox133,
        user_agent="rnet",
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

    resp = await client.get("https://tls.peet.ws/api/all")
    print("Status Code: ", resp.status)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    text = await resp.text()
    print("Text: ", text)


if __name__ == "__main__":
    asyncio.run(main())
