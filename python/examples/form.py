import asyncio
import rnet


async def main():
    client = rnet.Client()

    # use a list of tuples
    resp = await client.post(
        "https://httpbin.io/anything",
        form=[
            ("key1", "value1"),
            ("key2", "value2"),
            ("number", 123),
            ("flag", True),
            ("float", 45.67),
        ],
    )
    print(await resp.text())

    # OR use a dictionary
    resp = await client.post(
        "https://httpbin.io/anything",
        form={
            "keyA": "valueA",
            "keyB": "valueB",
            "number": 789,
            "flag": False,
            "float": 12.34,
        },
    )
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
