# `mcp.server.connection`

Distribution: `mcp`

## ResultT

`mcp.server.connection.ResultT`

```python
ResultT = TypeVar('ResultT', bound=BaseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _ALL_LOG_LEVELS

`mcp.server.connection._ALL_LOG_LEVELS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ALL_LOG_LEVELS: Final[frozenset[LoggingLevel]] = frozenset(_LOG_LEVELS)
```

## _LOG_LEVELS

`mcp.server.connection._LOG_LEVELS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_LOG_LEVELS: Final[tuple[LoggingLevel, ...]] = get_args(LoggingLevel)
```

Severity-ascending, from the `LoggingLevel` literal's declaration order (the
RFC 5424 scale) - the literal is the single source of the ordering.


## _ModelT

`mcp.server.connection._ModelT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ModelT = TypeVar('_ModelT', bound=BaseModel)
```

## _NO_CHANNEL

`mcp.server.connection._NO_CHANNEL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_NO_CHANNEL = _NoChannelOutbound()
```

## _RESULT_FOR

`mcp.server.connection._RESULT_FOR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_RESULT_FOR: dict[type[Request[Any, Any]], type[BaseModel]] = {CreateMessageRequest: CreateMessageResult, ElicitRequest: ElicitResult, ListRootsRequest: ListRootsResult, PingRequest: EmptyResult}
```

## __all__

`mcp.server.connection.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['Connection']
```

## _logger

`mcp.server.connection._logger`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_logger = logger
```

## logger

`mcp.server.connection.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## Connection

Import as `mcp.server.runner.Connection`  ·  defined at `mcp.server.connection.Connection`

```python
class Connection
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (22)**

- `def check_capability(self, capability: ClientCapabilities) -> bool`
  Return whether the connected client declared the given capability.
- `client_capabilities: ClientCapabilities | None = None`  _instance-attribute_
  The capabilities the peer declared: the handshake's on the loop path, the request envelope's on the modern path. `None` when none were declared. Kept in lockstep with `client_params` by its setter, and settable on its own for the modern en…
- `client_params: InitializeRequestParams | None`  _property, writable_
  The full `initialize` request params, or the equivalent built from the 2026-era envelope. `None` when no client info was supplied.
- `exit_stack: AsyncExitStack = AsyncExitStack()`  _instance-attribute_
  Per-connection teardown, unwound LIFO (shielded) when the connection closes. Push cleanup from handlers or middleware; exceptions are logged and swallowed.
- `def for_loop(cls, outbound: Outbound, session_id: str | None = None, protocol_version_hint: str | None = None) -> Connection`  _classmethod_
  A connection for the handshake-driven loop path.
- `def from_envelope(cls, protocol_version: str, client_info: Any, client_capabilities: Any, outbound: Outbound = _NO_CHANNEL) -> Connection`  _classmethod_
  A born-ready connection populated from a request's `_meta` envelope.
- `has_standalone_channel: bool`  _property_
  Whether this connection has a real back-channel for server-initiated messages. Derived from `outbound` - the no-channel sentinel is the only case that doesn't.
- `initialize_accepted: bool`  _property_
  True once the inbound request gate is open: `initialize` recorded the peer info, or the handshake completed outright (born-ready, or a bare `notifications/initialized`). Derived, never stored.
- `initialized: anyio.Event = anyio.Event()`  _instance-attribute_
  Set when `notifications/initialized` arrives (matches TS `oninitialized`); the point from which the spec permits server-initiated requests beyond ping/logging. Pre-set on connections built via `from_envelope`.
- `async def log(self, level: LoggingLevel, data: Any, logger: str | None = None, meta: Meta | None = None) -> None`  _async_
  Send a `notifications/message` log entry on the standalone stream. Best-effort.
- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
  Send a best-effort notification on the standalone stream.
- `outbound: Outbound = outbound`  _instance-attribute_
  The connection-scoped channel for server-initiated messages.
- `async def ping(self, meta: Meta | None = None, opts: CallOptions | None = None) -> None`  _async_
  Send a `ping` request on the standalone stream.
- `protocol_version: str = protocol_version`  _instance-attribute_
  The protocol version this connection speaks. Populated at construction by the factory and overwritten by `_handle_initialize` once the handshake commits on the loop path.
