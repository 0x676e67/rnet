import asyncio
from rnet import Client, Response
from rnet.browser import Browser, BrowserOS, BrowserOption
from rnet.emulation import TlsOptions, Http2Options, Emulation, PseudoId
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
            browser=Browser.Chrome134,
            browser_os=BrowserOS.Android,
        ),
        # Disable client default headers
        default_headers=False,
    )
    await print_response_info(resp)


async def request_with_emulation():
    """Test request with comprehensive emulation configuration

    Demonstrates advanced emulation configuration based on wreq example with:
    - Custom TLS options (curves, ciphers, sigalgs, ALPN, OCSP)
    - Custom HTTP/2 options (stream ID, window sizes, pseudo headers order)
    - Custom headers and original header order
    - Combined emulation configuration
    """
    print("\n[Testing Request with Emulation (from wreq example)]")

    # TLS options config
    tls_opts = TlsOptions(
        enable_ocsp_stapling=True,
        curves_list=":".join([
            "X25519",
            "P-256",
            "P-384"
        ]),
        cipher_list=":".join([
            "TLS_AES_128_GCM_SHA256",
            "TLS_AES_256_GCM_SHA384",
            "TLS_CHACHA20_POLY1305_SHA256",
            "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
            "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
            "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
            "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
            "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
            "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256"
        ]),
        sigalgs_list=":".join([
            "ecdsa_secp256r1_sha256",
            "rsa_pss_rsae_sha256",
            "rsa_pkcs1_sha256",
            "ecdsa_secp384r1_sha384",
            "rsa_pss_rsae_sha384",
            "rsa_pkcs1_sha384",
            "rsa_pss_rsae_sha512",
            "rsa_pkcs1_sha512",
            "rsa_pkcs1_sha1"
        ]),
        alpn_protocols=[AlpnProtocol.HTTP2, AlpnProtocol.HTTP1],
        min_tls_version=TlsVersion.TLS_1_2,
        max_tls_version=TlsVersion.TLS_1_3
    )

    # HTTP/2 options config
    http2_opts = Http2Options(
        initial_stream_id=3,
        initial_window_size=16777216,
        initial_connection_window_size=16711681 + 65535,
        headers_pseudo_order=[
            PseudoId.Method,
            PseudoId.Path,
            PseudoId.Authority,
            PseudoId.Scheme
        ]
    )

    # Default headers
    headers = {
        "User-Agent": "TwitterAndroid/10.89.0-release.0 (310890000-r-0) G011A/9 (google;G011A;google;G011A;0;;1;2016)",
        "Accept-Language": "en-US",
        "Accept-Encoding": "br, gzip, deflate",
        "Accept": "application/json",
        "Cache-Control": "no-store",
        "Cookie": "ct0=YOUR_CT0_VALUE;"
    }

    # The headers keep the original case and order
    orig_headers = [
        "cookie",
        "content-length",
        "USER-AGENT",
        "ACCEPT-LANGUAGE",
        "ACCEPT-ENCODING"
    ]

    # This provider encapsulates TLS, HTTP/1, HTTP/2, default headers, and original headers
    emulation = Emulation(
        tls_options=tls_opts,
        http2_options=http2_opts,
        headers=headers,
        orig_headers=orig_headers
    )

    # Create client with emulation
    client = Client(
        emulation=emulation,
        tls_info=True,
    )

    # Use the API you're already familiar with
    resp = await client.post("https://tls.peet.ws/api/all")
    await print_response_info(resp)


async def main():
    """Main function to run the Emulation examples

    Demonstrates different browser Emulation scenarios:
    1. Firefox with custom header order
    2. Chrome on Android with OS specification
    3. Request with comprehensive emulation (based on wreq example)
    """
    # First test with Firefox
    client = await request_firefox()

    # Then update and test with Chrome on Android
    await request_chrome_android(client)

    # Then test request with comprehensive emulation (from wreq example)
    await request_with_emulation()


if __name__ == "__main__":
    # Run the async main function
    asyncio.run(main())
