from pathlib import Path
import asyncio
import rnet
from rnet import Multipart, Part, Impersonate


async def main():
    resp = await rnet.post(
        "https://httpbin.org/anything",
        impersonate=Impersonate.Firefox135,
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
        ),
    )
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers.to_dict())
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    print("Text: ", await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