- `async def send_prompt_list_changed(self, meta: Meta | None = None) -> None`  _async_
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_
  Send a raw request on the standalone stream.
- `async def send_request(self, req: Request[Any, Any], result_type: type[BaseModel] | None = None, opts: CallOptions | None = None) -> BaseModel`  _async_
  Send a typed server-to-client request and return its typed result.
- `async def send_resource_list_changed(self, meta: Meta | None = None) -> None`  _async_
- `async def send_resource_updated(self, uri: str, meta: Meta | None = None) -> None`  _async_
- `async def send_tool_list_changed(self, meta: Meta | None = None) -> None`  _async_
- `session_id: str | None = session_id`  _instance-attribute_
- `state: dict[str, Any] = {}`  _instance-attribute_
  Per-connection scratch state; persists across requests on this connection.

Per-client connection state and standalone-stream `Outbound`.

Construct via `from_envelope` (modern single-exchange: born ready, no
back-channel) or `for_loop` (handshake-driven: ready once the client's
`notifications/initialized` arrives). Either way `protocol_version` is
populated at construction.


## NotifyOnlyOutbound

Import as `mcp.server.runner.NotifyOnlyOutbound`  ·  defined at `mcp.server.connection.NotifyOnlyOutbound`

```python
class NotifyOnlyOutbound(_NoChannelOutbound)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_NoChannelOutbound`

**Declared members (1)**

- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_

**Inherited (1)**

- from `mcp.server.connection._NoChannelOutbound`: `send_raw_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Connection-scoped `Outbound` that forwards notifications and refuses requests.

Installed by `serve_dual_era_loop` for modern (2026-07-28+) connections
over duplex stream transports: the pipe is real, so server notifications
ride it, but the modern protocol forbids server-initiated JSON-RPC
requests, so `send_raw_request` (inherited) refuses by construction.

Change notifications (`notifications/*/list_changed`,
`notifications/resources/updated`) are dropped with a debug log: at this
era they reach a client only through a `subscriptions/listen` stream it
opened, so a bare copy on the shared channel would be an unrequested
notification. Publish them on the server's `SubscriptionBus` instead.


## _NoChannelOutbound

`mcp.server.connection._NoChannelOutbound`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _NoChannelOutbound
```

**Declared members (2)**

- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_

Connection-scoped `Outbound` for the no-back-channel case.

The structural answer to "this connection cannot push to its peer":
`send_raw_request` raises `NoBackChannelError`; `notify` drops with a
debug log. `Connection.from_envelope` installs this so the modern
single-exchange path never needs a mode flag - the channel itself says no.


## _notification_params

`mcp.server.connection._notification_params`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _notification_params(payload: dict[str, Any] | None, meta: Meta | None) -> dict[str, Any] | None
```

## _typed

`mcp.server.connection._typed`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _typed(model: type[_ModelT], raw: Any) -> _ModelT | None
```

Validate a raw envelope value into a typed model.

A missing, null or mis-shaped value falls through to `ValidationError`
and is treated as not supplied so the request still routes. Spec methods
are separately re-validated by the kernel's per-version params surface,
which types the reserved `_meta` keys strictly.


## allowed_log_levels

Import as `mcp.server.context.allowed_log_levels`  ·  defined at `mcp.server.connection.allowed_log_levels`

```python
def allowed_log_levels(protocol_version: str, meta: Mapping[str, Any] | None) -> frozenset[LoggingLevel]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The `notifications/message` levels deliverable for one inbound request.

2026-07-28+ makes log delivery a per-request opt-in (server/utilities/
logging): the client sets the reserved `io.modelcontextprotocol/logLevel`
`_meta` key, absent means no levels - the server MUST NOT send - and
present means that level and above. An unrecognized value reads as absent;
spec methods already reject a malformed value at surface validation
before any handler runs, so that arm only serves custom methods, where
dropping is the safe direction. Connection-scoped emitters pass
`meta=None`: `logging/setLevel` is gone at 2026 and log delivery is
request-scoped only, so they deliver nothing. Handshake versions keep
their `logging/setLevel`-era semantics: every level may be sent, filtering
is the application's `logging/setLevel` handler's job as before.


