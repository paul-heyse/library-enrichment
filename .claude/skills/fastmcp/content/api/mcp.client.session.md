# `mcp.client.session`

Distribution: `mcp`

## ClientResponse

`mcp.client.session.ClientResponse`

```python
ClientResponse: TypeAdapter[types.ClientResult | types.ErrorData] = TypeAdapter(types.ClientResult | types.ErrorData)
```

## DEFAULT_CLIENT_INFO

`mcp.client.session.DEFAULT_CLIENT_INFO`

```python
DEFAULT_CLIENT_INFO = types.Implementation(name='mcp', version='0.1.0')
```

**Inferred type** (`ty`, not declared in the source): `Implementation`

## DISCOVER_TIMEOUT_SECONDS

`mcp.client.session.DISCOVER_TIMEOUT_SECONDS`

```python
DISCOVER_TIMEOUT_SECONDS = 10.0
```

**Inferred type** (`ty`, not declared in the source): `float*`

## IncomingMessage

Import as `mcp.client.IncomingMessage`  ·  defined at `mcp.client.session.IncomingMessage`

```python
IncomingMessage: TypeAlias = types.ServerNotification | Exception
```

**Also exported as** `mcp.client.IncomingMessage`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

What `message_handler` receives: the server notifications the session surfaces, plus transport-level exceptions.

`notifications/cancelled` is applied by the dispatcher and never surfaced, and a
`notifications/subscriptions/acknowledged` for a live `listen()` stream is consumed by that
stream, so neither reaches the handler.


## ReceiveResultT

`mcp.client.session.ReceiveResultT`

```python
ReceiveResultT = TypeVar('ReceiveResultT', bound=BaseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _CallToolResultAdapter

`mcp.client.session._CallToolResultAdapter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CallToolResultAdapter: TypeAdapter[types.CallToolResult | types.InputRequiredResult | types.Result] = TypeAdapter(types.CallToolResult | types.InputRequiredResult)
```

## _GetPromptResultAdapter

`mcp.client.session._GetPromptResultAdapter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_GetPromptResultAdapter: TypeAdapter[types.GetPromptResult | types.InputRequiredResult] = TypeAdapter(types.GetPromptResult | types.InputRequiredResult)
```

## _NOTIFICATION_QUEUE_SIZE

`mcp.client.session._NOTIFICATION_QUEUE_SIZE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_NOTIFICATION_QUEUE_SIZE: Final = 256
```

## _ReadResourceResultAdapter

`mcp.client.session._ReadResourceResultAdapter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ReadResourceResultAdapter: TypeAdapter[types.ReadResourceResult | types.InputRequiredResult] = TypeAdapter(types.ReadResourceResult | types.InputRequiredResult)
```

## logger

`mcp.client.session.logger`

```python
logger = logging.getLogger('client')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ClientRequestContext

Import as `mcp.client.ClientRequestContext`  ·  defined at `mcp.client.session.ClientRequestContext`

```python
class ClientRequestContext
```

**Also exported as** `mcp.client.ClientRequestContext`, `mcp.client.context.ClientRequestContext`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `meta: RequestParamsMeta | None = None`  _class-attribute, instance-attribute_
- `request_id: RequestId`  _instance-attribute_
- `session: ClientSession`  _instance-attribute_

Context for a server-initiated request, passed to the sampling/elicitation/list-roots callbacks.


## ClientSession

Import as `mcp.ClientSession`  ·  defined at `mcp.client.session.ClientSession`

```python
class ClientSession
```

**Also exported as** `mcp.ClientSession`, `mcp.client.ClientSession`

_18 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (28)**

- `def adopt(self, result: types.InitializeResult | types.DiscoverResult) -> None`
  Install negotiated state from a result the caller already holds (no wire traffic).
- `async def call_tool(self, name: str, arguments: dict[str, Any] | None = None, read_timeout_seconds: float | None = None, progress_callback: ProgressFnT | None = None, input_responses: types.InputResponses | None = None, request_state: str | None = None, meta: RequestParamsMeta | None = None, allow_input_required: bool = False, allow_claimed: bool = False) -> types.CallToolResult | types.InputRequiredResult | types.Result`  _async_
  Send a tools/call request with optional progress callback support.
- `async def complete(self, ref: types.ResourceTemplateReference | types.PromptReference, argument: dict[str, str], context_arguments: dict[str, str] | None = None) -> types.CompleteResult`  _async_
  Send a completion/complete request.
- `async def discover(self) -> types.DiscoverResult`  _async_
  Probe `server/discover` and adopt the result.
- `discover_result: types.DiscoverResult | None`  _property_
  The server's DiscoverResult. None unless `discover()` ran (or was adopted).
- `async def dispatch_input_request(self, ctx: ClientRequestContext, request: types.InputRequest) -> types.InputResponse | types.ErrorData`  _async_
  Route an input request through the client's callback table.
- `async def get_prompt(self, name: str, arguments: dict[str, str] | None = None, input_responses: types.InputResponses | None = None, request_state: str | None = None, meta: RequestParamsMeta | None = None, allow_input_required: bool = False) -> types.GetPromptResult | types.InputRequiredResult`  _async_
  Send a prompts/get request.
