# `fastmcp.server.session_scoped_event_store`

Distribution: `fastmcp`

## logger

`fastmcp.server.session_scoped_event_store.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## SessionScopedEventStore

Import as `fastmcp.server.http.SessionScopedEventStore`  ·  defined at `fastmcp.server.session_scoped_event_store.SessionScopedEventStore`

```python
class SessionScopedEventStore(EventStore)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `EventStore`

**Declared members (2)**

- `async def replay_events_after(self, last_event_id: EventId, send_callback: EventCallback) -> StreamId | None`  _async_
- `async def store_event(self, stream_id: StreamId, message: JSONRPCMessage | None) -> EventId`  _async_

EventStore adapter that isolates stream IDs to one transport session.


