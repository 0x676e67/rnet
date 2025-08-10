import pytest
import rnet
from rnet import Version, HeaderMap

client = rnet.Client(allow_redirects=True)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_request_disable_redirect():
    response = await client.get(
        "https://httpbin.org//redirect-to?url=https://google.com",
        allow_redirects=False,
    )
    assert response.status.is_redirection()
    assert response.url == "https://httpbin.org//redirect-to?url=https://google.com"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_request_enable_redirect():
    response = await client.get(
        "https://httpbin.org//redirect-to?url=https://google.com",
        allow_redirects=True,
    )
    assert response.status.is_success()
    assert response.url == "https://www.google.com/"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_client_request_disable_redirect():
    client = rnet.Client(allow_redirects=False)
    response = await client.get(
        "https://httpbin.org//redirect-to?url=https://google.com"
    )
    assert response.status.is_redirection()
    assert response.url == "https://httpbin.org//redirect-to?url=https://google.com"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_client_request_enable_redirect():
    response = await client.get(
        "https://httpbin.org//redirect-to?url=https://google.com"
    )
    assert response.status.is_success()
    assert response.url == "https://www.google.com/"
