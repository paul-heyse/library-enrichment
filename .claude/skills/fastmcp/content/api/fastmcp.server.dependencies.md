# `fastmcp.server.dependencies`

Distribution: `fastmcp`

## _DOCKET_AVAILABLE

`fastmcp.server.dependencies._DOCKET_AVAILABLE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DOCKET_AVAILABLE: bool | None = None
```

## _MIN_DOCKET_VERSION

`fastmcp.server.dependencies._MIN_DOCKET_VERSION`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MIN_DOCKET_VERSION = Version('0.19.0')
```

## __all__

`fastmcp.server.dependencies.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['AccessToken', 'CurrentAccessToken', 'CurrentContext', 'CurrentFastMCP', 'CurrentHeaders', 'CurrentRequest', 'FastMCPRequestContext', 'Progress', 'TokenClaim', 'bind_request_context', 'extract_version_spec', 'fastmcp_request_ctx', 'get_access_token', 'get_context', 'get_http_headers', 'get_http_request', 'get_server', 'get_session', 'is_docket_available', 'resolve_dependencies', 'transform_context_annotations', 'without_injected_parameters']
```

## _background_context_factory

`fastmcp.server.dependencies._background_context_factory`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_background_context_factory: Callable[[], Awaitable[Context | None]] | None = None
```

## _background_task_headers

`fastmcp.server.dependencies._background_task_headers`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_background_task_headers: ContextVar[dict[str, str] | None] = ContextVar('fastmcp_background_task_headers', default=None)
```

## _background_task_session_id

`fastmcp.server.dependencies._background_task_session_id`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_background_task_session_id: ContextVar[str | None] = ContextVar('fastmcp_background_task_session_id', default=None)
```

## _current_server

`fastmcp.server.dependencies._current_server`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_current_server: ContextVar[weakref.ref[FastMCP] | None] = ContextVar('server', default=None)
```

## _worker_server_resolver

`fastmcp.server.dependencies._worker_server_resolver`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_worker_server_resolver: Callable[[], FastMCP | None] | None = None
```

## fastmcp_request_ctx

Import as `fastmcp.server.context.fastmcp_request_ctx`  ·  defined at `fastmcp.server.dependencies.fastmcp_request_ctx`

```python
fastmcp_request_ctx: ContextVar[FastMCPRequestContext | None] = ContextVar('fastmcp_request_ctx', default=None)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## logger

`fastmcp.server.dependencies.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FastMCPRequestContext

Import as `fastmcp.server.context.FastMCPRequestContext`  ·  defined at `fastmcp.server.dependencies.FastMCPRequestContext`

