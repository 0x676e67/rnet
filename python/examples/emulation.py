import asyncio
from rnet import Client, Response
from rnet.browser import Browser, BrowserOS, BrowserOption
from rnet.emulation import TlsOptions, Http2Options, Emulation
from rnet.tls import TlsVersion, AlpnProtocol


async def print_response_info(resp: Response):
    """Helper function to print response details

    Args:
        resp: Response object from the request
    """
    async with resp:
        print("\n=== Response Information ===")
        print(f"Status Code: {resp.status}")
        print(f"Version: {resp.version}")
        print(f"Response URL: {resp.url}")
        print(f"Headers: {resp.headers}")
        print(f"Content-Length: {resp.content_length}")
        print(f"Remote Address: {resp.remote_addr}")
        print(f"Peer Certificate: {resp.peer_certificate}")
        print(f"Content: {await resp.text()}")
        print("========================\n")


async def request_firefox():
    """Test request using Firefox browser Emulation

    Demonstrates basic browser Emulation with custom header order
    """
    print("\n[Testing Firefox Emulation]")
    client = Client(
        emulation=Browser.Firefox135,
        tls_info=True,
    )
    resp = await client.get("https://tls.peet.ws/api/all")
    await print_response_info(resp)
    return client


async def request_chrome_android(client: Client):
    """Test request using Chrome on Android Emulation

    Demonstrates advanced Emulation with OS specification

    Args:
        client: Existing client instance to update
    """
    print("\n[Testing Chrome on Android Emulation]")
    resp = await client.get(
        "https://tls.peet.ws/api/all",
        emulation=BrowserOption(
            emulation=Browser.Chrome134,
            emulation_os=BrowserOS.Android,
        ),
        # Disable client default headers
        default_headers=False,
    )
    await print_response_info(resp)


async def request_custom_emulation(client: Client):
    """Test request using custom TLS and HTTP/2 emulation

    Demonstrates advanced emulation configuration with:
    - Custom TLS options (version, ciphers, ALPN)
    - Custom HTTP/2 options (window size, frame size, etc.)
    - Combined emulation configuration
    """
    print("\n[Testing Custom TLS/HTTP2 Emulation]")

    # Configure TLS options
    tls_opts = TlsOptions(
        min_tls_version=TlsVersion.TLS_1_2,
        max_tls_version=TlsVersion.TLS_1_3,
        cipher_list=":".join([
            "TLS_AES_128_GCM_SHA256",
            "TLS_AES_256_GCM_SHA384",
            "TLS_CHACHA20_POLY1305_SHA256",
            "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
            "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
            "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
            "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
            "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
            "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        ]),
        alpn_protocols=[AlpnProtocol.HTTP2, AlpnProtocol.HTTP1],
        session_ticket=True,
        enable_ocsp_stapling=True,
        grease_enabled=False
    )

    # Configure HTTP/2 options
    http2_opts = Http2Options(
        initial_window_size=6291456,
        initial_connection_window_size=15728640,
        max_frame_size=16384,
        header_table_size=65536,
        max_concurrent_streams=1000,
        enable_push=False,
        adaptive_window=True
    )

    # Combine both in emulation
    emulation = Emulation(
        tls_options=tls_opts,
        http2_options=http2_opts
    )

    # Create client with custom emulation
    client = Client(
        emulation=emulation,
        tls_info=True,
    )

    # Make request
    resp = await client.get(
        "https://tls.peet.ws/api/all",
        emulation=emulation,
        # Disable client default headers
        default_headers=False,
    )
    await print_response_info(resp)


async def main():
    """Main function to run the Emulation examples

    Demonstrates different browser Emulation scenarios:
    1. Firefox with custom header order
    2. Chrome on Android with OS specification
    3. Custom TLS/HTTP2 emulation with advanced options
    """
    # First test with Firefox
    client = await request_firefox()

    # Then update and test with Chrome on Android
    await request_chrome_android(client)

    # Then test custom TLS/HTTP2 emulation
    await request_custom_emulation(client)


if __name__ == "__main__":
    # Run the async main function
    asyncio.run(main())
