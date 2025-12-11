import pytest
import rnet
from rnet import redirect

client = rnet.Client(redirect=redirect.Policy.limited(10))


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_request_disable_redirect():
    response = await client.get(
        "https://google.com",
        redirect=redirect.Policy.none(),
    )
    assert response.status.is_redirection()
    assert response.url == "https://google.com/"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_request_enable_redirect():
    response = await client.get(
        "https://google.com",
        redirect=redirect.Policy.limited(),
    )
    assert response.status.is_success()
    assert response.url == "https://www.google.com/"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_client_request_disable_redirect():
    client = rnet.Client(redirect=redirect.Policy.none())
    response = await client.get("https://google.com")
    assert response.status.is_redirection()
    assert response.url == "https://google.com/"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_client_request_enable_redirect():
    response = await client.get("https://google.com")
    assert response.status.is_success()
    assert response.url == "https://www.google.com/"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_client_redirec_history():
    url = "https://google.com/"
    client = rnet.Client(redirect=redirect.Policy.limited())
    response = await client.get(url)
    assert response.status.is_success()
    assert response.url == "https://www.google.com/"

    history = response.history
    assert len(history) == 1
    assert history[0].url == "https://www.google.com/"
    assert history[0].previous == url
