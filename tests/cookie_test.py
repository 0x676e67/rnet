import pytest
import rnet
from rnet.cookie import Cookie


client = rnet.Client()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_cookie():
    jar = rnet.Jar()
    url = "https://httpbin.org/cookies"
    cookie = Cookie("test_cookie", "12345", domain="httpbin.org", path="/cookies")
    jar.add(cookie, url)
    cookie = jar.get("test_cookie", url)
    assert cookie is not None
    assert cookie.name == "test_cookie"
    assert cookie.value == "12345"
    assert cookie.domain == "httpbin.org"
    assert cookie.path == "/cookies"

    jar.clear()

    jar.add_cookie_str("test_cookie=12345; Path=/cookies; Domain=httpbin.org", url)
    cookie = jar.get("test_cookie", url)
    assert cookie is not None
    assert cookie.name == "test_cookie"
    assert cookie.value == "12345"
    assert cookie.domain == "httpbin.org"
    assert cookie.path == "/cookies"

    client = rnet.Client(cookie_provider=jar)
    response = await client.get(url)
    assert response.status.is_success()
    assert "test_cookie" in await response.text()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_get_all_cookies():
    jar = rnet.Jar()
    url = "https://httpbin.org/cookies"
    cookie1 = Cookie("test_cookie1", "12345", domain="httpbin.org", path="/cookies")
    cookie2 = Cookie("test_cookie2", "67890", domain="httpbin.org", path="/cookies")
    jar.add(cookie1, url)
    jar.add(cookie2, url)

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
    url = "https://httpbin.org/cookies"
    cookie = Cookie("test_cookie", "12345", domain="httpbin.org", path="/cookies")
    jar.add(cookie, url)

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
    url = "https://httpbin.org/cookies"
    cookie1 = Cookie("test_cookie1", "12345", domain="httpbin.org", path="/cookies")
    cookie2 = Cookie("test_cookie2", "67890", domain="httpbin.org", path="/cookies")

    jar.add(cookie1, url)
    jar.add(cookie2, url)

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
