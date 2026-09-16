# `fastmcp.utilities.asgi_transport`

Distribution: `fastmcp`

## StreamingASGITransport

Import as `fastmcp.utilities.tests.StreamingASGITransport`  ·  defined at `fastmcp.utilities.asgi_transport.StreamingASGITransport`

```python
class StreamingASGITransport(httpx2.AsyncBaseTransport)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `httpx2.AsyncBaseTransport`

**Declared members (1)**

- `async def handle_async_request(self, request: httpx2.Request) -> httpx2.Response`  _async_

Drive an ASGI application in-process, streaming each response as it is produced.

This is an `httpx2` transport, so it plugs into anything that accepts an
`httpx2.AsyncClient` — including FastMCP's client transports via their
`httpx_client_factory` argument.

Args:
    app: The ASGI application to drive (e.g. `FastMCP.http_app()`).
    cancel_on_close: When True (the default), closing the transport cancels every
        application task still running, so harness teardown can never hang. Set to
        False to wait for the application's own disconnect handling to complete
        instead, which the legacy SSE server transport relies on for cleanup.

Example:
    Drive a FastMCP server's real HTTP app with no sockets:
    ```python
    import httpx2
    from fastmcp import FastMCP
    from fastmcp.utilities.asgi_transport import StreamingASGITransport

    mcp = FastMCP("test")
    app = mcp.http_app(transport="http")

    async with app.router.lifespan_context(app):
        transport = StreamingASGITransport(app)
        async with httpx2.AsyncClient(
            transport=transport, base_url="http://testserver"
        ) as client:
            response = await client.get("/mcp")
    ```


## _StreamingResponseBody

`fastmcp.utilities.asgi_transport._StreamingResponseBody`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _StreamingResponseBody(httpx2.AsyncByteStream)
```

**Bases** `httpx2.AsyncByteStream`

**Declared members (1)**

- `async def aclose(self) -> None`  _async_

A response body that yields chunks as the application produces them.

Closing it tells the application the client has gone away (`http.disconnect`),
mirroring a peer that drops the connection mid-response.


## run_asgi_lifespan

Import as `fastmcp.utilities.tests.run_asgi_lifespan`  ·  defined at `fastmcp.utilities.asgi_transport.run_asgi_lifespan`

```python
async def run_asgi_lifespan(app: ASGIApp) -> AsyncIterator[None]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Run an ASGI application's lifespan, driving the protocol as a real server does.

The application's lifespan runs inside a dedicated task for the whole duration of
the context. This matters because a lifespan typically owns cancel scopes and task
groups — anyio requires those to be exited by the task that entered them, which
rules out entering the lifespan on one task and leaving it on another (as a pytest
fixture's setup and teardown phases may do).

Args:
    app: The ASGI application whose lifespan should run.

Raises:
    RuntimeError: If the application reports `lifespan.startup.failed`, or reports
        `lifespan.shutdown.failed` (or crashes during shutdown) while the context
        body itself completed successfully. A failure inside the body takes
        precedence and propagates unchanged.


