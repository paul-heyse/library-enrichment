# `mcp.server.streamable_http`

Distribution: `mcp`

## CONTENT_TYPE_JSON

`mcp.server.streamable_http.CONTENT_TYPE_JSON`

```python
CONTENT_TYPE_JSON = 'application/json'
```

**Inferred type** (`ty`, not declared in the source): `Literal["application/json"]`

## CONTENT_TYPE_SSE

`mcp.server.streamable_http.CONTENT_TYPE_SSE`

```python
CONTENT_TYPE_SSE = 'text/event-stream'
```

**Inferred type** (`ty`, not declared in the source): `Literal["text/event-stream"]`

## EventCallback

Import as `fastmcp.server.event_store.EventCallback`  ·  defined at `mcp.server.streamable_http.EventCallback`

```python
EventCallback = Callable[[EventMessage], Awaitable[None]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(EventMessage, /) -> Awaitable[None]'> ````

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## EventId

Import as `fastmcp.server.event_store.EventId`  ·  defined at `mcp.server.streamable_http.EventId`

```python
EventId = str
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'str'> ````

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## GET_STREAM_KEY

`mcp.server.streamable_http.GET_STREAM_KEY`

```python
GET_STREAM_KEY = '_GET_stream'
```

**Inferred type** (`ty`, not declared in the source): `Literal["_GET_stream"]`

## LAST_EVENT_ID_HEADER

`mcp.server.streamable_http.LAST_EVENT_ID_HEADER`

```python
LAST_EVENT_ID_HEADER = 'last-event-id'
```

**Inferred type** (`ty`, not declared in the source): `Literal["last-event-id"]`

## MCP_SESSION_ID_HEADER

Import as `mcp.server.streamable_http_manager.MCP_SESSION_ID_HEADER`  ·  defined at `mcp.server.streamable_http.MCP_SESSION_ID_HEADER`

```python
MCP_SESSION_ID_HEADER = 'mcp-session-id'
```

**Inferred type** (`ty`, not declared in the source): `Literal["mcp-session-id"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## REQUEST_CANCELLED

`mcp.server.streamable_http.REQUEST_CANCELLED`

```python
REQUEST_CANCELLED: Final = -32800
```

## REQUEST_STREAM_BUFFER_SIZE

`mcp.server.streamable_http.REQUEST_STREAM_BUFFER_SIZE`

```python
REQUEST_STREAM_BUFFER_SIZE: Final = 16
```

## SESSION_ID_PATTERN

`mcp.server.streamable_http.SESSION_ID_PATTERN`

```python
SESSION_ID_PATTERN = re.compile('^[\\x21-\\x7E]+$')
```

**Inferred type** (`ty`, not declared in the source): `Pattern[str]`

## SSEEvent

`mcp.server.streamable_http.SSEEvent`

```python
SSEEvent = dict[str, Any]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'dict[str, Any]'> ````

## StreamId

Import as `fastmcp.server.event_store.StreamId`  ·  defined at `mcp.server.streamable_http.StreamId`

```python
StreamId = str
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'str'> ````

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## logger

`mcp.server.streamable_http.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## EventMessage

Import as `fastmcp.server.event_store.EventMessage`  ·  defined at `mcp.server.streamable_http.EventMessage`

```python
class EventMessage
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `event_id: str | None = None`  _class-attribute, instance-attribute_
- `message: JSONRPCMessage`  _instance-attribute_

A JSONRPCMessage with an optional event ID for stream resumability.


## EventStore

Import as `mcp.server.streamable_http_manager.EventStore`  ·  defined at `mcp.server.streamable_http.EventStore`

```python
class EventStore(ABC)
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`

**Declared members (2)**

- `async def replay_events_after(self, last_event_id: EventId, send_callback: EventCallback) -> StreamId | None`  _abstractmethod, async_
  Replays events that occurred after the specified event ID.
- `async def store_event(self, stream_id: StreamId, message: JSONRPCMessage | None) -> EventId`  _abstractmethod, async_
  Stores an event for later retrieval.

Interface for resumability support via event storage.


## StreamableHTTPServerTransport

Import as `mcp.server.streamable_http_manager.StreamableHTTPServerTransport`  ·  defined at `mcp.server.streamable_http.StreamableHTTPServerTransport`

```python
class StreamableHTTPServerTransport
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (9)**

- `def close_sse_stream(self, request_id: RequestId) -> None`
  Close SSE connection for a specific request without terminating the stream.
- `def close_standalone_sse_stream(self) -> None`
  Close the standalone GET SSE stream, triggering client reconnection.
- `async def connect(self) -> AsyncGenerator[tuple[ReadStream[SessionMessage | Exception], WriteStream[SessionMessage]], None]`  _async_
  Context manager that provides read and write streams for a connection.
- `async def handle_request(self, scope: Scope, receive: Receive, send: Send) -> None`  _async_
  Application entry point that handles all HTTP requests.
- `idle_scope: anyio.CancelScope | None = None`  _instance-attribute_
  Created when `connect()` is entered if `idle_timeout` is set; cancelled once no request has been in flight for `idle_timeout` seconds.
- `is_json_response_enabled = is_json_response_enabled`  _instance-attribute_
- `is_terminated: bool`  _property_
  Check if this transport has been explicitly terminated.
- `mcp_session_id = mcp_session_id`  _instance-attribute_
- `async def terminate(self) -> None`  _async_
  Terminate the current session, closing all streams.

HTTP server transport with event streaming support for MCP.

Handles JSON-RPC messages in HTTP POST requests with SSE streaming.
Supports optional JSON responses and session management.


## check_accept_headers

`mcp.server.streamable_http.check_accept_headers`

```python
def check_accept_headers(request: Request) -> tuple[bool, bool]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return (has_json, has_sse) for the request's Accept header, with RFC 7231 wildcard handling.

Supports wildcard media types per RFC 7231, section 5.3.2:
- */* matches any media type
- application/* matches any application/ subtype
- text/* matches any text/ subtype


