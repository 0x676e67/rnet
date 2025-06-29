from rnet import Impersonate
from rnet import BlockingClient


def main():
    client = BlockingClient(
        impersonate=Impersonate.Firefox135,
        user_agent="rnet",
    )
    with client.get("https://tls.peet.ws/api/all") as resp:
        print("Status Code: ", resp.status_code)
        print("Version: ", resp.version)
        print("Response URL: ", resp.url)
        print("Headers: ", resp.headers)
        print("Encoding: ", resp.encoding)
        print("Content-Length: ", resp.content_length)
        print("Remote Address: ", resp.remote_addr)
        print("Text: ", resp.text())


if __name__ == "__main__":
    main()
