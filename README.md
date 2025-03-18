# rnet

[![CI](https://github.com/0x676e67/rnet/actions/workflows/ci.yml/badge.svg)](https://github.com/0x676e67/rnet/actions/workflows/ci.yml)
[![PyPI - License](https://img.shields.io/pypi/l/rnet)](https://github.com/0x676e67/rnet/blob/main/LICENSE)
![Python Version from PEP 621 TOML](https://img.shields.io/python/required-version-toml?tomlFilePath=https%3A%2F%2Fraw.githubusercontent.com%2F0x676e67%2Frnet%2Fmain%2Fpyproject.toml)
[![PyPI](https://img.shields.io/pypi/v/rnet)](https://pypi.org/project/rnet/)
[![PyPI Downloads](https://static.pepy.tech/badge/rnet)](https://pepy.tech/projects/rnet)

> 🚀 Help me work seamlessly with open source sharing by [sponsoring me on GitHub](https://github.com/0x676e67/0x676e67/blob/main/SPONSOR.md)

A blazing-fast Python HTTP client with TLS fingerprint, capable of mimicking `TLS` and `HTTP2` fingerprints of popular browsers like `Chrome`, `Safari`, `Firefox`, and `OkHttp`.

## Features

- Async and Blocking `Client`s
- Plain bodies, JSON, urlencoded, multipart
- Header Order
- Cookie Store
- Redirect Policy
- Rotating Proxies
- WebSocket Upgrade
- Async DNS Resolver
- HTTPS via BoringSSL
- Free-Threaded Safety

## Example

This asynchronous example demonstrates how to make a simple GET request using the `rnet` library. So you need install `rnet` and run the following code:

```bash
pip install asyncio rnet
```

And then the code:

```python
import asyncio
from rnet import Impersonate, Client


async def main():
    client = Client(
        impersonate=Impersonate.Firefox136
    )
    resp = await client.get("https://tls.browserleaks.com")
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Cookies: ", resp.cookies)
    print("Encoding: ", resp.encoding)
    print("Content-Length: ", resp.content_length)
    print("Remote Address: ", resp.remote_addr)
    print(await resp.text())


if __name__ == "__main__":
    asyncio.run(main())

```

Additional learning resources include:

- [API Documentation](https://github.com/0x676e67/rnet/blob/main/rnet.pyi)
- [Repository Examples](https://github.com/0x676e67/rnet/tree/main/examples)
- [Repository Tests](https://github.com/0x676e67/rnet/tree/main/tests)

## Platforms

1. Linux

- **musl**: `x86_64`, `aarch64`, `armv7`, `i686`
- **glibc >= 2.17**: `x86_64`
- **glibc >= 2.31**: `aarch64`, `armv7`, `i686`

2. macOS: `x86_64`,`aarch64`

3. Windows: `x86_64`,`i686`,`aarch64`

## Building

1. Install environment

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install maturin
pip install uv

uv venv
source .venv/bin/activate
```

2. Development

```bash
maturin develop --uv
python3 examples/client.py
```

3. Compile wheels

- Local Compilation

Install the BoringSSL build environment by referring to [boring](https://github.com/cloudflare/boring/blob/master/.github/workflows/ci.yml) and [boringssl](https://github.com/google/boringssl/blob/master/BUILDING.md#build-prerequisites).

```bash
maturin build --release
```

- Musllinux

Also install the Docker environment. The image might be outdated, so if building the image yourself is required, refer to [rust-cross-musl](https://github.com/0x676e67/toolchain/blob/master/rust-musl-cross/Dockerfile) and the upstream [rust-cross-musl](https://github.com/rust-cross/rust-musl-cross). The upstream [rust-cross-musl](https://github.com/rust-cross/rust-musl-cross) lacks the relevant platform linker environment variables, which must be added manually.
  
```bash
bash .github/musl_build.sh x86_64-unknown-linux-musl
bash .github/musl_build.sh aarch64-unknown-linux-musl
bash .github/musl_build.sh armv7-unknown-linux-musleabihf
bash .github/musl_build.sh i686-unknown-linux-musl
```

- Manylinux

For Manylinux compilation, refer to [manylinux](https://github.com/PyO3/maturin?tab=readme-ov-file#manylinux-and-auditwheel).

## Impersonate

In fact, most device models share the same `TLS`/`HTTP2` configuration, with the main difference being the `User-Agent`.

| **Browser**   | **Versions**                                                                                     |
|---------------|--------------------------------------------------------------------------------------------------|
| **Chrome**    | `Chrome100`, `Chrome101`, `Chrome104`, `Chrome105`, `Chrome106`, `Chrome107`, `Chrome108`, `Chrome109`, `Chrome114`, `Chrome116`, `Chrome117`, `Chrome118`, `Chrome119`, `Chrome120`, `Chrome123`, `Chrome124`, `Chrome126`, `Chrome127`, `Chrome128`, `Chrome129`, `Chrome130`, `Chrome131`, `Chrome132`, `Chrome133`, `Chrome134` |
| **Edge**      | `Edge101`, `Edge122`, `Edge127`, `Edge131`, `Edge134`                                                       |
| **Safari**    | `SafariIos17_2`, `SafariIos17_4_1`, `SafariIos16_5`, `Safari15_3`, `Safari15_5`, `Safari15_6_1`, `Safari16`, `Safari16_5`, `Safari17_0`, `Safari17_2_1`, `Safari17_4_1`, `Safari17_5`, `Safari18`,             `SafariIPad18`, `Safari18_2`, `Safari18_1_1`, `Safari18_3` |
| **OkHttp**    | `OkHttp3_9`, `OkHttp3_11`, `OkHttp3_13`, `OkHttp3_14`, `OkHttp4_9`, `OkHttp4_10`, `OkHttp4_12`, `OkHttp5`         |
| **Firefox**   | `Firefox109`, `Firefox117`, `Firefox128`, `Firefox133`, `Firefox135`, `FirefoxPrivate135`, `FirefoxAndroid135`, `Firefox136`, `FirefoxPrivate136`|

## Documentation

The python documentation is automatically supported by [pyo3-stub-gen](https://github.com/Jij-Inc/pyo3-stub-gen). It is not perfect. If you have any suggestions, you can submit a PR to improve it.

## Contributing

If you would like to submit your contribution, please open a [Pull Request](https://github.com/0x676e67/rnet/pulls).

## Getting help

Your question might already be answered on the [issues](https://github.com/0x676e67/rnet/issues)

## License

**rnet** © [0x676e67](https://github.com/0x676e67), Released under the [GPL-3.0](https://github.com/0x676e67/rnet/blob/main/LICENSE) License.
