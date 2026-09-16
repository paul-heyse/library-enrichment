# `mcp.server._streamable_http_modern`

Distribution: `mcp`

## _INVALID_BODY

`mcp.server._streamable_http_modern._INVALID_BODY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_INVALID_BODY: Final = JSONRPCError(jsonrpc='2.0', id=None, error=ErrorData(code=INVALID_REQUEST, message='Body must be a single JSON-RPC request or notification object'))
```

Well-formed JSON that is not one request or notification: a batch, a posted response, a malformed envelope.


## _MCP_PARAM_LIST_PAGE_CAP

`mcp.server._streamable_http_modern._MCP_PARAM_LIST_PAGE_CAP`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MCP_PARAM_LIST_PAGE_CAP: Final = 100
```

Page cap for the schema-resolving tools/list walk: a buggy paginator degrades to a logged skip, not a hang.


## _MCP_PARAM_PREFIX_LOWER

`mcp.server._streamable_http_modern._MCP_PARAM_PREFIX_LOWER`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MCP_PARAM_PREFIX_LOWER: Final = MCP_PARAM_HEADER_PREFIX.lower()
```

## _OK_STATUS

`mcp.server._streamable_http_modern._OK_STATUS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_OK_STATUS = 200
```

## _SSE_HEADERS

`mcp.server._streamable_http_modern._SSE_HEADERS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SSE_HEADERS: Final[list[tuple[bytes, bytes]]] = [(b'content-type', b'text/event-stream'), (b'cache-control', b'no-cache, no-transform'), (b'connection', b'keep-alive'), (b'x-accel-buffering', b'no')]
```

## _SSE_PING_INTERVAL

`mcp.server._streamable_http_modern._SSE_PING_INTERVAL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SSE_PING_INTERVAL: float = 15.0
```

Seconds between SSE comment-line keepalives once `text/event-stream` has committed.


## logger

`mcp.server._streamable_http_modern.logger`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
logger = logging.getLogger(__name__)
```

## _SingleExchangeDispatchContext

`mcp.server._streamable_http_modern._SingleExchangeDispatchContext`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _SingleExchangeDispatchContext
```

**Declared members (10)**

- `can_send_request: bool = field(default=False, init=False)`  _class-attribute, instance-attribute_
- `cancel_requested: anyio.Event = field(default_factory=anyio.Event)`  _class-attribute, instance-attribute_
- `message_metadata: MessageMetadata`  _instance-attribute_
- `async def notify(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> None`  _async_
- `async def progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
- `progress_token: ProgressToken | None = None`  _class-attribute, instance-attribute_
- `request_id: RequestId`  _instance-attribute_
- `async def send_raw_request(self, method: str, params: Mapping[str, Any] | None, opts: CallOptions | None = None) -> dict[str, Any]`  _async_
- `sink: MemoryObjectSendStream[bytes] | None = None`  _class-attribute, instance-attribute_
- `transport: TransportContext`  _instance-attribute_

`DispatchContext` for one inbound HTTP request.

Structurally satisfies `mcp.shared.dispatcher.DispatchContext`. The
back-channel is closed by construction: a 2026-07-28 server cannot send
requests to the client. The SSE sink, when present, carries request-scoped
notifications onto this request's response stream.


## _acknowledge_notification

`mcp.server._streamable_http_modern._acknowledge_notification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _acknowledge_notification(decoded: dict[str, Any], request: Request, scope: Scope, receive: Receive, send: Send) -> None
```

Answer an id-less POST body: `202` for a notification at a served version, a rejection otherwise.

Streamable-http §Sending Messages item 5 lets a server accept (202, no
body) or refuse (4xx) a notification POST; this entry accepts and drops.
The 2026-07-28 core protocol defines no client-to-server notifications over
HTTP (a client cancels by closing the response stream) and a per-request
entry holds no cross-request state for one to act on — honouring a posted
`notifications/cancelled` by client-chosen request id would let one
anonymous caller cancel another's work — but clients in the field still
POST them, and notifications are fire-and-forget, so they are acknowledged
as the handshake-era transport does rather than answered with an error
nobody reads. Header requirements for notification POSTs are undefined at
this revision; only the routing header that brought the POST here is
checked, so a version this entry does not serve is told so, as a request is.


## _is_notification_shaped

`mcp.server._streamable_http_modern._is_notification_shaped`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_notification_shaped(decoded: Any) -> bool
```

