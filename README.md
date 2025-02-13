# rnet

[![CI](https://github.com/0x676e67/rnet/actions/workflows/style.yml/badge.svg)](https://github.com/0x676e67/rnet/actions/workflows/style.yml)
[![GitHub License](https://img.shields.io/github/license/0x676e67/rnet)](https://github.com/0x676e67/rnet/blob/main/LICENSE)

> 🚀 Help me work seamlessly with open source sharing by [sponsoring me on GitHub](https://github.com/0x676e67/0x676e67/blob/main/SPONSOR.md)

Python HTTP Client with Black Magic, powered by FFI from [rquest](https://github.com/0x676e67/rquest).

## Wheels

* Linux (Musl/GNU): `x86_64`,`aarch64`,`armv7`,`i686`

* macOS: `x86_64`,`aarch64`

* Windows: `x86_64`,`i686`

## Example

This asynchronous example demonstrates how to make a simple GET request using the `rnet` library.

```python
import asyncio
import rnet
from rnet import Impersonate


async def main():
    resp = await rnet.get(
        "https://tls.peet.ws/api/all",
        impersonate=Impersonate.Firefox133,
        timeout=10,
    )
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers.to_dict())
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)

    text_content = await resp.text()
    print("Text: ", text_content)

if __name__ == "__main__":
    asyncio.run(main())
```

Additional learning resources include:

- [API Documentation](https://github.com/0x676e67/rnet/blob/main/rnet.pyi)
- [Repository Examples](https://github.com/0x676e67/rnet/tree/main/examples)

## Building

- Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Install maturin and uv

```bash
pip install maturin
pip install uv

uv venv
source .venv/bin/activate
```

- Development

```bash
maturin develop --uv
python3 examples/client.py
```

- Release wheels

```bash
maturin build --release
```

## Contributing

If you would like to submit your contribution, please open a [Pull Request](https://github.com/0x676e67/rnet/pulls).

## Getting help

Your question might already be answered on the [issues](https://github.com/0x676e67/rnet/issues)

## License

**rnet** © [0x676e67](https://github.com/0x676e67), Released under the [GPL-3.0](./LICENSE) License.
