# `mcp.server.session`

Distribution: `mcp`

## ResultT

`mcp.server.session.ResultT`

```python
ResultT = TypeVar('ResultT', bound=BaseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`mcp.server.session.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ServerSession']
```

## _logger

`mcp.server.session._logger`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_logger = logger
```

## logger

`mcp.server.session.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ServerSession

Import as `mcp.ServerSession`  ·  defined at `mcp.server.session.ServerSession`

```python
class ServerSession
```

**Also exported as** `mcp.ServerSession`

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (21)**

- `can_send_request: bool`  _property_
  Whether this request's channel can currently deliver a server-initiated request.
- `def check_client_capability(self, capability: types.ClientCapabilities) -> bool`
  Check if the client supports a specific capability.
- `client_capabilities: types.ClientCapabilities | None`  _property_
  The capabilities the client declared; `None` when none were declared.
- `client_params: types.InitializeRequestParams | None`  _property_
  The client's `initialize` request params; `None` when no client info was supplied.
- `async def create_message(self, messages: list[types.SamplingMessage], max_tokens: int, system_prompt: str | None = None, include_context: types.IncludeContext | None = None, temperature: float | None = None, stop_sequences: list[str] | None = None, metadata: dict[str, Any] | None = None, model_preferences: types.ModelPreferences | None = None, tools: list[types.Tool] | None = None, tool_choice: types.ToolChoice | None = None, related_request_id: types.RequestId | None = None) -> types.CreateMessageResult | types.CreateMessageResultWithTools`  _async_
  Send a sampling/create_message request.
- `async def elicit(self, message: str, requested_schema: types.ElicitRequestedSchema, related_request_id: types.RequestId | None = None) -> types.ElicitResult`  _async_
  Send a form mode elicitation/create request.
- `async def elicit_form(self, message: str, requested_schema: types.ElicitRequestedSchema, related_request_id: types.RequestId | None = None) -> types.ElicitResult`  _async_
  Send a form mode elicitation/create request.
- `async def elicit_url(self, message: str, url: str, elicitation_id: str, related_request_id: types.RequestId | None = None) -> types.ElicitResult`  _async_
  Send a URL mode elicitation/create request.
- `async def list_roots(self) -> types.ListRootsResult`  _async_
  Send a roots/list request.
- `protocol_version: str`  _property_
  The protocol version this connection speaks.
- `async def report_progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
  Report progress for the inbound request this session is scoped to.
- `async def send_elicit_complete(self, elicitation_id: str, related_request_id: types.RequestId | None = None) -> None`  _async_
  Send an elicitation completion notification.
- `async def send_log_message(self, level: types.LoggingLevel, data: Any, logger: str | None = None, related_request_id: types.RequestId | None = None) -> None`  _async_
  Send a log message notification.
- `async def send_notification(self, notification: types.ServerNotification, related_request_id: types.RequestId | None = None) -> None`  _async_
  Send a typed server-to-client notification.
- `async def send_ping(self) -> types.EmptyResult`  _async_
  Send a ping request.
- `async def send_progress_notification(self, progress_token: str | int, progress: float, total: float | None = None, message: str | None = None, related_request_id: str | None = None) -> None`  _async_
  Send a progress notification.
- `async def send_prompt_list_changed(self) -> None`  _async_
  Send a prompt list changed notification.
- `async def send_request(self, request: types.ServerRequest, result_type: type[ResultT], request_read_timeout_seconds: float | None = None, metadata: ServerMessageMetadata | None = None, progress_callback: ProgressFnT | None = None) -> ResultT`  _async_
  Send a typed server-to-client request and validate the result.
- `async def send_resource_list_changed(self) -> None`  _async_
  Send a resource list changed notification.
- `async def send_resource_updated(self, uri: str | AnyUrl) -> None`  _async_
  Send a resource updated notification.
- `async def send_tool_list_changed(self) -> None`  _async_
  Send a tool list changed notification.

Per-request proxy for server-to-client requests and notifications.

Built once per inbound request by the kernel's `_make_context`. Holds two
`Outbound` channels: the request-scoped one (the per-request
`DispatchContext`, which on streamable HTTP routes onto the originating
POST's response stream) and the connection's standalone channel
(`connection.outbound`). `related_request_id` on the public methods is the
selector — present means request-scoped, absent means standalone — and
never crosses the `Outbound` Protocol.


