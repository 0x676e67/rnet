import pytest
import rnet
from rnet.cookie import Cookie

client = rnet.Client()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_cookie():
    jar = rnet.Jar()
    url = "http://localhost:8080/cookies"
    cookie = Cookie("test_cookie", "12345", domain="localhost", path="/cookies")
    jar.add_cookie(cookie, url)
    cookie = jar.get("test_cookie", url)
    assert cookie is not None
    assert cookie.name == "test_cookie"
    assert cookie.value == "12345"
    assert cookie.domain == "localhost"
    assert cookie.path == "/cookies"

    jar.clear()

    jar.add_cookie_str("test_cookie=12345; Path=/cookies; Domain=localhost", url)
    cookie = jar.get("test_cookie", url)
    assert cookie is not None
    assert cookie.name == "test_cookie"
    assert cookie.value == "12345"
    assert cookie.domain == "localhost"
    assert cookie.path == "/cookies"

    client = rnet.Client(cookie_provider=jar)
    response = await client.get(url)
    assert response.status.is_success()
    assert "test_cookie" in await response.text()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_all_cookies():
    jar = rnet.Jar()
    url = "http://localhost:8080/cookies"
    cookie1 = Cookie("test_cookie1", "12345", domain="localhost", path="/cookies")
    cookie2 = Cookie("test_cookie2", "67890", domain="localhost", path="/cookies")
    jar.add_cookie(cookie1, url)
    jar.add_cookie(cookie2, url)

    cookies = jar.get_all()
    assert len(cookies) == 2
    cookie_names = [cookie.name for cookie in cookies]
    assert "test_cookie1" in cookie_names
    assert "test_cookie2" in cookie_names

    client = rnet.Client(cookie_provider=jar)
    response = await client.get(url)
    assert response.status.is_success()
    body = await response.text()
    assert "test_cookie1" in body
    assert "test_cookie2" in body


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_remove_cookie():
    jar = rnet.Jar()
    client = rnet.Client(cookie_provider=jar)
    url = "http://localhost:8080/cookies"
    cookie = Cookie("test_cookie", "12345", domain="localhost", path="/cookies")
    jar.add_cookie(cookie, url)

    # Verify the cookie is set
    response = await client.get(url)
    assert response.status.is_success()
    assert "test_cookie" in await response.text()

    # Remove the cookie
    jar.remove("test_cookie", url)

    # Verify the cookie is removed
    cookie = jar.get("test_cookie", url)
    assert cookie is None

    response = await client.get(url)
    assert response.status.is_success()
    assert "test_cookie" not in await response.text()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_clear_cookies():
    jar = rnet.Jar()
    client = rnet.Client(cookie_provider=jar)
    url = "http://localhost:8080/cookies"
    cookie1 = Cookie("test_cookie1", "12345", domain="localhost", path="/cookies")
    cookie2 = Cookie("test_cookie2", "67890", domain="localhost", path="/cookies")

    jar.add_cookie(cookie1, url)
    jar.add_cookie(cookie2, url)

    # Verify cookies are set

    response = await client.get(url)
    assert response.status.is_success()
    body = await response.text()
    assert "test_cookie1" in body
    assert "test_cookie2" in body

    # Clear all cookies
    jar.clear()

    # Verify all cookies are cleared
    assert jar.get("test_cookie1", url) is None
    assert jar.get("test_cookie2", url) is None

    response = await client.get(url)
    assert response.status.is_success()
    body = await response.text()
    assert "test_cookie1" not in body
    assert "test_cookie2" not in body


@pytest.mark.asyncio
async def test_client_cookie_jar_accessor():
    url = "http://localhost:8080/cookies"

    # 0) Unconfigured client should raise when accessing cookie_jar.
    client0 = rnet.Client()
    with pytest.raises(AttributeError):
        _ = client0.cookie_jar

    # 1) If a cookie_provider is passed, client.cookie_jar should return it (shared storage).
    jar = rnet.Jar()
    client = rnet.Client(cookie_provider=jar)
    client.cookie_jar.add_cookie_str(
        "test_cookie=12345; Path=/cookies; Domain=localhost", url
    )
    resp = await client.get(url)
    assert resp.status.is_success()
    assert "test_cookie" in await resp.text()

    # 2) If cookie_store=True is used without explicit provider, client.cookie_jar should exist.
    client2 = rnet.Client(cookie_store=True)
    client2.cookie_jar.add_cookie_str(
        "test_cookie=abc; Path=/cookies; Domain=localhost", url
    )
    resp2 = await client2.get(url)
    assert resp2.status.is_success()
    assert "test_cookie" in await resp2.text()

    # 3) If both cookie_provider and cookie_store=True are set, the provider must win.
    jar3 = rnet.Jar()
    client3 = rnet.Client(cookie_provider=jar3, cookie_store=True)
    client3.cookie_jar.add_cookie_str(
        "test_cookie=zzz; Path=/cookies; Domain=localhost", url
    )
    resp3 = await client3.get(url)
    assert resp3.status.is_success()
    assert "test_cookie" in await resp3.text()
