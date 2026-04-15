"""pyperf-based HTTP client benchmark suite.

Each benchmark measures *concurrent* HTTP throughput: the reported value is
wall-clock time per request (after normalising by the concurrency case), so lower
is better.  req/s ≈ 1 / mean_seconds.

Usage::

    python benchmark.py                          # 3 processes, 3 values, 1 warmup
    python benchmark.py --fast                   # quick estimate (~1 process)
    python benchmark.py --rigorous               # more accurate (6 processes)
    python benchmark.py -o results.json          # save JSON for later comparison
    python benchmark.py --http-workers=64
    python benchmark.py -v                       # verbose: show each value as it lands
"""

from __future__ import annotations

import asyncio
import inspect
import sys
import threading
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from importlib.metadata import PackageNotFoundError, version
from io import BytesIO
from typing import Any, Awaitable, Callable

import pyperf

# Import all HTTP clients
import pycurl
import aiohttp
import httpx
import niquests
import requests
import curl_cffi
import curl_cffi.requests
import wreq
import wreq.blocking
import ry

try:
    import uvloop  # type: ignore
except ImportError:
    uvloop = None
else:
    uvloop.install()

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

CONCURRENT_CASES = (10, 20, 50, 100, 150)
BODY_MODE_CASES = ("non-stream", "stream")
STREAM_CHUNK_SIZE = 64 * 1024
ECHO_PATH = "/echo"
PROBE_BODY = b"benchmark-probe"


# ---------------------------------------------------------------------------
# Dataclasses
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class BenchmarkCase:
    id: str
    runner: Callable[..., None]
    supports_stream: bool = True


@dataclass(frozen=True)
class AsyncBenchmarkCase:
    id: str
    runner: Callable[[str, BodyCase, bool, int], Awaitable[None]]
    supports_stream: bool = True


@dataclass(frozen=True)
class BodyCase:
    id: str
    body: bytes
    stream_chunks: tuple[bytes, ...]


def _build_body_case(case_id: str, size: int) -> BodyCase:
    body = b"a" * size
    chunks = tuple(
        body[offset : offset + STREAM_CHUNK_SIZE]
        for offset in range(0, len(body), STREAM_CHUNK_SIZE)
    )
    return BodyCase(case_id, body, chunks)


BODY_CASES = (
    _build_body_case("1k", 1024),
    _build_body_case("10k", 10 * 1024),
    _build_body_case("64k", 64 * 1024),
    _build_body_case("128k", 128 * 1024),
    _build_body_case("1024k", 1024 * 1024),
    _build_body_case("2048k", 2 * 1024 * 1024),
    _build_body_case("4096k", 4 * 1024 * 1024),
)


class PycurlSession:
    def __init__(self):
        self.c = pycurl.Curl()
        self.content = None

    def close(self):
        self.c.close()

    def __del__(self):
        self.close()

    def post(self, url: str, body: bytes):
        buffer = BytesIO()
        self.c.setopt(pycurl.URL, url)
        self.c.setopt(pycurl.POST, 1)
        self.c.setopt(pycurl.POSTFIELDS, body)
        self.c.setopt(pycurl.POSTFIELDSIZE, len(body))
        self.c.setopt(pycurl.WRITEDATA, buffer)
        self.c.perform()
        self.content = buffer.getvalue()
        return self

    @property
    def text(self) -> bytes | None:
        return self.content


# ---------------------------------------------------------------------------
# Utility helpers
# ---------------------------------------------------------------------------


def add_package_version(packages: list[tuple[Any, ...]]) -> list[tuple[Any, ...]]:
    results = []
    for package in packages:
        name, target, *rest = package
        try:
            label = f"{name} {version(name)}"
        except PackageNotFoundError:
            label = name
        results.append((label, target, *rest))
    return results


def maybe_close(resource: Any) -> None:
    close = getattr(resource, "close", None)
    if callable(close):
        close()


async def maybe_aclose(resource: Any) -> None:
    close = getattr(resource, "close", None)
    if not callable(close):
        return
    result = close()
    if inspect.isawaitable(result):
        await result


# ---------------------------------------------------------------------------
# Concurrency helpers
# ---------------------------------------------------------------------------


def iter_body_chunks(chunks: tuple[bytes, ...]):
    for chunk in chunks:
        yield chunk


