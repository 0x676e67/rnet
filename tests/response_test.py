import pytest
import rnet
from pathlib import Path
from rnet import Version, Multipart, Part, WebSocket, Message

client = rnet.Client(tls_info=True)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_websocket():
    ws = await client.websocket("wss://echo.websocket.org")
    async with ws:
        await ws.recv()
        await ws.send(Message.from_text("Hello, World!"))
        message = await ws.recv()
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
    async with resp:
        assert resp.status.is_success() is True
        text = await resp.text()
        assert "111" in text
        assert "000" in text
        assert "rnet" in text


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_cookies():
    url = "https://httpbin.org/cookies/set?mycookie=testvalue"
    resp = await client.get(url)
    async with resp:
        assert any(cookie.name == "mycookie" for cookie in resp.cookies)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_headers():
    url = "https://httpbin.org/headers"
    resp = await client.get(url)
    async with resp:
        assert resp.headers is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_getters():
    url = "https://httpbin.org/anything"
    resp = await client.get(url, version=Version.HTTP_11)
    async with resp:
        assert resp.url == url
        assert resp.status.is_success()
        assert resp.version == Version.HTTP_11


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_json():
    url = "https://httpbin.org/json"
    resp = await client.get(url)
    async with resp:
        json = await resp.json()
        assert json is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_text():
    url = "https://httpbin.org/html"
    resp = await client.get(url)
    async with resp:
        text = await resp.text()
        assert text is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_bytes():
    url = "https://httpbin.org/image/png"
    resp = await client.get(url)
    async with resp:
        bytes = await resp.bytes()
        assert bytes is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_stream():
    url = "https://httpbin.org/stream/1"
    resp = await client.get(url)
    async with resp:
        async with resp.stream() as streamer:
            async for bytes in streamer:
                assert bytes is not None


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_peer_certificate():
    resp = await client.get("https://httpbin.org/anything")
    async with resp:
        assert resp.peer_certificate() is not None
