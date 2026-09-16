# `mcp.server.streamable_http_manager`

Distribution: `mcp`

## DEFAULT_MAX_SESSIONS

Import as `mcp.server.lowlevel.server.DEFAULT_MAX_SESSIONS`  ·  defined at `mcp.server.streamable_http_manager.DEFAULT_MAX_SESSIONS`

```python
DEFAULT_MAX_SESSIONS: Final = 10000
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Default maximum number of concurrent stateful Streamable HTTP sessions per session manager.


## DEFAULT_SESSION_IDLE_TIMEOUT

Import as `mcp.server.lowlevel.server.DEFAULT_SESSION_IDLE_TIMEOUT`  ·  defined at `mcp.server.streamable_http_manager.DEFAULT_SESSION_IDLE_TIMEOUT`

```python
DEFAULT_SESSION_IDLE_TIMEOUT: Final = 30 * 60
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Default idle period in seconds after which a stateful Streamable HTTP session is closed (30 minutes).


## logger

`mcp.server.streamable_http_manager.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## StreamableHTTPASGIApp

Import as `mcp.server.lowlevel.server.StreamableHTTPASGIApp`  ·  defined at `mcp.server.streamable_http_manager.StreamableHTTPASGIApp`

```python
class StreamableHTTPASGIApp
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `session_manager = session_manager`  _instance-attribute_

ASGI application for Streamable HTTP server transport.


## StreamableHTTPSessionManager

Import as `mcp.server.lowlevel.server.StreamableHTTPSessionManager`  ·  defined at `mcp.server.streamable_http_manager.StreamableHTTPSessionManager`

```python
class StreamableHTTPSessionManager
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (12)**

- `app = app`  _instance-attribute_
- `asgi_app = RequestBodyLimitMiddleware(self._handle_request, max_request_body_size)`  _instance-attribute_
- `event_store = event_store`  _instance-attribute_
- `async def handle_request(self, scope: Scope, receive: Receive, send: Send) -> None`  _async_
  Process ASGI request with proper session handling and transport setup.
- `json_response = json_response`  _instance-attribute_
- `max_request_body_size = max_request_body_size`  _instance-attribute_
- `max_sessions = max_sessions`  _instance-attribute_
- `retry_interval = retry_interval`  _instance-attribute_
- `async def run(self) -> AsyncIterator[None]`  _async_
  Run the session manager with proper lifecycle management.
- `security_settings = security_settings`  _instance-attribute_
- `session_idle_timeout = session_idle_timeout`  _instance-attribute_
- `stateless = stateless`  _instance-attribute_

Manages StreamableHTTP sessions with optional resumability via event store.

This class abstracts away the complexity of session management, event storage,
and request handling for StreamableHTTP transports. It handles:

1. Session tracking for clients
2. Resumability via an optional event store
3. Connection management and lifecycle
4. Request handling and transport setup
5. Idle session cleanup

Important: Only one StreamableHTTPSessionManager instance should be created
per application. The instance cannot be reused after its run() context has
completed. If you need to restart the manager, create a new instance.

Args:
    app: The MCP server instance
    event_store: Optional event store for resumability support. If provided, enables resumable connections
        where clients can reconnect and receive missed events. If None, sessions are still tracked but not
        resumable.
    json_response: Whether to use JSON responses instead of SSE streams
    stateless: If True, creates a completely fresh transport for each request with no session tracking or
        state persistence between requests.
    security_settings: Optional transport security settings.
    retry_interval: Retry interval in milliseconds to suggest to clients in SSE retry field. Used for SSE
        polling behavior.
    session_idle_timeout: Idle timeout in seconds for stateful sessions. A session that has had no HTTP
        request in flight for this long (no request being served, no open GET stream) is terminated and
        removed; its ID then answers 404 and the client has to initialize a new session. When retry_interval
        is also configured, ensure the idle timeout comfortably exceeds the retry interval to avoid reaping
        sessions during normal SSE polling gaps. Defaults to 1800 (30 minutes); None disables the timeout so
        sessions live until the client deletes them or the manager shuts down. Unused in stateless mode.
    max_request_body_size: Maximum size in bytes for Streamable HTTP request bodies. Requests that
        exceed this limit receive a 413 response before parsing or session creation. Defaults to 4 MiB.
    max_sessions: Maximum number of concurrent stateful sessions. While that many sessions are open, a
        request that would open another one receives a 503 response; existing sessions are unaffected and
        room frees up as they end or expire. Defaults to 10 000; None removes the limit. Unused in stateless
        mode.


## _error_response

`mcp.server.streamable_http_manager._error_response`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _error_response(message: str, status_code: int, code: int = INVALID_REQUEST) -> Response
```

A JSON-RPC error body (no request id) with the given HTTP status.


## _send_and_report_status

`mcp.server.streamable_http_manager._send_and_report_status`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _send_and_report_status(app: ASGIApp, scope: Scope, receive: Receive, send: Send) -> int | None
```

Run `app` for one request and return the HTTP status it answered with (None if it sent no response).


