import asyncio
import time
from io import BytesIO
from concurrent.futures import ThreadPoolExecutor, as_completed
from importlib.metadata import version

import aiohttp
import httpx
import matplotlib.pyplot as plt
import niquests
import numpy as np
import pandas as pd
import pycurl
import requests
import tls_client
import curl_cffi
import curl_cffi.requests
import rnet
import rnet.blocking


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


def session_get_test(session_class, url, requests_number):
    s = session_class()
    try:
        for _ in range(requests_number):
            if session_class.__module__ == "rnet.blocking":
                s.get(url).bytes()
            else:
                s.get(url).text
    finally:
        if hasattr(s, "close"):
            s.close()


def non_session_get_test(session_class, url, requests_number):
    for _ in range(requests_number):
        s = session_class()
        try:
            if session_class.__module__ == "rnet.blocking":
                s.get(url).bytes()
            else:
                s.get(url).text
        finally:
            if hasattr(s, "close"):
                s.close()


async def async_session_get_test(session_class, url, requests_number):
    async def aget(s, url):
        if session_class.__module__ == "aiohttp.client":
            async with s.get(url) as resp:
                return await resp.read()
        if session_class.__name__ == "Client":
            resp = await s.get(url)
            return await resp.bytes()
        else:
            resp = await s.get(url)
            return resp.text

    try:
        async with session_class() as s:
            tasks = [aget(s, url) for _ in range(requests_number)]
            await asyncio.gather(*tasks)
    except TypeError:
        s = session_class()
        tasks = [aget(s, url) for _ in range(requests_number)]
        await asyncio.gather(*tasks)
        if hasattr(s, "aclose"):
            await s.aclose()
        elif hasattr(s, "close"):
            s.close()


async def async_non_session_get_test(session_class, url, requests_number):
    async def aget(s, url):
        if session_class.__module__ == "aiohttp.client":
            async with s.get(url) as resp:
                return await resp.read()
        if session_class.__name__ == "Client":
            resp = await s.get(url)
            return await resp.bytes()
        else:
            resp = await s.get(url)
            return resp.text

    for _ in range(requests_number):
        try:
            async with session_class() as s:
                await aget(s, url)
        except TypeError:
            s = session_class()
            await aget(s, url)
            if hasattr(s, "aclose"):
                await s.aclose()
            elif hasattr(s, "close"):
                s.close()


def run_sync_tests(packages, url, requests_number):
    results = []
    for name, session_class in packages:
        # Test with session
        start = time.perf_counter()
        cpu_start = time.process_time()
        session_get_test(session_class, url, requests_number)
        dur = round(time.perf_counter() - start, 2)
        cpu_dur = round(time.process_time() - cpu_start, 2)
        results.append(
            {
                "name": name,
                "session": "Sync-Session",
                "size": url.split("/")[-1],
                "time": dur,
                "cpu_time": cpu_dur,
            }
        )

        # Test without session
        start = time.perf_counter()
        cpu_start = time.process_time()
        non_session_get_test(session_class, url, requests_number)
        dur = round(time.perf_counter() - start, 2)
        cpu_dur = round(time.process_time() - cpu_start, 2)
        results.append(
            {
                "name": name,
                "session": "Sync-NonSession",
                "size": url.split("/")[-1],
                "time": dur,
                "cpu_time": cpu_dur,
            }
        )
    return results


def run_threaded_tests(packages, url, requests_number, threads):
    results = []
    for name, session_class in packages:
        # Test with session
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
        dur = round(time.perf_counter() - start, 2)
        cpu_dur = round(time.process_time() - cpu_start, 2)
        results.append(
            {
                "name": name,
                "session": "Threaded-Session",
                "threads": threads,
                "size": url.split("/")[-1],
                "time": dur,
                "cpu_time": cpu_dur,
            }
        )

        # Test without session
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
        dur = round(time.perf_counter() - start, 2)
        cpu_dur = round(time.process_time() - cpu_start, 2)
        results.append(
            {
                "name": name,
                "session": "Threaded-NonSession",
                "threads": threads,
                "size": url.split("/")[-1],
                "time": dur,
                "cpu_time": cpu_dur,
            }
        )
    return results


