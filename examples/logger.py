import asyncio
import logging
import colorlog
from rnet import Impersonate, Client


formatter = colorlog.ColoredFormatter(
    "%(log_color)s%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s",
    datefmt=None,
    reset=True,
    log_colors={
        "DEBUG": "cyan",
        "INFO": "green",
        "WARNING": "yellow",
        "ERROR": "red",
        "CRITICAL": "red,bg_white",
    },
    secondary_log_colors={},
    style="%",
)

handler = logging.StreamHandler()
handler.setFormatter(formatter)
logger = colorlog.getLogger()
logger.addHandler(handler)
logger.setLevel(logging.DEBUG)


async def main():
    client = Client(impersonate=Impersonate.Firefox133, user_agent="rnet")
    resp = await client.get("https://httpbin.org/stream/20")
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    streamer = resp.stream()
    async for chunk in streamer:
        print("Chunk: ", chunk)
        await asyncio.sleep(0.1)


if __name__ == "__main__":
    asyncio.run(main())
