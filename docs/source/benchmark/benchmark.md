## Benchmark Guide

This page is not meant to sell a benchmark result. Its purpose is to show how to run the benchmark in a way that is reproducible and easy to interpret.

The `bench` module contains two scripts:

- `bench/server.py`: a local benchmark server that returns fixed 20 KB, 50 KB, and 200 KB response bodies.
- `bench/benchmark.py`: a `pyperf`-based runner that covers sync and async clients, with both session and non-session scenarios.

---

## 1. What The Benchmark Covers

### Sync clients

- `requests`
- `httpx`
- `niquests`
- `curl_cffi`
- `pycurl`
- `ry`
- `wreq.blocking`

### Async clients

- `httpx`
- `niquests`
- `curl_cffi`
- `aiohttp`
- `ry`
- `wreq`

### Four benchmark groups

- `sync-non-session`: creates a fresh client for each request, with concurrency driven by an external thread pool.
- `sync-session`: reuses a single client or session, still driven by an external thread pool.
- `async-non-session`: creates a fresh async client inside each coroutine.
- `async-session`: reuses a single async client and runs requests via `asyncio.gather`.

Each group is tested against the `20k`, `50k`, and `200k` payload endpoints.

---

## 2. Environment Setup

Use a clean environment if you want stable numbers. Running the benchmark on a machine that is already busy will make the results noisy.

```bash
pip install -e .[bench]
```

If the project is already installed via `maturin develop`, it is still worth checking that the benchmark dependencies are present, especially `pyperf`, `aiohttp`, `curl_cffi`, and `pycurl`.

---

## 3. Start The Local Server

```bash
cd bench
python server.py --host 127.0.0.1 --port 8000
```

Available options:

- `--workers`: number of worker processes. By default, this uses the CPU count.
- `--host`: defaults to `0.0.0.0`. For local runs, `127.0.0.1` is usually the clearer choice.
- `--port`: defaults to `8000`.

The server exposes three endpoints:

- `/20k`
- `/50k`
- `/200k`

Before running any benchmark cases, `benchmark.py` probes `base_url/20k`. If the server is not reachable, it exits immediately with an error.

---

## 4. Run The Benchmark

Run the benchmark from another terminal:

```bash
cd bench
python benchmark.py
```

### Common modes

```bash
# Quick pass for a rough trend
python benchmark.py --fast

# More stable numbers with a larger sample
python benchmark.py --rigorous

# Save results for later comparison
python benchmark.py -o results.json
```

### Common options

```bash
python benchmark.py \
	--http-base-url=http://127.0.0.1:8000 \
	--http-requests=100 \
	--http-workers=32
```

- `--http-base-url`: base URL of the benchmark server.
- `--http-requests`: number of concurrent HTTP requests issued per benchmark call.
- `--http-workers`: thread pool size for sync benchmarks only.

Default values:

- `http-base-url = http://127.0.0.1:8000`
- `http-requests = 100`
- `http-workers = 32`

---

## 5. How To Read The Output

`pyperf` reports the mean and standard deviation for each benchmark case. One detail matters here:

- the script sets `inner_loops=http_requests` for every case.
- in practice, that means the reported mean can be read as average time per request.

If you want a rough throughput estimate, use:

$$
\mathrm{req/s} \approx \frac{1}{\mathrm{mean\_seconds}}
$$

If you prefer to think in total throughput, you can derive it from total time and request count, but do not mix up per-request latency with total time per benchmark round.

---

## 6. A Reasonable Comparison Workflow

To avoid drawing conclusions from a single lucky run, use a repeatable process:

1. Fix the machine power profile and CPU frequency policy.
2. Close unrelated processes, especially browsers and sync clients.
3. Run `--fast` first to catch obvious anomalies.
4. Run `--rigorous` and save the result file.
5. After code changes, rerun with the exact same parameters before comparing anything.

Run at least three rounds and compare the median trend. The best single run is usually the least useful one.

---

## 7. Common Mistakes

- Comparing results produced with different parameters.
The same comparison must keep `http-requests`, `http-workers`, payload size, and server worker count fixed.

- Looking at only one payload size.
Small responses such as `20k` and larger ones such as `200k` often expose different bottlenecks.

- Treating session results as a substitute for non-session results.
These scenarios have different cost models, so the conclusions are not interchangeable.

- Extrapolating loopback results to the public internet.
This benchmark is mainly useful for comparing client-side overhead under controlled conditions. It is not a direct model of real-world end-to-end latency.

---

## 8. Recommended Baseline Commands

```bash
# Terminal A
cd bench
python server.py --host 127.0.0.1 --port 8000 --workers 8

# Terminal B
cd bench
python benchmark.py --rigorous --http-requests=100 --http-workers=32 -o bench-rigorous.json
```

If the goal is a quick regression signal for CI, a lighter run is usually enough:

```bash
python benchmark.py --fast --http-requests=50 --http-workers=16 -o bench-fast.json
```

---

## 9. Summary

The value of this benchmark is not an absolute ranking. It is a controlled way to compare the relative performance of Python HTTP clients under the same machine, the same server, and the same request model.