def run_async_tests(async_packages, url, requests_number):
    results = []
    for name, session_class in async_packages:
        # Test with session
        start = time.perf_counter()
        cpu_start = time.process_time()
        asyncio.run(async_session_get_test(session_class, url, requests_number))
        dur = round(time.perf_counter() - start, 2)
        cpu_dur = round(time.process_time() - cpu_start, 2)
        results.append(
            {
                "name": name,
                "session": "Async-Session",
                "size": url.split("/")[-1],
                "time": dur,
                "cpu_time": cpu_dur,
            }
        )

        # Test without session
        start = time.perf_counter()
        cpu_start = time.process_time()
        asyncio.run(async_non_session_get_test(session_class, url, requests_number))
        dur = round(time.perf_counter() - start, 2)
        cpu_dur = round(time.process_time() - cpu_start, 2)
        results.append(
            {
                "name": name,
                "session": "Async-NonSession",
                "size": url.split("/")[-1],
                "time": dur,
                "cpu_time": cpu_dur,
            }
        )
    return results


def plot_benchmark_multi(df, filename):
    """
    Draw multi-subplot, multi-group, multi-metric bar charts for time/cpu_time/different payload sizes.
    Generate separate plots for sync/async and session/non-session combinations.
    """
    # Keep only necessary columns
    df = df[["name", "session", "threads", "size", "time", "cpu_time"]].copy()
    df["threads"] = df["threads"].fillna(1).astype(int)

    # Get unique session types
    existing_session_types = df["session"].unique()

    sizes = sorted(df["size"].unique(), key=lambda x: int(x.replace("k", "")))
    stat_types = ["time", "cpu_time"]

    # Separate main sessions (non-threaded) and threaded sessions
    main_sessions = [s for s in existing_session_types if not s.startswith("Threaded")]
    threaded_sessions = [s for s in existing_session_types if s.startswith("Threaded")]

    # Plot main sessions (sync and async)
    if main_sessions:
        num_sessions = len(main_sessions)
        # Allocate more height for each subplot to ensure sufficient spacing
        subplot_height = 8  # Fixed height for each subplot
        total_height = subplot_height * num_sessions + 2  # Extra 2 inches for spacing

        fig, axes = plt.subplots(
            num_sessions,
            1,
            figsize=(20, total_height),
            constrained_layout=False,  # Disable constrained_layout, use manual layout
        )

        if num_sessions == 1:
            axes = [axes]

        for idx, session in enumerate(main_sessions):
            ax = axes[idx]
            subdf = df[df["session"] == session]
            names = subdf["name"].unique()
            x = np.arange(len(names))
            width = 0.12

            max_height = 0

            for i, size in enumerate(sizes):
                for j, stat in enumerate(stat_types):
                    vals = []
                    for name in names:
                        v = subdf[(subdf["name"] == name) & (subdf["size"] == size)][
                            stat
                        ]
                        vals.append(v.values[0] if not v.empty else 0)
                    offset = (i * len(stat_types) + j) * width
                    rects = ax.bar(x + offset, vals, width, label=f"{stat} {size}")
                    ax.bar_label(rects, padding=2, fontsize=7, rotation=90)
                    if vals:
                        max_height = max(max_height, max(vals))

            ax.set_xticks(x + (len(sizes) * len(stat_types) * width) / 2 - width / 2)
            ax.set_xticklabels(names, rotation=0, ha="center", fontsize=8)
            ax.set_ylabel("Time (s)")
            ax.set_title(f"Benchmark | {session}", fontsize=12, fontweight="bold")
            ax.legend(loc="upper left", ncol=3, prop={"size": 7})
            ax.tick_params(axis="x", labelsize=8)
            ax.grid(True, alpha=0.3)

            if max_height > 0:
                ax.set_ylim(0, max_height * 1.35)

        plt.subplots_adjust(
            hspace=0.5, top=0.95, bottom=0.1, left=0.08, right=0.98
        )  # Set explicit margins for all sides
        plt.savefig(filename, format="jpg", dpi=150, bbox_inches="tight")
        plt.show()

    # Plot threaded sessions separately
    if threaded_sessions:
        threaded_df = df[df["session"].str.startswith("Threaded")].copy()
        thread_counts = sorted(threaded_df["threads"].unique())

        fig2, axes2 = plt.subplots(
            len(thread_counts),
            1,
            figsize=(20, 10 * len(thread_counts)),
            constrained_layout=False,  # Disable constrained_layout, use manual layout
        )

        if len(thread_counts) == 1:
            axes2 = [axes2]

        for idx, thread_count in enumerate(thread_counts):
            ax = axes2[idx]
            thread_df = threaded_df[threaded_df["threads"] == thread_count]

            # Get all unique session types for this thread count
            thread_session_types = thread_df["session"].unique()

            names = thread_df["name"].unique()
            x = np.arange(len(names))
            width = 0.08
            max_height = 0
            bar_index = 0

            # Plot each session type
            for session_type in thread_session_types:
                session_df = thread_df[thread_df["session"] == session_type]
                session_label = session_type.replace("Threaded-", "")

                for i, size in enumerate(sizes):
                    for j, stat in enumerate(stat_types):
                        vals = []
                        for name in names:
                            v = session_df[
                                (session_df["name"] == name)
                                & (session_df["size"] == size)
                            ][stat]
                            vals.append(v.values[0] if not v.empty else 0)
                        offset = bar_index * width
                        rects = ax.bar(
                            x + offset,
                            vals,
                            width,
                            label=f"{session_label} {stat} {size}",
                        )
                        ax.bar_label(rects, padding=2, fontsize=6, rotation=90)
                        if vals:
                            max_height = max(max_height, max(vals))
                        bar_index += 1

            ax.set_xticks(x + (bar_index * width) / 2 - width / 2)
            ax.set_xticklabels(names, rotation=0, ha="center", fontsize=8)
            ax.set_ylabel("Time (s)")
            ax.set_title(
                f"Benchmark | Threaded ({thread_count} threads)",
                fontsize=12,
                fontweight="bold",
            )
            ax.legend(loc="upper left", ncol=4, prop={"size": 6})
            ax.tick_params(axis="x", labelsize=8)
            ax.grid(True, alpha=0.3)

            if max_height > 0:
                ax.set_ylim(0, max_height * 1.35)

        threaded_filename = filename.replace(".jpg", "_threaded.jpg")
        plt.subplots_adjust(
            hspace=0.5, top=0.95, bottom=0.1, left=0.08, right=0.98
        )  # Set explicit margins for all sides
        plt.savefig(threaded_filename, format="jpg", dpi=150, bbox_inches="tight")
        plt.show()


