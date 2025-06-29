import rnet
import asyncio
import rnet.exceptions as exceptions

rnet_errors = (
    exceptions.DNSResolverError,
    exceptions.BodyError,
    exceptions.BuilderError,
    exceptions.ConnectionError,
    exceptions.ConnectionResetError,
    exceptions.DecodingError,
    exceptions.RedirectError,
    exceptions.TimeoutError,
    exceptions.StatusError,
    exceptions.RequestError,
    exceptions.UpgradeError,
    exceptions.URLParseError,
    exceptions.MIMEParseError,
)


async def test_bad_builder():
    print("\n--- BuilderError (bad builder) ---")
    try:
        await rnet.get("htt://httpbin.org/status/404")
    except rnet_errors as e:
        print(f"Caught: {type(e).__name__}: {e}")
    except Exception as e:
        print(f"Other error: {type(e).__name__}: {e}")


async def test_timeout_error():
    print("\n--- TimeoutError (timeout) ---")
    try:
        await rnet.get("https://httpbin.org/delay/10", timeout=1)
    except rnet_errors as e:
        print(f"Caught: {type(e).__name__}: {e}")
    except Exception as e:
        print(f"Other error: {type(e).__name__}: {e}")


async def test_connection_error():
    print("\n--- ConnectionError (refused) ---")
    try:
        await rnet.get("http://127.0.0.1:9999")
    except rnet_errors as e:
        print(f"Caught: {type(e).__name__}: {e}")
    except Exception as e:
        print(f"Other error: {type(e).__name__}: {e}")


async def test_urlparse_error():
    print("\n--- URLParseError (bad url) ---")
    try:
        await rnet.get("ht!tp://bad_url")
    except rnet_errors as e:
        print(f"Caught: {type(e).__name__}: {e}")
    except Exception as e:
        print(f"Other error: {type(e).__name__}: {e}")


async def main():
    await test_bad_builder()
    await test_timeout_error()
    await test_connection_error()
    await test_urlparse_error()


if __name__ == "__main__":
    asyncio.run(main())
