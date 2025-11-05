import asyncio
from ipaddress import IPv4Address
from rnet import Client
from rnet.exceptions import ConnectionError


async def main():
    client = Client(
        resolve_to_addrs={
            "www.google.com": [IPv4Address("31.13.94.49")],
        },
    )

    try:
        resp = await client.get("https://www.google.com")
        text = await resp.text()
        print("Text: ", text)
    except ConnectionError as e:
        print("Connection error:", e)


if __name__ == "__main__":
    asyncio.run(main())
