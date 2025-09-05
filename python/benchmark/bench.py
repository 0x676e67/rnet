import argparse
import pandas as pd
from core import (  # type: ignore
    PycurlSession,
    add_package_version,
    run_sync_tests,
    run_threaded_tests,
    run_async_tests,
)
from chart import plot_benchmark_multi  # type: ignore

import aiohttp
import httpx
import niquests
import requests
import tls_client
import curl_cffi
import curl_cffi.requests
import rnet
import rnet.blocking


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
        default="benchmark_multi.jpg",
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
    args = parse_arguments()

    # Use command line arguments
    requests_number = args.requests
    thread_counts = args.threads

    print("Starting benchmark with:")
    print(f"  Requests per test: {requests_number}")
    print(f"  Thread counts: {thread_counts}")
    print(f"  Base URL: {args.base_url}")
    print()

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

    for size in ["20k", "50k", "200k"]:
        url = f"{args.base_url}/{size}"
        print(f"Testing with {size} payload...")

        all_results += run_sync_tests(sync_packages, url, requests_number)
        all_results += run_async_tests(async_packages, url, requests_number)

        for threads in thread_counts:
            all_results += run_threaded_tests(
                sync_packages, url, requests_number, threads
            )

    print(f"Saving results to {args.output}...")
    df = pd.DataFrame(all_results)
    df.to_csv(args.output, index=False)

    print(f"Generating chart {args.chart}...")
    plot_benchmark_multi(df, args.chart)

    print("Benchmark completed!")


if __name__ == "__main__":
    main()
