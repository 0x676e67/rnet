# rnet

[![CI](https://github.com/0x676e67/rnet/actions/workflows/ci.yml/badge.svg)](https://github.com/0x676e67/rnet/actions/workflows/ci.yml)
[![PyPI - License](https://img.shields.io/pypi/l/rnet)](https://github.com/0x676e67/rnet/blob/main/LICENSE)
![Python Version from PEP 621 TOML](https://img.shields.io/python/required-version-toml?tomlFilePath=https%3A%2F%2Fraw.githubusercontent.com%2F0x676e67%2Frnet%2Fmain%2Fpyproject.toml&logo=python)
[![PyPI](https://img.shields.io/pypi/v/rnet?logo=python)](https://pypi.org/project/rnet/)
[![PyPI Downloads](https://static.pepy.tech/badge/rnet)](https://pepy.tech/projects/rnet)

> 🚀 Help me work seamlessly with open source sharing by [sponsoring me on GitHub](https://github.com/0x676e67/0x676e67/blob/main/SPONSOR.md)

A blazing-fast Python HTTP client with advanced browser fingerprinting that accurately emulates **Chrome**, **Firefox**, **Safari**, **Opera**, and **OkHttp**, with precise TLS and HTTP2 signatures, and powered by [wreq](https://github.com/0x676e67/wreq) for high performance.

## Features

- Async and Blocking `Client`s
- Plain bodies, JSON, urlencoded, multipart
- Cookie Store
- Redirect Policy
- Original Headers
- Rotating Proxies
- WebSocket Upgrade
- Connection Pooling
- Streaming Transfers
- Zero-Copy Transfers
- HTTPS via BoringSSL
- Free-Threaded Safety
- Automatic Decompression

## Example

The following example uses the `asyncio` runtime with `rnet` installed via pip:

```bash
pip install asyncio rnet==3.0.0rc8
```

And then the code:

```python
import asyncio
from rnet import Client, Emulation


async def main():
    # Build a client
    client = Client(emulation=Emulation.Firefox143)

    # Use the API you're already familiar with
    resp = await client.get("https://tls.peet.ws/api/all")

    # Print the response
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())

```

Additional learning resources include:

- [DeepWiki](https://deepwiki.com/0x676e67/rnet)
- [API Documentation](https://github.com/0x676e67/rnet/blob/main/python/rnet)
- [Repository Tests](https://github.com/0x676e67/rnet/tree/main/tests)
- [Synchronous Examples](https://github.com/0x676e67/rnet/tree/main/python/examples/blocking)
- [Asynchronous Examples](https://github.com/0x676e67/rnet/tree/main/python/examples)
- <details>
  <summary>Available OS emulations</summary>

  | **OS**      | **Description**                |
  | ----------- | ------------------------------ |
  | **Windows** | Windows (any version)          |
  | **MacOS**   | macOS (any version)            |
  | **Linux**   | Linux (any distribution)       |
  | **Android** | Android (mobile)               |
  | **iOS**     | iOS (iPhone/iPad)              |

  </details>
- <details>
  <summary>Available browser emulations</summary>

  | **Browser** | **Versions**                                                                                                                                                                                                                                                                                                                                                                            |
  | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | **Chrome**  | `Chrome100`, `Chrome101`, `Chrome104`, `Chrome105`, `Chrome106`, `Chrome107`, `Chrome108`, `Chrome109`, `Chrome110`, `Chrome114`, `Chrome116`, `Chrome117`, `Chrome118`, `Chrome119`, `Chrome120`, `Chrome123`, `Chrome124`, `Chrome126`, `Chrome127`, `Chrome128`, `Chrome129`, `Chrome130`, `Chrome131`, `Chrome132`, `Chrome133`, `Chrome134`, `Chrome135`, `Chrome136`, `Chrome137`, `Chrome138`, `Chrome139`, `Chrome140` |
  | **Safari**  | `SafariIos17_2`, `SafariIos17_4_1`, `SafariIos16_5`, `Safari15_3`, `Safari15_5`, `Safari15_6_1`, `Safari16`, `Safari16_5`, `Safari17_0`, `Safari17_2_1`, `Safari17_4_1`, `Safari17_5`, `Safari18`, `SafariIPad18`, `Safari18_2`, `SafariIos18_1_1`, `Safari18_3`, `Safari18_3_1`, `Safari18_5`,  `Safari26`, `SafariIos26`, `SafariIPad26`                                                                                          |
  | **Firefox** | `Firefox109`, `Firefox117`, `Firefox128`, `Firefox133`, `Firefox135`, `FirefoxPrivate135`, `FirefoxAndroid135`, `Firefox136`, `FirefoxPrivate136`, `Firefox139`, `Firefox142`, `Firefox143`                                                                                                                                                                                                                         |
  | **OkHttp**  | `OkHttp3_9`, `OkHttp3_11`, `OkHttp3_13`, `OkHttp3_14`, `OkHttp4_9`, `OkHttp4_10`, `OkHttp4_12`, `OkHttp5`                                                                                                                                                                                                                                                                               |
  | **Edge**    | `Edge101`, `Edge122`, `Edge127`, `Edge131`, `Edge134`                                                                                                                                                                                                                                                                                                                                   |
  | **Opera**   | `Opera116`, `Opera117`, `Opera118`, `Opera119`                                                                                                                                                                                                                                                                                                                                          |

  </details>

## Platforms

1. Linux

- **glibc >= 2.34**: `x86_64`, `aarch64`, `armv7`, `i686`
- **musl**: `x86_64`, `aarch64`, `armv7`, `i686`

2. macOS: `x86_64`,`aarch64`

3. Windows: `x86_64`,`i686`,`aarch64`

## Building

1. Development

Install the BoringSSL build environment by referring to [boring](https://github.com/cloudflare/boring/blob/master/.github/workflows/ci.yml) and [boringssl](https://github.com/google/boringssl/blob/master/BUILDING.md#build-prerequisites).

```bash
# on ubuntu or debian
sudo apt install -y build-essential cmake perl pkg-config libclang-dev musl-tools git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install uv maturin

uv venv
source .venv/bin/activate

# Development
maturin develop --uv

# Build wheels
maturin build --release
```

2. Musllinux

Make sure you have Docker installed. The provided image may be outdated, so you might need to build it yourself. See [rust-cross-musl](https://github.com/0x676e67/toolchain/blob/master/rust-musl-cross/Dockerfile) and the upstream [rust-cross/rust-musl-cross](https://github.com/rust-cross/rust-musl-cross) for reference.  
**Note:** The upstream image does not include some platform-specific linker environment variables; you may need to add these manually.

```bash
bash .github/musl_build.sh i686-unknown-linux-musl
bash .github/musl_build.sh x86_64-unknown-linux-musl
bash .github/musl_build.sh aarch64-unknown-linux-musl
bash .github/musl_build.sh armv7-unknown-linux-musleabihf
```

3. Manylinux

For Manylinux compilation, refer to [manylinux](https://github.com/PyO3/maturin?tab=readme-ov-file#manylinux-and-auditwheel).

## Services

Help sustain the ongoing development of this open-source project by reaching out for [commercial support](mailto:gngppz@gmail.com). Receive private guidance, expert reviews, or direct access to the maintainer, with personalized technical assistance tailored to your needs.

## Benchmark

Outperforms `requests`, `httpx`, `Python-TLS-Client`, and `curl_cffi`. See the [benchmark](https://github.com/0x676e67/rnet/tree/main/python/benchmark) for details. Benchmark data is for reference only—actual performance may vary based on your environment and use case.

## License

Licensed under either of Apache License, Version 2.0 ([LICENSE](./LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the [Apache-2.0](./LICENSE) license, shall be licensed as above, without any additional terms or conditions.

## Sponsors

<a href="https://hypersolutions.co/?utm_source=github&utm_medium=readme&utm_campaign=rnet" target="_blank"><img src="https://raw.githubusercontent.com/0x676e67/rnet/main/.github/assets/hypersolutions.jpg" height="47" width="149"></a>

TLS fingerprinting alone isn't enough for modern bot protection. **[Hyper Solutions](https://hypersolutions.co?utm_source=github&utm_medium=readme&utm_campaign=rnet)** provides the missing piece - API endpoints that generate valid antibot tokens for:

**Akamai** • **DataDome** • **Kasada** • **Incapsula**

No browser automation. Just simple API calls that return the exact cookies and headers these systems require.

🚀 **[Get Your API Key](https://hypersolutions.co?utm_source=github&utm_medium=readme&utm_campaign=rnet)** | 📖 **[Docs](https://docs.justhyped.dev)** | 💬 **[Discord](https://discord.gg/akamai)**

---

<a href="https://dashboard.capsolver.com/passport/register?inviteCode=y7CtB_a-3X6d" target="_blank"><img src="https://raw.githubusercontent.com/0x676e67/rnet/main/.github/assets/capsolver.jpg" height="47" width="149"></a>

[CapSolver](https://www.capsolver.com/?utm_source=github&utm_medium=banner_repo&utm_campaign=rnet) leverages AI-powered Auto Web Unblock to bypass Captchas effortlessly, providing fast, reliable, and cost-effective data access with seamless integration into Colly, Puppeteer, and Playwright—use code **`RNET`** for a 6% bonus!