async def aiter_body_chunks(chunks: tuple[bytes, ...]):
    for chunk in chunks:
        yield chunk


def sync_request_body(body_case: BodyCase, stream: bool) -> bytes | Any:
    if stream:
        return iter_body_chunks(body_case.stream_chunks)
    return body_case.body


def async_request_body(body_case: BodyCase, stream: bool) -> bytes | Any:
    if stream:
        return aiter_body_chunks(body_case.stream_chunks)
    return body_case.body


def format_benchmark_name(
    group: str,
    client_id: str,
    body_mode: str,
    body_case: BodyCase,
    concurrency: int,
) -> str:
    return "/".join(
        (
            group,
            f"client={client_id}",
            f"upload={body_mode}",
            f"body={body_case.id}",
            f"concurrency={concurrency}",
        )
    )


def _run_concurrent_requests(
    fetch_fn: Callable[[], None], count: int, workers: int
) -> None:
    """Run *fetch_fn* concurrently *count* times using at most *workers* threads."""
    with ThreadPoolExecutor(max_workers=min(workers, count)) as executor:
        futures = [executor.submit(fetch_fn) for _ in range(count)]
        for future in as_completed(futures):
            future.result()


def run_parallel_non_session_case(
    runner_fn: Callable[[str, BodyCase, bool], None],
    url: str,
    body_case: BodyCase,
    stream: bool,
    count: int,
    workers: int,
) -> None:
    """Parallel non-session: each worker creates its own client."""
    _run_concurrent_requests(
        lambda: runner_fn(url, body_case, stream), count, workers
    )


# ---------------------------------------------------------------------------
# Sync session benchmarks  (shared client, concurrent requests)
# ---------------------------------------------------------------------------


def requests_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int, workers: int
) -> None:
    with requests.Session() as s:
        _run_concurrent_requests(
            lambda: s.post(url, data=sync_request_body(body_case, stream)).content,
            count,
            workers,
        )
        s.close()


def httpx_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int, workers: int
) -> None:
    with httpx.Client() as s:
        _run_concurrent_requests(
            lambda: s.post(url, content=sync_request_body(body_case, stream)).content,
            count,
            workers,
        )
        s.close()


def niquests_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int, workers: int
) -> None:
    with niquests.Session() as s:
        _run_concurrent_requests(
            lambda: s.post(url, data=sync_request_body(body_case, stream)).content,
            count,
            workers,
        )
        s.close()


def curl_cffi_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int, workers: int
) -> None:
    with curl_cffi.requests.Session() as s:
        _run_concurrent_requests(
            lambda: s.post(url, data=body_case.body).content,
            count,
            workers,
        )
        s.close()


def wreq_blocking_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int, workers: int
) -> None:
    with wreq.blocking.Client() as s:
        _run_concurrent_requests(
            lambda: s.post(url, body=sync_request_body(body_case, stream)).bytes(),
            count,
            workers,
        )
        s.close()


def pycurl_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int, workers: int
) -> None:
    # pycurl Curl handles are not thread-safe; use thread-local storage so each
    # worker thread gets its own handle that persists across its assigned requests.
    _local = threading.local()

    def _fetch() -> None:
        if not hasattr(_local, "curl"):
            _local.curl = pycurl.Curl()
        buf = BytesIO()
        _local.curl.setopt(pycurl.URL, url)
        _local.curl.setopt(pycurl.POST, 1)
        _local.curl.setopt(pycurl.POSTFIELDS, body_case.body)
        _local.curl.setopt(pycurl.POSTFIELDSIZE, len(body_case.body))
        _local.curl.setopt(pycurl.WRITEDATA, buf)
        _local.curl.perform()

    _run_concurrent_requests(_fetch, count, workers)


def ry_blocking_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int, workers: int
) -> None:
    s = ry.BlockingClient()
    try:
        _run_concurrent_requests(
            lambda: s.post(url, body=sync_request_body(body_case, stream)).bytes(),
            count,
            workers,
        )
    finally:
        maybe_close(s)


# ---------------------------------------------------------------------------
# Sync non-session benchmarks  (one request per call; parallelism is external)
# ---------------------------------------------------------------------------


def requests_non_session_test(url: str, body_case: BodyCase, stream: bool) -> None:
    with requests.post(url, data=sync_request_body(body_case, stream)) as resp:
        resp.content
        resp.close()


