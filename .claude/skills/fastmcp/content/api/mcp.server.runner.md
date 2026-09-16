# `mcp.server.runner`

Distribution: `mcp`

## LifespanT

`mcp.server.runner.LifespanT`

```python
LifespanT = TypeVar('LifespanT', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _EXIT_STACK_CLOSE_TIMEOUT

`mcp.server.runner._EXIT_STACK_CLOSE_TIMEOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_EXIT_STACK_CLOSE_TIMEOUT: float = 5
```

Bound for `aclose_shielded`'s exit-stack unwind; a hung cleanup callback
must not wedge shutdown.


## _INIT_EXEMPT

`mcp.server.runner._INIT_EXEMPT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_INIT_EXEMPT: frozenset[str] = frozenset({'ping'})
```

## _PRE_REQUEST_REPLAY_LIMIT

`mcp.server.runner._PRE_REQUEST_REPLAY_LIMIT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PRE_REQUEST_REPLAY_LIMIT: int = 8
```

How many frames arriving ahead of the client's first request are kept
for the chosen era's loop (a bare `notifications/initialized` is the one that
matters); further ones are dropped and never decide the era.


## __all__

`mcp.server.runner.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['CallNext', 'ServerMiddleware', 'ServerRunner', 'aclose_shielded', 'modern_on_request', 'serve_connection', 'serve_dual_era_loop', 'serve_loop', 'serve_one']
```

## logger

`mcp.server.runner.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ServerRunner

`mcp.server.runner.ServerRunner`

```python
class ServerRunner(Generic[LifespanT])
```

**Bases** `Generic[LifespanT]`

**Declared members (6)**

- `connection: Connection`  _instance-attribute_
- `init_options: InitializationOptions | None = None`  _class-attribute, instance-attribute_
  `InitializeResult` payload. Defaults to `server.create_initialization_options()`.
- `lifespan_state: LifespanT`  _instance-attribute_
- `on_notify: OnNotify`  _cached, property_
- `on_request: OnRequest`  _cached, property_
- `server: Server[LifespanT]`  _instance-attribute_

Per-connection handler kernel. One instance per client connection.


## _NoServerRequestsDispatchContext

`mcp.server.runner._NoServerRequestsDispatchContext`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _NoServerRequestsDispatchContext
```

**Declared members (8)**

- `can_send_request: bool`  _property_
- `cancel_requested: anyio.Event`  _property_
- `message_metadata: MessageMetadata`  _property_
- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
- `async def progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
- `request_id: RequestId | None`  _property_
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_
- `transport: TransportContext`  _property_

Delegating `DispatchContext` that refuses server-initiated requests.

Wraps the loop dispatcher's per-message context for modern-era dispatch:
the modern protocol forbids server-initiated JSON-RPC requests, so
`send_raw_request` refuses while notifications and progress still ride
the duplex pipe.


## _apply_middleware

`mcp.server.runner._apply_middleware`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _apply_middleware(middleware: ServerMiddleware[Any], call_next: CallNext, ctx: ServerRequestContext[Any, Any]) -> Awaitable[HandlerResult]
```

Adapt one middleware to the `CallNext` shape: bind `call_next`, take
`ctx` at call time so a rewritten context flows down the chain.


## _dump_result

`mcp.server.runner._dump_result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _dump_result(result: Any) -> dict[str, Any]
```

## _extract_meta

`mcp.server.runner._extract_meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _extract_meta(params: Mapping[str, Any] | None) -> RequestParamsMeta | None
```

Lift `_meta` from raw params; `None` when absent or malformed, so
context construction is independent of params validity.


## _has_modern_envelope

`mcp.server.runner._has_modern_envelope`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_modern_envelope(params: Mapping[str, Any] | None) -> bool
```

Whether `params._meta` carries the reserved protocol-version key.

The `io.modelcontextprotocol/protocolVersion` key exists only in
2026-07-28+ envelopes and its prefix is spec-reserved, so legacy traffic
never mints it (a bare `_meta` is not evidence - legacy requests carry
`progressToken` there). The version key alone is the signal, not the full
required pair, so a half-built envelope still routes modern and gets the
classifier's INVALID_PARAMS naming the missing key.


## _initialize_after_modern_data

`mcp.server.runner._initialize_after_modern_data`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _initialize_after_modern_data(params: Mapping[str, Any] | None) -> dict[str, Any]
```

Error data for an `initialize` arriving on a modern-locked connection.

The typed -32022 payload when the client's proposed version is parseable;
otherwise just the supported list (the point is naming what we serve).


## _replay_from_opening_request

`mcp.server.runner._replay_from_opening_request`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _replay_from_opening_request(read_stream: ReadStream[SessionMessage | Exception]) -> AsyncIterator[tuple[JSONRPCRequest | None, ReadStream[SessionMessage | Exception]]]
```

Peek at the client's first request without consuming it.

Yields that request together with a stream that replays it - preceded by
up to `_PRE_REQUEST_REPLAY_LIMIT` earlier frames - and relays the rest of
`read_stream` behind it, sender contexts included. The request is `None`
if the channel closes before one arrives.


## _sender_context

`mcp.server.runner._sender_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _sender_context(stream: ReadStream[Any]) -> contextvars.Context
```

The per-message sender context a context-aware stream carries, else the current one.


## _serve_legacy_stream

`mcp.server.runner._serve_legacy_stream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _serve_legacy_stream(server: Server[LifespanT], read_stream: ReadStream[SessionMessage | Exception], write_stream: WriteStream[SessionMessage], lifespan_state: LifespanT, session_id: str | None, init_options: InitializationOptions | None, raise_exceptions: bool) -> None
```

