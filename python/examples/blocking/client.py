import logging
import colorlog
from rnet import Emulation, Proxy
from rnet.blocking import Client

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


def main():
    client = Client(
        emulation=Emulation.Firefox133,
        user_agent="rnet",
        proxies=[
            Proxy.http("socks5h://abc:def@127.0.0.1:1080"),
            Proxy.https(url="socks5h://127.0.0.1:1080", username="abc", password="def"),
            Proxy.http(url="http://abc:def@127.0.0.1:1080", custom_http_auth="abcedf"),
            Proxy.all(
                url="socks5h://abc:def@127.0.0.1:1080",
                exclusion="google.com, facebook.com, twitter.com",
            ),
        ],
    )
    resp = client.get("https://api.ip.sb/ip")
    print("Status Code: ", resp.status)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    print("Text: ", resp.text())


if __name__ == "__main__":
    main()
