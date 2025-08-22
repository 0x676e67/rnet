# Benchmark

benchmark between rnet and other python http clients

Sync clients
------

- curl_cffi
- requests
- niquests
- pycurl
- [python-tls-client](https://github.com/FlorianREGAZ/Python-Tls-Client.git)
- httpx
- rnet

Async clients
------

- curl_cffi
- httpx
- aiohttp
- rnet

Target
------


All the clients run with session/client enabled.

## Run benchmark

```bash
# Install dependencies  
pip install -r requirements.txt

# Start server
python server.py

# Start benchmark
python bench.py
```
