from pathlib import Path
from rnet import BlockingClient, Multipart, Part


def file_to_bytes_stream(file_path):
    with open(file_path, "rb") as f:
        while chunk := f.read(1024):
            yield chunk


def main():
    client = BlockingClient()
    resp = client.post(
        "https://httpbin.org/anything",
        multipart=Multipart(
            # Upload text data
            Part(name="def", value="111", filename="def.txt", mime="text/plain"),
            # Upload binary data
            Part(name="abc", value=b"000", filename="abc.txt", mime="text/plain"),
            # Uoload file data
            Part(
                name="LICENSE",
                value=Path("./LICENSE"),
                filename="LICENSE",
                mime="text/plain",
            ),
            # Upload bytes stream file data
            Part(
                name="README",
                value=file_to_bytes_stream("./README.md"),
                filename="README.md",
                mime="text/plain",
            ),
        ),
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
