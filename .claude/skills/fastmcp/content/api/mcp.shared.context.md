# `mcp.shared.context`

Distribution: `mcp`

## TransportT

`mcp.shared.context.TransportT`

```python
TransportT = TypeVar('TransportT', bound=TransportContext, default=TransportContext, covariant=True)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`mcp.shared.context.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['BaseContext']
```

## BaseContext

Import as `mcp.server.context.BaseContext`  ·  defined at `mcp.shared.context.BaseContext`

```python
class BaseContext(Generic[TransportT])
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[TransportT]`

**Declared members (7)**

- `can_send_request: bool`  _property_
  Whether the back-channel can currently deliver server-initiated requests.
- `cancel_requested: anyio.Event`  _property_
  Set when the peer sends `notifications/cancelled` for this request.
- `meta: RequestParamsMeta | None`  _property_
  The inbound request's `_meta` field, if present.
- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
  Send a notification to the peer on the back-channel.
- `async def report_progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
  Report progress for this request, if the peer supplied a progress token.
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_
  Send a request to the peer on the back-channel.
- `transport: TransportT`  _property_
  Transport-specific metadata for this inbound request.

Per-request context wrapping a `DispatchContext`.

`ServerRunner` constructs one per inbound request and passes it to the
user's handler.


