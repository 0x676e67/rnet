import pytest
import rnet

client = rnet.Client(tls_info=True)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_auth():
    resp = await client.get(
        "https://httpbin.org/anything",
        auth="token",
    )
    json = await resp.json()
    authorization = json["headers"]["Authorization"]
    assert authorization == "token"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_bearer_auth():
    resp = await client.get(
        "https://httpbin.org/anything",
        bearer_auth="token",
    )
    json = await resp.json()
    authorization = json["headers"]["Authorization"]
    assert authorization == "Bearer token"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_basic_auth():
    resp = await client.get(
        "https://httpbin.org/anything",
        basic_auth=("user", "pass"),
    )
    json = await resp.json()
    authorization = json["headers"]["Authorization"]
    assert authorization == "Basic dXNlcjpwYXNz"
