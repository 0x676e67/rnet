from rnet import BlockingClient


def gen():
    for i in range(10):
        yield i.to_bytes()


def main():
    client = BlockingClient()
    resp = client.post(
        "https://httpbin.org/anything",
        headers={"Content-Type": "application/x-www-form-urlencoded"},
        body=gen(),
    )
    print(resp.json())


if __name__ == "__main__":
    main()
