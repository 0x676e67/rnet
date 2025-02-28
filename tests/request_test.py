import pytest
import rnet

client = rnet.Client(tls_info=True)


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
async def test_send_bytes_stream():
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
