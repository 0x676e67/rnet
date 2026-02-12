import time

import rnet
import rnet.blocking


def main():
    with rnet.blocking.get("https://httpbin.io/stream/20") as resp:
        with resp.stream() as streamer:
            for chunk in streamer:
                print(chunk)
                time.sleep(0.1)


if __name__ == "__main__":
    main()
