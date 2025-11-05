from ipaddress import IPv4Address
import pytest
from rnet import Client
from rnet.exceptions import ConnectionError
from rnet.dns import ResolverOptions, LookupIpStrategy


@pytest.mark.asyncio
@pytest.mark.flaky(reruns=3, reruns_delay=2)
async def test_dns_resolve_override():
    dns_options = ResolverOptions()
    dns_options.add_resolve("www.google.com", [IPv4Address("192.168.1.1")])
    client = Client(
        dns_options=dns_options,
    )

    try:
        await client.get("https://www.google.com")
        assert False, "ConnectionError was expected"
    except ConnectionError:
        pass
                                                           