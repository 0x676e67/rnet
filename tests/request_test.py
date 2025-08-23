import pytest
import rnet
from rnet import Version, HeaderMap

client = rnet.Client(tls_info=True)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_gzip():
    url = "https://httpbin.org/gzip"
    resp = await client.get(url)
    async with resp:
        text = await resp.text()
        assert text is not None
        assert "gzipped" in text


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_deflate():
    url = "https://httpbin.org/deflate"
    resp = await client.get(url)
    async with resp:
        text = await resp.text()
        assert text is not None
        assert "deflated" in text


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_brotli():
    url = "https://httpbin.org/brotli"
    resp = await client.get(url)
    async with resp:
        text = await resp.text()
        assert text is not None
        assert "brotli" in text


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_auth():
    resp = await client.get(
        "https://httpbin.org/anything",
        auth="token",
    )
    async with resp:
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
    async with resp:
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
    async with resp:
        json = await resp.json()
        authorization = json["headers"]["Authorization"]
        assert authorization == "Basic dXNlcjpwYXNz"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_with_version():
    url = "https://httpbin.org/anything"
    resp = await client.get(url, version=Version.HTTP_11)
    async with resp:
        assert resp.version == Version.HTTP_11

    resp = await client.get(url, version=Version.HTTP_2)
    async with resp:
        assert resp.version == Version.HTTP_2


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_headers():
    url = "https://httpbin.org/headers"
    headers = {"foo": "bar"}
    resp = await client.get(url, headers=headers)
    async with resp:
        json = await resp.json()
        assert json["headers"]["Foo"] == "bar"

    resp = await client.get(url, headers=HeaderMap(headers))
    async with resp:
        json = await resp.json()
        assert json["headers"]["Foo"] == "bar"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_disable_default_headers():
    url = "https://httpbin.org/headers"
    headers = {"foo": "bar"}
    client = rnet.Client(tls_info=True, headers=headers)
    resp = await client.get(url)
    async with resp:
        json = await resp.json()
        assert json["headers"]["Foo"] == "bar"

    resp = await client.get(url, default_headers=False)
    async with resp:
        json = await resp.json()
        assert "Foo" not in json["headers"]


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_cookies():
    url = "https://httpbin.org/cookies"
    resp = await client.get(url, cookies={"foo": "bar"})
    async with resp:
        json = await resp.json()
        assert json["cookies"] == {"foo": "bar"}


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_form():
    url = "https://httpbin.org/post"
    resp = await client.post(url, form=[("foo", "bar")])
    async with resp:
        json = await resp.json()
        assert json["form"] == {"foo": "bar"}


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_json():
    url = "https://httpbin.org/post"
    resp = await client.post(url, json={"foo": "bar"})
    async with resp:
        json = await resp.json()
        assert json["json"] == {"foo": "bar"}


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_text():
    url = "https://httpbin.org/post"
    resp = await client.post(url, body="hello")
    async with resp:
        json = await resp.json()
        assert json["data"] == "hello"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_bytes():
    url = "https://httpbin.org/post"
    resp = await client.post(url, body=b"hello")
    async with resp:
        json = await resp.json()
        assert json["data"] == "hello"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_async_bytes_stream():
    async def file_bytes_stream():
        with open("README.md", "rb") as f:
            while True:
                chunk = f.read(1024)
                if not chunk:
                    break
                yield chunk

    url = "https://httpbin.org/post"
    resp = await client.post(url, body=file_bytes_stream())
    async with resp:
        json = await resp.json()
        assert json["data"] in open("README.md").read()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_sync_bytes_stream():
    def file_to_bytes_stream(file_path):
        with open(file_path, "rb") as f:
            while chunk := f.read(1024):
                yield chunk

    url = "https://httpbin.org/post"
    resp = await client.post(url, body=file_to_bytes_stream("README.md"))
    async with resp:
        json = await resp.json()
        assert json["data"] in open("README.md").read()
