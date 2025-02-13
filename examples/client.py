import asyncio
import rnet

import asyncio
import rnet
from rnet import Impersonate, Client


async def main():
    client = Client(
        impersonate=Impersonate.Firefox133,
        user_agent="rnet/0.0.1",
    )


if __name__ == "__main__":
    asyncio.run(main())
