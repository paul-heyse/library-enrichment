# `fastmcp.utilities.tests`

Distribution: `fastmcp`

## ASGIServer

`fastmcp.utilities.tests.ASGIServer`

```python
class ASGIServer
```

**Declared members (6)**

- `app: ASGIApp`  _instance-attribute_
- `def client(self, headers: dict[str, str] | None = None, auth: httpx2.Auth | Literal['oauth'] | str | None = None, client_kwargs: Any = {}) -> Client`
  An unconnected FastMCP `Client` pointed at the in-process app.
- `def http_client(self, headers: dict[str, str] | None = None, timeout: httpx2.Timeout | None = None, auth: httpx2.Auth | None = None, kwargs: Any = {}) -> httpx2.AsyncClient`
  An `httpx2.AsyncClient` bound to the in-process app, for raw HTTP assertions.
- `def transport(self, kwargs: Any = {}) -> StreamableHttpTransport | SSETransport`
  A FastMCP client transport wired to the in-process app.
- `transport_type: Literal['http', 'streamable-http', 'sse']`  _instance-attribute_
- `url: str`  _instance-attribute_

A FastMCP server's real HTTP app, reachable in-process with no sockets.

Yielded by `asgi_server`. The `url` looks like an ordinary server URL and the app
behind it is the genuine article — auth middleware, session manager, SSE framing and
redirects all run — but every request is dispatched straight into the ASGI
application on the current event loop.

Because nothing is listening on the network, a plain `httpx2.AsyncClient()` cannot
reach this server. Use `client()` for a FastMCP client, `http_client()` for raw HTTP
assertions, and `transport()` when you need to build the client transport yourself.


## HeadlessOAuth

`fastmcp.utilities.tests.HeadlessOAuth`

```python
class HeadlessOAuth(OAuth)
```

**Bases** `OAuth`

**Declared members (2)**

- `async def callback_handler(self) -> AuthorizationCodeResult`  _async_
  Parse stored response and return the authorization code result.
- `async def redirect_handler(self, authorization_url: str) -> None`  _async_
  Make HTTP request to authorization URL and store response for callback handler.

**Inherited (2)**

- from `fastmcp.client.auth.oauth.OAuth`: `async_auth_flow`, `httpx_client_factory`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OAuth provider that bypasses browser interaction for testing.

This simulates the complete OAuth flow programmatically by making HTTP requests
instead of opening a browser and running a callback server. Useful for automated testing.


## _run_server

`fastmcp.utilities.tests._run_server`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _run_server(mcp_server: FastMCP, transport: Literal['sse'], port: int) -> None
```

## _wait_for_port

`fastmcp.utilities.tests._wait_for_port`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _wait_for_port(host: str, port: int, timeout: float = 5.0) -> None
```

Poll until a TCP connection to `host:port` is accepted, or raise on timeout.


## asgi_client

`fastmcp.utilities.tests.asgi_client`

```python
async def asgi_client(server: FastMCP, transport: Literal['http', 'streamable-http', 'sse'] = 'http', path: str | None = None, headers: dict[str, str] | None = None, auth: httpx2.Auth | Literal['oauth'] | str | None = None, client_kwargs: Any = {}) -> AsyncGenerator[Client, None]
```

Serve a FastMCP server over HTTP in-process and yield a connected `Client`.

This is the shortest path to testing a server over a real HTTP stack. The server's
Starlette app is built and started, and requests are dispatched straight into it on
the current event loop — no port, no uvicorn, no subprocess — but middleware,
authentication, session management and SSE streaming all behave as in production.

Reach for `asgi_server` instead when a fixture must serve several tests that each
build their own client, or when a test needs raw HTTP access to the app.

Args:
    server: FastMCP server instance.
    transport: Transport type ("http", "streamable-http", or "sse").
    path: URL path for the server (defaults to "/mcp", or "/sse" for SSE).
    headers: HTTP headers to send with every request.
    auth: Client authentication, as accepted by the HTTP transports.
    **client_kwargs: Additional arguments forwarded to `Client`.

