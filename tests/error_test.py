import pytest
import rnet
import rnet.exceptions as exceptions


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_proxy_connection_error():
    invalid_proxies = [
        "http://invalid.proxy:8080",
        "https://invalid.proxy:8080",
        "socks4://invalid.proxy:8080",
        "socks4a://invalid.proxy:8080",
        "socks5://invalid.proxy:8080",
        "socks5h://invalid.proxy:8080",
    ]
    target_urls = ["https://example.com", "http://example.com"]
    for proxy in invalid_proxies:
        for url in target_urls:
            with pytest.raises(exceptions.ProxyConnectionError):
                await rnet.get(url, proxy=rnet.Proxy.all(proxy))
