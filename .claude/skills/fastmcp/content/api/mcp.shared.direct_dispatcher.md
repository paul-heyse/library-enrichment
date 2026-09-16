# `mcp.shared.direct_dispatcher`

Distribution: `mcp`

## DIRECT_TRANSPORT_KIND

`mcp.shared.direct_dispatcher.DIRECT_TRANSPORT_KIND`

```python
DIRECT_TRANSPORT_KIND = 'direct'
```

**Inferred type** (`ty`, not declared in the source): `Literal["direct"]`

## _Notify

`mcp.shared.direct_dispatcher._Notify`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_Notify = Callable[[str, Mapping[str, Any] | None], Awaitable[None]]
```

## _Request

`mcp.shared.direct_dispatcher._Request`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_Request = Callable[[str, Mapping[str, Any] | None, CallOptions | None], Awaitable[dict[str, Any]]]
```

## __all__

`mcp.shared.direct_dispatcher.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['DirectDispatcher', 'create_direct_dispatcher_pair']
```

## logger

`mcp.shared.direct_dispatcher.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## DirectDispatcher

`mcp.shared.direct_dispatcher.DirectDispatcher`

```python
class DirectDispatcher
```

**Declared members (5)**

- `def close(self) -> None`
- `def connect_to(self, peer: DirectDispatcher) -> None`
- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
  Send a notification by invoking the peer's `on_notify` directly.
- `async def run(self, on_request: OnRequest, on_notify: OnNotify, on_notify_intercept: OnNotifyIntercept | None = None, task_status: anyio.abc.TaskStatus[None] = anyio.TASK_STATUS_IGNORED) -> None`  _async_
  Mark this side ready and park until `close()` is called.
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_
  Send a request by invoking the peer's `on_request` directly.

A `Dispatcher` that calls a peer's handlers directly, in-process.

Two instances are wired together with `create_direct_dispatcher_pair`; each
holds a reference to the other. `send_raw_request` on one awaits the peer's
`on_request`. `run` parks until `close` is called.

Lifecycle mirrors `JSONRPCDispatcher`: `send_raw_request` requires `run()`
to have started, and once a side has closed - via `close()` or `run()`
ending - `send_raw_request` raises `MCPError` (`CONNECTION_CLOSED`) and
inbound requests fail the peer's call the same way instead of invoking the
handler. Notifications are fire-and-forget in both directions: after close
they are silently dropped.


## _DirectDispatchContext

`mcp.shared.direct_dispatcher._DirectDispatchContext`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _DirectDispatchContext
```

**Declared members (8)**

- `can_send_request: bool`  _property_
- `cancel_requested: anyio.Event = field(default_factory=anyio.Event)`  _class-attribute, instance-attribute_
- `message_metadata: MessageMetadata = None`  _class-attribute, instance-attribute_
  Always `None`: in-memory dispatch attaches no transport metadata.
- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
- `async def progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
- `request_id: RequestId | None = None`  _class-attribute, instance-attribute_
  The caller-supplied `CallOptions["request_id"]`, else a dispatcher-synthesized id for requests; `None` for notifications.
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_
- `transport: TransportContext`  _instance-attribute_

`DispatchContext` for an inbound request on a `DirectDispatcher`.

The back-channel callables target the *originating* side, so a handler's
`send_raw_request` reaches the peer that made the inbound request.


## create_direct_dispatcher_pair

Import as `mcp.client.client.create_direct_dispatcher_pair`  ·  defined at `mcp.shared.direct_dispatcher.create_direct_dispatcher_pair`

```python
def create_direct_dispatcher_pair(can_send_request: bool = True, headers: Mapping[str, str] | None = None, raise_handler_exceptions: bool = True) -> tuple[DirectDispatcher, DirectDispatcher]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create two `DirectDispatcher` instances wired to each other.

Args:
    can_send_request: Sets `TransportContext.can_send_request` on both
        sides. Pass `False` to simulate a transport with no back-channel.
    headers: Sets `TransportContext.headers` on both sides.
    raise_handler_exceptions: When `True` (the default - this is an
        in-process debugging substrate), an unmapped handler exception
        reaches the caller as `MCPError` with the original chained as
        ``__cause__``. When `False` it is sanitized to an opaque
        `INTERNAL_ERROR` so the in-process path matches the wire.

Returns:
    A `(client, server)` pair. The wiring is symmetric, so the roles
    are conventional only.


