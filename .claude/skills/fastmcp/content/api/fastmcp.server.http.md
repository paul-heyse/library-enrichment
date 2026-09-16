# `fastmcp.server.http`

Distribution: `fastmcp`

## DEFAULT_HOSTS

`fastmcp.server.http.DEFAULT_HOSTS`

```python
DEFAULT_HOSTS = ('127.0.0.1', 'localhost', '::1')
```

**Inferred type** (`ty`, not declared in the source): `tuple[Literal["127.0.0.1"], Literal["localhost"], Literal["::1"]]`

## HostOriginProtection

Import as `fastmcp.server.mixins.transport.HostOriginProtection`  ·  defined at `fastmcp.server.http.HostOriginProtection`

```python
HostOriginProtection = bool | Literal['auto']
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'bool | Literal["auto"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## HostOriginProtectionMode

`fastmcp.server.http.HostOriginProtectionMode`

```python
HostOriginProtectionMode = Literal['auto', 'strict']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["auto", "strict"]'> ````

## _current_http_request

`fastmcp.server.http._current_http_request`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_current_http_request: ContextVar[Request | None] = ContextVar('http_request', default=None)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## logger

`fastmcp.server.http.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FastMCPStreamableHTTPSessionManager

`fastmcp.server.http.FastMCPStreamableHTTPSessionManager`

```python
class FastMCPStreamableHTTPSessionManager(StreamableHTTPSessionManager)
```

**Bases** `StreamableHTTPSessionManager`

**Declared members (1)**

- `event_store: EventStore | None`  _property, writable_

Session manager that scopes resumability storage per transport session.


## HostOriginGuardMiddleware

`fastmcp.server.http.HostOriginGuardMiddleware`

```python
class HostOriginGuardMiddleware
```

**Declared members (6)**

- `allowed_hosts = tuple(allowed_hosts or ())`  _instance-attribute_
- `allowed_origins = tuple(allowed_origins or ())`  _instance-attribute_
- `app = app`  _instance-attribute_
- `has_explicit_allowed_hosts = allowed_hosts is not None`  _instance-attribute_
- `has_explicit_allowed_origins = allowed_origins is not None`  _instance-attribute_
- `mode = mode`  _instance-attribute_

Validate Host and Origin headers before requests reach MCP sessions.


## RequestContextMiddleware

`fastmcp.server.http.RequestContextMiddleware`

```python
class RequestContextMiddleware
```

**Declared members (1)**

- `app = app`  _instance-attribute_

Middleware that stores each request in a ContextVar and sets transport type.


## StarletteWithLifespan

Import as `fastmcp.server.mixins.transport.StarletteWithLifespan`  ·  defined at `fastmcp.server.http.StarletteWithLifespan`

```python
class StarletteWithLifespan(Starlette)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Starlette`

**Declared members (1)**

- `lifespan: Lifespan[Starlette]`  _property_

## StreamableHTTPASGIApp

`fastmcp.server.http.StreamableHTTPASGIApp`

```python
class StreamableHTTPASGIApp
```

**Declared members (1)**

- `session_manager = session_manager`  _instance-attribute_

ASGI application wrapper for Streamable HTTP server transport.


## _format_origin_host

`fastmcp.server.http._format_origin_host`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_origin_host(host: str) -> str
```

## _host_matches

`fastmcp.server.http._host_matches`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _host_matches(host: str, allowed_hosts: Sequence[str]) -> bool
```

## _is_loopback_host

`fastmcp.server.http._is_loopback_host`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_loopback_host(host: str) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _is_unspecified_host

`fastmcp.server.http._is_unspecified_host`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_unspecified_host(host: str) -> bool
```

## _normalize_host

`fastmcp.server.http._normalize_host`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_host(host: str) -> str
```

## _normalize_origin

`fastmcp.server.http._normalize_origin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_origin(origin: str) -> str
```

## _origin_host

`fastmcp.server.http._origin_host`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _origin_host(origin: str) -> str
```