```python
class FastMCPRequestContext
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (7)**

- `close_sse_stream: Any | None`  _instance-attribute_
- `lifespan_context: Any`  _instance-attribute_
- `meta: dict[str, Any] | None`  _instance-attribute_
  The raw ``_meta`` block lifted from the request params, if any.
- `protocol_version: str`  _instance-attribute_
- `request: Request | None`  _instance-attribute_
- `request_id: str | None`  _instance-attribute_
- `session: ServerSession`  _instance-attribute_

FastMCP-owned wrapper around the SDK's per-request context.

The SDK v2 runner hands each handler a fresh ``ServerRequestContext`` as an
argument rather than exposing it through a ContextVar. FastMCP owns this
ContextVar (``fastmcp_request_ctx``) and each request adapter binds a
``FastMCPRequestContext`` at the top of the handler (and the initialize
middleware binds it too).

A wrapper rather than the raw context because the SDK's
``ServerRequestContext.meta`` is a bare ``RequestParamsMeta`` TypedDict that
only carries ``progress_token`` — it does not carry ``_meta.fastmcp`` or the
distributed-trace parent. Those live in the raw params dict under ``_meta``,
which this wrapper lifts once so downstream consumers have a stable surface.


## InMemoryProgress

`fastmcp.server.dependencies.InMemoryProgress`

```python
class InMemoryProgress
```

**Declared members (6)**

- `current: int | None`  _property_
- `async def increment(self, amount: int = 1) -> None`  _async_
  Atomically increment the current progress value.
- `message: str | None`  _property_
- `async def set_message(self, message: str | None) -> None`  _async_
  Update the progress status message.
- `async def set_total(self, total: int) -> None`  _async_
  Set the total/target value for progress tracking.
- `total: int`  _property_

In-memory progress tracker for immediate tool execution.

Provides the same interface as Docket's Progress but stores state in memory
instead of Redis. Useful for testing and immediate execution where
progress doesn't need to be observable across processes.


## Progress

Import as `fastmcp.dependencies.Progress`  ·  defined at `fastmcp.server.dependencies.Progress`

```python
class Progress(Dependency['Progress'])
```

**Also exported as** `fastmcp.dependencies.Progress`

**Bases** `Dependency['Progress']`

**Declared members (6)**

- `current: int | None`  _property_
  Current progress value.
- `async def increment(self, amount: int = 1) -> None`  _async_
  Atomically increment the current progress value.
- `message: str | None`  _property_
  Current progress message.
- `async def set_message(self, message: str | None) -> None`  _async_
  Update the progress status message.
- `async def set_total(self, total: int) -> None`  _async_
  Set the total/target value for progress tracking.
- `total: int`  _property_
  Total/target progress value.

Progress dependency that works in both server and worker contexts.

In a Docket worker, delegates to the execution's Redis-backed progress
(observable across processes). Otherwise, uses in-memory tracking.

The shared default instance acts as a stateless factory — ``__aenter__``
creates a fresh ``Progress`` per invocation so concurrent tasks never
share mutable state.


## ProgressLike

Import as `fastmcp.dependencies.ProgressLike`  ·  defined at `fastmcp.server.dependencies.ProgressLike`

```python
class ProgressLike(Protocol)
```

**Also exported as** `fastmcp.dependencies.ProgressLike`

**Bases** `Protocol`

**Declared members (6)**

- `current: int | None`  _property_
  Current progress value.
- `async def increment(self, amount: int = 1) -> None`  _async_
  Atomically increment the current progress value.
- `message: str | None`  _property_
  Current progress message.
- `async def set_message(self, message: str | None) -> None`  _async_
  Update the progress status message.
- `async def set_total(self, total: int) -> None`  _async_
  Set the total/target value for progress tracking.
- `total: int`  _property_
  Total/target progress value.

Protocol for progress tracking interface.

Defines the common interface between InMemoryProgress (server context)
and Docket's Progress (worker context).


## _CurrentAccessToken

`fastmcp.server.dependencies._CurrentAccessToken`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CurrentAccessToken(Dependency[AccessToken])
```

**Bases** `Dependency[AccessToken]`

Async context manager for AccessToken dependency.


## _CurrentContext

`fastmcp.server.dependencies._CurrentContext`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CurrentContext(Dependency['Context'])
```

**Bases** `Dependency['Context']`

Async context manager for Context dependency.

Returns the active context from _current_context (normal MCP request).

The shared default instance is a stateless factory. All per-invocation
state lives on the returned Context, so concurrent calls never share
mutable state.


## _CurrentFastMCP

`fastmcp.server.dependencies._CurrentFastMCP`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CurrentFastMCP(Dependency['FastMCP'])
```

**Bases** `Dependency['FastMCP']`

Async context manager for FastMCP server dependency.


## _CurrentHeaders

`fastmcp.server.dependencies._CurrentHeaders`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CurrentHeaders(Dependency[dict[str, str]])
```

**Bases** `Dependency[dict[str, str]]`

Async context manager for HTTP Headers dependency.


## _CurrentRequest

`fastmcp.server.dependencies._CurrentRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CurrentRequest(Dependency[Request])
```

**Bases** `Dependency[Request]`

Async context manager for HTTP Request dependency.


## _OptionalCurrentContext

`fastmcp.server.dependencies._OptionalCurrentContext`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _OptionalCurrentContext(Dependency['Context | None'])
```