def main():
    response_sizes = ["20k", "50k", "200k"]
    requests_number = 400
    thread_counts = [1, 4, 8, 16]

    sync_packages = [
        ("tls_client", tls_client.Session),
        ("httpx", httpx.Client),
        ("requests", requests.Session),
        ("niquests", niquests.Session),
        ("rnet", rnet.blocking.Client),
        ("curl_cffi", curl_cffi.requests.Session),
        ("pycurl", PycurlSession),
    ]
    async_packages = [
        ("httpx", httpx.AsyncClient),
        ("aiohttp", aiohttp.ClientSession),
        ("rnet", rnet.Client),
        ("curl_cffi", curl_cffi.requests.AsyncSession),
    ]

    sync_packages = add_package_version(sync_packages)
    async_packages = add_package_version(async_packages)

    all_results = []

    for size in response_sizes:
        url = f"http://127.0.0.1:8000/{size}"

        all_results += run_sync_tests(sync_packages, url, requests_number)
        all_results += run_async_tests(async_packages, url, requests_number)

        for threads in thread_counts:
            all_results += run_threaded_tests(
                sync_packages, url, requests_number, threads
            )

    df = pd.DataFrame(all_results)
    df.to_csv("benchmark_results.csv", index=False)
    plot_benchmark_multi(df, "benchmark_multi.jpg")


if __name__ == "__main__":
    main()
