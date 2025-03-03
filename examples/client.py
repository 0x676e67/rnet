import asyncio
import logging
import colorlog
from rnet import Impersonate, Client, Proxy


formatter = colorlog.ColoredFormatter(
    "%(log_color)s%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s",
    datefmt=None,
    reset=True,
    log_colors={
        "DEBUG": "cyan",
        "INFO": "green",
        "WARNING": "yellow",
        "ERROR": "red",
        "CRITICAL": "red,bg_white",
    },
    secondary_log_colors={},
    style="%",
)

handler = logging.StreamHandler()
handler.setFormatter(formatter)
logger = colorlog.getLogger()
logger.addHandler(handler)
logger.setLevel(logging.DEBUG)


async def main():
    client = Client(
        impersonate=Impersonate.Firefox133,
        user_agent="rnet",
        proxies=[
            Proxy.http("socks5h://abc:def@127.0.0.1:6152"),
            Proxy.https(url="socks5h://127.0.0.1:6153", username="abc", password="def"),
            Proxy.http(url="http://abc:def@127.0.0.1:6152", custom_http_auth="abcedf"),
            Proxy.all(
                url="socks5h://abc:def@127.0.0.1:6153",
                exclusion="google.com, facebook.com, twitter.com",
            ),
        ],
    )
    resp = await client.get("https://api.ip.sb/ip")
    print("Status Code: ", resp.status_code)
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
