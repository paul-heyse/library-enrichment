# `mcp.client.extension`

Distribution: `mcp`

## ClaimedT

`mcp.client.extension.ClaimedT`

```python
ClaimedT = TypeVar('ClaimedT', bound=Result)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## NotifyParamsT

`mcp.client.extension.NotifyParamsT`

```python
NotifyParamsT = TypeVar('NotifyParamsT', bound=BaseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _CLAIM_METHODS

`mcp.client.extension._CLAIM_METHODS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CLAIM_METHODS: Final[frozenset[str]] = frozenset({'tools/call'})
```

The closed set of verbs a claim may attach to; widen together with the `method` Literal.


## _RESERVED_WIRE_ALIASES

`mcp.client.extension._RESERVED_WIRE_ALIASES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_RESERVED_WIRE_ALIASES: Final[frozenset[str]] = frozenset({'requestState', 'inputRequests'})
```

Typed optional fields of the core result surface that pre-validates every inbound result.


## __all__

`mcp.client.extension.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ClaimContext', 'ClientExtension', 'NotificationBinding', 'ResultClaim', 'UnexpectedClaimedResult', 'advertise']
```

## ClaimContext

Import as `mcp.client.ClaimContext`  ·  defined at `mcp.client.extension.ClaimContext`

```python
class ClaimContext
```

**Also exported as** `mcp.client.ClaimContext`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `read_timeout_seconds: float | None`  _instance-attribute_
- `session: ClientSession`  _instance-attribute_
- `tool_name: str`  _instance-attribute_

Host-injected context for one `ResultClaim.resolve` call.


## ClientExtension

Import as `mcp.client.ClientExtension`  ·  defined at `mcp.client.extension.ClientExtension`

```python
class ClientExtension
```

**Also exported as** `mcp.client.ClientExtension`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `def claims(self) -> Sequence[ResultClaim[Any]]`
  Extra result shapes this extension claims, with their resolvers.
- `identifier: str`  _instance-attribute_
- `def notifications(self) -> Sequence[NotificationBinding[Any]]`
  Server notifications this extension observes.
- `def settings(self) -> dict[str, Any]`
  Per-extension settings advertised at `ClientCapabilities.extensions[identifier]`.

Base class for an opt-in client extension; override only what you need.

The surface is declarative, fixed at construction, and never receives the client.


## NotificationBinding

Import as `mcp.client.NotificationBinding`  ·  defined at `mcp.client.extension.NotificationBinding`

```python
class NotificationBinding(Generic[NotifyParamsT])
```

**Also exported as** `mcp.client.NotificationBinding`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[NotifyParamsT]`

**Declared members (3)**

- `handler: Callable[[NotifyParamsT], Awaitable[None]]`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params_type: type[NotifyParamsT]`  _instance-attribute_

Deliver server notifications for `method` (the bare wire name) to `handler`.

Observation-only: validated params arrive one at a time per binding, in
dispatch order, through a bounded queue that drops the oldest with a warning
on overflow. Stream transports dispatch each notification independently, so
near-simultaneous notifications may be dispatched out of wire order. Methods
the negotiated version's core tables handle are never delivered to bindings.


## ResultClaim

Import as `mcp.client.ResultClaim`  ·  defined at `mcp.client.extension.ResultClaim`

```python
class ResultClaim(Generic[ClaimedT])
```

**Also exported as** `mcp.client.ResultClaim`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[ClaimedT]`

**Declared members (5)**

- `method: Literal['tools/call'] = 'tools/call'`  _class-attribute, instance-attribute_
- `model: type[ClaimedT]`  _instance-attribute_
- `protocol_versions: frozenset[str] | None = None`  _class-attribute, instance-attribute_
- `resolve: Callable[[ClaimedT, ClaimContext], Awaitable[CallToolResult]]`  _instance-attribute_
- `result_type: str`  _instance-attribute_

One extra result shape on one spec verb, keyed by the wire `resultType`.

Active only while the declaring extension is constructed into the client and
the negotiated protocol version admits it. `resolve` finishes a claimed
result, may send follow-ups through `ctx.session`, and must return the
verb's ordinary result. All field constraints are enforced at construction.


## UnexpectedClaimedResult

Import as `mcp.client.UnexpectedClaimedResult`  ·  defined at `mcp.client.extension.UnexpectedClaimedResult`

```python
class UnexpectedClaimedResult(RuntimeError)
```

**Also exported as** `mcp.client.UnexpectedClaimedResult`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RuntimeError`

**Declared members (1)**

- `result = result`  _instance-attribute_

A claimed (extension) result arrived on a `call_tool` that did not opt in.

The parsed value is carried as `result`; the server may already hold state it
references. Opt in via `Client(extensions=[...])` or `allow_claimed=True`.


## _AdvertiseOnly

`mcp.client.extension._AdvertiseOnly`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _AdvertiseOnly(ClientExtension)
```

**Bases** `ClientExtension`

**Declared members (2)**

- `identifier = identifier`  _instance-attribute_
- `def settings(self) -> dict[str, Any]`

**Inherited (2)**

- from `mcp.client.extension.ClientExtension`: `claims`, `notifications`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Ad-only extension returned by `advertise()`.


## _wire_keys

`mcp.client.extension._wire_keys`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _wire_keys(name: str, field: FieldInfo) -> frozenset[str]
```

Every top-level wire key this field can read from or write to.


## advertise

Import as `mcp.client.advertise`  ·  defined at `mcp.client.extension.advertise`

```python
def advertise(identifier: str, settings: dict[str, Any] | None = None) -> ClientExtension
```

**Also exported as** `mcp.client.advertise`

Advertise an extension identifier (with optional settings) and nothing else.

Advertising an extension you do not implement asserts wire support you do
not have; for behavioral extensions construct the real extension instead.