**Bases** `Dependency['Context | None']`

Context dependency that returns None instead of raising when no context
is active. Used for ``ctx: Context = None`` parameter patterns.

Delegates entirely to ``_CurrentContext`` — just catches the RuntimeError.
Cleanup is handled by ``_CurrentContext.__aexit__`` reading from the
task-local ContextVar.


## _TokenClaim

`fastmcp.server.dependencies._TokenClaim`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _TokenClaim(Dependency[str])
```

**Bases** `Dependency[str]`

**Declared members (1)**

- `claim_name = claim_name`  _instance-attribute_

Dependency that extracts a specific claim from the access token.


## CurrentAccessToken

Import as `fastmcp.dependencies.CurrentAccessToken`  ·  defined at `fastmcp.server.dependencies.CurrentAccessToken`

```python
def CurrentAccessToken() -> AccessToken
```

**Also exported as** `fastmcp.dependencies.CurrentAccessToken`

Get the current access token for the authenticated user.

This dependency provides access to the AccessToken for the current
authenticated request. Raises an error if no authentication is present.

Returns:
    A dependency that resolves to the active AccessToken

Raises:
    RuntimeError: If no authenticated user (use get_access_token() for optional)

Example:
    ```python
    from fastmcp.server.dependencies import CurrentAccessToken
    from fastmcp.server.auth import AccessToken

    @mcp.tool()
    async def get_user_id(token: AccessToken = CurrentAccessToken()) -> str:
        return token.claims.get("sub", "unknown")
    ```


## CurrentContext

Import as `fastmcp.dependencies.CurrentContext`  ·  defined at `fastmcp.server.dependencies.CurrentContext`

```python
def CurrentContext() -> Context
```

**Also exported as** `fastmcp.dependencies.CurrentContext`

Get the current FastMCP Context instance.

This dependency provides access to the active FastMCP Context for the
current MCP operation (tool/resource/prompt call).

Returns:
    A dependency that resolves to the active Context instance

Raises:
    RuntimeError: If no active context found (during resolution)

Example:
    ```python
    from fastmcp.dependencies import CurrentContext

    @mcp.tool()
    async def log_progress(ctx: Context = CurrentContext()) -> str:
        ctx.report_progress(50, 100, "Halfway done")
        return "Working"
    ```


## CurrentFastMCP

Import as `fastmcp.dependencies.CurrentFastMCP`  ·  defined at `fastmcp.server.dependencies.CurrentFastMCP`

```python
def CurrentFastMCP() -> FastMCP
```

**Also exported as** `fastmcp.dependencies.CurrentFastMCP`

Get the current FastMCP server instance.

This dependency provides access to the active FastMCP server.

Returns:
    A dependency that resolves to the active FastMCP server

Raises:
    RuntimeError: If no server in context (during resolution)

Example:
    ```python
    from fastmcp.dependencies import CurrentFastMCP

    @mcp.tool()
    async def introspect(server: FastMCP = CurrentFastMCP()) -> str:
        return f"Server: {server.name}"
    ```


## CurrentHeaders

Import as `fastmcp.dependencies.CurrentHeaders`  ·  defined at `fastmcp.server.dependencies.CurrentHeaders`

```python
def CurrentHeaders() -> dict[str, str]
```

**Also exported as** `fastmcp.dependencies.CurrentHeaders`

Get the current HTTP request headers.

This dependency provides access to the HTTP headers for the current request,
including the `authorization` and `cookie` headers, which `get_http_headers()`
withholds by default. Returns an empty dictionary when no HTTP request is
available, making it safe to use in code that might run over any transport.

Returns:
    A dependency that resolves to a dictionary of header name -> value

Example:
    ```python
    from fastmcp.server.dependencies import CurrentHeaders

    @mcp.tool()
    async def get_auth_type(headers: dict = CurrentHeaders()) -> str:
        auth = headers.get("authorization", "")
        return "Bearer" if auth.startswith("Bearer ") else "None"
    ```


## CurrentRequest

Import as `fastmcp.dependencies.CurrentRequest`  ·  defined at `fastmcp.server.dependencies.CurrentRequest`

```python
def CurrentRequest() -> Request
```

**Also exported as** `fastmcp.dependencies.CurrentRequest`

Get the current HTTP request.

This dependency provides access to the Starlette Request object for the
current HTTP request. Only available when running over HTTP transports
(SSE or Streamable HTTP).

Returns:
    A dependency that resolves to the active Starlette Request

Raises:
    RuntimeError: If no HTTP request in context (e.g., STDIO transport)

Example:
    ```python
    from fastmcp.server.dependencies import CurrentRequest
    from starlette.requests import Request

    @mcp.tool()
    async def get_client_ip(request: Request = CurrentRequest()) -> str:
        return request.client.host if request.client else "Unknown"
    ```


## OptionalCurrentContext

`fastmcp.server.dependencies.OptionalCurrentContext`

```python
def OptionalCurrentContext() -> Context | None
```

Get the current FastMCP Context, or None when no context is active.


## TokenClaim

Import as `fastmcp.dependencies.TokenClaim`  ·  defined at `fastmcp.server.dependencies.TokenClaim`

```python
def TokenClaim(name: str) -> str
```

**Also exported as** `fastmcp.dependencies.TokenClaim`

Get a specific claim from the access token.

This dependency extracts a single claim value from the current access token.
It's useful for getting user identifiers, roles, or other token claims
without needing the full token object.

Args:
    name: The name of the claim to extract (e.g., "oid", "sub", "email")

Returns:
    A dependency that resolves to the claim value as a string

Raises:
    RuntimeError: If no access token is available or claim is missing

Example:
    ```python
    from fastmcp.server.dependencies import TokenClaim

    @mcp.tool()
    async def add_expense(
        user_id: str = TokenClaim("oid"),  # Azure object ID
        amount: float,
    ):
        # user_id is automatically injected from the token
        await db.insert({"user_id": user_id, "amount": amount})
    ```


## _clear_signature_caches

`fastmcp.server.dependencies._clear_signature_caches`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _clear_signature_caches(fn: Callable[..., Any]) -> None
```

