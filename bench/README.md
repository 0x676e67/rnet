# Benchmark

benchmark between wreq and other python http clients

Sync clients
------

- curl_cffi
- requests
- niquests
- pycurl
- httpx
- wreq
- ry

Async clients
------

- curl_cffi
- httpx
- niquests
- aiohttp
- wreq
- ry

Target
------


All the clients run with session/client enabled.

## Run benchmark

```bash
# Install project + benchmark dependencies
pip install -e .[bench]

# Start server
python server.py

# Run benchmark suite
python benchmark.py
```