def httpx_non_session_test(url: str, body_case: BodyCase, stream: bool) -> None:
    resp = httpx.post(url, content=sync_request_body(body_case, stream))
    _ = resp.content
    resp.close()


def niquests_non_session_test(url: str, body_case: BodyCase, stream: bool) -> None:
    with niquests.post(url, data=sync_request_body(body_case, stream)) as resp:
        _ = resp.content
        resp.close()


def curl_cffi_non_session_test(url: str, body_case: BodyCase, stream: bool) -> None:
    resp = curl_cffi.requests.post(url, data=body_case.body)
    _ = resp.content
    resp.close()


def wreq_blocking_non_session_test(
    url: str, body_case: BodyCase, stream: bool
) -> None:
    with wreq.blocking.post(url, body=sync_request_body(body_case, stream)) as resp:
        resp.bytes()
        resp.close()


def pycurl_non_session_test(url: str, body_case: BodyCase, stream: bool) -> None:
    s = PycurlSession()
    try:
        s.post(url, body_case.body).content
    finally:
        maybe_close(s)


def ry_blocking_non_session_test(url: str, body_case: BodyCase, stream: bool) -> None:
    s = ry.BlockingClient()
    try:
        s.post(url, body=sync_request_body(body_case, stream)).bytes()
    finally:
        maybe_close(s)


# ---------------------------------------------------------------------------
# Async session benchmarks  (shared client, asyncio.gather)
# ---------------------------------------------------------------------------


async def httpx_async_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    async with httpx.AsyncClient() as s:
        await asyncio.gather(
            *[
                s.post(url, content=async_request_body(body_case, stream))
                for _ in range(count)
            ]
        )


async def aiohttp_async_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    async with aiohttp.ClientSession() as s:

        async def _fetch() -> None:
            async with await s.post(url, data=async_request_body(body_case, stream)) as resp:
                await resp.read()

        await asyncio.gather(*[_fetch() for _ in range(count)])


async def niquests_async_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    s = niquests.AsyncSession()
    try:

        async def _fetch() -> None:
            resp = await s.post(url, data=body_case.body)
            _ = resp.content

        await asyncio.gather(*[_fetch() for _ in range(count)])
    finally:
        await maybe_aclose(s)


async def wreq_async_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    s = wreq.Client()
    try:

        async def _fetch() -> None:
            resp = await s.post(url, body=async_request_body(body_case, stream))
            await resp.bytes()

        await asyncio.gather(*[_fetch() for _ in range(count)])
    finally:
        await maybe_aclose(s)


async def curl_cffi_async_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    s = curl_cffi.requests.AsyncSession()
    try:

        async def _fetch() -> None:
            resp = await s.post(url, data=body_case.body)
            _ = resp.content

        await asyncio.gather(*[_fetch() for _ in range(count)])
    finally:
        await s.close()


async def ry_async_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    s = ry.HttpClient()
    try:

        async def _fetch():
            resp = await s.post(url, body=async_request_body(body_case, stream))
            return await resp.bytes()

        await asyncio.gather(*[_fetch() for _ in range(count)])
    finally:
        await maybe_aclose(s)


# ---------------------------------------------------------------------------
# Async non-session benchmarks  (one request per call; parallelism is external)
# ---------------------------------------------------------------------------


async def httpx_async_non_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    async def _fetch() -> None:
        async with httpx.AsyncClient() as s:
            resp = await s.post(url, content=async_request_body(body_case, stream))
            _ = resp.content

    await asyncio.gather(*[_fetch() for _ in range(count)])


async def aiohttp_async_non_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    async def _fetch() -> None:
        async with aiohttp.ClientSession() as s:
            async with await s.post(url, data=async_request_body(body_case, stream)) as resp:
                await resp.read()

    await asyncio.gather(*[_fetch() for _ in range(count)])


async def niquests_async_non_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    async def _fetch() -> None:
        async with niquests.AsyncSession() as s:
            resp = await s.post(url, data=body_case.body)
            _ = resp.content

    await asyncio.gather(*[_fetch() for _ in range(count)])


async def wreq_async_non_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    async def _fetch() -> None:
        async with wreq.Client() as s:
            resp = await s.post(url, body=async_request_body(body_case, stream))
            await resp.bytes()

    await asyncio.gather(*[_fetch() for _ in range(count)])


