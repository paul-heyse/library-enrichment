# `fastmcp.server.mixins.transport`

Distribution: `fastmcp`

## logger

`fastmcp.server.mixins.transport.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## TransportMixin

Import as `fastmcp.server.mixins.TransportMixin`  ·  defined at `fastmcp.server.mixins.transport.TransportMixin`

```python
class TransportMixin
```

**Also exported as** `fastmcp.server.mixins.TransportMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `def custom_route(self: FastMCP, path: str, methods: list[str], name: str | None = None, include_in_schema: bool = True) -> Callable[[Callable[[Request], Awaitable[Response]]], Callable[[Request], Awaitable[Response]]]`
  Decorator to register a custom HTTP route on the FastMCP server.
- `def http_app(self: FastMCP, path: str | None = None, middleware: list[ASGIMiddleware] | None = None, json_response: bool | None = None, stateless_http: bool | None = None, transport: Literal['http', 'streamable-http', 'sse'] = 'http', event_store: EventStore | None = None, retry_interval: int | None = None, host_origin_protection: HostOriginProtection | None = None, allowed_hosts: list[str] | None = None, allowed_origins: list[str] | None = None, session_idle_timeout: float | None = None) -> StarletteWithLifespan`
  Create a Starlette app using the specified HTTP transport.
- `def run(self: FastMCP, transport: Transport | None = None, show_banner: bool | None = None, transport_kwargs: Any = {}) -> None`
  Run the FastMCP server. Note this is a synchronous function.
- `async def run_async(self: FastMCP, transport: Transport | None = None, show_banner: bool | None = None, transport_kwargs: Any = {}) -> None`  _async_
  Run the FastMCP server asynchronously.
- `async def run_http_async(self: FastMCP, show_banner: bool = True, transport: Literal['http', 'streamable-http', 'sse'] = 'http', host: str | None = None, port: int | None = None, log_level: str | None = None, path: str | None = None, uvicorn_config: dict[str, Any] | None = None, middleware: list[ASGIMiddleware] | None = None, json_response: bool | None = None, stateless_http: bool | None = None, stateless: bool | None = None, host_origin_protection: HostOriginProtection | None = None, allowed_hosts: list[str] | None = None, allowed_origins: list[str] | None = None, sockets: list[socket.socket] | None = None) -> None`  _async_
  Run the server using HTTP transport.
- `async def run_stdio_async(self: FastMCP, show_banner: bool = True, log_level: str | None = None, stateless: bool = False) -> None`  _async_
  Run the server using stdio transport.

Mixin providing transport-related methods for FastMCP.

Includes HTTP/stdio/SSE transport handling and custom HTTP routes.


## _format_host_for_url

`fastmcp.server.mixins.transport._format_host_for_url`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_host_for_url(host: str) -> str
```

Format a host for inclusion in a URL, bracketing IPv6 addresses.

A bare IPv6 address like ``::1`` must be wrapped in brackets when placed
before a ``:port`` suffix, otherwise the result (``http://::1:8000``) is an
invalid URL. Hostnames and IPv4 addresses are returned unchanged, as are
addresses that are already bracketed.


## _resolve_allowed_hosts_for_run

`fastmcp.server.mixins.transport._resolve_allowed_hosts_for_run`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolve_allowed_hosts_for_run(host: str, host_origin_protection: HostOriginProtection, allowed_hosts: list[str] | None, configured_allowed_hosts: list[str] | None) -> list[str] | None
```

