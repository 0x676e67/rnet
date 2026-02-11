# rnet

[![CI](https://github.com/0x676e67/rnet/actions/workflows/ci.yml/badge.svg)](https://github.com/0x676e67/rnet/actions/workflows/ci.yml)
![GitHub License](https://img.shields.io/github/license/0x676e67/rnet?color=blue)
![Python Version from PEP 621 TOML](https://img.shields.io/python/required-version-toml?tomlFilePath=https%3A%2F%2Fraw.githubusercontent.com%2F0x676e67%2Frnet%2Fmain%2Fpyproject.toml&logo=python)
[![PyPI](https://img.shields.io/pypi/v/rnet?logo=python)](https://pypi.org/project/rnet/)
[![PyPI Downloads](https://static.pepy.tech/badge/rnet)](https://pepy.tech/projects/rnet)

> 🚀 Help me work seamlessly with open source sharing by [sponsoring me on GitHub](https://github.com/0x676e67/0x676e67/blob/main/SPONSOR.md)

An ergonomic and modular Python HTTP client for advanced and low-level emulation, featuring customizable TLS, JA3/JA4, and HTTP/2 fingerprinting capabilities, powered by [wreq](https://github.com/0x676e67/wreq).

## Features

- Async and Blocking `Client`s
- Plain bodies, JSON, urlencoded, multipart
- HTTP Trailer
- Cookie Store
- Redirect Policy
- Original Header
- Rotating Proxies
- Connection Pooling
- Streaming Transfers
- Zero-Copy Transfers
- WebSocket Upgrade
- HTTPS via BoringSSL
- Free-Threaded Safety
- Automatic Decompression
- Certificate Store (CAs & mTLS)

## Example

The following example uses the `asyncio` runtime with `rnet` installed via pip:

```bash
pip install asyncio rnet --pre --upgrade
```

And then the code:

```python
import asyncio
from rnet import Client, Emulation


async def main():
    # Build a client
    client = Client(emulation=Emulation.Safari26)

    # Use the API you're already familiar with
    resp = await client.get("https://tls.peet.ws/api/all")
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())
```

Additional learning resources include:

- [Documentation](getting-started/quickstart.md)
- [Synchronous Examples](https://github.com/0x676e67/rnet/tree/main/python/examples/blocking)
- [Asynchronous Examples](https://github.com/0x676e67/rnet/tree/main/python/examples)

## Behavior

1. **HTTP/2 over TLS**

Due to the complexity of TLS encryption and the widespread adoption of HTTP/2, browser fingerprints such as **JA3**, **JA4**, and **Akamai** cannot be reliably emulated using simple fingerprint strings. Instead of parsing and emulating these string-based fingerprints, `rnet` provides fine-grained control over TLS and HTTP/2 extensions and settings for precise browser behavior emulation.

2. **Device Emulation**

Most browser device models share identical TLS and HTTP/2 configurations, differing only in the `User-Agent` string.

??? note "Available OS emulations"

    | **OS**      | **Description**                |
    | ----------- | ------------------------------ |
    | **Windows** | Windows (any version)          |
    | **MacOS**   | macOS (any version)            |
    | **Linux**   | Linux (any distribution)       |
    | **Android** | Android (mobile)               |
    | **iOS**     | iOS (iPhone/iPad)              |

??? note "Available browser emulations"

    | **Browser** | **Versions**                                                                                                                                                                                                                                                                                                                                                                            |
    | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | **Chrome**  | `Chrome100`, `Chrome101`, `Chrome104`, `Chrome105`, `Chrome106`, `Chrome107`, `Chrome108`, `Chrome109`, `Chrome110`, `Chrome114`, `Chrome116`, `Chrome117`, `Chrome118`, `Chrome119`, `Chrome120`, `Chrome123`, `Chrome124`, `Chrome126`, `Chrome127`, `Chrome128`, `Chrome129`, `Chrome130`, `Chrome131`, `Chrome132`, `Chrome133`, `Chrome134`, `Chrome135`, `Chrome136`, `Chrome137`, `Chrome138`, `Chrome139`, `Chrome140`, `Chrome141`, `Chrome142`, `Chrome143` |
    | **Safari**  | `SafariIos17_2`, `SafariIos17_4_1`, `SafariIos16_5`, `Safari15_3`, `Safari15_5`, `Safari15_6_1`, `Safari16`, `Safari16_5`, `Safari17_0`, `Safari17_2_1`, `Safari17_4_1`, `Safari17_5`, `Safari18`, `SafariIPad18`, `Safari18_2`, `SafariIos18_1_1`, `Safari18_3`, `Safari18_3_1`, `Safari18_5`,  `Safari26`, `Safari26_1`, `Safari26_2`, `SafariIos26`, `SafariIos26_2`, `SafariIPad26`, `SafariIPad26_2`  |
    | **Firefox** | `Firefox109`, `Firefox117`, `Firefox128`, `Firefox133`, `Firefox135`, `FirefoxPrivate135`, `FirefoxAndroid135`, `Firefox136`, `FirefoxPrivate136`, `Firefox139`, `Firefox142`, `Firefox143`, `Firefox144`, `Firefox145`, `Firefox146` |
    | **OkHttp**  | `OkHttp3_9`, `OkHttp3_11`, `OkHttp3_13`, `OkHttp3_14`, `OkHttp4_9`, `OkHttp4_10`, `OkHttp4_12`, `OkHttp5`                                                                                                                                                                                                                                                                               |
    | **Edge**    | `Edge101`, `Edge122`, `Edge127`, `Edge131`, `Edge134`, `Edge135`, `Edge136`, `Edge137`, `Edge138`, `Edge139`, `Edge140`, `Edge141`, `Edge142`|
    | **Opera**   | `Opera116`, `Opera117`, `Opera118`, `Opera119`                                                                                                                                                                 |

## Building

1. Platforms

- Linux(**glibc**/**musl**): `x86_64`, `aarch64`, `armv7`, `i686`
- macOS: `x86_64`,`aarch64`
- Windows: `x86_64`,`i686`,`aarch64`
- Android: `aarch64`, `x86_64`

2. Development

Install the BoringSSL build environment by referring to [boring](https://github.com/cloudflare/boring/blob/master/.github/workflows/ci.yml) and [boringssl](https://github.com/google/boringssl/blob/master/BUILDING.md#build-prerequisites).

```bash
# on ubuntu or debian
sudo apt install -y build-essential cmake perl pkg-config libclang-dev musl-tools git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install uv maturin

uv venv
source .venv/bin/activate

# development
maturin develop --uv

# build wheels
maturin build --release
```

## Benchmark

Outperforms `requests`, `httpx`, `aiohttp`, and `curl_cffi`, and you can see the [benchmark](https://github.com/0x676e67/rnet/tree/main/bench) for details — benchmark data is for reference only and actual performance may vary based on your environment and use case.

## Services

Help sustain the ongoing development of this open-source project by reaching out for [commercial support](mailto:gngppz@gmail.com). Receive private guidance, expert reviews, or direct access to the maintainer, with personalized technical assistance tailored to your needs.

## License

Licensed under either of Apache License, Version 2.0 ([LICENSE](https://github.com/0x676e67/rnet/blob/main/LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the [Apache-2.0](https://github.com/0x676e67/rnet/blob/main/LICENSE) license, shall be licensed as above, without any additional terms or conditions.
