from rnet import BlockingClient


def main():
    client = BlockingClient()
    resp = client.post(
        "https://httpbin.org/anything",
        json={"key": "value"},
    )
    print(resp.text())


if __name__ == "__main__":
    main()
