"""
HTTP Client Benchmark Tool

This module provides comprehensive benchmarking for various HTTP client libraries.
Each client has dedicated test methods to eliminate runtime overhead from dynamic dispatch.
"""

import argparse
import asyncio
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from importlib.metadata import version
from io import BytesIO
from typing import Any, Dict, List, Tuple

import pandas as pd
import pycurl

# Import all HTTP clients
import aiohttp
import httpx
import niquests
import requests
import tls_client
import curl_cffi
import curl_cffi.requests
import rnet
import rnet.blocking
import uvloop
import ry

from chart import plot_benchmark_multi

# Install uvloop for better async performance
uvloop.install()


# =============================================================================
# Helper Classes
# =============================================================================


class PycurlSession:
    """Wrapper for pycurl to match session interface"""

    def __init__(self):
        self.c = pycurl.Curl()
        self.content = None

    def close(self):
        self.c.close()

    def __del__(self):
        self.close()

    def get(self, url):
        buffer = BytesIO()
        self.c.setopt(pycurl.URL, url)
        self.c.setopt(pycurl.WRITEDATA, buffer)
        self.c.perform()
        self.content = buffer.getvalue()
        return self

    @property
    def text(self):
        return self.content


# =============================================================================
# Utility Functions
# =============================================================================


def add_package_version(packages: List[Tuple[str, Any]]) -> List[Tuple[str, Any]]:
    """Add version information to package names"""
    return [(f"{name} {version(name)}", cls) for name, cls in packages]


def record_test_result(
    name: str,
    session_type: str,
    url: str,
    start_time: float,
    cpu_start: float,
    threads: int | None = None,
) -> Dict[str, Any]:
    """Record benchmark test results"""
    dur = round((time.perf_counter() - start_time) * 1000, 2)  # Convert to milliseconds
    cpu_dur = round((time.process_time() - cpu_start) * 1000, 2)  # Convert to milliseconds

    result = {
        "name": name,
        "session": session_type,
        "size": url.split("/")[-1],
        "time": dur,
        "cpu_time": cpu_dur,
    }

    if threads is not None:
        result["threads"] = threads

    return result


# =============================================================================
# Sync Client Implementations - Session Mode
# =============================================================================


def requests_session_test(url: str, count: int):
    """Benchmark requests.Session"""
    s = requests.Session()
    try:
        for _ in range(count):
            s.get(url).content
    finally:
        s.close()


def httpx_session_test(url: str, count: int):
    """Benchmark httpx.Client"""
    s = httpx.Client()
    try:
        for _ in range(count):
            s.get(url).content
    finally:
        s.close()


def niquests_session_test(url: str, count: int):
    """Benchmark niquests.Session"""
    s = niquests.Session()
    try:
        for _ in range(count):
            s.get(url).content
    finally:
        s.close()


def tls_client_session_test(url: str, count: int):
    """Benchmark tls_client.Session"""
    s = tls_client.Session()
    try:
        for _ in range(count):
            s.get(url).content
    finally:
        s.close()


def curl_cffi_session_test(url: str, count: int):
    """Benchmark curl_cffi.requests.Session"""
    s = curl_cffi.requests.Session()
    try:
        for _ in range(count):
            s.get(url).content
    finally:
        s.close()


def rnet_blocking_session_test(url: str, count: int):
    """Benchmark rnet.blocking.Client"""
    s = rnet.blocking.Client()
    for _ in range(count):
        s.get(url).bytes()


def pycurl_session_test(url: str, count: int):
    """Benchmark PycurlSession"""
    s = PycurlSession()
    try:
        for _ in range(count):
            s.get(url).content
    finally:
        s.close()


def ry_blocking_session_test(url: str, count: int):
    """Benchmark ry blocking Client"""
    s = ry.BlockingClient()
    for _ in range(count):
        s.get(url).bytes()


# =============================================================================
# Sync Client Implementations - Non-Session Mode
# =============================================================================


def requests_non_session_test(url: str, count: int):
    """Benchmark requests without session"""
    for _ in range(count):
        s = requests.Session()
        try:
            s.get(url).content
        finally:
            s.close()


def httpx_non_session_test(url: str, count: int):
    """Benchmark httpx without session"""
    for _ in range(count):
        s = httpx.Client()
        try:
            s.get(url).content
        finally:
            s.close()


