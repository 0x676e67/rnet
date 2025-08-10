import asyncio
import signal
import rnet
from rnet import Message, WebSocket


async def send_message(ws):
    for i in range(20):
        print(f"Sending: Message {i + 1}")
        await ws.send(Message.from_text(f"Message {i + 1}"))
        await asyncio.sleep(0.1)


async def receive_message(ws):
    async for message in ws:
        print("Received: ", message)
        if message.data == b"Message 20":
            print("Closing connection...")
            break


"""
Run websocket server

To test this example:

    git clone https://github.com/tokio-rs/axum && cd axum
    cargo run -p example-websockets-http2

Then run this Python script to connect to the websocket server.
"""


async def main():
    client = rnet.Client(verify=False)
    ws: WebSocket = await client.websocket("wss://127.0.0.1:3000/ws", force_http2=True)
    async with ws:
        print("Status Code: ", ws.status)
        print("Version: ", ws.version)
        print("Headers: ", ws.headers)
        print("Remote Address: ", ws.remote_addr)

        if ws.status.as_int() == 200:
            print("WebSocket connection established successfully.")
            send_task = asyncio.create_task(send_message(ws))
            receive_task = asyncio.create_task(receive_message(ws))

            async def close():
                await ws.close()
                send_task.cancel()
                receive_task.cancel()

            loop = asyncio.get_running_loop()
            for sig in (signal.SIGINT, signal.SIGTERM):
                loop.add_signal_handler(sig, lambda: asyncio.create_task(close()))

            await asyncio.gather(send_task, receive_task)


if __name__ == "__main__":
    asyncio.run(main())
