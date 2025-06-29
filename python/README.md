# rnet Python Module Overview

`rnet` is a high-performance, async-first HTTP(S) and WebSocket client library for Python, powered by a Rust backend. It is designed for modern web automation, scraping, and networking scenarios, providing both ergonomic Python APIs and the speed and safety of Rust.

## Features

- **Async and Sync HTTP(S) Client**: Unified API for both asynchronous and blocking HTTP requests.
- **WebSocket Support**: Native async WebSocket client with full protocol support.
- **Advanced Proxy Support**: HTTP, HTTPS, SOCKS4/5, and custom proxy authentication.
- **HeaderMap and Cookie Management**: Flexible header and cookie handling, compatible with modern web standards.
- **Impersonation**: Easily switch between browser-like client fingerprints (user-agent, TLS, etc.).
- **Connection Pooling and Timeout**: Fine-grained control over connection reuse and timeouts.
- **Comprehensive Exception Hierarchy**: All network and protocol errors are mapped to Python exceptions for robust error handling.
- **Type Hints and IDE Support**: Complete `.pyi` stubs for all public APIs, enabling full autocompletion and type checking in modern editors.

## Usage Example

```python
import asyncio
from rnet import Client, Impersonate, Proxy

async def main():
    client = Client(
        impersonate=Impersonate.Chrome120,
        proxies=[Proxy.all("http://127.0.0.1:8080")],
    )
    resp = await client.get("https://httpbin.org/get")
    print(await resp.text())

if __name__ == "__main__":
    asyncio.run(main())
```

## Type Hints and Editor Support

- All public classes and functions are fully type-annotated.
- `.pyi` stub files are provided for all modules, ensuring autocompletion and type checking in VSCode, PyCharm, etc.
- For best experience, ensure your editor is configured to recognize the `rnet` package and its stubs.

## Notes

- This package is implemented as a Rust extension module for Python. All performance-critical logic is in Rust, while the Python layer provides a clean, Pythonic API.
- If you encounter IDE warnings about unresolved imports (e.g., `rnet.header`), this is a limitation of static analysis for native extensions. Functionality and type hints are not affected.

---

For more details, see the API documentation or the examples directory.
