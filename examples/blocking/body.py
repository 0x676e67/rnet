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
