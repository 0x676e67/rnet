from ipaddress import IPv4Address
import pytest
import rnet
from rnet import Client
from rnet.exceptions import ConnectionError


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_dns_resolve_override():
    client = Client(
        resolve_to_addrs={
            "www.google.com": [IPv4Address("192.168.1.1")],
        },
    )

    try:
        await client.get("https://www.google.com")
        assert False, "ConnectionError was expected"
    except ConnectionError:
        pass
                                                           