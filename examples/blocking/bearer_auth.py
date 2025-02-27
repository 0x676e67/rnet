from rnet import BlockingClient


def main():
    client = BlockingClient()
    resp = client.get(
        "https://httpbin.org/anything",
        bearer_auth="token",
    )
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    print("Text: ", resp.text())


if __name__ == "__main__":
    main()
