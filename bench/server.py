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
    host = "0.0.0.0"
    port = 8000
    print(f"Starting server on {host}:{port}...")
    uvicorn.run(
        "server:app",
        host=host,
        port=port,
        log_level="error",
        access_log=False,
    )
