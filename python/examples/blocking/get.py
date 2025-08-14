from rnet.blocking import Client
from rnet.emulation import Emulation


def main():
    client = Client()
    resp = client.get(
        "https://tls.peet.ws/api/all",
        timeout=10,
        emulation=Emulation.Firefox139,
    )
    print(resp.text())


if __name__ == "__main__":
    main()