async def curl_cffi_async_non_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    async def _fetch() -> None:
        async with curl_cffi.requests.AsyncSession() as s:
            resp = await s.post(url, data=body_case.body)
            _ = resp.content

    await asyncio.gather(*[_fetch() for _ in range(count)])


async def ry_async_non_session_test(
    url: str, body_case: BodyCase, stream: bool, count: int
) -> None:
    async def _fetch() -> None:
        s = ry.HttpClient()
        try:
            resp = await s.post(url, body=async_request_body(body_case, stream))
            await resp.bytes()
        finally:
            await maybe_aclose(s)

    await asyncio.gather(*[_fetch() for _ in range(count)])


# ---------------------------------------------------------------------------
# Benchmark case builders
# ---------------------------------------------------------------------------


def build_sync_session_cases() -> list[BenchmarkCase]:
    cases = []
    if httpx is not None:
        cases.append(("httpx", httpx_session_test, True))
    if requests is not None:
        cases.append(("requests", requests_session_test, True))
    if niquests is not None:
        cases.append(("niquests", niquests_session_test, True))
    if curl_cffi.requests is not None:
        cases.append(("curl_cffi", curl_cffi_session_test, False))
    if pycurl is not None:
        cases.append(("pycurl", pycurl_session_test, False))
    if ry is not None:
        cases.append(("ry", ry_blocking_session_test, True))
    if wreq.blocking is not None:
        cases.append(("wreq", wreq_blocking_session_test, True))
    return [
        BenchmarkCase(name, runner, supports_stream)
        for name, runner, supports_stream in add_package_version(cases)
    ]


def build_sync_non_session_cases() -> list[BenchmarkCase]:
    cases = []
    if httpx is not None:
        cases.append(("httpx", httpx_non_session_test, True))
    if requests is not None:
        cases.append(("requests", requests_non_session_test, True))
    if niquests is not None:
        cases.append(("niquests", niquests_non_session_test, True))
    if curl_cffi.requests is not None:
        cases.append(("curl_cffi", curl_cffi_non_session_test, False))
    if pycurl is not None:
        cases.append(("pycurl", pycurl_non_session_test, False))
    if ry is not None:
        cases.append(("ry", ry_blocking_non_session_test, True))
    if wreq.blocking is not None:
        cases.append(("wreq", wreq_blocking_non_session_test, True))
    return [
        BenchmarkCase(name, runner, supports_stream)
        for name, runner, supports_stream in add_package_version(cases)
    ]


def build_async_session_cases() -> list[AsyncBenchmarkCase]:
    cases = []
    if httpx is not None:
        cases.append(("httpx", httpx_async_session_test, True))
    if niquests is not None:
        cases.append(("niquests", niquests_async_session_test, False))
    if curl_cffi.requests is not None:
        cases.append(("curl_cffi", curl_cffi_async_session_test, False))
    if aiohttp is not None:
        cases.append(("aiohttp", aiohttp_async_session_test, True))
    if ry is not None:
        cases.append(("ry", ry_async_session_test, True))
    if wreq is not None:
        cases.append(("wreq", wreq_async_session_test, True))
    return [
        AsyncBenchmarkCase(name, runner, supports_stream)
        for name, runner, supports_stream in add_package_version(cases)
    ]


def build_async_non_session_cases() -> list[AsyncBenchmarkCase]:
    cases = []
    if httpx is not None:
        cases.append(("httpx", httpx_async_non_session_test, True))
    if niquests is not None:
        cases.append(("niquests", niquests_async_non_session_test, False))
    if curl_cffi.requests is not None:
        cases.append(("curl_cffi", curl_cffi_async_non_session_test, False))
    if aiohttp is not None:
        cases.append(("aiohttp", aiohttp_async_non_session_test, True))
    if ry is not None:
        cases.append(("ry", ry_async_non_session_test, True))
    if wreq is not None:
        cases.append(("wreq", wreq_async_non_session_test, True))
    return [
        AsyncBenchmarkCase(name, runner, supports_stream)
        for name, runner, supports_stream in add_package_version(cases)
    ]


SYNC_SESSION_CASES = build_sync_session_cases()
SYNC_NON_SESSION_CASES = build_sync_non_session_cases()
ASYNC_SESSION_CASES = build_async_session_cases()
ASYNC_NON_SESSION_CASES = build_async_non_session_cases()


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------


