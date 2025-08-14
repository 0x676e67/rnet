import time

from rnet.blocking import Client


def main():
    client = Client()
    with client.get("https://httpbin.org/stream/20") as resp:
        with resp.stream() as streamer:
            for chunk in streamer:
                print("Chunk: ", chunk)
                time.sleep(0.1)


if __name__ == "__main__":
    main()
