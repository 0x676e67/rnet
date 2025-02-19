import pytest
import rnet
from rnet import Impersonate, ImpersonateOS


@pytest.mark.asyncio
async def test_update_headers():
    client = rnet.Client()
    headers = {"user-agent": "rnet"}
    client.update(headers=headers)
    assert client.headers.to_dict() == {"user-agent": b"rnet"}


@pytest.mark.asyncio
async def test_update_cookies():
    url = "https://tls.peet.ws"
    client = rnet.Client(cookie_store=True)
    cookies = ["foo=bar"]
    client.set_cookies(url, cookies)
    assert client.get_cookies(url) == cookies


@pytest.mark.asyncio
async def test_update_impersonate():
    client = rnet.Client(impersonate=Impersonate.Firefox133)
    assert (
        client.user_agent
        == "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:133.0) Gecko/20100101 Firefox/133.0"
    )
    client.update(
        impersonate=Impersonate.Firefox135,
        impersonate_os=ImpersonateOS.Windows,
        Impersonate_skip_headers=False,
    )
    assert (
        client.user_agent
        == "Mozilla/5.0 (Windows NT 10.0; rv:135.0) Gecko/20100101 Firefox/135.0"
    )


@pytest.mark.asyncio
async def test_base_url():
    base_url = "https://httpbin.org"
    client = rnet.Client(tls_info=True, base_url=base_url)
    url = "/anything"
    response = await client.get(url)
    assert response.url == base_url + url