def main() -> int:
    # Quick server probe before spawning any subprocesses.
    if "--worker" not in sys.argv:
        base_url_probe = "http://127.0.0.1:8000"
        for arg in sys.argv[1:]:
            if arg.startswith("--http-base-url="):
                base_url_probe = arg.split("=", 1)[1]
                break
        probe_url = f"{base_url_probe.rstrip('/')}{ECHO_PATH}"
        try:
            probe_request = urllib.request.Request(
                probe_url,
                data=PROBE_BODY,
                headers={"Content-Type": "application/octet-stream"},
                method="POST",
            )
            with urllib.request.urlopen(probe_request, timeout=2) as resp:
                echoed_body = resp.read()
                if echoed_body != PROBE_BODY:
                    raise OSError("echo probe returned an unexpected body")
        except (OSError, urllib.error.URLError) as exc:
            print(
                f"ERROR: benchmark server unavailable at {probe_url}: {exc}",
                file=sys.stderr,
            )
            print("Start bench/server.py before running benchmarks.", file=sys.stderr)
            return 1

    # 3 processes × 3 values × 1 warmup = 9 measurements per benchmark.
    # Pass --fast for a quick estimate or --rigorous for higher confidence.
    runner = pyperf.Runner(processes=3, values=3, warmups=1)
    runner.argparser.add_argument(
        "--http-base-url",
        default="http://127.0.0.1:8000",
        metavar="URL",
        help="Base URL for the benchmark server. (default: %(default)s)",
    )
    runner.argparser.add_argument(
        "--http-workers",
        type=int,
        default=None,
        metavar="N",
        help="Override thread pool size for sync benchmarks. Defaults to the concurrency case.",
    )

    args = runner.parse_args()
    base_url: str = args.http_base_url.rstrip("/")
    sync_workers_override: int | None = args.http_workers
    target_url = f"{base_url}{ECHO_PATH}"

    # Register all benchmarks.  pyperf assigns each a task ID in order;
    # worker subprocesses run only the specific task they are assigned.
    # inner_loops=concurrency normalises the reported time to per-request,
    # so the reported mean is time/request and req/s = concurrency / mean.

    for concurrency in CONCURRENT_CASES:
        sync_workers = sync_workers_override or concurrency

        for body_mode in BODY_MODE_CASES:
            stream = body_mode == "stream"

            for body_case in BODY_CASES:
                for case in SYNC_NON_SESSION_CASES:
                    if stream and not case.supports_stream:
                        continue
                    runner.bench_func(
                        format_benchmark_name(
                            "sync-non-session",
                            case.id,
                            body_mode,
                            body_case,
                            concurrency,
                        ),
                        run_parallel_non_session_case,
                        case.runner,
                        target_url,
                        body_case,
                        stream,
                        concurrency,
                        sync_workers,
                        inner_loops=concurrency,
                    )

            for body_case in BODY_CASES:
                for case in SYNC_SESSION_CASES:
                    if stream and not case.supports_stream:
                        continue
                    runner.bench_func(
                        format_benchmark_name(
                            "sync-session",
                            case.id,
                            body_mode,
                            body_case,
                            concurrency,
                        ),
                        case.runner,
                        target_url,
                        body_case,
                        stream,
                        concurrency,
                        sync_workers,
                        inner_loops=concurrency,
                    )

            for body_case in BODY_CASES:
                for case in ASYNC_NON_SESSION_CASES:
                    if stream and not case.supports_stream:
                        continue
                    runner.bench_async_func(
                        format_benchmark_name(
                            "async-non-session",
                            case.id,
                            body_mode,
                            body_case,
                            concurrency,
                        ),
                        case.runner,
                        target_url,
                        body_case,
                        stream,
                        concurrency,
                        inner_loops=concurrency,
                    )

            for body_case in BODY_CASES:
                for case in ASYNC_SESSION_CASES:
                    if stream and not case.supports_stream:
                        continue
                    runner.bench_async_func(
                        format_benchmark_name(
                            "async-session",
                            case.id,
                            body_mode,
                            body_case,
                            concurrency,
                        ),
                        case.runner,
                        target_url,
                        body_case,
                        stream,
                        concurrency,
                        inner_loops=concurrency,
                    )

    return 0


if __name__ == "__main__":
    sys.exit(main())