Clear signature-related caches for a function.

Called after modifying a function's signature to ensure downstream
code sees the updated signature.


## _lift_meta

`fastmcp.server.dependencies._lift_meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _lift_meta(ctx: ServerRequestContext) -> dict[str, Any] | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Lift the raw ``_meta`` block from the request params.

``ctx.params`` is the raw params mapping (or None); its ``_meta`` key holds
the full metadata block (``fastmcp.version``, traceparent, progressToken,
...). ``ctx.meta`` (a TypedDict) only carries ``progress_token``, so version
and trace extraction must read from here.


## _resolve_fastmcp_dependencies

`fastmcp.server.dependencies._resolve_fastmcp_dependencies`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _resolve_fastmcp_dependencies(fn: Callable[..., Any], arguments: dict[str, Any]) -> AsyncGenerator[dict[str, Any], None]
```

Resolve uncalled-for dependencies for a FastMCP function.

Sets up the context that uncalled-for's Depends() needs:
- A cache for resolved dependencies
- An AsyncExitStack for managing context manager lifetimes
- A resolution frame, so CallArgument() can read the call's arguments

The Docket instance (for CurrentDocket dependency) is managed separately
by the server's lifespan and made available via ContextVar.

Note: This does NOT set up Docket's Execution context. If user code needs
Docket-specific dependencies like TaskArgument(), TaskKey(), etc., those
will fail with clear errors about missing context.

Args:
    fn: The function to resolve dependencies for
    arguments: The arguments passed to the function

Yields:
    Dictionary of resolved dependencies merged with provided arguments


## bind_request_context

Import as `fastmcp.server.extensions.bind_request_context`  ·  defined at `fastmcp.server.dependencies.bind_request_context`

