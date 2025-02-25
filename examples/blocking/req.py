from rnet import BlockingClient, Method


def main():
    client = BlockingClient()
    resp = client.request(Method.GET, "https://www.google.com/")
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Cookies: ", resp.cookies["AEC"])
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)


if __name__ == "__main__":
    main()
