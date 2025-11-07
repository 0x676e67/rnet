import pytest
import rnet

client = rnet.Client()


def test_url_with_spaces_blocking():
    resp = rnet.blocking.Client().get("https://httpbin.org/anything/foo bar/")
    assert resp.status.as_int() == 200
    assert "foo%20bar" in resp.url


def test_url_with_multiple_spaces_blocking():
    resp = rnet.blocking.Client().get("https://httpbin.org/anything/hello world test/")
    assert resp.status.as_int() == 200
    assert "hello%20world%20test" in resp.url


def test_url_with_special_chars_blocking():
    resp = rnet.blocking.Client().get("https://httpbin.org/anything/test<>{}/")
    assert resp.status.as_int() == 200
    assert "%3C" in resp.url or "%3E" in resp.url or "%7B" in resp.url or "%7D" in resp.url


def test_normal_url_blocking():
    resp = rnet.blocking.Client().get("https://httpbin.org/anything/normal-url")
    assert resp.status.as_int() == 200
    assert "normal-url" in resp.url


def test_url_with_already_encoded_chars_blocking():
    resp = rnet.blocking.Client().get("https://httpbin.org/anything/foo%20bar/")
    assert resp.status.as_int() == 200
    assert "foo%20bar" in resp.url


def test_url_with_query_params_and_spaces():
    resp = rnet.blocking.Client().get("https://httpbin.org/anything?key=value with space")
    assert resp.status.as_int() == 200
    assert "%20" in resp.url


@pytest.mark.asyncio
async def test_url_with_spaces_async():
    resp = await client.get("https://httpbin.org/anything/async test/")
    async with resp:
        assert resp.status.as_int() == 200
        assert "async%20test" in resp.url


@pytest.mark.asyncio
async def test_url_with_special_chars_async():
    resp = await client.get("https://httpbin.org/anything/test[]/")
    async with resp:
        assert resp.status.as_int() == 200
        assert "%5B" in resp.url or "%5D" in resp.url