## _origin_matches

`fastmcp.server.http._origin_matches`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _origin_matches(origin: str, allowed_origins: Sequence[str]) -> bool
```

## _origin_port

`fastmcp.server.http._origin_port`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _origin_port(scheme: str, port: int | None) -> int | None
```

## _request_origin

`fastmcp.server.http._request_origin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _request_origin(scope: Scope, host: str) -> str
```

## create_base_app

`fastmcp.server.http.create_base_app`

```python
def create_base_app(routes: list[BaseRoute], middleware: list[Middleware], debug: bool = False, lifespan: Callable | None = None) -> StarletteWithLifespan
```

Create a base Starlette app with common middleware and routes.

Args:
    routes: List of routes to include in the app
    middleware: List of middleware to include in the app
    debug: Whether to enable debug mode
    lifespan: Optional lifespan manager for the app

Returns:
    A Starlette application


## create_sse_app

Import as `fastmcp.server.mixins.transport.create_sse_app`  ·  defined at `fastmcp.server.http.create_sse_app`

```python
def create_sse_app(server: FastMCP[LifespanResultT], message_path: str, sse_path: str, auth: AuthProvider | None = None, debug: bool = False, routes: list[BaseRoute] | None = None, middleware: list[Middleware] | None = None) -> StarletteWithLifespan
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return an instance of the SSE server app.

Args:
    server: The FastMCP server instance
    message_path: Path for SSE messages
    sse_path: Path for SSE connections
    auth: Optional authentication provider (AuthProvider)
    debug: Whether to enable debug mode
    routes: Optional list of custom routes
    middleware: Optional list of middleware
Returns:
    A Starlette application with RequestContextMiddleware


## create_streamable_http_app

Import as `fastmcp.server.mixins.transport.create_streamable_http_app`  ·  defined at `fastmcp.server.http.create_streamable_http_app`

```python
def create_streamable_http_app(server: FastMCP[LifespanResultT], streamable_http_path: str, event_store: EventStore | None = None, retry_interval: int | None = None, auth: AuthProvider | None = None, json_response: bool = False, stateless_http: bool = False, debug: bool = False, routes: list[BaseRoute] | None = None, middleware: list[Middleware] | None = None, host_origin_protection: HostOriginProtection = False, allowed_hosts: Sequence[str] | None = None, allowed_origins: Sequence[str] | None = None, session_idle_timeout: float | None = None) -> StarletteWithLifespan
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return an instance of the StreamableHTTP server app.

Args:
    server: The FastMCP server instance
    streamable_http_path: Path for StreamableHTTP connections
    event_store: Optional event store for SSE polling/resumability
    retry_interval: Optional retry interval in milliseconds for SSE polling.
        Controls how quickly clients should reconnect after server-initiated
        disconnections. Requires event_store to be set. Defaults to SDK default.
    auth: Optional authentication provider (AuthProvider)
    json_response: Whether to use JSON response format
    stateless_http: Whether to use stateless mode (new transport per request)
    debug: Whether to enable debug mode
    routes: Optional list of custom routes
    middleware: Optional list of middleware
    host_origin_protection: Whether to validate Host and Origin headers
        before requests reach the MCP endpoint. Defaults to False for
        compatibility. "auto" protects localhost-bound servers and explicit
        host/origin allowlists.
    allowed_hosts: Additional hostnames that may appear in the Host header.
    allowed_origins: Additional browser origins trusted by the request guard.
        Configure CORS separately when browser JavaScript must read
        cross-origin responses.
    session_idle_timeout: Maximum time in seconds a session may remain idle
        before it is terminated. The deadline is pushed forward on every
        request. When None, sessions never expire from inactivity. Not
        supported in stateless mode.

Returns:
    A Starlette application with StreamableHTTP support


## set_http_request

`fastmcp.server.http.set_http_request`

```python
def set_http_request(request: Request) -> Generator[Request, None, None]
```