- `async def initialize(self) -> types.InitializeResult`  _async_
- `initialize_result: types.InitializeResult | None`  _property_
  The server's InitializeResult. None unless `initialize()` ran (or was adopted).
- `instructions: str | None`  _property_
  Server-provided instructions text, if any.
- `async def list_prompts(self, params: types.PaginatedRequestParams | None = None) -> types.ListPromptsResult`  _async_
  Send a prompts/list request.
- `async def list_resource_templates(self, params: types.PaginatedRequestParams | None = None) -> types.ListResourceTemplatesResult`  _async_
  Send a resources/templates/list request.
- `async def list_resources(self, params: types.PaginatedRequestParams | None = None) -> types.ListResourcesResult`  _async_
  Send a resources/list request.
- `async def list_tools(self, params: types.PaginatedRequestParams | None = None) -> types.ListToolsResult`  _async_
  Send a tools/list request.
- `protocol_version: str | None`  _property_
  Negotiated protocol version. None until `initialize()`, `discover()`, or `adopt()`.
- `async def read_resource(self, uri: str, input_responses: types.InputResponses | None = None, request_state: str | None = None, meta: RequestParamsMeta | None = None, allow_input_required: bool = False) -> types.ReadResourceResult | types.InputRequiredResult`  _async_
  Send a resources/read request.
- `async def send_discover(self, version: str) -> dict[str, Any]`  _async_
  Send a single ``server/discover`` at ``version`` and return the raw result dict.
- `async def send_notification(self, notification: types.ClientNotification) -> None`  _async_
  Send a one-way notification. Usable before entering the context manager.
- `async def send_ping(self, meta: RequestParamsMeta | None = None) -> types.EmptyResult`  _async_
  Send a ping request.
- `async def send_progress_notification(self, progress_token: str | int, progress: float, total: float | None = None, message: str | None = None, meta: RequestParamsMeta | None = None) -> None`  _async_
  Send a progress notification.
- `async def send_request(self, request: types.ClientRequest | types.Request[Any, Any], result_type: type[ReceiveResultT] | TypeAdapter[ReceiveResultT], request_read_timeout_seconds: float | None = None, metadata: ClientMessageMetadata | None = None, progress_callback: ProgressFnT | None = None) -> ReceiveResultT`  _async_
  Send a request and wait for its typed result.
- `async def send_roots_list_changed(self) -> None`  _async_
  Send a roots/list_changed notification.
- `server_capabilities: types.ServerCapabilities | None`  _property_
  Server capabilities. None until `initialize()`, `discover()`, or `adopt()`.
- `server_info: types.Implementation | None`  _property_
  Server name/version. None until `initialize()`, `discover()`, or `adopt()`.
- `async def set_logging_level(self, level: types.LoggingLevel, meta: RequestParamsMeta | None = None) -> types.EmptyResult`  _async_
  Send a logging/setLevel request.
- `async def subscribe_resource(self, uri: str, meta: RequestParamsMeta | None = None) -> types.EmptyResult`  _async_
  Send a resources/subscribe request (2025-era servers only).
- `async def unsubscribe_resource(self, uri: str, meta: RequestParamsMeta | None = None) -> types.EmptyResult`  _async_
  Send a resources/unsubscribe request (2025-era servers only).
- `async def validate_tool_result(self, name: str, result: types.CallToolResult) -> None`  _async_
  Revalidate a `CallToolResult` against the tool's declared output schema.

Client half of an MCP connection, running on a `Dispatcher`.

Construct it over a transport's stream pair (or pass a pre-built
`dispatcher=`), enter as an async context manager, then call
`initialize()`. The dispatcher owns the receive loop and request
correlation; this class owns the typed MCP layer and the constructor
callbacks. Transport `Exception` items reach `message_handler` on any
stream-backed dispatcher (`JSONRPCDispatcher`), whether built here from a
stream pair or supplied without a stream-exception hook of its own; an
in-process `DirectDispatcher` carries none.

Extension `result_claims` fold into tools/call parsing at `adopt()`;
`notification_bindings` observe vendor notifications via bounded FIFOs.


## ElicitationFnT

Import as `mcp.client.client.ElicitationFnT`  ·  defined at `mcp.client.session.ElicitationFnT`

```python
class ElicitationFnT(Protocol)
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

## ListRootsFnT

Import as `mcp.client.client.ListRootsFnT`  ·  defined at `mcp.client.session.ListRootsFnT`

```python
class ListRootsFnT(Protocol)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

## LoggingFnT

Import as `mcp.client.client.LoggingFnT`  ·  defined at `mcp.client.session.LoggingFnT`

```python
class LoggingFnT(Protocol)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

## MessageHandlerFnT

Import as `mcp.client.client.MessageHandlerFnT`  ·  defined at `mcp.client.session.MessageHandlerFnT`

```python
class MessageHandlerFnT(Protocol)
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

## SamplingFnT

Import as `mcp.client.client.SamplingFnT`  ·  defined at `mcp.client.session.SamplingFnT`

