from rnet.blocking import Client
from rnet.proxy import Proxy


def main():
    client = Client()
    resp = client.post(
        "https://httpbin.io/anything",
        proxy=Proxy.all("http://127.0.0.1:6152"),
    )
    print(resp.text())


if __name__ == "__main__":
    main()
