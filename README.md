# rnet

[![CI](https://github.com/0x676e67/rnet/actions/workflows/ci.yml/badge.svg)](https://github.com/0x676e67/rnet/actions/workflows/style.yml)
[![GitHub License](https://img.shields.io/github/license/0x676e67/rnet)](https://github.com/0x676e67/rnet/blob/main/LICENSE)
[![PyPI](https://img.shields.io/pypi/v/rnet)](https://pypi.org/project/rnet/)
![Python Version from PEP 621 TOML](https://img.shields.io/python/required-version-toml?tomlFilePath=https%3A%2F%2Fraw.githubusercontent.com%2F0x676e67%2Frnet%2Fmain%2Fpyproject.toml)
![PyPI - Format](https://img.shields.io/pypi/format/rnet)

> 🚀 Help me work seamlessly with open source sharing by [sponsoring me on GitHub](https://github.com/0x676e67/0x676e67/blob/main/SPONSOR.md)

Asynchronous Python HTTP Client with Black Magic, powered by FFI from [rquest](https://github.com/0x676e67/rquest).

## Features

- Plain, Form, JSON, urlencoded
- Header Order
- Redirect Policy
- Cookie Store
- HTTP Proxies
- Websocket Upgrade
- HTTPS via BoringSSL
- Perfectly Chrome, Safari, and Firefox

## Wheels

- Linux (MUSL/GNU-GLIBC-2.34): `x86_64`,`aarch64`,`armv7`,`i686`

- macOS: `x86_64`,`aarch64`

- Windows: `x86_64`,`i686`

## Example

This asynchronous example demonstrates how to make a simple GET request using the `rnet` library. So you need install `rnet` and run the following code:

```bash
pip install rnet
```

And then the code:

```python
import asyncio
from rnet import Impersonate, Client


async def main():
    client = Client(
        impersonate=Impersonate.Chrome131,
        user_agent="rnet",
    )
    resp = await client.get("https://tls.peet.ws/api/all")
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
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

1. Compile wheels

- Local Compilation

You need to install the BoringSSL build environment. You can refer to [boring-ci](https://github.com/cloudflare/boring/blob/master/.github/workflows/ci.yml) and [boringssl](https://github.com/google/boringssl/blob/master/BUILDING.md#build-prerequisites).

```bash
maturin build --release
```

- Musllinux

You also need to install the Docker environment. The image might be outdated, so if you need to build the image yourself, refer to [rust-cross-musl](https://github.com/0x676e67/toolchain/blob/master/rust-musl-cross/Dockerfile) and the upstream [rust-cross-musl](https://github.com/rust-cross/rust-musl-cross). The upstream [rust-cross-musl](https://github.com/rust-cross/rust-musl-cross) lacks the relevant platform linker environment variables, which you need to add yourself.
  
```bash
bash .github/musl_build.sh x86_64-unknown-linux-musl
bash .github/musl_build.sh aarch64-unknown-linux-musl
bash .github/musl_build.sh armv7-unknown-linux-musleabihf
bash .github/musl_build.sh i686-unknown-linux-musl
```

- Manylinux

For Manylinux compilation, refer to [manylinux](https://github.com/PyO3/maturin?tab=readme-ov-file#manylinux-and-auditwheel).

## Documentation

The python documentation is automatically supported by [pyo3-stub-gen](https://github.com/Jij-Inc/pyo3-stub-gen). It is not perfect. If you have any suggestions, you can submit a PR to improve it.

## Contributing

If you would like to submit your contribution, please open a [Pull Request](https://github.com/0x676e67/rnet/pulls).

## Getting help

Your question might already be answered on the [issues](https://github.com/0x676e67/rnet/issues)

## License

**rnet** © [0x676e67](https://github.com/0x676e67), Released under the [GPL-3.0](./LICENSE) License.
