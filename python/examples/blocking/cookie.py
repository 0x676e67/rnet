from rnet import BlockingClient, Method


def main():
    client = BlockingClient()
    resp = client.request(Method.GET, "https://www.google.com/")
    for resp in resp.cookies:
        print(f"{resp.name}: {resp.value}")


if __name__ == "__main__":
    main()
