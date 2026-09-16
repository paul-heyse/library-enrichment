# `mcp.shared.jsonrpc_dispatcher`

Distribution: `mcp`

## PeerCancelMode

`mcp.shared.jsonrpc_dispatcher.PeerCancelMode`

```python
PeerCancelMode = Literal['interrupt', 'signal']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["interrupt", "signal"]'> ``` --- How `notifications/cancelled` is applied: `"interrupt"` (default) cancels the handler's scope; `"signal"` only sets `ctx.cancel_requested` and lets the handler run to completion. Either way the cancelled request is never answered - the handler's eventual result or error is dropped, not written.`

How `notifications/cancelled` is applied: `"interrupt"` (default) cancels
the handler's scope; `"signal"` only sets `ctx.cancel_requested` and lets the
handler run to completion. Either way the cancelled request is never
answered - the handler's eventual result or error is dropped, not written.


## TransportT

`mcp.shared.jsonrpc_dispatcher.TransportT`

```python
TransportT = TypeVar('TransportT', bound=TransportContext, default=TransportContext)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _ABANDON_WRITE_TIMEOUT

`mcp.shared.jsonrpc_dispatcher._ABANDON_WRITE_TIMEOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ABANDON_WRITE_TIMEOUT: float = 5
```

Bound for courtesy-cancel writes on the abandon paths; the caller-cancel
arm shields its write, so a wedged transport would otherwise hang it uncancellably.


## _SHUTDOWN_WRITE_TIMEOUT

`mcp.shared.jsonrpc_dispatcher._SHUTDOWN_WRITE_TIMEOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SHUTDOWN_WRITE_TIMEOUT: float = 1
```

Tighter bound for the shutdown-arm error write so a wedged transport can't hold session close.


## __all__

`mcp.shared.jsonrpc_dispatcher.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['JSONRPCDispatcher', 'cancelled_request_id_from_params', 'handler_exception_to_error_data', 'progress_token_from_params']
```

## logger

`mcp.shared.jsonrpc_dispatcher.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## JSONRPCDispatcher

Import as `mcp.client.client.JSONRPCDispatcher`  ·  defined at `mcp.shared.jsonrpc_dispatcher.JSONRPCDispatcher`

```python
class JSONRPCDispatcher(Dispatcher[TransportT])
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Dispatcher[TransportT]`

**Declared members (4)**

- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None, _related_request_id: RequestId | None = None) -> None`  _async_
  Send a fire-and-forget notification.
- `on_stream_exception = on_stream_exception`  _instance-attribute_
  Observer for ``Exception`` items on the read stream. Mutable so a session can bind it after the dispatcher is built (e.g. ``ClientSession`` routing into ``message_handler``); only consulted inside ``run()`` so pre-enter assignment is safe.
- `async def run(self, on_request: OnRequest, on_notify: OnNotify, on_notify_intercept: OnNotifyIntercept | None = None, task_status: anyio.abc.TaskStatus[None] = anyio.TASK_STATUS_IGNORED) -> None`  _async_
  Drive the receive loop until the read stream closes.
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None, _related_request_id: RequestId | None = None) -> dict[str, Any]`  _async_
  Send a JSON-RPC request and await its response.

`Dispatcher` over the `SessionMessage` stream contract.

Explicit Protocol base so pyright checks conformance at the class definition.


## _InFlight

`mcp.shared.jsonrpc_dispatcher._InFlight`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _InFlight(Generic[TransportT])
```

**Bases** `Generic[TransportT]`

**Declared members (2)**

- `dctx: _JSONRPCDispatchContext[TransportT]`  _instance-attribute_
- `scope: anyio.CancelScope`  _instance-attribute_

An inbound request currently being handled.


## _JSONRPCDispatchContext

`mcp.shared.jsonrpc_dispatcher._JSONRPCDispatchContext`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _JSONRPCDispatchContext(Generic[TransportT])
```

**Bases** `Generic[TransportT]`

**Declared members (9)**

- `can_send_request: bool`  _property_
- `cancel_requested: anyio.Event = field(default_factory=anyio.Event)`  _class-attribute, instance-attribute_
- `def close(self) -> None`
- `message_metadata: MessageMetadata = None`  _class-attribute, instance-attribute_
  Transport-attached `SessionMessage.metadata` that the server lifts onto its request context.
- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
- `async def progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
- `request_id: RequestId | None`  _property_
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_
- `transport: TransportT`  _instance-attribute_

