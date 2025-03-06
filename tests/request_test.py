import pytest
import rnet
from rnet import Version

client = rnet.Client(tls_info=True)


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_with_version():
    url = "https://httpbin.org/anything"
    response = await client.get(url, version=Version.HTTP_11)
    assert response.version == Version.HTTP_11

    response = await client.get(url, version=Version.HTTP_2)
    assert response.version == Version.HTTP_2


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_headers():
    url = "https://httpbin.org/headers"
    response = await client.get(url, headers={"foo": "bar"})
    json = await response.json()
    assert json["headers"]["Foo"] == "bar"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_cookies():
    url = "https://httpbin.org/cookies"
    response = await client.get(url, cookies={"foo": "bar"})
    json = await response.json()
    assert json["cookies"] == {"foo": "bar"}


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_form():
    url = "https://httpbin.org/post"
    response = await client.post(url, form=[("foo", "bar")])
    json = await response.json()
    assert json["form"] == {"foo": "bar"}


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_json():
    url = "https://httpbin.org/post"
    response = await client.post(url, json={"foo": "bar"})
    json = await response.json()
    assert json["json"] == {"foo": "bar"}


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_text():
    url = "https://httpbin.org/post"
    response = await client.post(url, body="hello")
    json = await response.json()
    assert json["data"] == "hello"


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_bytes():
    url = "https://httpbin.org/post"
    response = await client.post(url, body=b"hello")
    json = await response.json()
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
    response = await client.post(url, body=file_bytes_stream())
    json = await response.json()
    assert json["data"] in open("README.md").read()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_send_sync_bytes_stream():
    def file_to_bytes_stream(file_path):
        with open(file_path, "rb") as f:
            while chunk := f.read(1024):
                yield chunk

    url = "https://httpbin.org/post"
    response = await client.post(url, body=file_to_bytes_stream("README.md"))
    json = await response.json()
    assert json["data"] in open("README.md").read()