def niquests_non_session_test(url: str, count: int):
    """Benchmark niquests without session"""
    for _ in range(count):
        s = niquests.Session()
        try:
            s.get(url).content
        finally:
            s.close()


def tls_client_non_session_test(url: str, count: int):
    """Benchmark tls_client without session"""
    for _ in range(count):
        s = tls_client.Session()
        try:
            s.get(url).content
        finally:
            s.close()


def curl_cffi_non_session_test(url: str, count: int):
    """Benchmark curl_cffi without session"""
    for _ in range(count):
        s = curl_cffi.requests.Session()
        try:
            s.get(url).content
        finally:
            s.close()


def rnet_blocking_non_session_test(url: str, count: int):
    """Benchmark rnet.blocking without session"""
    for _ in range(count):
        s = rnet.blocking.Client()
        s.get(url).bytes()


def pycurl_non_session_test(url: str, count: int):
    """Benchmark pycurl without session"""
    for _ in range(count):
        s = PycurlSession()
        try:
            s.get(url).content
        finally:
            s.close()


def ry_blocking_non_session_test(url: str, count: int):
    """Benchmark ry blocking without session"""
    for _ in range(count):
        s = ry.BlockingClient()
        s.get(url).bytes()


# =============================================================================
# Async Client Implementations - Session Mode
# =============================================================================


async def httpx_async_session_test(url: str, count: int):
    """Benchmark httpx.AsyncClient with session"""
    async with httpx.AsyncClient() as s:

        async def _fetch():
            resp = await s.get(url)
            return resp.content

        tasks = [_fetch() for _ in range(count)]
        await asyncio.gather(*tasks)


async def aiohttp_async_session_test(url: str, count: int):
    """Benchmark aiohttp.ClientSession"""
    async with aiohttp.ClientSession() as s:

        async def _fetch():
            async with await s.get(url) as resp:
                return await resp.read()

        tasks = [_fetch() for _ in range(count)]
        await asyncio.gather(*tasks)


async def niquests_async_session_test(url: str, count: int):
    """Benchmark niquests.AsyncSession"""
    s = niquests.AsyncSession()

    async def _fetch():
        resp = await s.get(url)
        return resp.content

    tasks = [_fetch() for _ in range(count)]
    await asyncio.gather(*tasks)


async def rnet_async_session_test(url: str, count: int):
    """Benchmark rnet.Client with session"""
    s = rnet.Client()

    async def _fetch():
        resp = await s.get(url)
        return await resp.bytes()

    tasks = [_fetch() for _ in range(count)]
    await asyncio.gather(*tasks)


async def curl_cffi_async_session_test(url: str, count: int):
    """Benchmark curl_cffi.requests.AsyncSession"""
    s = curl_cffi.requests.AsyncSession()
    try:

        async def _fetch():
            resp = await s.get(url)
            return resp.text

        tasks = [_fetch() for _ in range(count)]
        await asyncio.gather(*tasks)
    finally:
        await s.close()


async def ry_async_session_test(url: str, count: int):
    """Benchmark ry.HttpClient"""
    s = ry.HttpClient()

    async def _fetch():
        resp = await s.get(url)
        return await resp.bytes()

    tasks = [_fetch() for _ in range(count)]
    await asyncio.gather(*tasks)


# =============================================================================
# Async Client Implementations - Non-Session Mode
# =============================================================================


async def httpx_async_non_session_test(url: str, count: int):
    """Benchmark httpx.AsyncClient without session"""
    for _ in range(count):
        async with httpx.AsyncClient() as s:
            resp = await s.get(url)
            resp.text


async def aiohttp_async_non_session_test(url: str, count: int):
    """Benchmark aiohttp without session"""
    for _ in range(count):
        async with aiohttp.ClientSession() as s:
            async with await s.get(url) as resp:
                await resp.read()


async def niquests_async_non_session_test(url: str, count: int):
    """Benchmark niquests without session"""
    for _ in range(count):
        s = niquests.AsyncSession()
        try:
            resp = await s.get(url)
            resp.content
        finally:
            await s.close()


async def rnet_async_non_session_test(url: str, count: int):
    """Benchmark rnet without session"""
    for _ in range(count):
        s = rnet.Client()
        resp = await s.get(url)
        await resp.bytes()


