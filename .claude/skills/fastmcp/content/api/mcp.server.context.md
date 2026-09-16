# `mcp.server.context`

Distribution: `mcp`

## CallNext

Import as `mcp.server.runner.CallNext`  ·  defined at `mcp.server.context.CallNext`

```python
CallNext = Callable[['ServerRequestContext[Any, Any]'], Awaitable[HandlerResult]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(ServerRequestContext[Any, Any], /) -> Awaitable[BaseModel | dict[str, Any] | None]'> ``` --- Invokes the rest of the chain with the given context. What a context rewrite (`dataclasses.replace(ctx, ...)`) can alter depends on the tier: `ServerMiddleware` runs before params validation, so its rewrites change what the handler is invoked with; an `Extension` interceptor runs after, so its rewrites change only what the handler observes on `ctx`.`

**Also exported as** `mcp.server.runner.CallNext`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Invokes the rest of the chain with the given context. What a context
rewrite (`dataclasses.replace(ctx, ...)`) can alter depends on the tier:
`ServerMiddleware` runs before params validation, so its rewrites change what
the handler is invoked with; an `Extension` interceptor runs after, so its
rewrites change only what the handler observes on `ctx`.


## HandlerResult

Import as `mcp.server.runner.HandlerResult`  ·  defined at `mcp.server.context.HandlerResult`

```python
HandlerResult = BaseModel | dict[str, Any] | None
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'BaseModel | dict[str, Any] | None'> ``` --- What a request handler (or middleware) may return. `ServerRunner` serializes all three to a result dict.`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

What a request handler (or middleware) may return. `ServerRunner` serializes
all three to a result dict.


## LifespanContextT

Import as `mcp.server.mcpserver.context.LifespanContextT`  ·  defined at `mcp.server.context.LifespanContextT`

```python
LifespanContextT = TypeVar('LifespanContextT', default=dict[str, Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## LifespanT_co

`mcp.server.context.LifespanT_co`

```python
LifespanT_co = TypeVar('LifespanT_co', default=Any, covariant=True)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## RequestT

Import as `mcp.server.mcpserver.context.RequestT`  ·  defined at `mcp.server.context.RequestT`

```python
RequestT = TypeVar('RequestT', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _MwLifespanT

`mcp.server.context._MwLifespanT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MwLifespanT = TypeVar('_MwLifespanT')
```

## _logger

`mcp.server.context._logger`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_logger = logger
```

## logger

`mcp.server.context.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## Context

`mcp.server.context.Context`

```python
class Context(BaseContext[TransportContext], Generic[LifespanT_co])
```

**Bases** `BaseContext[TransportContext]`, `Generic[LifespanT_co]`

**Declared members (5)**

- `connection: Connection`  _property_
  The per-client `Connection` for this request's connection.
- `headers: Mapping[str, str] | None`  _property_
  Request headers carried by this message, when the transport has them.
- `lifespan: LifespanT_co`  _property_
  The server-wide lifespan output (what `Server(..., lifespan=...)` yielded).
- `async def log(self, level: LoggingLevel, data: Any, logger: str | None = None, meta: Meta | None = None) -> None`  _async_
  Send a request-scoped `notifications/message` log entry.
- `session_id: str | None`  _property_
  The transport's session id for this connection, when one exists.

**Inherited (7)**

- from `mcp.shared.context.BaseContext`: `can_send_request`, `cancel_requested`, `meta`, `notify`, `report_progress`, `send_raw_request`, `transport`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Server-side per-request context.

Extends `BaseContext` (transport metadata, the raw back-channel, progress
reporting) with `lifespan`, `connection`, and request-scoped `log`.

Not currently constructed by `ServerRunner`, which hands handlers a
`ServerRequestContext` instead.


## ServerMiddleware

Import as `mcp.server.runner.ServerMiddleware`  ·  defined at `mcp.server.context.ServerMiddleware`

```python
class ServerMiddleware(Protocol[_MwLifespanT])
```

**Also exported as** `mcp.server.runner.ServerMiddleware`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol[_MwLifespanT]`

Context-tier middleware: `(ctx, call_next) -> result`.

Runs at the top of `ServerRunner._on_request` / `_on_notify` after `ctx`
is built but before any validation, lookup, or handshake. Wraps every
inbound request and notification: `initialize`, the pre-init gate,
`METHOD_NOT_FOUND`, params validation, the handler call, and
`notifications/initialized` all run inside `call_next(ctx)`.
`notifications/cancelled` is observed too; the dispatcher applies the
cancellation itself, then forwards the notification. A request-side
failure reaches the middleware as a raised `MCPError` (or
`ValidationError` for malformed params) so observation/logging middleware
can record it. Listed outermost-first on `Server.middleware`.

The method and the raw inbound params are `ctx.method` and `ctx.params` (no
model validation has happened yet). To rewrite either before the handler
runs, pass an adjusted context: `await call_next(replace(ctx, params=...))`.
`ctx.request_id is None` distinguishes a notification from a request. For
notifications `call_next(ctx)` returns `None` (a dropped or unhandled
notification also returns `None`) and the middleware's own return value is
discarded.

!!! warning
    `initialize` is handled inline - the dispatcher does not read
    further inbound messages until the middleware chain returns. Awaiting a
    server-to-client request (`ctx.session.send_request`, `send_ping`, ...)
    while handling `initialize` therefore deadlocks the connection: the
    response can never be dequeued. Send-and-forget notifications are safe.
    `initialize` is observed but not rewritable: the post-chain handshake
    commit reads the wire params, so to veto the handshake raise *before*
    `call_next()`.

`Server[L].middleware` holds `ServerMiddleware[L]`, so an app-specific
middleware sees `ctx.lifespan_context: L`. While the context is the
mutable `ServerRequestContext` dataclass it is invariant in `L`, so a
reusable middleware should be typed `ServerMiddleware[Any]` to register on
any `Server[L]`.


## ServerRequestContext

Import as `mcp.server.ServerRequestContext`  ·  defined at `mcp.server.context.ServerRequestContext`

```python
class ServerRequestContext(Generic[LifespanContextT, RequestT])
```

**Also exported as** `mcp.server.ServerRequestContext`

_15 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[LifespanContextT, RequestT]`

**Declared members (10)**

- `close_sse_stream: CloseSSEStreamCallback | None = None`  _class-attribute, instance-attribute_
- `close_standalone_sse_stream: CloseSSEStreamCallback | None = None`  _class-attribute, instance-attribute_
- `lifespan_context: LifespanContextT`  _instance-attribute_
- `meta: RequestParamsMeta | None = None`  _class-attribute, instance-attribute_
- `method: str`  _instance-attribute_
- `params: Mapping[str, Any] | None = None`  _class-attribute, instance-attribute_
- `protocol_version: str`  _instance-attribute_
- `request: RequestT | None = None`  _class-attribute, instance-attribute_
- `request_id: RequestId | None = None`  _class-attribute, instance-attribute_
- `session: ServerSession`  _instance-attribute_

Per-request context handed to lowlevel request and notification handlers.

Built by `ServerRunner._make_context` for each inbound message. Carries the
connection-scoped `ServerSession` (server-to-client requests and
notifications), per-request metadata, and any per-message data the
transport attached (the HTTP request, SSE stream-close callbacks).


