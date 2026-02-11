import os

from starlette.applications import Starlette
from starlette.responses import PlainTextResponse
from starlette.routing import Route

random_20k = os.urandom(20 * 1024)
random_50k = os.urandom(50 * 1024)
random_200k = os.urandom(200 * 1024)


app = Starlette(
    routes=[
        Route("/20k", lambda r: PlainTextResponse(random_20k)),
        Route("/50k", lambda r: PlainTextResponse(random_50k)),
        Route("/200k", lambda r: PlainTextResponse(random_200k)),
    ],
)

if __name__ == "__main__":
    import uvicorn
    import multiprocessing
    import argparse

    parser = argparse.ArgumentParser(description="Start benchmark server")
    parser.add_argument(
        "--workers",
        type=int,
        default=None,
        help="Number of worker processes (default: CPU count)",
    )
    parser.add_argument(
        "--port", type=int, default=8000, help="Port to bind to (default: 8000)"
    )
    parser.add_argument(
        "--host", type=str, default="0.0.0.0", help="Host to bind to (default: 0.0.0.0)"
    )
    args = parser.parse_args()

    host = args.host
    port = args.port
    workers = args.workers or multiprocessing.cpu_count()

    max_workers = workers

    print(
        f"Starting server on {host}:{port} with {max_workers} workers (CPU cores/threads: {multiprocessing.cpu_count()})..."
    )
    uvicorn.run(
        "server:app",
        host=host,
        port=port,
        workers=max_workers,
        log_level="error",
        access_log=False,
    )
