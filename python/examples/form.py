import asyncio
import rnet


async def main():
    client = rnet.Client()

    # use a list of tuples
    resp = await client.post(
        "https://httpbin.org/anything",
        form=[("key", "value")],
    )
    print(await resp.text())

    # OR use a dictionary
    resp = await client.post(
        "https://httpbin.org/anything",
        form={
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
            "key4": "value4",
            "key5": "value5",
        },
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