Concrete `DispatchContext` produced for each inbound JSON-RPC message.


## _OutboundPlan

`mcp.shared.jsonrpc_dispatcher._OutboundPlan`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _OutboundPlan
```

**Declared members (2)**

- `cancel_on_abandon: bool`  _instance-attribute_
- `metadata: MessageMetadata`  _instance-attribute_

Outbound metadata plus whether abandoning the request sends a courtesy `notifications/cancelled`.


## _Pending

`mcp.shared.jsonrpc_dispatcher._Pending`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Pending
```

**Declared members (3)**

- `on_progress: ProgressFnT | None = None`  _class-attribute, instance-attribute_
- `receive: MemoryObjectReceiveStream[dict[str, Any] | ErrorData]`  _instance-attribute_
- `send: MemoryObjectSendStream[dict[str, Any] | ErrorData]`  _instance-attribute_

An outbound request awaiting its response.


## _contained_notify

`mcp.shared.jsonrpc_dispatcher._contained_notify`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _contained_notify(fn: OnNotify) -> OnNotify
```

Wrap a notification handler so it can't crash the dispatcher (same boundary as `_shielded_progress`).


## _default_transport_builder

`mcp.shared.jsonrpc_dispatcher._default_transport_builder`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _default_transport_builder(metadata: MessageMetadata) -> TransportContext
```

The `TransportContext` for a message, honoring the transport's own verdict when it stamps one.

A message reads as riding a full duplex pipe (`can_send_request=True`)
unless the transport that framed it says otherwise on the metadata it
attached, so a transport whose response has no room for a server request
(streamable HTTP in JSON-response mode) needs no wiring from whoever drives
its streams.


## _plan_outbound

`mcp.shared.jsonrpc_dispatcher._plan_outbound`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _plan_outbound(related_request_id: RequestId | None, opts: CallOptions | None) -> _OutboundPlan
```

Choose the outbound `SessionMessage.metadata` and the abandon-cancellation policy.

`related_request_id` wins over resumption hints (they are dropped). Only
hints that actually reach the transport suppress the courtesy cancel - a
request that is neither resumable nor cancelled would leak the peer's work.


## _shielded_progress

`mcp.shared.jsonrpc_dispatcher._shielded_progress`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _shielded_progress(fn: ProgressFnT) -> ProgressFnT
```

Wrap a user progress callback so an exception can't cancel the dispatcher's task group.


## cancelled_request_id_from_params

Import as `mcp.client.session.cancelled_request_id_from_params`  ·  defined at `mcp.shared.jsonrpc_dispatcher.cancelled_request_id_from_params`

```python
def cancelled_request_id_from_params(params: Mapping[str, Any] | None) -> RequestId | None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Read `params.requestId` from a `notifications/cancelled` (`as_request_id` shape rules).


## handler_exception_to_error_data

Import as `mcp.server.runner.handler_exception_to_error_data`  ·  defined at `mcp.shared.jsonrpc_dispatcher.handler_exception_to_error_data`

```python
def handler_exception_to_error_data(exc: BaseException) -> ErrorData | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Map a handler-raised exception to its wire `ErrorData`.

The two rungs every dispatcher shares: an `MCPError` carries its own
`ErrorData`; a pydantic `ValidationError` is the spec's INVALID_PARAMS
with empty ``data`` (no pydantic text on the wire). Returns ``None`` for
any other exception so each caller applies its own catch-all -
`JSONRPCDispatcher` currently pins ``code=0`` for v1 compat,
the modern HTTP entry uses `INTERNAL_ERROR`.


## progress_token_from_params

`mcp.shared.jsonrpc_dispatcher.progress_token_from_params`

```python
def progress_token_from_params(params: Mapping[str, Any] | None) -> ProgressToken | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Read `params._meta.progressToken`; reject bool (bool subclasses int, so True would alias 1).


