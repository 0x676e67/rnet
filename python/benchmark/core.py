import asyncio
import time
from io import BytesIO
from concurrent.futures import ThreadPoolExecutor, as_completed
from importlib.metadata import version
from typing import Callable, Any, Dict, Tuple

import pycurl


class PycurlSession:
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


def add_package_version(packages):
    return [(f"{name} {version(name)}", cls) for name, cls in packages]


# Helper function: Execute a single HTTP request
def _execute_request(session, url, session_class):
    """Common logic for executing a single HTTP request"""
    if session_class.__module__ == "rnet.blocking":
        return session.get(url).bytes()
    else:
        return session.get(url).text


# Helper function: Safely close session
def _safe_close_session(session):
    """Common logic for safely closing a session"""
    if hasattr(session, "close"):
        session.close()


# Helper function: Execute async HTTP request
async def _async_execute_request(session, url, session_class):
    """Common logic for executing an async HTTP request"""
    if session_class.__module__ == "aiohttp.client":
        async with session.get(url) as resp:
            return await resp.read()
    if session_class.__name__ == "Client":
        resp = await session.get(url)
        return await resp.bytes()
    else:
        resp = await session.get(url)
        return resp.text


# Helper function: Safely close async session
async def _safe_close_async_session(session):
    """Common logic for safely closing an async session"""
    if hasattr(session, "aclose"):
        await session.aclose()
    elif hasattr(session, "close"):
        session.close()


# Helper function: Record test result
def _record_test_result(
    name: str,
    session_type: str,
    url: str,
    start_time: float,
    cpu_start: float,
    threads: int | None = None,
) -> Dict[str, Any]:
    """Common logic for recording test results"""
    dur = round(time.perf_counter() - start_time, 2)
    cpu_dur = round(time.process_time() - cpu_start, 2)

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


# Helper function: Execute timed test
def _execute_timed_test(test_func: Callable, *args) -> Tuple[float, float]:
    """Common logic for executing timed tests"""
    start = time.perf_counter()
    cpu_start = time.process_time()
    test_func(*args)
    return start, cpu_start


def session_get_test(session_class, url, requests_number):
    s = session_class()
    try:
        for _ in range(requests_number):
            _execute_request(s, url, session_class)
    finally:
        _safe_close_session(s)


def non_session_get_test(session_class, url, requests_number):
    for _ in range(requests_number):
        s = session_class()
        try:
            _execute_request(s, url, session_class)
        finally:
            _safe_close_session(s)


async def async_session_get_test(session_class, url, requests_number):
    try:
        async with session_class() as s:
            tasks = [
                _async_execute_request(s, url, session_class)
                for _ in range(requests_number)
            ]
            await asyncio.gather(*tasks)
    except TypeError:
        s = session_class()
        tasks = [
            _async_execute_request(s, url, session_class)
            for _ in range(requests_number)
        ]
        await asyncio.gather(*tasks)
        await _safe_close_async_session(s)


async def async_non_session_get_test(session_class, url, requests_number):
    for _ in range(requests_number):
        try:
            async with session_class() as s:
                await _async_execute_request(s, url, session_class)
        except TypeError:
            s = session_class()
            await _async_execute_request(s, url, session_class)
            await _safe_close_async_session(s)


# Helper function: Run test for a single package
def _run_single_package_test(
    name: str,
    session_class,
    url: str,
    requests_number: int,
    test_func: Callable,
    session_type: str,
    threads: int | None = None,
) -> Dict[str, Any]:
    """Common logic for running test on a single package"""
    start, cpu_start = _execute_timed_test(
        test_func, session_class, url, requests_number
    )
    return _record_test_result(name, session_type, url, start, cpu_start, threads)


# Helper function: Run async test for a single package
def _run_single_async_package_test(
    name: str,
    session_class,
    url: str,
    requests_number: int,
    test_func: Callable,
    session_type: str,
) -> Dict[str, Any]:
    """Common logic for running async test on a single package"""
    start = time.perf_counter()
    cpu_start = time.process_time()
    asyncio.run(test_func(session_class, url, requests_number))
    return _record_test_result(name, session_type, url, start, cpu_start)


def run_sync_tests(packages, url, requests_number):
    results = []
    for name, session_class in packages:
        # Test with session
        results.append(
            _run_single_package_test(
                name,
                session_class,
                url,
                requests_number,
                session_get_test,
                "Sync-Session",
            )
        )

        # Test without session
        results.append(
            _run_single_package_test(
                name,
                session_class,
                url,
                requests_number,
                non_session_get_test,
                "Sync-NonSession",
            )
        )
    return results


def run_threaded_tests(packages, url, requests_number, threads):
    results = []
    for name, session_class in packages:
        # Test with session - using ThreadPoolExecutor
        start = time.perf_counter()
        cpu_start = time.process_time()
        with ThreadPoolExecutor(threads) as executor:
            futures = [
                executor.submit(
                    session_get_test, session_class, url, requests_number // threads
                )
                for _ in range(threads)
            ]
            for f in as_completed(futures):
                f.result()
        results.append(
            _record_test_result(
                name, "Threaded-Session", url, start, cpu_start, threads
            )
        )

        # Test without session - using ThreadPoolExecutor
        start = time.perf_counter()
        cpu_start = time.process_time()
        with ThreadPoolExecutor(threads) as executor:
            futures = [
                executor.submit(
                    non_session_get_test, session_class, url, requests_number // threads
                )
                for _ in range(threads)
            ]
            for f in as_completed(futures):
                f.result()
        results.append(
            _record_test_result(
                name, "Threaded-NonSession", url, start, cpu_start, threads
            )
        )

    return results


def run_async_tests(async_packages, url, requests_number):
    results = []
    for name, session_class in async_packages:
        # Test with session
        results.append(
            _run_single_async_package_test(
                name,
                session_class,
                url,
                requests_number,
                async_session_get_test,
                "Async-Session",
            )
        )

        # Test without session
        results.append(
            _run_single_async_package_test(
                name,
                session_class,
                url,
                requests_number,
                async_non_session_get_test,
                "Async-NonSession",
            )
        )
    return results
