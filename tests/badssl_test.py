import pytest
import rnet
from rnet.emulation import Emulation


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_badssl():
    client = rnet.Client(verify=False)
    resp = await client.get("https://self-signed.badssl.com/")
    assert resp.status == 200


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_alps_new_endpoint():
    url = "https://google.com"
    client = rnet.Client(emulation=Emulation.Chrome133)
    response = await client.get(url)
    text = await response.text()
    assert text is not None
