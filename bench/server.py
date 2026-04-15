import argparse
import multiprocessing

import granian
from starlette.applications import Starlette
from starlette.requests import Request
from starlette.responses import Response
from starlette.routing import Route


async def echo(request: Request) -> Response:
    body = await request.body()
    content_type = request.headers.get("content-type", "application/octet-stream")
    return Response(body, media_type=content_type)


app = Starlette(
    routes=[
        Route(
            "/{path:path}",
            echo,
            methods=["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"],
        )
    ],
)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Start benchmark server")
    parser.add_argument(
        "--workers",
        type=int,
        default=multiprocessing.cpu_count(),
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
    workers = args.workers

    print(
        f"Starting server on {host}:{port} with {workers} workers (CPU cores/threads: {multiprocessing.cpu_count()})..."
    )
    granian.Granian(
        "server:app",
        address=host,
        port=port,
        interface="asgi",
        workers=workers,
        runtime_threads=1,
        websockets=False,
        log_enabled=True,
        log_access=False,
    ).serve()
