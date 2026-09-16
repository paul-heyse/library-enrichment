# `mcp.server.subscriptions`

Distribution: `mcp`

## __all__

`mcp.server.subscriptions.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['SUBSCRIPTION_ID_META_KEY', 'InMemorySubscriptionBus', 'ListenHandler', 'PromptsListChanged', 'ResourceUpdated', 'ResourcesListChanged', 'ServerEvent', 'SubscriptionBus', 'ToolsListChanged']
```

## logger

`mcp.server.subscriptions.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## InMemorySubscriptionBus

Import as `mcp.server.mcpserver.server.InMemorySubscriptionBus`  ·  defined at `mcp.server.subscriptions.InMemorySubscriptionBus`

```python
class InMemorySubscriptionBus
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `async def publish(self, event: ServerEvent) -> None`  _async_
  Deliver `event` to every subscribed listener.
- `def subscribe(self, listener: Callable[[ServerEvent], None]) -> Callable[[], None]`
  Register `listener` and return an idempotent unsubscribe callable.

In-process `SubscriptionBus`: synchronous fan-out to listeners in subscription order.


## ListenHandler

Import as `mcp.server.mcpserver.server.ListenHandler`  ·  defined at `mcp.server.subscriptions.ListenHandler`

```python
class ListenHandler
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `def close(self) -> None`
  Initiate graceful closure of every open listen stream.

Serves `subscriptions/listen`: one call is one subscription stream.

Register on a lowlevel `Server` via `on_subscriptions_listen=` (or
`add_request_handler`); `MCPServer` does so automatically. Each call
acknowledges the honored filter first, then forwards matching bus events
onto the request's response stream until the client disconnects (which
cancels the handler; the stream just ends, per the spec's abrupt-close
contract) or `close` ends all streams gracefully.

Served on any transport that can carry the request's response stream:
streamable HTTP's SSE mode, or a duplex stream pair such as stdio.

`max_subscriptions` bounds concurrent streams (further listen requests are
rejected with `INTERNAL_ERROR`, before the ack). `max_buffered_events`
bounds each stream's event backlog: a stream whose client has stopped
reading is ended at the cap (the client re-listens and refetches - there
is no replay, so ending the stream loses nothing the backlog wasn't
already losing).


## SubscriptionBus

Import as `mcp.server.mcpserver.server.SubscriptionBus`  ·  defined at `mcp.server.subscriptions.SubscriptionBus`

```python
class SubscriptionBus(Protocol)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

**Declared members (2)**

- `async def publish(self, event: ServerEvent) -> None`  _async_
  Deliver `event` to every subscribed listener.
- `def subscribe(self, listener: Callable[[ServerEvent], None]) -> Callable[[], None]`
  Register `listener` and return an idempotent unsubscribe callable.

Fan-out seam between event publishers and open listen streams.

Implement this over an external pub/sub backend (Redis, NATS, ...) to fan
events out across replicas: `publish` forwards the event to the backend,
and each replica's bus invokes its local listeners for events arriving
from the backend. The same instance can be shared across servers.

`publish` is async so backend implementations can do network I/O.
`subscribe` is synchronous local registration. Listeners are synchronous,
must not raise, and are invoked on the server's event loop.


## _honored_subset

`mcp.server.subscriptions._honored_subset`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _honored_subset(requested: SubscriptionFilter) -> SubscriptionFilter
```

The subset of `requested` the server will deliver, for the ack.

Every requested kind is honored - whether an event kind ever fires
depends on what the server publishes, exactly as a subscription to a
nonexistent resource URI is honored and never fires. Non-true flags and
an empty URI list are dropped rather than echoed as falsy values.


## _safe_unsubscribe

`mcp.server.subscriptions._safe_unsubscribe`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _safe_unsubscribe(unsubscribe: Callable[[], None]) -> None
```

Run a bus's unsubscribe callable, isolating the stream from it raising.

The callable comes from a custom `SubscriptionBus`; a raising one is
logged and skipped so it cannot stop the stream's own cleanup from
releasing its subscription slot.


