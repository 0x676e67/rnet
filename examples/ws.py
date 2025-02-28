import asyncio
import signal
import rnet
from rnet import Message


async def send_message(ws):
    for i in range(20):
        print(f"Sending: Message {i + 1}")
        await ws.send(Message.from_text(f"Message {i + 1}"))
        await asyncio.sleep(0.1)


async def receive_message(ws):
    # while True:
    #     try:
    #         message = await ws.recv()
    #         print("Received: ", message)
    #         if message.data == b"Message 20":
    #             print("Closing connection...")
    #             break
    #     except asyncio.CancelledError:
    #         break
    # or
    async for message in ws:
        print("Received: ", message)
        if message.data == b"Message 20":
            print("Closing connection...")
            break


async def main():
    async with await rnet.websocket("wss://echo.websocket.org") as ws:
        print("Status Code: ", ws.status)
        print("Version: ", ws.version)
        print("Headers: ", ws.headers)
        print("Remote Address: ", ws.remote_addr)

        if ws.ok:
            send_task = asyncio.create_task(send_message(ws))
            receive_task = asyncio.create_task(receive_message(ws))

            async def close_ws():
                await ws.close()
                send_task.cancel()
                receive_task.cancel()

            loop = asyncio.get_running_loop()
            for sig in (signal.SIGINT, signal.SIGTERM):
                loop.add_signal_handler(sig, lambda: asyncio.create_task(close_ws()))

            await asyncio.gather(send_task, receive_task)


if __name__ == "__main__":
    asyncio.run(main())
