import pytest
import rnet
from rnet.emulation import Emulation
from rnet.tls import CertStore


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_badssl():
    client = rnet.Client(verify=False)
    resp = await client.get("https://self-signed.badssl.com/")
    async with resp:
        assert resp.status.is_success()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_badssl_invalid_cert():
    url = "https://self-signed.badssl.com/"
    client = rnet.Client(verify=False, tls_info=True)
    resp = await client.get(url)
    async with resp:
        assert resp.status.is_success()
        peer_der_cert = resp.peer_certificate()
        assert peer_der_cert is not None
        assert isinstance(peer_der_cert, bytes)
        assert len(peer_der_cert) > 0

        cert_store = CertStore(der_certs=[peer_der_cert])
        assert cert_store is not None

        client = rnet.Client(verify=cert_store)
        resp = await client.get(url)
        async with resp:
            assert resp.status.is_success()


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_alps_new_endpoint():
    url = "https://google.com"
    client = rnet.Client(emulation=Emulation.Chrome133)
    resp = await client.get(url)
    async with resp:
        text = await resp.text()
        assert text is not None
