from rnet.blocking import Client
from rnet.browser import Browser


def main():
    client = Client()
    resp = client.get(
        "https://tls.peet.ws/api/all",
        timeout=10,
        emulation=Browser.Firefox139,
    )
    print(resp.text())


if __name__ == "__main__":
    main()
