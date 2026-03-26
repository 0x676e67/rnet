import datetime
from wreq.blocking import Client
from wreq.emulation import Emulation


def main():
    client = Client()
    resp = client.get(
        "https://tls.peet.ws/api/all",
        timeout=datetime.timedelta(seconds=10),
        emulation=Emulation.Firefox139,
    )
    print(resp.text())


if __name__ == "__main__":
    main()
