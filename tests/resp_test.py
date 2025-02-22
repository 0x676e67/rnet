import pytest
import rnet
from pathlib import Path
from rnet import Version, Multipart, Part

client = rnet.Client(tls_info=True)


@pytest.mark.asyncio
async def test_multiple_requests():
    resp = await client.post(
        "https://httpbin.org/anything",
        multipart=Multipart(
            Part(name="def", value="111", filename="def.txt", mime="text/plain"),
            Part(name="abc", value=b"000", filename="abc.txt", mime="text/plain"),
            Part(
                name="LICENSE",
                value=Path("Cargo.toml"),
                filename="LICENSE",
                mime="text/plain",
            ),
        ),
    )
    assert resp.status == 200
    assert resp.status_code.is_success() is True
    text = await resp.text()
    assert "111" in text
    assert "000" in text
    assert "Asynchronous Python HTTP Client with Black Magic" in text


@pytest.mark.asyncio
async def test_set_and_get_cookie():
    url = "https://httpbin.org/cookies/set?mycookie=testvalue"
    response = await client.get(url)
    assert response.cookies == {"mycookie": "testvalue"}


@pytest.mark.asyncio
async def test_get_headers():
    url = "https://httpbin.org/headers"
    response = await client.get(url)
    headers = response.headers.to_dict()
    assert headers is not None


@pytest.mark.asyncio
async def test_getters():
    url = "https://httpbin.org/anything"
    response = await client.get(url, version=Version.HTTP_11)
    assert response.url == url
    assert response.status_code.is_success() is True
    assert response.ok is True
    assert response.version == Version.HTTP_11


@pytest.mark.asyncio
async def test_get_json():
    url = "https://httpbin.org/json"
    response = await client.get(url)
    json = await response.json()
    assert json is not None


@pytest.mark.asyncio
async def test_get_text():
    url = "https://httpbin.org/html"
    response = await client.get(url)
    text = await response.text()
    assert text is not None


@pytest.mark.asyncio
async def test_get_bytes():
    url = "https://httpbin.org/image/png"
    response = await client.get(url)
    bytes = await response.bytes()
    assert bytes is not None


@pytest.mark.asyncio
async def test_peer_certificate():
    client = rnet.Client(tls_info=True)
    resp = await client.get("https://httpbin.org/anything")
    assert resp.peer_certificate() is not None
