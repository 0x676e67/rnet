# rnet Python Module Overview

`rnet` is a high-performance, async-first HTTP(S) and WebSocket client library for Python, powered by a Rust backend. It is designed for modern web automation, scraping, and networking scenarios, providing both ergonomic Python APIs and the speed and safety of Rust.

## Usage Example

```python
import asyncio
from rnet import Client, Emulation, Proxy

async def main():
    client = Client(
        emulation=Emulation.Chrome120,
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
