import signal
import time
import threading
from rnet import Message
from rnet.blocking import Client


def send_message(ws, stop_event):
    for i in range(20):
        if stop_event.is_set():
            break
        print(f"Sending: Message {i + 1}")
        ws.send(Message.from_text(f"Message {i + 1}"))
        time.sleep(0.1)
    ws.send(Message.from_text("CLOSE"))


def receive_message(ws, stop_event):
    for message in ws:
        print("Received: ", message)
        if message.data == b"CLOSE":
            print("Closing connection...")
            stop_event.set()
            break


def main():
    client = Client()
    with client.websocket("wss://echo.websocket.org") as ws:
        print("Status Code: ", ws.status)
        print("Version: ", ws.version)
        print("Headers: ", ws.headers)
        print("Remote Address: ", ws.remote_addr)

        if ws.status.as_int() == 101:
            print("WebSocket connection established.")
            stop_event = threading.Event()
            send_task = threading.Thread(target=send_message, args=(ws, stop_event))
            receive_task = threading.Thread(
                target=receive_message, args=(ws, stop_event)
            )

            send_task.start()
            receive_task.start()

            def close_ws():
                stop_event.set()
                ws.close()
                send_task.join()
                receive_task.join()

            def signal_handler(sig, frame):
                close_ws()
                exit(0)

            signal.signal(signal.SIGINT, signal_handler)
            signal.signal(signal.SIGTERM, signal_handler)

            send_task.join()
            receive_task.join()


if __name__ == "__main__":
    main()
