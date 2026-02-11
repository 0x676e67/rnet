from rnet import Proxy
from rnet.blocking import Client
from rnet.emulation import Emulation


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
    print("Remote Address: ", resp.remote_addr)
    print("Text: ", resp.text())


if __name__ == "__main__":
    main()
