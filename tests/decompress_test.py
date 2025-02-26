import pytest
import rnet

client = rnet.Client(tls_info=True)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_gzip():
    url = "https://httpbin.org/gzip"
    response = await client.get(url)
    text = await response.text()
    assert text is not None
    assert "gzipped" in text


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_deflate():
    url = "https://httpbin.org/deflate"
    response = await client.get(url)
    text = await response.text()
    assert text is not None
    assert "deflated" in text


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_brotli():
    url = "https://httpbin.org/brotli"
    response = await client.get(url)
    text = await response.text()
    assert text is not None
    assert "brotli" in text
