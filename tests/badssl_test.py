import pytest
import rnet


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_badssl():
    client = rnet.Client(verify=False)
    resp = await client.get("https://self-signed.badssl.com/")
    assert resp.status == 200

    client.update(impersonate=rnet.Impersonate.Chrome100)
    resp = await client.get("https://self-signed.badssl.com/")
    assert resp.status == 200
