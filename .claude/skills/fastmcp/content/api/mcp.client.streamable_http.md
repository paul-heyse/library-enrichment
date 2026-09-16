# `mcp.client.streamable_http`

Distribution: `mcp`

## DEFAULT_RECONNECTION_DELAY_MS

`mcp.client.streamable_http.DEFAULT_RECONNECTION_DELAY_MS`

```python
DEFAULT_RECONNECTION_DELAY_MS = 1000
```

**Inferred type** (`ty`, not declared in the source): `Literal[1000]`

## LAST_EVENT_ID

`mcp.client.streamable_http.LAST_EVENT_ID`

```python
LAST_EVENT_ID = 'last-event-id'
```

**Inferred type** (`ty`, not declared in the source): `Literal["last-event-id"]`

## MAX_RECONNECTION_ATTEMPTS

`mcp.client.streamable_http.MAX_RECONNECTION_ATTEMPTS`

```python
MAX_RECONNECTION_ATTEMPTS = 2
```

**Inferred type** (`ty`, not declared in the source): `Literal[2]`

## MCP_SESSION_ID

`mcp.client.streamable_http.MCP_SESSION_ID`

```python
MCP_SESSION_ID = 'mcp-session-id'
```

**Inferred type** (`ty`, not declared in the source): `Literal["mcp-session-id"]`

## SessionMessageOrError

`mcp.client.streamable_http.SessionMessageOrError`

```python
SessionMessageOrError = SessionMessage | Exception
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'SessionMessage | Exception'> ````

## StreamReader

`mcp.client.streamable_http.StreamReader`

```python
StreamReader = ContextReceiveStream[SessionMessage]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'ContextReceiveStream[SessionMessage]'> ````

## StreamWriter

`mcp.client.streamable_http.StreamWriter`

```python
StreamWriter = ContextSendStream[SessionMessageOrError]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'ContextSendStream[SessionMessage | Exception]'> ````

## logger

`mcp.client.streamable_http.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## RequestContext

`mcp.client.streamable_http.RequestContext`

```python
class RequestContext
```

**Declared members (5)**

- `client: httpx2.AsyncClient`  _instance-attribute_
- `metadata: ClientMessageMetadata | None`  _instance-attribute_
- `read_stream_writer: StreamWriter`  _instance-attribute_
- `session_id: str | None`  _instance-attribute_
- `session_message: SessionMessage`  _instance-attribute_

Context for a request operation.


## ResumptionError

`mcp.client.streamable_http.ResumptionError`

```python
class ResumptionError(StreamableHTTPError)
```

**Bases** `StreamableHTTPError`

Raised when resumption request is invalid.


## StreamableHTTPError

`mcp.client.streamable_http.StreamableHTTPError`

```python
class StreamableHTTPError(Exception)
```

**Bases** `Exception`

Base exception for StreamableHTTP transport errors.


## StreamableHTTPTransport

`mcp.client.streamable_http.StreamableHTTPTransport`

```python
class StreamableHTTPTransport
```

**Declared members (5)**

- `async def handle_get_stream(self, client: httpx2.AsyncClient, read_stream_writer: StreamWriter) -> None`  _async_
  Handle GET stream for server-initiated messages with auto-reconnect.
- `async def post_writer(self, client: httpx2.AsyncClient, write_stream_reader: StreamReader, read_stream_writer: StreamWriter, write_stream: ContextSendStream[SessionMessage], start_get_stream: Callable[[], None], tg: TaskGroup) -> None`  _async_
  Handle writing requests to the server.
- `session_id: str | None = None`  _instance-attribute_
- `async def terminate_session(self, client: httpx2.AsyncClient) -> None`  _async_
  Terminate the session by sending a DELETE request.
- `url = url`  _instance-attribute_

StreamableHTTP client transport implementation.


## _InFlightPost

`mcp.client.streamable_http._InFlightPost`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _InFlightPost
```

**Declared members (2)**

- `modern: bool`  _instance-attribute_
- `scope: anyio.CancelScope`  _instance-attribute_

A request POST in flight: its abort scope and the era it was sent under.

`modern` is the negotiated-version cache as of this request's dequeue, so a
later cancel frame is interpreted under the era the request actually ran
with, not whatever the cache says by then.


## _unfollowed_redirect

`mcp.client.streamable_http._unfollowed_redirect`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _unfollowed_redirect(response: httpx2.Response) -> str | None
```

Describe a redirect `stream_within_origin` left unfollowed, or None if `response` is not one.


## streamable_http_client

Import as `mcp.client.client.streamable_http_client`  ·  defined at `mcp.client.streamable_http.streamable_http_client`

```python
async def streamable_http_client(url: str, http_client: httpx2.AsyncClient | None = None, terminate_on_close: bool = True) -> AsyncGenerator[TransportStreams, None]
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Client transport for StreamableHTTP.

Args:
    url: The MCP server endpoint URL.
    http_client: Optional pre-configured httpx2.AsyncClient. If None, a default
        client with recommended MCP timeouts will be created. To configure headers,
        authentication, or other HTTP settings, create an httpx2.AsyncClient and pass it here.
        Whichever client is used, MCP requests follow a redirect only when it stays on the
        endpoint's origin (same scheme, host and port, or http to https on the same host with
        default ports) and keeps the request method (307/308 for a POST; any status for the GET
        stream); any other redirect is not followed and the message it answered fails with an
        error naming the location. The
        client's `follow_redirects` setting is not consulted; the SDK's OAuth providers apply the
        same rule to the requests they make.
    terminate_on_close: If True, send a DELETE request to terminate the session when the context exits.

Yields:
    Tuple containing:
        - read_stream: Stream for reading messages from the server
        - write_stream: Stream for sending messages to the server

Example:
    See examples/snippets/clients/ for usage patterns.


