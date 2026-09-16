# `fastmcp.server.event_store`

Distribution: `fastmcp`

## _LOCK_STRIPES

`fastmcp.server.event_store._LOCK_STRIPES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_LOCK_STRIPES = 64
```

## _jsonrpc_message_adapter

`fastmcp.server.event_store._jsonrpc_message_adapter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_jsonrpc_message_adapter: TypeAdapter[JSONRPCMessage] = TypeAdapter(JSONRPCMessage)
```

## logger

`fastmcp.server.event_store.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## EventEntry

`fastmcp.server.event_store.EventEntry`

```python
class EventEntry(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (3)**

- `event_id: str`  _instance-attribute_
- `message: dict | None`  _instance-attribute_
- `stream_id: str`  _instance-attribute_

Stored event entry.


## EventStore

`fastmcp.server.event_store.EventStore`

```python
class EventStore(SDKEventStore)
```

**Bases** `SDKEventStore`

**Declared members (2)**

- `async def replay_events_after(self, last_event_id: EventId, send_callback: EventCallback) -> StreamId | None`  _async_
  Replay events that occurred after the specified event ID.
- `async def store_event(self, stream_id: StreamId, message: JSONRPCMessage | None) -> EventId`  _async_
  Store an event and return its ID.

EventStore implementation backed by AsyncKeyValue.

Enables SSE polling/resumability by storing events that can be replayed
when clients reconnect. Works with any AsyncKeyValue backend (memory, Redis, etc.)
following the same pattern as ResponseCachingMiddleware and OAuthProxy.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.event_store import EventStore

    # Default in-memory storage
    event_store = EventStore()

    # Or with a custom backend
    from key_value.aio.stores.redis import RedisStore
    redis_backend = RedisStore(url="redis://localhost")
    event_store = EventStore(storage=redis_backend)

    mcp = FastMCP("MyServer")
    app = mcp.http_app(event_store=event_store, retry_interval=2000)
    ```

Args:
    storage: AsyncKeyValue backend. Defaults to MemoryStore.
    max_events_per_stream: Maximum events to retain per stream. Default 100.
    ttl: Event TTL in seconds. Default 3600 (1 hour). Set to None for no expiration.


## StreamEventList

`fastmcp.server.event_store.StreamEventList`

```python
class StreamEventList(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (1)**

- `event_ids: list[str]`  _instance-attribute_

List of event IDs for a stream.


