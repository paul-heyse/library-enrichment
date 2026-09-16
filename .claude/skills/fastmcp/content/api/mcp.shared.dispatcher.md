# `mcp.shared.dispatcher`

Distribution: `mcp`

## OnNotify

Import as `mcp.server.runner.OnNotify`  ·  defined at `mcp.shared.dispatcher.OnNotify`

```python
OnNotify = Callable[[DispatchContext[TransportContext], str, Mapping[str, Any] | None], Awaitable[None]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( DispatchContext[TransportContext], str, Mapping[str, Any] | None, / ) -> Awaitable[None]'> ``` --- Handler for inbound notifications: `(ctx, method, params)`.`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Handler for inbound notifications: `(ctx, method, params)`.


## OnNotifyIntercept

Import as `mcp.shared.direct_dispatcher.OnNotifyIntercept`  ·  defined at `mcp.shared.dispatcher.OnNotifyIntercept`

```python
OnNotifyIntercept = Callable[[str, Mapping[str, Any] | None], bool]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( str, Mapping[str, Any] | None, / ) -> bool'> ``` --- Synchronous receive-order intercept for inbound notifications: `(method, params) -> consumed`. Runs before `on_notify` is scheduled so correlation state advances in wire order relative to response resolution (the client's listen demux depends on this). Returning True consumes the notification. Must not block the receive path.`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Synchronous receive-order intercept for inbound notifications: `(method, params) -> consumed`.

Runs before `on_notify` is scheduled so correlation state advances in wire order
relative to response resolution (the client's listen demux depends on this).
Returning True consumes the notification. Must not block the receive path.


## OnRequest

Import as `mcp.server.runner.OnRequest`  ·  defined at `mcp.shared.dispatcher.OnRequest`

```python
OnRequest = Callable[[DispatchContext[TransportContext], str, Mapping[str, Any] | None], Awaitable[dict[str, Any]]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( DispatchContext[TransportContext], str, Mapping[str, Any] | None, / ) -> Awaitable[dict[str, Any]]'> ``` --- Handler for inbound requests: `(ctx, method, params) -> result`. Raise `MCPError` to send an error response.`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Handler for inbound requests: `(ctx, method, params) -> result`. Raise `MCPError` to send an error response.


## TransportT_co

`mcp.shared.dispatcher.TransportT_co`

```python
TransportT_co = TypeVar('TransportT_co', bound=TransportContext, covariant=True)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`mcp.shared.dispatcher.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['CallOptions', 'DispatchContext', 'Dispatcher', 'OnNotify', 'OnNotifyIntercept', 'OnRequest', 'Outbound', 'ProgressFnT', 'as_request_id', 'coerce_request_id', 'run_notify_intercept']
```

## logger

`mcp.shared.dispatcher.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## CallOptions

Import as `mcp.shared.peer.CallOptions`  ·  defined at `mcp.shared.dispatcher.CallOptions`

```python
class CallOptions(TypedDict)
```

_10 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TypedDict`

**Declared members (7)**

- `cancel_on_abandon: bool`  _instance-attribute_
  Whether abandoning this request (timeout or caller cancellation) sends `notifications/cancelled`.
- `headers: dict[str, str]`  _instance-attribute_
  Transport-layer hint: HTTP transports merge these onto the outgoing request; non-HTTP transports ignore.
- `on_progress: ProgressFnT`  _instance-attribute_
  Receive `notifications/progress` updates for this request.
- `on_resumption_token: Callable[[str], Awaitable[None]]`  _instance-attribute_
  Receive a resumption token when the transport issues one for this request.
- `request_id: RequestId`  _instance-attribute_
  Send the request under this caller-supplied id instead of a dispatcher-minted one.
- `resumption_token: str`  _instance-attribute_
  Opaque token to resume a previously interrupted request.
- `timeout: float`  _instance-attribute_
  Seconds to wait for a result before raising and sending `notifications/cancelled`.

Per-call options for `Outbound.send_raw_request`.

All keys are optional. Dispatchers ignore keys they do not understand.


## DispatchContext

Import as `mcp.server.runner.DispatchContext`  ·  defined at `mcp.shared.dispatcher.DispatchContext`

```python
class DispatchContext(Outbound, Protocol[TransportT_co])
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Outbound`, `Protocol[TransportT_co]`

**Declared members (6)**

- `can_send_request: bool`  _property_
  Whether the back-channel can currently deliver server-initiated requests.
- `cancel_requested: anyio.Event`  _property_
  Set when the peer sends `notifications/cancelled` for this request.
- `message_metadata: MessageMetadata`  _property_
  The metadata the transport attached to this inbound message, if any.
- `async def progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
  Report progress for the inbound request, if the peer supplied a progress token.
- `request_id: RequestId | None`  _property_
  The id of the inbound request, or `None` for a notification.
- `transport: TransportT_co`  _property_
  Transport-specific metadata for this inbound message.

**Inherited (2)**

- from `mcp.shared.dispatcher.Outbound`: `notify`, `send_raw_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Per-request context handed to `on_request` / `on_notify`.

Carries the transport metadata for the inbound message and provides the
back-channel for sending requests/notifications to the peer while handling
it. `send_raw_request` raises `NoBackChannelError` if `can_send_request`
is `False`.


## Dispatcher

Import as `mcp.client.client.Dispatcher`  ·  defined at `mcp.shared.dispatcher.Dispatcher`

```python
class Dispatcher(Outbound, Protocol[TransportT_co])
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Outbound`, `Protocol[TransportT_co]`

**Declared members (1)**

- `async def run(self, on_request: OnRequest, on_notify: OnNotify, on_notify_intercept: OnNotifyIntercept | None = None, task_status: anyio.abc.TaskStatus[None] = anyio.TASK_STATUS_IGNORED) -> None`  _async_
  Drive the receive loop until the underlying channel closes.

**Inherited (2)**

- from `mcp.shared.dispatcher.Outbound`: `notify`, `send_raw_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A duplex request/notification channel with call-return semantics.

Implementations own correlation of outbound requests to inbound results, the
receive loop, per-request concurrency, and cancellation/progress wiring.

The lifecycle surface is provisional; `run()` may change in a 2.x minor
release.


## Outbound

Import as `mcp.shared.peer.Outbound`  ·  defined at `mcp.shared.dispatcher.Outbound`

```python
class Outbound(Protocol)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

**Declared members (2)**

- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
  Send a fire-and-forget notification.
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_
  Send a request and await its raw result dict.

Anything that can send requests and notifications to the peer.

Both `Dispatcher` (top-level outbound) and `DispatchContext` (back-channel
during an inbound request) extend this. The MCP type layer (`ClientPeer`,
`Connection`) builds typed `send_request` / convenience methods on top of
this raw channel.


## ProgressFnT

Import as `mcp.client.client.ProgressFnT`  ·  defined at `mcp.shared.dispatcher.ProgressFnT`

```python
class ProgressFnT(Protocol)
```

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

Callback invoked when a progress notification arrives for a pending request.


## as_request_id

Import as `mcp.client.session.as_request_id`  ·  defined at `mcp.shared.dispatcher.as_request_id`

```python
def as_request_id(value: object) -> RequestId | None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Narrow an untyped wire value to a `RequestId`, or None; rejects bool (True would alias request id 1).


## coerce_request_id

Import as `mcp.shared.direct_dispatcher.coerce_request_id`  ·  defined at `mcp.shared.dispatcher.coerce_request_id`

```python
def coerce_request_id(request_id: RequestId) -> RequestId
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Coerce a stringified int request id back to int so a peer-echoed id still correlates (matches the TS SDK).

This is the collision/correlation domain dispatchers share: "7" and 7 are one
id for correlation purposes, even where the wire carries the verbatim value.


## run_notify_intercept

Import as `mcp.shared.direct_dispatcher.run_notify_intercept`  ·  defined at `mcp.shared.dispatcher.run_notify_intercept`

```python
def run_notify_intercept(intercept: OnNotifyIntercept | None, method: str, params: Mapping[str, Any] | None) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Invoke `intercept`, containing a raise to that one notification (never the receive loop).


