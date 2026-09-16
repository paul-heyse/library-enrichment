# `mcp.client.subscriptions`

Distribution: `mcp`

## OnEvent

`mcp.client.subscriptions.OnEvent`

```python
OnEvent = Callable[[ServerEvent], Awaitable[None]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(ToolsListChanged | PromptsListChanged | ResourcesListChanged | ResourceUpdated, /) -> Awaitable[None]'> ``` --- Per-event barrier awaited before a `Subscription` returns each event to its consumer.`

Per-event barrier awaited before a `Subscription` returns each event to its consumer.


## _MAX_PENDING_EVENTS

`mcp.client.subscriptions._MAX_PENDING_EVENTS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MAX_PENDING_EVENTS = 1024
```

Backlog backstop: the spec allows sub-resource URIs, so distinct pending
`ResourceUpdated` events are unbounded; overflowing this cap settles the
subscription lost rather than growing client memory.


## _SubscriptionEnd

`mcp.client.subscriptions._SubscriptionEnd`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SubscriptionEnd = Literal['graceful', 'lost', 'local']
```

## __all__

`mcp.client.subscriptions.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ListenNotSupportedError', 'OnEvent', 'PromptsListChanged', 'ResourceUpdated', 'ResourcesListChanged', 'ServerEvent', 'Subscription', 'SubscriptionLost', 'ToolsListChanged', 'listen']
```

## _listen_ids

`mcp.client.subscriptions._listen_ids`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_listen_ids = count(1)
```

Process-wide `listen-N` sequence: string ids can never collide with a dispatcher's minted ints.


## ListenNotSupportedError

`mcp.client.subscriptions.ListenNotSupportedError`

```python
class ListenNotSupportedError(RuntimeError)
```

**Bases** `RuntimeError`

**Declared members (1)**

- `negotiated_version = negotiated_version`  _instance-attribute_

`subscriptions/listen` requires a 2026-07-28 connection.


## ListenRoute

Import as `mcp.client.session.ListenRoute`  ·  defined at `mcp.client.subscriptions.ListenRoute`

```python
class ListenRoute
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (9)**

- `acked = anyio.Event()`  _instance-attribute_
- `def consume(self, event: ServerEvent) -> None`
  Remove a peeked event from the backlog.
- `def deliver(self, event: ServerEvent) -> None`
  Queue an event within the honored filter, deduplicated against the backlog.
- `end: _SubscriptionEnd | None = None`  _instance-attribute_
- `error: MCPError | None = None`  _instance-attribute_
- `honored: types.SubscriptionFilter | None = None`  _instance-attribute_
- `async def next_event(self) -> ServerEvent | _SubscriptionEnd`  _async_
  Peek the next pending event, or the stream's end once the backlog drains.
- `def set_acked(self, honored: types.SubscriptionFilter) -> None`
  Record the acknowledged filter; the first ack wins.
- `def settle(self, end: _SubscriptionEnd, error: MCPError | None = None) -> None`
  Record the stream's end; the first reason wins and wakes both waiters.

Package-internal demux state for one listen stream, fed synchronously in receive order by the session.


## Subscription

Import as `mcp.client.client.Subscription`  ·  defined at `mcp.client.subscriptions.Subscription`

```python
class Subscription
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `honored = honored`  _instance-attribute_
  The subset of the requested filter the server agreed to deliver.
- `subscription_id = subscription_id`  _instance-attribute_
  The listen request's JSON-RPC id, stamped into every frame's `_meta`.

One open `subscriptions/listen` stream: an async iterator of typed events.

Produced by `listen()` / `Client.listen()`, not constructed directly.


## SubscriptionLost

`mcp.client.subscriptions.SubscriptionLost`

```python
class SubscriptionLost(RuntimeError)
```

**Bases** `RuntimeError`

The stream ended without the server's graceful close; re-listen and refetch.


## listen

`mcp.client.subscriptions.listen`

```python
async def listen(session: ClientSession, tools_list_changed: bool = False, prompts_list_changed: bool = False, resources_list_changed: bool = False, resource_subscriptions: Sequence[str] = (), on_event: OnEvent | None = None) -> AsyncIterator[Subscription]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Open one `subscriptions/listen` stream on `session` (2026-07-28 only).

Entering sends the request and returns once the server's acknowledgment
arrives; exiting ends the subscription. `on_event` is awaited before each
event is returned - the seam `Client.listen` uses to finish cache eviction
before the consumer can refetch.

Raises:
    ListenNotSupportedError: negotiated version predates 2026-07-28.
    MCPError: the server rejected the request, or the connection failed pre-ack.
    SubscriptionLost: the stream ended before it was acknowledged.
    TimeoutError: the session's read timeout elapsed before the acknowledgment.