Whether a decoded POST body is a single JSON object without an `id` member.

JSON-RPC 2.0 §4.1: a notification is a request object without an "id"
member, so key presence — not which model happens to validate — picks the
arm. (The notification model ignores unknown keys; letting it catch a
request whose id is malformed would 202 a message that is owed an error.)


## _mcp_param_rejection

`mcp.server._streamable_http_modern._mcp_param_rejection`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _mcp_param_rejection(app: Server[Any], request: Request, req: JSONRPCRequest, verdict: InboundModernRoute, lifespan_state: Any) -> InboundLadderRejection | None
```

Validate a `tools/call` request's `Mcp-Param-*` headers against the called tool's schema.

Runs pre-dispatch, before any SSE machinery, so a rejection is always a
plain `application/json` 400 (the spec's MUST). With no `tools/list` handler
the catalog is undiscoverable and there is no recognized header to validate.


## _sse_event

`mcp.server._streamable_http_modern._sse_event`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _sse_event(msg: JSONRPCResponse | JSONRPCError | JSONRPCNotification) -> bytes
```

Serialise a JSON-RPC message as one SSE `event: message` frame.

SSE mode begins after the handler has emitted, so a `JSONRPCError` here
always carries the request's id; the `id: null` case lives in `_write`.


## _to_jsonrpc_response

`mcp.server._streamable_http_modern._to_jsonrpc_response`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _to_jsonrpc_response(request_id: RequestId, coro: Awaitable[dict[str, Any]]) -> JSONRPCResponse | JSONRPCError
```

Await ``coro`` and wrap its outcome as the JSON-RPC reply for ``request_id``.

The exception-to-wire boundary for the modern HTTP entry, composed around
`serve_one`: `modern_error_data` maps the shared ladder and surfaces
anything else as a generic `INTERNAL_ERROR` so handler internals never
reach the wire.


## _tool_input_schema

`mcp.server._streamable_http_modern._tool_input_schema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _tool_input_schema(app: Server[Any], request: Request, request_id: RequestId, verdict: InboundModernRoute, lifespan_state: Any, name: str) -> Any | None
```

Resolve `name`'s inputSchema from the server's own registered `tools/list` handler.

The listing runs through the normal `serve_one` path, so a visibility-scoped
catalog yields exactly what *this* caller was advertised. Returns None
(caller skips validation) when the listing fails or never advertises the tool.


## _write

`mcp.server._streamable_http_modern._write`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _write(msg: JSONRPCResponse | JSONRPCError, scope: Scope, receive: Receive, send: Send) -> None
```

Serialise a JSON-RPC reply with the table-mapped HTTP status.


## _write_rejection

`mcp.server._streamable_http_modern._write_rejection`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _write_rejection(rejection: InboundLadderRejection, request_id: RequestId | None, scope: Scope, receive: Receive, send: Send) -> None
```

Send a ladder rejection as its JSON-RPC error with the table-mapped HTTP status.


## handle_modern_request

Import as `mcp.server.streamable_http_manager.handle_modern_request`  ·  defined at `mcp.server._streamable_http_modern.handle_modern_request`

```python
async def handle_modern_request(app: Server[Any], security_settings: TransportSecuritySettings | None, json_response: bool, lifespan_state: Any, scope: Scope, receive: Receive, send: Send) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

ASGI handler for a single stateless-era POST.

Called from `StreamableHTTPSessionManager.handle_request` when the
`MCP-Protocol-Version` header names a modern revision; the manager enters
`app.lifespan` once at startup and passes the state in. Never sets
`Mcp-Session-Id`.


