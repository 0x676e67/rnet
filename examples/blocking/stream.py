from rnet import BlockingClient


def main():
    client = BlockingClient()
    resp = client.get("https://httpbin.org/stream/20")
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers.to_dict())
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)

    with resp.stream() as streamer:
        for chunk in streamer:
            print("Chunk: ", chunk)


if __name__ == "__main__":
    main()