```python
def bind_request_context(ctx: ServerRequestContext) -> Generator[FastMCPRequestContext, None, None]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Bind a ``FastMCPRequestContext`` for the duration of a handler.

Constructs the wrapper from the SDK's per-request context and sets/resets
the ``fastmcp_request_ctx`` ContextVar. Every request adapter and the
initialize middleware enters this so ``Context`` and dependency helpers can
read the active request from the ContextVar.


## extract_version_spec

Import as `fastmcp.server.mixins.mcp_operations.extract_version_spec`  ·  defined at `fastmcp.server.dependencies.extract_version_spec`

```python
def extract_version_spec(meta: dict[str, Any] | None) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract the FastMCP component version from a lifted ``_meta`` block.


## get_access_token

Import as `fastmcp.server.sessions.get_access_token`  ·  defined at `fastmcp.server.dependencies.get_access_token`

```python
def get_access_token() -> AccessToken | None
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the FastMCP access token from the current context.

This function first tries to get the token from the current HTTP request's scope,
which is more reliable for long-lived connections where the SDK's auth_context_var
may become stale after token refresh. Falls back to the SDK's context var if no
request is available.

Returns:
    The access token if an authenticated user is available, None otherwise.


## get_context

Import as `fastmcp.server.providers.proxy.get_context`  ·  defined at `fastmcp.server.dependencies.get_context`

```python
def get_context() -> Context
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the current FastMCP Context instance directly.


## get_http_headers

Import as `fastmcp.server.providers.openapi.components.get_http_headers`  ·  defined at `fastmcp.server.dependencies.get_http_headers`

```python
def get_http_headers(include_all: bool = False, include: set[str] | None = None) -> dict[str, str]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract headers from the current HTTP request if available.

Never raises an exception, even if there is no active HTTP request (in which case
an empty dict is returned).

By default, strips problematic headers like `content-length`, and credential
headers like `authorization` and `cookie`, that cause issues if forwarded to
downstream services. If `include_all` is True, all headers are returned.

The `include` parameter allows specific headers to be included even if they would
normally be excluded. This is useful for proxy transports that need to forward
authorization headers to upstream MCP servers.


## get_http_request

`fastmcp.server.dependencies.get_http_request`

```python
def get_http_request() -> Request
```

Get the current HTTP request.

Tries MCP SDK's request_ctx first, then falls back to FastMCP's HTTP context.


## get_server

Import as `fastmcp.server.sessions.get_server`  ·  defined at `fastmcp.server.dependencies.get_server`

```python
def get_server() -> FastMCP
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the current FastMCP server instance directly.

