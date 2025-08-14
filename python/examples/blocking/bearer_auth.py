from rnet.blocking import Client


def main():
    client = Client()
    resp = client.get(
        "https://httpbin.org/anything",
        bearer_auth="token",
    )
    print(resp.text())


if __name__ == "__main__":
    main()