Yields:
    A connected `Client`.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.utilities.tests import asgi_client

    async def test_greet():
        mcp = FastMCP("test")

        @mcp.tool
        def greet(name: str) -> str:
            return f"Hello, {name}!"

        async with asgi_client(mcp) as client:
            result = await client.call_tool("greet", {"name": "World"})
            assert result.data == "Hello, World!"
    ```


## asgi_server

`fastmcp.utilities.tests.asgi_server`

```python
async def asgi_server(server: FastMCP, transport: Literal['http', 'streamable-http', 'sse'] = 'http', path: str | None = None, http_app_kwargs: Any = {}) -> AsyncGenerator[ASGIServer, None]
```

Serve a FastMCP server's HTTP app in-process, with no socket and no uvicorn.

This is the fastest way to test a FastMCP server over HTTP. The server's real
Starlette app is built with `http_app()` and its lifespan is started, then every
request is dispatched directly into the app on the current event loop. That skips
port binding, uvicorn startup and connection setup entirely, while still exercising
the full HTTP stack: middleware, authentication, session management and SSE
streaming all run exactly as they do in production.

Use this as a fixture when several tests share one server but each needs its own
client. For a single test, `asgi_client` hands you a connected client in one step.

Args:
    server: FastMCP server instance.
    transport: Transport type ("http", "streamable-http", or "sse").
    path: URL path for the server (defaults to "/mcp", or "/sse" for SSE).
    **http_app_kwargs: Additional arguments forwarded to `server.http_app()`.

Yields:
    An `ASGIServer` describing how to reach the app.

Example:
    ```python
    import pytest
    from fastmcp import FastMCP
    from fastmcp.utilities.tests import ASGIServer, asgi_server

    mcp = FastMCP("test")

    @mcp.tool
    def greet(name: str) -> str:
        return f"Hello, {name}!"

    @pytest.fixture
    async def server():
        async with asgi_server(mcp) as running_server:
            yield running_server

    async def test_greet(server: ASGIServer):
        async with server.client() as client:
            result = await client.call_tool("greet", {"name": "World"})
            assert result.data == "Hello, World!"

    async def test_greet_with_headers(server: ASGIServer):
        async with server.client(headers={"X-Tenant": "acme"}) as client:
            result = await client.call_tool("greet", {"name": "World"})
            assert result.data == "Hello, World!"
    ```


## run_server_async

`fastmcp.utilities.tests.run_server_async`

```python
async def run_server_async(server: FastMCP, port: int | None = None, transport: Literal['http', 'streamable-http', 'sse'] = 'http', path: str = '/mcp', host: str = '127.0.0.1') -> AsyncGenerator[str, None]
```

Start a FastMCP server on a real port as an asyncio task.

This runs a real uvicorn server in the current process, bound to a real TCP port,
and yields its URL. Use it when the behaviour under test is genuinely about the
network — real sockets, TLS, or a server that must be reachable by something other
than an in-process client. Otherwise prefer `asgi_client` or `asgi_server`, which
exercise the same HTTP stack without binding a port.

Args:
    server: FastMCP server instance
    port: Port to bind to (default: find available port)
    transport: Transport type ("http", "streamable-http", or "sse")
    path: URL path for the server (default: "/mcp")
    host: Host to bind to (default: "127.0.0.1")

Yields:
    Server URL string

Example:
    ```python
    import pytest
    from fastmcp import FastMCP, Client
    from fastmcp.client.transports import StreamableHttpTransport
    from fastmcp.utilities.tests import run_server_async

    @pytest.fixture
    async def server():
        mcp = FastMCP("test")

        @mcp.tool()
        def greet(name: str) -> str:
            return f"Hello, {name}!"

        async with run_server_async(mcp) as url:
            yield url

    async def test_greet(server: str):
        async with Client(StreamableHttpTransport(server)) as client:
            result = await client.call_tool("greet", {"name": "World"})
            assert result.content[0].text == "Hello, World!"
    ```


## run_server_in_process

`fastmcp.utilities.tests.run_server_in_process`

```python
def run_server_in_process(server_fn: Callable[..., None], args: Any = (), provide_host_and_port: bool = True, host: str = '127.0.0.1', port: int | None = None, kwargs: Any = {}) -> Generator[str, None, None]
```

Context manager that runs a FastMCP server in a separate process and
returns the server URL. When the context manager is exited, the server process is killed.

Args:
    server_fn: The function that runs a FastMCP server. FastMCP servers are
        not pickleable, so we need a function that creates and runs one.
    *args: Arguments to pass to the server function.
    provide_host_and_port: Whether to provide the host and port to the server function as kwargs.
    host: Host to bind the server to (default: "127.0.0.1").
    port: Port to bind the server to (default: find available port).
    **kwargs: Keyword arguments to pass to the server function.

Returns:
    The server URL.


## temporary_settings

`fastmcp.utilities.tests.temporary_settings`

```python
def temporary_settings(kwargs: Any = {})
```

Temporarily override FastMCP setting values.

Args:
    **kwargs: The settings to override, including nested settings.

Example:
    Temporarily override a setting:
    ```python
    import fastmcp
    from fastmcp.utilities.tests import temporary_settings

    with temporary_settings(log_level='DEBUG'):
        assert fastmcp.settings.log_level == 'DEBUG'
    assert fastmcp.settings.log_level == 'INFO'
    ```