In a background-task worker the tasks extension's resolver is consulted
first, so a mounted-child task resolves to the child server rather than the
root that started the worker (#3571).

Returns:
    The active FastMCP server

Raises:
    RuntimeError: If no server in context


## get_session

Import as `fastmcp.server.sessions.get_session`  ·  defined at `fastmcp.server.dependencies.get_session`

```python
async def get_session(session_id: str) -> Session
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resolve and validate a `Session` for an explicit `session_id`.

Pair with a `session_id: SessionId` tool argument (the agent obtains an id
from `create_session` and passes it back). For a single per-user bucket with
nothing for the agent to pass, inject `session: UserSession` instead.

State is keyed by `(principal, session_id)`: the authenticated principal is
the isolation wall and `session_id` organizes sessions within it. The id must
have been minted by `create_session` under the current principal; an id that
was never created, or created under a different principal, raises
`InvalidSession` rather than resolving to a fresh empty bucket (the specific
reason is logged at debug level, never returned to the caller).

Like `get_server()`, this resolves through the task-aware server, so it needs
no foreground context — it works from a `task=True` tool's Docket worker as
well as a normal request.


## is_docket_available

`fastmcp.server.dependencies.is_docket_available`

```python
def is_docket_available() -> bool
```

Check if a compatible pydocket (>= 0.19.0) is installed and importable.

Three things have to be true for fastmcp's task features to work:
  1. pydocket distribution metadata is discoverable
  2. its version is at least ``_MIN_DOCKET_VERSION`` (older versions are
     missing symbols like ``docket.dependencies.current_execution``,
     which fastmcp imports on the request hot path)
  3. the package actually imports — guards against broken/partial
     installs where metadata exists but ``import docket`` blows up

Any of those failing means we treat docket as unavailable and fall back
to the no-tasks code paths instead of crashing deep inside a request.


## resolve_dependencies

`fastmcp.server.dependencies.resolve_dependencies`

```python
async def resolve_dependencies(fn: Callable[..., Any], arguments: dict[str, Any]) -> AsyncGenerator[dict[str, Any], None]
```

Resolve dependencies for a FastMCP function.

This function:
1. Filters out any dependency parameter names from user arguments (security)
2. Resolves Depends() parameters via the DI system

The filtering prevents external callers from overriding injected parameters by
providing values for dependency parameter names. This is a security feature.
The filtered arguments also feed the resolution frame, so a CallArgument()
reference to a dependency parameter resolves the dependency and never a
caller-supplied value.

Note: Context injection is handled via transform_context_annotations() which
converts `ctx: Context` to `ctx: Context = Depends(get_context)` at registration
time, so all injection goes through the unified DI system.

Args:
    fn: The function to resolve dependencies for
    arguments: User arguments (may contain keys that match dependency names,
              which will be filtered out)

Yields:
    Dictionary of filtered user args + resolved dependencies

Example:
    ```python
    async with resolve_dependencies(my_tool, {"name": "Alice"}) as kwargs:
        result = my_tool(**kwargs)
        if inspect.isawaitable(result):
            result = await result
    ```


## set_background_context_factory

`fastmcp.server.dependencies.set_background_context_factory`

```python
def set_background_context_factory(factory: Callable[[], Awaitable[Context | None]] | None) -> None
```

Install (or clear) the background-task ``Context`` factory.

The factory returns an already-entered ``Context`` (so ``_current_context``
is set for cleanup) when called inside a worker, or ``None`` when there is
no task context. Passing ``None`` restores core's no-worker-fallback
behavior.


## set_worker_server_resolver

`fastmcp.server.dependencies.set_worker_server_resolver`

```python
def set_worker_server_resolver(resolver: Callable[[], FastMCP | None] | None) -> None
```

Install (or clear) the worker-server resolver used by ``get_server()``.


## transform_context_annotations

`fastmcp.server.dependencies.transform_context_annotations`

```python
def transform_context_annotations(fn: Callable[..., Any]) -> Callable[..., Any]
```

Transform injected-by-type params into Dependency-defaulted params.

Transforms ALL params typed as Context (into ``= CurrentContext()``) and as
UserSession (into ``= CurrentSession()``) to use Docket's DI system, unless
they already have a Dependency-based default.

This unifies the legacy type annotation DI with Docket's Depends() system,
allowing both patterns to work through a single resolution path.

Note: Only POSITIONAL_OR_KEYWORD parameters are reordered (params with defaults
after those without). KEYWORD_ONLY parameters keep their position since Python
allows them to have defaults in any order.

Args:
    fn: Function to transform

Returns:
    Function with modified signature (same function object, updated __signature__)


## without_injected_parameters

`fastmcp.server.dependencies.without_injected_parameters`

```python
def without_injected_parameters(fn: Callable[..., Any], run_in_thread: bool = True) -> Callable[..., Any]
```

Create a wrapper function without injected parameters.

Returns a wrapper that excludes Context and Docket dependency parameters,
making it safe to use with Pydantic TypeAdapter for schema generation and
validation. The wrapper internally handles all dependency resolution and
Context injection when called.

Handles:
- Legacy Context injection (always works)
- Depends() injection (always works - uses docket or vendored DI engine)

Args:
    fn: Original function with Context and/or dependencies
    run_in_thread: For sync ``fn``, whether to dispatch the call to a worker
        thread after resolving dependencies. Defaults to True. Set to False
        to call ``fn`` inline on the event loop thread — required for
        thread-affinity libraries (e.g. Windows COM). Ignored for async fns.

Returns:
    Async wrapper function without injected parameters


