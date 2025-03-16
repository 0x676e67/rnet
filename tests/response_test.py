import pytest
import rnet
from pathlib import Path
from rnet import Version, Multipart, Part, WebSocket, Message

client = rnet.Client(tls_info=True)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_websocket():
    ws: WebSocket = await client.websocket("wss://echo.websocket.org")
    await ws.recv()
    await ws.send(Message.from_text("Hello, World!"))
    message: Message = await ws.recv()
    assert message.data == b"Hello, World!"
    await ws.close()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_multiple_requests():
    async def file_to_bytes_stream(file_path):
        with open(file_path, "rb") as f:
            while chunk := f.read(1024):
                yield chunk

    resp = await client.post(
        "https://httpbin.org/anything",
        multipart=Multipart(
            Part(name="def", value="111", filename="def.txt", mime="text/plain"),
            Part(name="abc", value=b"000", filename="abc.txt", mime="text/plain"),
            Part(
                name="LICENSE",
                value=Path("./LICENSE"),
                filename="LICENSE",
                mime="text/plain",
            ),
            Part(
                name="Cargo.toml",
                value=file_to_bytes_stream("./Cargo.toml"),
                filename="Cargo.toml",
                mime="text/plain",
            ),
        ),
    )
    assert resp.status == 200
    assert resp.status_code.is_success() is True
    text = await resp.text()
    assert "111" in text
    assert "000" in text
    assert "rnet" in text


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_cookies():
    url = "https://httpbin.org/cookies/set?mycookie=testvalue"
    response: rnet.Response = await client.get(url)
    assert any(cookie.name == "mycookie" for cookie in response.cookies)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_headers():
    url = "https://httpbin.org/headers"
    response = await client.get(url)
    headers = response.headers
    assert headers is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_getters():
    url = "https://httpbin.org/anything"
    response = await client.get(url, version=Version.HTTP_11)
    assert response.url == url
    assert response.status_code.is_success() is True
    assert response.ok is True
    assert response.version == Version.HTTP_11


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_json():
    url = "https://httpbin.org/json"
    response = await client.get(url)
    json = await response.json()
    assert json is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_text():
    url = "https://httpbin.org/html"
    response = await client.get(url)
    text = await response.text()
    assert text is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_bytes():
    url = "https://httpbin.org/image/png"
    response = await client.get(url)
    bytes = await response.bytes()
    assert bytes is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_stream():
    url = "https://httpbin.org/stream/1"
    response = await client.get(url)
    async with response.stream() as streamer:
        async for bytes in streamer:
            assert bytes is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_peer_certificate():
    resp = await client.get("https://httpbin.org/anything")
    assert resp.peer_certificate() is not None
