from rnet.blocking import Client


def main():
    client = Client()
    resp = client.get(
        "https://httpbin.org/anything",
        basic_auth=("username", "password"),
    )
    print(resp.text())


if __name__ == "__main__":
    main()