async def curl_cffi_async_non_session_test(url: str, count: int):
    """Benchmark curl_cffi without session"""
    for _ in range(count):
        s = curl_cffi.requests.AsyncSession()
        try:
            resp = await s.get(url)
            resp.text
        finally:
            await s.close()


async def ry_async_non_session_test(url: str, count: int):
    """Benchmark ry without session"""
    for _ in range(count):
        s = ry.HttpClient()
        resp = await s.get(url)
        await resp.bytes()


# =============================================================================
# Test Mappings
# =============================================================================

# Mapping of sync client classes to their dedicated test functions
SYNC_SESSION_TESTS = {
    requests.Session: requests_session_test,
    httpx.Client: httpx_session_test,
    niquests.Session: niquests_session_test,
    tls_client.Session: tls_client_session_test,
    curl_cffi.requests.Session: curl_cffi_session_test,
    rnet.blocking.Client: rnet_blocking_session_test,
    PycurlSession: pycurl_session_test,
    ry.BlockingClient: ry_blocking_session_test,
}

SYNC_NON_SESSION_TESTS = {
    requests.Session: requests_non_session_test,
    httpx.Client: httpx_non_session_test,
    niquests.Session: niquests_non_session_test,
    tls_client.Session: tls_client_non_session_test,
    curl_cffi.requests.Session: curl_cffi_non_session_test,
    rnet.blocking.Client: rnet_blocking_non_session_test,
    PycurlSession: pycurl_non_session_test,
    ry.BlockingClient: ry_blocking_non_session_test,
}

# Mapping of async client classes to their dedicated test functions
ASYNC_SESSION_TESTS = {
    httpx.AsyncClient: httpx_async_session_test,
    aiohttp.ClientSession: aiohttp_async_session_test,
    niquests.AsyncSession: niquests_async_session_test,
    rnet.Client: rnet_async_session_test,
    curl_cffi.requests.AsyncSession: curl_cffi_async_session_test,
    ry.HttpClient: ry_async_session_test,
}

ASYNC_NON_SESSION_TESTS = {
    httpx.AsyncClient: httpx_async_non_session_test,
    aiohttp.ClientSession: aiohttp_async_non_session_test,
    niquests.AsyncSession: niquests_async_non_session_test,
    rnet.Client: rnet_async_non_session_test,
    curl_cffi.requests.AsyncSession: curl_cffi_async_non_session_test,
    ry.HttpClient: ry_async_non_session_test,
}


# =============================================================================
# Test Runners
# =============================================================================


def run_sync_tests(
    packages: List[Tuple[str, Any]], url: str, requests_number: int
) -> List[Dict[str, Any]]:
    """Run synchronous benchmark tests"""
    results = []

    for name, session_class in packages:
        # Test with session
        if session_class in SYNC_SESSION_TESTS:
            start = time.perf_counter()
            cpu_start = time.process_time()
            SYNC_SESSION_TESTS[session_class](url, requests_number)
            results.append(
                record_test_result(name, "Sync-Session", url, start, cpu_start)
            )

        # Test without session
        if session_class in SYNC_NON_SESSION_TESTS:
            start = time.perf_counter()
            cpu_start = time.process_time()
            SYNC_NON_SESSION_TESTS[session_class](url, requests_number)
            results.append(
                record_test_result(name, "Sync-NonSession", url, start, cpu_start)
            )

    return results


def run_threaded_tests(
    packages: List[Tuple[str, Any]], url: str, requests_number: int, threads: int
) -> List[Dict[str, Any]]:
    """Run multi-threaded benchmark tests"""
    results = []

    for name, session_class in packages:
        # Test with session - using ThreadPoolExecutor
        if session_class in SYNC_SESSION_TESTS:
            start = time.perf_counter()
            cpu_start = time.process_time()
            with ThreadPoolExecutor(threads) as executor:
                futures = [
                    executor.submit(
                        SYNC_SESSION_TESTS[session_class],
                        url,
                        requests_number // threads,
                    )
                    for _ in range(threads)
                ]
                for f in as_completed(futures):
                    f.result()
            results.append(
                record_test_result(
                    name, "Threaded-Session", url, start, cpu_start, threads
                )
            )

        # Test without session - using ThreadPoolExecutor
        if session_class in SYNC_NON_SESSION_TESTS:
            start = time.perf_counter()
            cpu_start = time.process_time()
            with ThreadPoolExecutor(threads) as executor:
                futures = [
                    executor.submit(
                        SYNC_NON_SESSION_TESTS[session_class],
                        url,
                        requests_number // threads,
                    )
                    for _ in range(threads)
                ]
                for f in as_completed(futures):
                    f.result()
            results.append(
                record_test_result(
                    name, "Threaded-NonSession", url, start, cpu_start, threads
                )
            )

    return results


