import asyncio
import rnet

import asyncio
import rnet
from rnet import Impersonate, Version


async def main():
    resp = await rnet.get(
        "https://mozilla-modern.badssl.com/",
        impersonate=Impersonate.Firefox133,
        danger_accept_invalid_certs=True,
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
