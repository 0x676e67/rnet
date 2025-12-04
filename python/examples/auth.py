import rnet
import asyncio


async def main():
    client = rnet.Client(emulation=rnet.Emulation.OkHttp3_11)
    response: rnet.Response = await client.get(url="https://www.google.com")
    print(response)
    # print(await response.text())
        


if __name__ == "__main__":
    asyncio.run(main())