```python
class SamplingFnT(Protocol)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

## _active_claims_at

`mcp.client.session._active_claims_at`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _active_claims_at(claims_by_extension: Mapping[str, tuple[ResultClaim[Any], ...]], version: str) -> dict[str, ResultClaim[Any]]
```

Claims active at `version`, keyed by wire tag; empty at any legacy version.


## _build_call_tool_adapter

`mcp.client.session._build_call_tool_adapter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_call_tool_adapter(active: Mapping[str, ResultClaim[Any]]) -> TypeAdapter[types.CallToolResult | types.InputRequiredResult | types.Result]
```

Build a discriminated tools/call adapter: a core arm plus one arm per active claim.


## _claim_active

`mcp.client.session._claim_active`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _claim_active(claim: ResultClaim[Any], version: str) -> bool
```

A claim is active at modern versions only, narrowed by its optional version subset.


## _clamp_inbound_ttl

`mcp.client.session._clamp_inbound_ttl`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _clamp_inbound_ttl(raw: dict[str, Any]) -> None
```

Floor a negative inbound `ttlMs` to 0 before `ge=0` validation fails the call (2026-07-28 caching SHOULD).


## _default_elicitation_callback

`mcp.client.session._default_elicitation_callback`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _default_elicitation_callback(context: ClientRequestContext, params: types.ElicitRequestParams) -> types.ElicitResult | types.ErrorData
```

## _default_list_roots_callback

`mcp.client.session._default_list_roots_callback`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _default_list_roots_callback(context: ClientRequestContext) -> types.ListRootsResult | types.ErrorData
```

## _default_logging_callback

`mcp.client.session._default_logging_callback`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _default_logging_callback(params: types.LoggingMessageNotificationParams) -> None
```

## _default_message_handler

`mcp.client.session._default_message_handler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _default_message_handler(message: IncomingMessage) -> None
```

## _default_sampling_callback

`mcp.client.session._default_sampling_callback`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _default_sampling_callback(context: ClientRequestContext, params: types.CreateMessageRequestParams) -> types.CreateMessageResult | types.CreateMessageResultWithTools | types.ErrorData
```

## _index_bindings

`mcp.client.session._index_bindings`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _index_bindings(notification_bindings: Sequence[NotificationBinding[Any]] | None) -> dict[str, NotificationBinding[Any]]
```

Index bindings by wire method, rejecting duplicates.


## _index_claims

`mcp.client.session._index_claims`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _index_claims(result_claims: Mapping[str, Sequence[ResultClaim[Any]]] | None, extensions: dict[str, dict[str, Any]] | None) -> dict[str, tuple[ResultClaim[Any], ...]]
```

Validate and copy the claims-by-extension mapping.


## _input_required_unexpected

`mcp.client.session._input_required_unexpected`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _input_required_unexpected(method: str) -> RuntimeError
```

## _later_revision_fields

`mcp.client.session._later_revision_fields`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _later_revision_fields(method: str, version: str) -> frozenset[str]
```

Result keys a revision newer than `version` declares for `method` but `version` doesn't.

The version-free result types carry every revision's fields, so such a key
(e.g. 2026-07-28 `ttlMs`/`cacheScope` on a pre-2026 session) is outside the
negotiated contract yet would still parse into the model and trip that later
revision's constraints. Empty at the newest known revision.


## _make_handshake_stamp

`mcp.client.session._make_handshake_stamp`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_handshake_stamp(protocol_version: str) -> Callable[[dict[str, Any], CallOptions], None]
```

## _make_modern_stamp

`mcp.client.session._make_modern_stamp`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_modern_stamp(protocol_version: str, client_info: dict[str, Any], capabilities: dict[str, Any], resolve_param_headers: Callable[[str, Mapping[str, Any]], dict[str, str]], log_level: types.LoggingLevel | None = None) -> Callable[[dict[str, Any], CallOptions], None]
```

## _parse_server_info_stamp

`mcp.client.session._parse_server_info_stamp`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_server_info_stamp(result: types.DiscoverResult) -> types.Implementation | None
```

The typed identity from a discover result's `_meta` serverInfo stamp.

The stamp is display-only per the spec, so absent and malformed both read
as `None` rather than failing the connection.


## _preconnect_stamp

`mcp.client.session._preconnect_stamp`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _preconnect_stamp(data: dict[str, Any], opts: CallOptions) -> None
```

## _same_schema

`mcp.client.session._same_schema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _same_schema(a: dict[str, Any] | None, b: dict[str, Any] | None) -> bool
```

JSON equality for two output schemas.

Python `==` is not JSON equality: it conflates `True`/`1` and `False`/`0`, which JSON
Schema keeps distinct (`const: true` vs `const: 1`). Canonical serialization compares as
JSON does; where it is stricter (`1` vs `1.0`), erring toward "changed" only costs a
recompile, never a stale validator.


## _wire_fields

`mcp.client.session._wire_fields`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _wire_fields(target: type[BaseModel] | UnionType) -> frozenset[str]
```

Top-level wire keys `target` declares (its members', for a union).


