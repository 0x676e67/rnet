from rnet import BlockingClient


def main():
    client = BlockingClient()
    resp = client.get(
        "https://tls.peet.ws/api/all",
        timeout=10,
    )
    print(resp.text())


if __name__ == "__main__":
    main()
