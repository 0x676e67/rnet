import asyncio
from rnet import Impersonate, ImpersonateOS, ImpersonateOption, Client, Response


async def print_response_info(resp: Response):
    """Helper function to print response details

    Args:
        resp: Response object from the request
    """
    async with resp:
        print("\n=== Response Information ===")
        print(f"Status Code: {resp.status_code}")
        print(f"Version: {resp.version}")
        print(f"Response URL: {resp.url}")
        print(f"Headers: {resp.headers}")
        print(f"Encoding: {resp.encoding}")
        print(f"Content-Length: {resp.content_length}")
        print(f"Remote Address: {resp.remote_addr}")
        print(f"Peer Certificate: {resp.peer_certificate()}")
        print(f"Content: {await resp.text()}")
        print("========================\n")


async def request_firefox():
    """Test request using Firefox browser impersonation

    Demonstrates basic browser impersonation with custom header order
    """
    print("\n[Testing Firefox Impersonation]")
    client = Client(
        impersonate=Impersonate.Firefox135,
        headers_order=["accept-encoding", "user-agent", "accept"],
        tls_info=True,
    )
    resp = await client.get("https://tls.peet.ws/api/all")
    await print_response_info(resp)
    return client


async def request_chrome_android(client: Client):
    """Test request using Chrome on Android impersonation

    Demonstrates advanced impersonation with OS specification

    Args:
        client: Existing client instance to update
    """
    print("\n[Testing Chrome on Android Impersonation]")
    client.update(
        impersonate=ImpersonateOption(
            impersonate=Impersonate.Chrome134,
            impersonate_os=ImpersonateOS.Android,
        )
    )
    resp = await client.get("https://tls.peet.ws/api/all")
    await print_response_info(resp)


async def main():
    """Main function to run the impersonation examples

    Demonstrates different browser impersonation scenarios:
    1. Firefox with custom header order
    2. Chrome on Android with OS specification
    """
    # First test with Firefox
    client = await request_firefox()

    # Then update and test with Chrome on Android
    await request_chrome_android(client)


if __name__ == "__main__":
    # Run the async main function
    asyncio.run(main())
