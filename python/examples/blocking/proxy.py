from rnet import BlockingClient


def main():
    client = BlockingClient()
    resp = client.post(
        "https://httpbin.org/anything",
        proxy="http://127.0.0.1:6152",
    )
    print(resp.text())


if __name__ == "__main__":
    main()