def run_async_tests(
    async_packages: List[Tuple[str, Any]], url: str, requests_number: int
) -> List[Dict[str, Any]]:
    """Run asynchronous benchmark tests"""
    results = []

    for name, session_class in async_packages:
        # Test with session
        if session_class in ASYNC_SESSION_TESTS:
            start = time.perf_counter()
            cpu_start = time.process_time()
            asyncio.run(ASYNC_SESSION_TESTS[session_class](url, requests_number))
            results.append(
                record_test_result(name, "Async-Session", url, start, cpu_start)
            )

        # Test without session
        if session_class in ASYNC_NON_SESSION_TESTS:
            start = time.perf_counter()
            cpu_start = time.process_time()
            asyncio.run(ASYNC_NON_SESSION_TESTS[session_class](url, requests_number))
            results.append(
                record_test_result(name, "Async-NonSession", url, start, cpu_start)
            )

    return results


# =============================================================================
# Main Execution
# =============================================================================


def parse_arguments():
    """Parse command line arguments"""
    parser = argparse.ArgumentParser(
        description="HTTP Client Benchmark Tool",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument(
        "--requests", "-r", type=int, default=400, help="Number of requests per test"
    )

    parser.add_argument(
        "--threads",
        "-t",
        type=int,
        nargs="+",
        default=[1, 4, 8, 16],
        help="Thread counts to test (e.g., --threads 1 2 4 8)",
    )

    parser.add_argument(
        "--output",
        "-o",
        type=str,
        default="benchmark_results.csv",
        help="Output CSV file name",
    )

    parser.add_argument(
        "--chart",
        "-c",
        type=str,
        default="benchmark_multi.png",
        help="Output chart file name",
    )

    parser.add_argument(
        "--base-url",
        type=str,
        default="http://127.0.0.1:8000",
        help="Base URL for the benchmark server",
    )

    return parser.parse_args()


def main():
    """Main benchmark execution"""
    args = parse_arguments()

    # Use command line arguments
    requests_number = args.requests
    thread_counts = args.threads

    print("Starting benchmark with:")
    print(f"  Requests per test: {requests_number}")
    print(f"  Thread counts: {thread_counts}")
    print(f"  Base URL: {args.base_url}")
    print()

    # Define sync packages
    sync_packages = [
        ("tls_client", tls_client.Session),
        ("httpx", httpx.Client),
        ("requests", requests.Session),
        ("niquests", niquests.Session),
        ("curl_cffi", curl_cffi.requests.Session),
        ("pycurl", PycurlSession),
        ("ry", ry.BlockingClient),
        ("rnet", rnet.blocking.Client),
    ]

    # Define async packages
    async_packages = [
        ("httpx", httpx.AsyncClient),
        ("niquests", niquests.AsyncSession),
        ("curl_cffi", curl_cffi.requests.AsyncSession),
        ("aiohttp", aiohttp.ClientSession),
        ("ry", ry.HttpClient),
        ("rnet", rnet.Client),
    ]

    # Add version information
    sync_packages = add_package_version(sync_packages)
    async_packages = add_package_version(async_packages)

    all_results = []

    # Run tests for different payload sizes
    for size in ["20k", "50k", "200k"]:
        url = f"{args.base_url}/{size}"
        print(f"Testing with {size} payload...")

        # Run sync tests
        all_results += run_sync_tests(sync_packages, url, requests_number)

        # Run async tests
        all_results += run_async_tests(async_packages, url, requests_number)

        # Run threaded tests
        for threads in thread_counts:
            all_results += run_threaded_tests(
                sync_packages, url, requests_number, threads
            )

    # Save results
    print(f"Saving results to {args.output}...")
    df = pd.DataFrame(all_results)
    df.to_csv(args.output, index=False)

    # Generate chart
    print(f"Generating chart {args.chart}...")
    plot_benchmark_multi(df, args.chart)

    print("Benchmark completed!")


if __name__ == "__main__":
    main()