Serve a 2025 handshake connection; enveloped requests are refused.


## _serve_modern_stream

`mcp.server.runner._serve_modern_stream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _serve_modern_stream(server: Server[LifespanT], read_stream: ReadStream[SessionMessage | Exception], write_stream: WriteStream[SessionMessage], lifespan_state: LifespanT, raise_exceptions: bool) -> None
```

Serve a 2026-07-28 connection: every request carries its own envelope.


## aclose_shielded

`mcp.server.runner.aclose_shielded`

```python
async def aclose_shielded(connection: Connection) -> None
```

Unwind ``connection.exit_stack`` under a shielded, bounded scope.

Called from a driver's ``finally``: the shield lets per-connection cleanup
callbacks run even when the driver itself is being cancelled, the
`_EXIT_STACK_CLOSE_TIMEOUT` bound stops a hung callback wedging shutdown,
and a raising callback is logged-and-swallowed so it never masks the
driver's own exception.


## modern_error_data

`mcp.server.runner.modern_error_data`

```python
def modern_error_data(exc: Exception) -> ErrorData
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Map a modern request's handler exception to its wire `ErrorData`.

The exception-to-wire fact shared by the modern entries (the
single-exchange HTTP path and the dual-era stream loop), so an identical
modern request fails identically on every transport: `MCPError` and
`ValidationError` map via the shared `handler_exception_to_error_data`
ladder; anything else is logged server-side and surfaced as a generic
INTERNAL_ERROR so handler internals never reach the wire.


## modern_on_request

Import as `mcp.client.client.modern_on_request`  ·  defined at `mcp.server.runner.modern_on_request`

```python
def modern_on_request(server: Server[LifespanT], lifespan_state: LifespanT) -> OnRequest
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return an `OnRequest` callback that serves each call via `serve_one` with a fresh per-request `Connection`.

Wire this into the server side of a `DirectDispatcher` peer-pair to drive an
in-process server on the modern per-request-envelope path (each request
carries protocol version, client info, and capabilities in `params._meta`;
no `initialize` handshake). The dispatch context is wrapped in the
server-requests denial, so the modern prohibition on server-initiated
JSON-RPC requests holds on this entry like on the others. Like `serve_one`,
this raises whatever the handler chain raises - the dispatcher owns the
exception-to-error mapping.


## serve_connection

Import as `mcp.server.streamable_http_manager.serve_connection`  ·  defined at `mcp.server.runner.serve_connection`

```python
async def serve_connection(server: Server[LifespanT], dispatcher: Dispatcher[Any], connection: Connection, lifespan_state: LifespanT, init_options: InitializationOptions | None = None, task_status: anyio.abc.TaskStatus[None] = anyio.TASK_STATUS_IGNORED) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Drive ``dispatcher`` until the underlying channel closes.

The loop-mode driver: builds the kernel, hands `on_request`/`on_notify`
to `dispatcher.run()`, and tears down `connection.exit_stack` (shielded)
on the way out. The entry constructs the `Connection`; this only consumes
it.


## serve_dual_era_loop

Import as `mcp.server.lowlevel.server.serve_dual_era_loop`  ·  defined at `mcp.server.runner.serve_dual_era_loop`

```python
async def serve_dual_era_loop(server: Server[LifespanT], read_stream: ReadStream[SessionMessage | Exception], write_stream: WriteStream[SessionMessage], lifespan_state: LifespanT, session_id: str | None = None, init_options: InitializationOptions | None = None, raise_exceptions: bool = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Drive `server` over a duplex stream pair, in the era the client opens with.

The client's first request decides the connection's protocol era, once:
a request carrying the 2026-07-28 per-request `_meta` envelope opens a
modern connection, and anything else - the `initialize` handshake, which
does not exist at 2026 versions even when a client stamps the envelope on
it - opens a legacy one. The deciding frame is replayed into the chosen
serving loop along with everything the client sent before it. A later
claim from the other era is refused: `initialize` on a modern connection
gets UNSUPPORTED_PROTOCOL_VERSION naming the served versions, and an
enveloped request on a legacy connection gets INVALID_REQUEST.


## serve_loop

Import as `mcp.server.streamable_http_manager.serve_loop`  ·  defined at `mcp.server.runner.serve_loop`

```python
async def serve_loop(server: Server[LifespanT], read_stream: ReadStream[SessionMessage | Exception], write_stream: WriteStream[SessionMessage], lifespan_state: LifespanT, session_id: str | None = None, init_options: InitializationOptions | None = None, raise_exceptions: bool = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Drive ``server`` in handshake-only loop mode over a stream pair until the channel closes.

Builds the loop-mode `JSONRPCDispatcher` + `Connection` and hands them to
`serve_connection`. The streamable-HTTP manager (which owns its lifespan
and serves the modern era on the single-exchange entry instead) calls
this; `Server.run` drives `serve_dual_era_loop`, which extends the same
dispatcher recipe (notably the `inline_methods={"initialize"}` rule) with
era routing.


## serve_one

`mcp.server.runner.serve_one`

```python
async def serve_one(server: Server[LifespanT], dctx: DispatchContext[TransportContext], method: str, params: Mapping[str, Any] | None, connection: Connection, lifespan_state: LifespanT) -> dict[str, Any]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Handle a single request ``(method, params)`` and return its result dict.

The single-exchange driver: builds the kernel, runs `on_request` once under
`dctx`, and tears down `connection.exit_stack` (shielded) on the way out.
The entry constructs the (born-ready) `Connection` and the `dctx`; this
only consumes them.

Raises whatever the handler chain raises (`MCPError` / `ValidationError` /
unmapped); callers own the exception-to-wire mapping.


