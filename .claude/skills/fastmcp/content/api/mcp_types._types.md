# `mcp_types._types`

Distribution: `mcp-types`

## CLIENT_CAPABILITIES_META_KEY

Import as `mcp_types.CLIENT_CAPABILITIES_META_KEY`  ·  defined at `mcp_types._types.CLIENT_CAPABILITIES_META_KEY`

```python
CLIENT_CAPABILITIES_META_KEY = 'io.modelcontextprotocol/clientCapabilities'
```

**Also exported as** `mcp_types.CLIENT_CAPABILITIES_META_KEY`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Reserved request `_meta` key: per-request `ClientCapabilities` (2026-07-28). SDK-managed.


## CLIENT_INFO_META_KEY

Import as `mcp_types.CLIENT_INFO_META_KEY`  ·  defined at `mcp_types._types.CLIENT_INFO_META_KEY`

```python
CLIENT_INFO_META_KEY = 'io.modelcontextprotocol/clientInfo'
```

**Also exported as** `mcp_types.CLIENT_INFO_META_KEY`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Reserved request `_meta` key: the client `Implementation` (2026-07-28). SDK-managed.


## CORE_RESULT_TYPES

Import as `mcp_types.CORE_RESULT_TYPES`  ·  defined at `mcp_types._types.CORE_RESULT_TYPES`

```python
CORE_RESULT_TYPES: Final[frozenset[str]] = frozenset(get_args(_CoreResultType))
```

**Also exported as** `mcp_types.CORE_RESULT_TYPES`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The `resultType` tags owned by the core protocol vocabulary; extension claims may not re-key them.


## ClientNotification

Import as `mcp_types.ClientNotification`  ·  defined at `mcp_types._types.ClientNotification`

```python
ClientNotification = CancelledNotification | ProgressNotification | InitializedNotification | RootsListChangedNotification
```

**Also exported as** `mcp.ClientNotification`, `mcp_types.ClientNotification`

Notifications sent from the client to the server.

`TaskStatusNotification` is deliberately excluded (types-only).


## ClientRequest

Import as `mcp_types.ClientRequest`  ·  defined at `mcp_types._types.ClientRequest`

```python
ClientRequest = PingRequest | InitializeRequest | CompleteRequest | SetLevelRequest | GetPromptRequest | ListPromptsRequest | ListResourcesRequest | ListResourceTemplatesRequest | ReadResourceRequest | SubscribeRequest | UnsubscribeRequest | CallToolRequest | ListToolsRequest | DiscoverRequest | SubscriptionsListenRequest
```

**Also exported as** `mcp.ClientRequest`, `mcp_types.ClientRequest`

Union of client-to-server request payloads across all supported protocol versions.

The 2025-11-25 task requests are deliberately excluded (types-only).


## ClientResult

Import as `mcp_types.ClientResult`  ·  defined at `mcp_types._types.ClientResult`

```python
ClientResult = EmptyResult | CreateMessageResult | CreateMessageResultWithTools | ListRootsResult | ElicitResult
```

**Also exported as** `mcp.ClientResult`, `mcp_types.ClientResult`

## ContentBlock

Import as `mcp_types.ContentBlock`  ·  defined at `mcp_types._types.ContentBlock`

```python
ContentBlock = TextContent | ImageContent | AudioContent | ResourceLink | EmbeddedResource
```

**Also exported as** `mcp_types.ContentBlock`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A content block that can be used in prompts and tool results.


## DEFAULT_NEGOTIATED_VERSION

Import as `mcp_types.DEFAULT_NEGOTIATED_VERSION`  ·  defined at `mcp_types._types.DEFAULT_NEGOTIATED_VERSION`

```python
DEFAULT_NEGOTIATED_VERSION: Final[str] = '2025-03-26'
```

**Also exported as** `mcp_types.DEFAULT_NEGOTIATED_VERSION`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The default negotiated version of the Model Context Protocol when no version is specified.

We need this to satisfy the MCP specification, which requires the server to assume a specific version if none is
provided by the client.

See the "Protocol Version Header" at
https://modelcontextprotocol.io/specification/2025-11-25/basic/transports#protocol-version-header.


## ElicitRequestParams

Import as `mcp_types.ElicitRequestParams`  ·  defined at `mcp_types._types.ElicitRequestParams`

```python
ElicitRequestParams: TypeAlias = ElicitRequestURLParams | ElicitRequestFormParams
```

**Also exported as** `fastmcp.client.elicitation.ElicitRequestParams`, `mcp_types.ElicitRequestParams`

Parameters for elicitation requests - either form or URL mode.


## ElicitRequestedSchema

Import as `mcp_types.ElicitRequestedSchema`  ·  defined at `mcp_types._types.ElicitRequestedSchema`

```python
ElicitRequestedSchema: TypeAlias = dict[str, Any]
```

**Also exported as** `mcp_types.ElicitRequestedSchema`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## IconTheme

Import as `mcp_types.IconTheme`  ·  defined at `mcp_types._types.IconTheme`

```python
IconTheme = Literal['light', 'dark']
```

**Also exported as** `mcp_types.IconTheme`

Theme an icon is designed for. Wire values of `Icon.theme` (2025-11-25+).


## IncludeContext

Import as `mcp_types.IncludeContext`  ·  defined at `mcp_types._types.IncludeContext`

```python
IncludeContext = Literal['none', 'thisServer', 'allServers']
```

**Also exported as** `mcp.IncludeContext`, `mcp_types.IncludeContext`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Scope of MCP-server context a sampling request asks the client to attach.

"thisServer" and "allServers" are deprecated (SEP-2596).


## InputRequest

Import as `mcp_types.InputRequest`  ·  defined at `mcp_types._types.InputRequest`

```python
InputRequest: TypeAlias = CreateMessageRequest | ListRootsRequest | ElicitRequest
```

**Also exported as** `mcp_types.InputRequest`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A single server-initiated input request embedded in `InputRequiredResult` (2026-07-28).

Discriminated by `method`. On 2026-07-28 these embedded payloads take the place
of standalone server-to-client JSON-RPC requests.


## InputRequests

Import as `mcp_types.InputRequests`  ·  defined at `mcp_types._types.InputRequests`

```python
InputRequests: TypeAlias = dict[str, InputRequest]
```

**Also exported as** `mcp_types.InputRequests`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A map of server-initiated requests that the client must fulfill (2026-07-28).

Keys are server-assigned identifiers. Carried by `InputRequiredResult.input_requests`
and by the tasks extension.


## InputResponse

Import as `mcp_types.InputResponse`  ·  defined at `mcp_types._types.InputResponse`

```python
InputResponse: TypeAlias = CreateMessageResult | CreateMessageResultWithTools | ListRootsResult | ElicitResult
```

**Also exported as** `mcp_types.InputResponse`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A client response to a single server-initiated input request (2026-07-28).

`CreateMessageResultWithTools` is this SDK's array-content split of the schema's
single `CreateMessageResult` arm; the wire union has three arms.


## InputResponses

Import as `mcp_types.InputResponses`  ·  defined at `mcp_types._types.InputResponses`

```python
InputResponses: TypeAlias = dict[str, InputResponse]
```

**Also exported as** `mcp_types.InputResponses`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A map of client responses to server-initiated input requests (2026-07-28).

Keys match those of the `InputRequests` map the server sent. Also used by the
tasks extension's `tasks/update` params.


## LOG_LEVEL_META_KEY

Import as `mcp_types.LOG_LEVEL_META_KEY`  ·  defined at `mcp_types._types.LOG_LEVEL_META_KEY`

```python
LOG_LEVEL_META_KEY = 'io.modelcontextprotocol/logLevel'
```

**Also exported as** `mcp_types.LOG_LEVEL_META_KEY`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Reserved request `_meta` key: desired log level for this request (2026-07-28).

Deprecated (with the rest of logging) by SEP-2577 in the same revision that
introduces it. If absent, the server must not send log notifications.


## LoggingLevel

Import as `mcp_types.LoggingLevel`  ·  defined at `mcp_types._types.LoggingLevel`

```python
LoggingLevel = Literal['debug', 'info', 'notice', 'warning', 'error', 'critical', 'alert', 'emergency']
```

**Also exported as** `mcp.LoggingLevel`, `mcp_types.LoggingLevel`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The severity of a log message.

These map to syslog severities (RFC-5424 section 6.2.1). Logging is deprecated
in 2026-07-28 (SEP-2577); the level scale is unchanged across versions.


## Meta

`mcp_types._types.Meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
Meta: TypeAlias = dict[str, Any]
```

## MethodT

`mcp_types._types.MethodT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
MethodT = TypeVar('MethodT', bound=str)
```

## NotificationParamsT

`mcp_types._types.NotificationParamsT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
NotificationParamsT = TypeVar('NotificationParamsT', bound=NotificationParams | dict[str, Any] | None)
```

## PROTOCOL_VERSION_META_KEY

Import as `mcp_types.PROTOCOL_VERSION_META_KEY`  ·  defined at `mcp_types._types.PROTOCOL_VERSION_META_KEY`

```python
PROTOCOL_VERSION_META_KEY = 'io.modelcontextprotocol/protocolVersion'
```

**Also exported as** `mcp_types.PROTOCOL_VERSION_META_KEY`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Reserved request `_meta` key: the MCP protocol version for this request (2026-07-28).

SDK-managed; for HTTP its value must match the `MCP-Protocol-Version` header.


## ProgressToken

Import as `mcp_types.ProgressToken`  ·  defined at `mcp_types._types.ProgressToken`

```python
ProgressToken = str | int
```

**Also exported as** `mcp_types.ProgressToken`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A progress token, used to associate progress notifications with the original request.


## RequestParamsT

`mcp_types._types.RequestParamsT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
RequestParamsT = TypeVar('RequestParamsT', bound=RequestParams | dict[str, Any] | None)
```

## ResultType

Import as `mcp_types.ResultType`  ·  defined at `mcp_types._types.ResultType`

```python
ResultType = _CoreResultType | str
```

**Also exported as** `mcp_types.ResultType`

Tags a `Result` so the client knows how to parse it (2026-07-28).

"complete" means the result is final; "input_required" means it is an
`InputRequiredResult`. The union is open (the tasks extension reserves "task").
Absent `resultType` is equivalent to "complete".


## Role

Import as `mcp_types.Role`  ·  defined at `mcp_types._types.Role`

```python
Role = Literal['user', 'assistant']
```

**Also exported as** `mcp_types.Role`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The sender or recipient of messages and data in a conversation.


## SERVER_INFO_META_KEY

Import as `mcp_types.SERVER_INFO_META_KEY`  ·  defined at `mcp_types._types.SERVER_INFO_META_KEY`

```python
SERVER_INFO_META_KEY = 'io.modelcontextprotocol/serverInfo'
```

**Also exported as** `mcp_types.SERVER_INFO_META_KEY`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Reserved result `_meta` key: the server `Implementation` (2026-07-28). SDK-managed.

Servers SHOULD stamp it on every result. The value is self-reported and
unverified - display, logging, and debugging only; never behavior or security.


## SamplingContent

Import as `mcp_types.SamplingContent`  ·  defined at `mcp_types._types.SamplingContent`

```python
SamplingContent: TypeAlias = TextContent | ImageContent | AudioContent
```

**Also exported as** `mcp.SamplingContent`, `mcp_types.SamplingContent`

Basic content types for sampling responses (without tool use).

Used for backwards-compatible CreateMessageResult when tools are not used.


## SamplingMessageContentBlock

Import as `mcp_types.SamplingMessageContentBlock`  ·  defined at `mcp_types._types.SamplingMessageContentBlock`

```python
SamplingMessageContentBlock: TypeAlias = TextContent | ImageContent | AudioContent | ToolUseContent | ToolResultContent
```

**Also exported as** `mcp.SamplingMessageContentBlock`, `mcp_types.SamplingMessageContentBlock`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Content block types allowed in sampling messages.

This is the widest (2025-11-25+) membership; older sessions allow only a subset
on the wire. Serialization never narrows a value to fit; version gating is the
session layer's responsibility. Deprecated in 2026-07-28 (SEP-2577).


## ServerNotification

Import as `mcp_types.ServerNotification`  ·  defined at `mcp_types._types.ServerNotification`

```python
ServerNotification = CancelledNotification | ProgressNotification | LoggingMessageNotification | ResourceUpdatedNotification | ResourceListChangedNotification | ToolListChangedNotification | PromptListChangedNotification | ElicitCompleteNotification | SubscriptionsAcknowledgedNotification
```

**Also exported as** `mcp.ServerNotification`, `mcp_types.ServerNotification`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Union of server-to-client notification payloads across all supported protocol versions.

`TaskStatusNotification` is deliberately excluded (types-only).


## ServerRequest

Import as `mcp_types.ServerRequest`  ·  defined at `mcp_types._types.ServerRequest`

```python
ServerRequest = PingRequest | CreateMessageRequest | ListRootsRequest | ElicitRequest
```

**Also exported as** `mcp.ServerRequest`, `mcp_types.ServerRequest`

Union of standalone JSON-RPC requests a server can send to a client.

Live through 2025-11-25 only: 2026-07-28 has no server-to-client JSON-RPC
requests (these payloads are embedded in `InputRequiredResult` instead).


## ServerResult

Import as `mcp_types.ServerResult`  ·  defined at `mcp_types._types.ServerResult`

```python
ServerResult = EmptyResult | InitializeResult | DiscoverResult | CompleteResult | GetPromptResult | ListPromptsResult | ListResourcesResult | ListResourceTemplatesResult | ReadResourceResult | CallToolResult | ListToolsResult | SubscriptionsListenResult | InputRequiredResult
```

**Also exported as** `mcp.ServerResult`, `mcp_types.ServerResult`

Union of every result payload a server can return for a client request.

`InputRequiredResult` is deliberately last: both of its fields are optional,
so an earlier position would shadow other members during union resolution.


## StopReason

Import as `mcp_types.StopReason`  ·  defined at `mcp_types._types.StopReason`

```python
StopReason = Literal['endTurn', 'stopSequence', 'maxTokens', 'toolUse'] | str
```

**Also exported as** `mcp.StopReason`, `mcp_types.StopReason`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The reason why sampling stopped, if known.

An open union to allow provider-specific stop reasons. "toolUse" is 2025-11-25+.


## TaskStatus

Import as `mcp_types.TaskStatus`  ·  defined at `mcp_types._types.TaskStatus`

```python
TaskStatus = Literal['working', 'input_required', 'completed', 'failed', 'cancelled']
```

**Also exported as** `mcp_types.TaskStatus`

The status of a task (2025-11-25 only).


## _CoreResultType

`mcp_types._types._CoreResultType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CoreResultType = Literal['complete', 'input_required']
```

## client_notification_adapter

Import as `mcp_types.client_notification_adapter`  ·  defined at `mcp_types._types.client_notification_adapter`

```python
client_notification_adapter = TypeAdapter[ClientNotification](ClientNotification)
```

**Also exported as** `mcp_types.client_notification_adapter`

## client_request_adapter

Import as `mcp_types.client_request_adapter`  ·  defined at `mcp_types._types.client_request_adapter`

```python
client_request_adapter = TypeAdapter[ClientRequest](ClientRequest)
```

**Also exported as** `mcp_types.client_request_adapter`

## client_result_adapter

Import as `mcp_types.client_result_adapter`  ·  defined at `mcp_types._types.client_result_adapter`

```python
client_result_adapter = TypeAdapter[ClientResult](ClientResult)
```

**Also exported as** `mcp_types.client_result_adapter`

## server_notification_adapter

Import as `mcp_types.server_notification_adapter`  ·  defined at `mcp_types._types.server_notification_adapter`

```python
server_notification_adapter = TypeAdapter[ServerNotification](ServerNotification)
```

**Also exported as** `mcp_types.server_notification_adapter`

## server_request_adapter

Import as `mcp_types.server_request_adapter`  ·  defined at `mcp_types._types.server_request_adapter`

```python
server_request_adapter = TypeAdapter[ServerRequest](ServerRequest)
```

**Also exported as** `mcp_types.server_request_adapter`

## server_result_adapter

Import as `mcp_types.server_result_adapter`  ·  defined at `mcp_types._types.server_result_adapter`

```python
server_result_adapter = TypeAdapter[ServerResult](ServerResult)
```

**Also exported as** `mcp_types.server_result_adapter`

## Annotations

Import as `mcp_types.Annotations`  ·  defined at `mcp_types._types.Annotations`

```python
class Annotations(MCPModel)
```

**Also exported as** `mcp_types.Annotations`

_11 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (3)**

- `audience: list[Role] | None = None`  _class-attribute, instance-attribute_
  Who the intended audience is, e.g. `["user", "assistant"]`.
- `last_modified: str | None = None`  _class-attribute, instance-attribute_
  ISO 8601 timestamp of when the item was last modified.
- `priority: Annotated[float, Field(ge=0.0, le=1.0)] | None = None`  _class-attribute, instance-attribute_
  How important this data is for operating the server: 1 means effectively required, 0 means entirely optional.

Optional annotations the client can use to inform how objects are used or displayed.


## AudioContent

Import as `mcp_types.AudioContent`  ·  defined at `mcp_types._types.AudioContent`

```python
class AudioContent(MCPModel)
```

**Also exported as** `mcp_types.AudioContent`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (5)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `data: str`  _instance-attribute_
  The base64-encoded audio data.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.
- `mime_type: str`  _instance-attribute_
  The MIME type of the audio. Different providers may support different audio types.
- `type: Literal['audio'] = 'audio'`  _class-attribute, instance-attribute_

Audio provided to or from an LLM.


## BaseMetadata

Import as `mcp_types.BaseMetadata`  ·  defined at `mcp_types._types.BaseMetadata`

```python
class BaseMetadata(MCPModel)
```

**Also exported as** `mcp_types.BaseMetadata`

**Bases** `MCPModel`

**Declared members (2)**

- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.

Base class for entities with a programmatic name and an optional display title.


## BlobResourceContents

Import as `mcp_types.BlobResourceContents`  ·  defined at `mcp_types._types.BlobResourceContents`

```python
class BlobResourceContents(ResourceContents)
```

**Also exported as** `mcp_types.BlobResourceContents`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ResourceContents`

**Declared members (1)**

- `blob: str`  _instance-attribute_
  A base64-encoded string representing the binary data of the item.

**Inherited (3)**

- from `mcp_types._types.ResourceContents`: `meta`, `mime_type`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Binary contents of a resource.


## CacheableResult

Import as `mcp_types.CacheableResult`  ·  defined at `mcp_types._types.CacheableResult`

```python
class CacheableResult(Result)
```

**Also exported as** `mcp_types.CacheableResult`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (2)**

- `cache_scope: Literal['public', 'private'] = 'private'`  _class-attribute, instance-attribute_
  Analogous to HTTP `Cache-Control: public` vs `private`: "public" allows shared caches to serve the response to any user; "private" forbids that.
- `ttl_ms: Annotated[int, Field(ge=0)] = 0`  _class-attribute, instance-attribute_
  How long (ms) the client MAY cache this response, analogous to HTTP `Cache-Control: max-age`. 0 means immediately stale.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Base class for results that carry client-side caching directives (2026-07-28).

Both fields are required on the 2026-07-28 wire. The SDK defaults to
`ttl_ms=0` (immediately stale) and `cache_scope="private"` so a handler
that doesn't set them still produces a valid 2026-07-28 result without
accidentally enabling shared caching.


## CallToolRequest

Import as `mcp_types.CallToolRequest`  ·  defined at `mcp_types._types.CallToolRequest`

```python
class CallToolRequest(Request[CallToolRequestParams, Literal['tools/call']])
```

**Also exported as** `mcp.CallToolRequest`, `mcp_types.CallToolRequest`

**Bases** `Request[CallToolRequestParams, Literal['tools/call']]`

**Declared members (2)**

- `method: Literal['tools/call'] = 'tools/call'`  _class-attribute, instance-attribute_
- `params: CallToolRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Used by the client to invoke a tool provided by the server.


## CallToolRequestParams

Import as `mcp_types.CallToolRequestParams`  ·  defined at `mcp_types._types.CallToolRequestParams`

```python
class CallToolRequestParams(InputResponseRequestParams)
```

**Also exported as** `mcp_types.CallToolRequestParams`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `InputResponseRequestParams`

**Declared members (3)**

- `arguments: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `name: str`  _instance-attribute_
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller requests task-augmented execution (2025-11-25 only).

**Inherited (3)**

- from `mcp_types._types.InputResponseRequestParams`: `input_responses`, `request_state`
- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## CallToolResult

Import as `mcp_types.CallToolResult`  ·  defined at `mcp_types._types.CallToolResult`

```python
class CallToolResult(Result)
```

**Also exported as** `mcp_types.CallToolResult`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (4)**

- `content: list[ContentBlock]`  _instance-attribute_
  A list of content objects that represent the unstructured result of the tool call.
- `is_error: bool = False`  _class-attribute, instance-attribute_
  Whether the tool call ended in an error.
- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.
- `structured_content: Any = None`  _class-attribute, instance-attribute_
  An optional JSON value representing the structured result of the tool call.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server's response to a tool call.

Errors that originate from the tool SHOULD be reported inside the result
with `is_error` set to true, not as an MCP protocol-level error, so the LLM
can see and self-correct. Errors in finding the tool, or any other
exceptional condition, should be reported as an MCP error response.


## CancelTaskRequest

Import as `mcp_types.CancelTaskRequest`  ·  defined at `mcp_types._types.CancelTaskRequest`

```python
class CancelTaskRequest(Request[CancelTaskRequestParams, Literal['tasks/cancel']])
```

**Also exported as** `mcp_types.CancelTaskRequest`

**Bases** `Request[CancelTaskRequestParams, Literal['tasks/cancel']]`

**Declared members (2)**

- `method: Literal['tasks/cancel'] = 'tasks/cancel'`  _class-attribute, instance-attribute_
- `params: CancelTaskRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A request to cancel a task (2025-11-25 only).


## CancelTaskRequestParams

Import as `mcp_types.CancelTaskRequestParams`  ·  defined at `mcp_types._types.CancelTaskRequestParams`

```python
class CancelTaskRequestParams(RequestParams)
```

**Also exported as** `mcp_types.CancelTaskRequestParams`

**Bases** `RequestParams`

**Declared members (1)**

- `task_id: str`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## CancelTaskResult

Import as `mcp_types.CancelTaskResult`  ·  defined at `mcp_types._types.CancelTaskResult`

```python
class CancelTaskResult(Result, Task)
```

**Also exported as** `mcp_types.CancelTaskResult`

**Bases** `Result`, `Task`

**Inherited (8)**

- from `mcp_types._types.Result`: `meta`
- from `mcp_types._types.Task`: `created_at`, `last_updated_at`, `poll_interval`, `status`, `status_message`, `task_id`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The response to a tasks/cancel request (2025-11-25 only).


## CancelledNotification

Import as `mcp_types.CancelledNotification`  ·  defined at `mcp_types._types.CancelledNotification`

```python
class CancelledNotification(Notification[CancelledNotificationParams, Literal['notifications/cancelled']])
```

**Also exported as** `mcp_types.CancelledNotification`

**Bases** `Notification[CancelledNotificationParams, Literal['notifications/cancelled']]`

**Declared members (2)**

- `method: Literal['notifications/cancelled'] = 'notifications/cancelled'`  _class-attribute, instance-attribute_
- `params: CancelledNotificationParams`  _instance-attribute_

This notification can be sent by either side to indicate that it is canceling a
previously-issued request.

The request SHOULD still be in-flight, but due to communication latency, it
is always possible that this notification MAY arrive after the request has
already finished. A client MUST NOT attempt to cancel its `initialize` request.


## CancelledNotificationParams

Import as `mcp_types.CancelledNotificationParams`  ·  defined at `mcp_types._types.CancelledNotificationParams`

```python
class CancelledNotificationParams(NotificationParams)
```

**Also exported as** `mcp_types.CancelledNotificationParams`

**Bases** `NotificationParams`

**Declared members (2)**

- `reason: str | None = None`  _class-attribute, instance-attribute_
  An optional string describing the reason for the cancellation.
- `request_id: RequestId | None = None`  _class-attribute, instance-attribute_
  The ID of the request to cancel.

**Inherited (1)**

- from `mcp_types._types.NotificationParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## ClientCapabilities

Import as `mcp_types.ClientCapabilities`  ·  defined at `mcp_types._types.ClientCapabilities`

```python
class ClientCapabilities(MCPModel)
```

**Also exported as** `mcp.ClientCapabilities`, `mcp_types.ClientCapabilities`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (6)**

- `elicitation: ElicitationCapability | None = None`  _class-attribute, instance-attribute_
  Present if the client supports elicitation from the user.
- `experimental: dict[str, dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
  Experimental, non-standard capabilities that the client supports.
- `extensions: dict[str, dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
  MCP extensions the client supports (2026-07-28). Keys are extension identifiers; values are per-extension settings (empty object = no settings).
- `roots: RootsCapability | None = None`  _class-attribute, instance-attribute_
  Present if the client supports listing roots.
- `sampling: SamplingCapability | None = None`  _class-attribute, instance-attribute_
  Present if the client supports sampling from an LLM. Can contain fine-grained capabilities like context and tools support.
- `tasks: ClientTasksCapability | None = None`  _class-attribute, instance-attribute_
  Present if the client supports task-augmented requests (2025-11-25 only).

Capabilities a client may support.

Not a closed set: any client can define additional capabilities. Sent once in
`initialize` through 2025-11-25; per-request in `_meta` on 2026-07-28.


## ClientTasksCapability

Import as `mcp_types.ClientTasksCapability`  ·  defined at `mcp_types._types.ClientTasksCapability`

```python
class ClientTasksCapability(MCPModel)
```

**Also exported as** `mcp_types.ClientTasksCapability`

**Bases** `MCPModel`

**Declared members (3)**

- `cancel: TasksCancelCapability | None = None`  _class-attribute, instance-attribute_
- `list: TasksListCapability | None = None`  _class-attribute, instance-attribute_
- `requests: ClientTasksRequestsCapability | None = None`  _class-attribute, instance-attribute_

Capability for client tasks operations (2025-11-25 only).


## ClientTasksRequestsCapability

Import as `mcp_types.ClientTasksRequestsCapability`  ·  defined at `mcp_types._types.ClientTasksRequestsCapability`

```python
class ClientTasksRequestsCapability(MCPModel)
```

**Also exported as** `mcp_types.ClientTasksRequestsCapability`

**Bases** `MCPModel`

**Declared members (2)**

- `elicitation: TasksElicitationCapability | None = None`  _class-attribute, instance-attribute_
- `sampling: TasksSamplingCapability | None = None`  _class-attribute, instance-attribute_

Specifies which request types the client can augment with tasks (2025-11-25 only).


## CompleteRequest

Import as `mcp_types.CompleteRequest`  ·  defined at `mcp_types._types.CompleteRequest`

```python
class CompleteRequest(Request[CompleteRequestParams, Literal['completion/complete']])
```

**Also exported as** `mcp.CompleteRequest`, `mcp_types.CompleteRequest`

**Bases** `Request[CompleteRequestParams, Literal['completion/complete']]`

**Declared members (2)**

- `method: Literal['completion/complete'] = 'completion/complete'`  _class-attribute, instance-attribute_
- `params: CompleteRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A request from the client to the server, to ask for completion options.


## CompleteRequestParams

Import as `mcp_types.CompleteRequestParams`  ·  defined at `mcp_types._types.CompleteRequestParams`

```python
class CompleteRequestParams(RequestParams)
```

**Also exported as** `mcp_types.CompleteRequestParams`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (3)**

- `argument: CompletionArgument`  _instance-attribute_
- `context: CompletionContext | None = None`  _class-attribute, instance-attribute_
  Additional, optional context for completions.
- `ref: ResourceTemplateReference | PromptReference`  _instance-attribute_
  The prompt or resource-template reference to complete against.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## CompleteResult

Import as `mcp_types.CompleteResult`  ·  defined at `mcp_types._types.CompleteResult`

```python
class CompleteResult(Result)
```

**Also exported as** `mcp_types.CompleteResult`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (2)**

- `completion: Completion`  _instance-attribute_
  The completion values, with optional total / has-more pagination hints.
- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server's response to a completion/complete request.


## Completion

Import as `mcp_types.Completion`  ·  defined at `mcp_types._types.Completion`

```python
class Completion(MCPModel)
```

**Also exported as** `mcp_types.Completion`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (3)**

- `has_more: bool | None = None`  _class-attribute, instance-attribute_
  Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown.
- `total: int | None = None`  _class-attribute, instance-attribute_
  The total number of completion options available. This can exceed the number of values actually sent in the response.
- `values: list[str]`  _instance-attribute_
  An array of completion values. Must not exceed 100 items.

Completion information.


## CompletionArgument

Import as `mcp_types.CompletionArgument`  ·  defined at `mcp_types._types.CompletionArgument`

```python
class CompletionArgument(MCPModel)
```

**Also exported as** `mcp_types.CompletionArgument`

**Bases** `MCPModel`

**Declared members (2)**

- `name: str`  _instance-attribute_
  The name of the argument.
- `value: str`  _instance-attribute_
  The value of the argument to use for completion matching.

The argument's information for completion requests.


## CompletionContext

Import as `mcp_types.CompletionContext`  ·  defined at `mcp_types._types.CompletionContext`

```python
class CompletionContext(MCPModel)
```

**Also exported as** `mcp_types.CompletionContext`

**Bases** `MCPModel`

**Declared members (1)**

- `arguments: dict[str, str] | None = None`  _class-attribute, instance-attribute_
  Previously-resolved variables in a URI template or prompt.

Additional, optional context for completions.


## CompletionsCapability

Import as `mcp_types.CompletionsCapability`  ·  defined at `mcp_types._types.CompletionsCapability`

```python
class CompletionsCapability(MCPModel)
```

**Also exported as** `mcp_types.CompletionsCapability`

**Bases** `MCPModel`

Capability for completions operations.


## CreateMessageRequest

Import as `mcp_types.CreateMessageRequest`  ·  defined at `mcp_types._types.CreateMessageRequest`

```python
class CreateMessageRequest(Request[CreateMessageRequestParams, Literal['sampling/createMessage']])
```

**Also exported as** `mcp.CreateMessageRequest`, `mcp_types.CreateMessageRequest`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Request[CreateMessageRequestParams, Literal['sampling/createMessage']]`

**Declared members (2)**

- `method: Literal['sampling/createMessage'] = 'sampling/createMessage'`  _class-attribute, instance-attribute_
- `params: CreateMessageRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A request from the server to sample an LLM via the client.

The client has full discretion over which model to select and should inform
the user before sampling (human in the loop). A standalone JSON-RPC request
through 2025-11-25; on 2026-07-28 it is embedded in
`InputRequiredResult.input_requests` instead. Deprecated in 2026-07-28 (SEP-2577).


## CreateMessageRequestParams

Import as `mcp_types.CreateMessageRequestParams`  ·  defined at `mcp_types._types.CreateMessageRequestParams`

```python
class CreateMessageRequestParams(RequestParams)
```

**Also exported as** `mcp_types.CreateMessageRequestParams`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (11)**

- `include_context: IncludeContext | None = None`  _class-attribute, instance-attribute_
  A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request. Default is "none". "thisServer" and "allServers" are deprecated (SEP-2596).
- `max_tokens: int`  _instance-attribute_
  The maximum number of tokens to sample, as requested by the server.
- `messages: list[SamplingMessage]`  _instance-attribute_
  The conversation to sample from.
- `metadata: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Optional metadata to pass through to the LLM provider. Provider-specific.
- `model_preferences: ModelPreferences | None = None`  _class-attribute, instance-attribute_
  The server's preferences for which model to select. The client MAY ignore these preferences.
- `stop_sequences: list[str] | None = None`  _class-attribute, instance-attribute_
- `system_prompt: str | None = None`  _class-attribute, instance-attribute_
  An optional system prompt the server wants to use for sampling.
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller requests task-augmented execution (2025-11-25 only).
- `temperature: float | None = None`  _class-attribute, instance-attribute_
- `tool_choice: ToolChoice | None = None`  _class-attribute, instance-attribute_
  Controls how the model uses tools (2025-11-25+). Requires the `sampling.tools` client capability.
- `tools: list[Tool] | None = None`  _class-attribute, instance-attribute_
  Tools the model may use during generation (2025-11-25+). Requires the `sampling.tools` client capability.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## CreateMessageResult

Import as `mcp_types.CreateMessageResult`  ·  defined at `mcp_types._types.CreateMessageResult`

```python
class CreateMessageResult(Result)
```

**Also exported as** `mcp.CreateMessageResult`, `mcp_types.CreateMessageResult`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (4)**

- `content: SamplingContent`  _instance-attribute_
  Response content. Single content block (text, image, or audio).
- `model: str`  _instance-attribute_
  The name of the model that generated the message.
- `role: Role`  _instance-attribute_
  The role of the message sender (typically 'assistant' for LLM responses).
- `stop_reason: StopReason | None = None`  _class-attribute, instance-attribute_
  The reason why sampling stopped, if known.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The client's response to a sampling/createMessage request from the server.

This is the backwards-compatible version that returns single content (no arrays).
Used when the request does not include tools.

On 2026-07-28 this travels embedded in an `InputResponses` map rather than
as a top-level JSON-RPC result. Deprecated in 2026-07-28 (SEP-2577).


## CreateMessageResultWithTools

Import as `mcp_types.CreateMessageResultWithTools`  ·  defined at `mcp_types._types.CreateMessageResultWithTools`

```python
class CreateMessageResultWithTools(Result)
```

**Also exported as** `mcp.CreateMessageResultWithTools`, `mcp_types.CreateMessageResultWithTools`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (5)**

- `content: SamplingMessageContentBlock | list[SamplingMessageContentBlock]`  _instance-attribute_
  Response content. May be a single content block or an array. May include ToolUseContent if stop_reason is 'toolUse'.
- `content_as_list: list[SamplingMessageContentBlock]`  _property_
  Returns the content as a list of content blocks, regardless of whether it was originally a single block or a list.
- `model: str`  _instance-attribute_
  The name of the model that generated the message.
- `role: Role`  _instance-attribute_
  The role of the message sender (typically 'assistant' for LLM responses).
- `stop_reason: StopReason | None = None`  _class-attribute, instance-attribute_
  The reason why sampling stopped, if known. 'toolUse' indicates the model wants to use a tool.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The client's response to a sampling/createMessage request when tools were provided.

This version supports array content for tool use flows (2025-11-25 and later).


## CreateTaskResult

Import as `mcp_types.CreateTaskResult`  ·  defined at `mcp_types._types.CreateTaskResult`

```python
class CreateTaskResult(Result)
```

**Also exported as** `mcp_types.CreateTaskResult`

**Bases** `Result`

**Declared members (1)**

- `task: Task`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A response to a task-augmented request (2025-11-25 only).


## DiscoverRequest

Import as `mcp_types.DiscoverRequest`  ·  defined at `mcp_types._types.DiscoverRequest`

```python
class DiscoverRequest(Request[RequestParams | None, Literal['server/discover']])
```

**Also exported as** `mcp_types.DiscoverRequest`

**Bases** `Request[RequestParams | None, Literal['server/discover']]`

**Declared members (2)**

- `method: Literal['server/discover'] = 'server/discover'`  _class-attribute, instance-attribute_
- `params: RequestParams | None = None`  _class-attribute, instance-attribute_
  Required on the 2026-07-28 wire (for `_meta`); the session layer materializes it.

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Asks the server to advertise its supported protocol versions, capabilities,
and other metadata (2026-07-28).

Servers speaking 2026-07-28 MUST implement this; clients MAY call it but are
not required to (version negotiation can also happen via per-request `_meta`).


## DiscoverResult

Import as `mcp_types.DiscoverResult`  ·  defined at `mcp_types._types.DiscoverResult`

```python
class DiscoverResult(CacheableResult)
```

**Also exported as** `mcp_types.DiscoverResult`

**Bases** `CacheableResult`

**Declared members (4)**

- `capabilities: ServerCapabilities`  _instance-attribute_
- `instructions: str | None = None`  _class-attribute, instance-attribute_
  Natural-language guidance describing the server and its features, e.g. for a system prompt. Should not duplicate information already in tool descriptions.
- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; required on the 2026-07-28 wire, ignored by older peers, and defaulted on inbound bodies that omit it.
- `supported_versions: list[str]`  _instance-attribute_
  MCP protocol versions this server supports; the client should pick one for subsequent requests.

**Inherited (3)**

- from `mcp_types._types.CacheableResult`: `cache_scope`, `ttl_ms`
- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The result returned by the server for a `server/discover` request (2026-07-28).


## ElicitCompleteNotification

Import as `mcp_types.ElicitCompleteNotification`  ·  defined at `mcp_types._types.ElicitCompleteNotification`

```python
class ElicitCompleteNotification(Notification[ElicitCompleteNotificationParams, Literal['notifications/elicitation/complete']])
```

**Also exported as** `mcp_types.ElicitCompleteNotification`

**Bases** `Notification[ElicitCompleteNotificationParams, Literal['notifications/elicitation/complete']]`

**Declared members (2)**

- `method: Literal['notifications/elicitation/complete'] = 'notifications/elicitation/complete'`  _class-attribute, instance-attribute_
- `params: ElicitCompleteNotificationParams`  _instance-attribute_

A notification from the server to the client, informing it that a URL mode
elicitation has been completed.

Clients MAY use the notification to automatically retry requests that received a
URLElicitationRequiredError, update the user interface, or otherwise continue
an interaction. However, because delivery of the notification is not guaranteed,
clients must not wait indefinitely for a notification from the server.

New in protocol 2025-11-25 with URL mode itself.


## ElicitCompleteNotificationParams

Import as `mcp_types.ElicitCompleteNotificationParams`  ·  defined at `mcp_types._types.ElicitCompleteNotificationParams`

```python
class ElicitCompleteNotificationParams(NotificationParams)
```

**Also exported as** `mcp_types.ElicitCompleteNotificationParams`

**Bases** `NotificationParams`

**Declared members (1)**

- `elicitation_id: str`  _instance-attribute_
  The unique identifier of the elicitation that was completed.

**Inherited (1)**

- from `mcp_types._types.NotificationParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for elicitation completion notifications.


## ElicitRequest

Import as `mcp_types.ElicitRequest`  ·  defined at `mcp_types._types.ElicitRequest`

```python
class ElicitRequest(Request[ElicitRequestParams, Literal['elicitation/create']])
```

**Also exported as** `mcp_types.ElicitRequest`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Request[ElicitRequestParams, Literal['elicitation/create']]`

**Declared members (2)**

- `method: Literal['elicitation/create'] = 'elicitation/create'`  _class-attribute, instance-attribute_
- `params: ElicitRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A request from the server to elicit additional information from the user via the client.


## ElicitRequestFormParams

Import as `mcp_types.ElicitRequestFormParams`  ·  defined at `mcp_types._types.ElicitRequestFormParams`

```python
class ElicitRequestFormParams(RequestParams)
```

**Also exported as** `mcp_types.ElicitRequestFormParams`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (4)**

- `message: str`  _instance-attribute_
  The message to present to the user describing what information is being requested.
- `mode: Literal['form'] = 'form'`  _class-attribute, instance-attribute_
  The elicitation mode (always "form" for this type).
- `requested_schema: ElicitRequestedSchema`  _instance-attribute_
  A restricted subset of JSON Schema defining the structure of the expected response. Only top-level properties are allowed, without nesting.
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller requests task-augmented execution (2025-11-25 only).

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for form mode elicitation requests.

Form mode collects non-sensitive information from the user via an in-band form
rendered by the client.


## ElicitRequestURLParams

Import as `mcp_types.ElicitRequestURLParams`  ·  defined at `mcp_types._types.ElicitRequestURLParams`

```python
class ElicitRequestURLParams(RequestParams)
```

**Also exported as** `mcp_types.ElicitRequestURLParams`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (5)**

- `elicitation_id: str | None = None`  _class-attribute, instance-attribute_
  The ID of the elicitation, which must be unique within the context of the server.
- `message: str`  _instance-attribute_
  The message to present to the user explaining why the interaction is needed.
- `mode: Literal['url'] = 'url'`  _class-attribute, instance-attribute_
  The elicitation mode (always "url" for this type).
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller requests task-augmented execution (2025-11-25 only).
- `url: str`  _instance-attribute_
  The URL that the user should navigate to.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for URL mode elicitation requests.

URL mode directs users to external URLs for sensitive out-of-band interactions
like OAuth flows, credential collection, or payment processing. New in 2025-11-25.


## ElicitResult

Import as `mcp_types.ElicitResult`  ·  defined at `mcp_types._types.ElicitResult`

```python
class ElicitResult(Result)
```

**Also exported as** `fastmcp.client.elicitation.MCPElicitResult`, `mcp_types.ElicitResult`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (2)**

- `action: Literal['accept', 'decline', 'cancel']`  _instance-attribute_
  The user action in response to the elicitation. - "accept": User submitted the form/confirmed the action (or consented to URL navigation) - "decline": User explicitly declined the action - "cancel": User dismissed without making an explici…
- `content: dict[str, str | int | float | bool | list[str] | None] | None = None`  _class-attribute, instance-attribute_
  The submitted form data, only present when action is "accept" in form mode. Contains values matching the requested schema. Values can be strings, integers, floats, booleans, arrays of strings, or null. For URL mode, this field is omitted.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The client's response to an elicitation request.


## ElicitationCapability

Import as `mcp_types.ElicitationCapability`  ·  defined at `mcp_types._types.ElicitationCapability`

```python
class ElicitationCapability(MCPModel)
```

**Also exported as** `mcp_types.ElicitationCapability`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (2)**

- `form: FormElicitationCapability | None = None`  _class-attribute, instance-attribute_
  Present if the client supports form mode elicitation.
- `url: UrlElicitationCapability | None = None`  _class-attribute, instance-attribute_
  Present if the client supports URL mode elicitation (2025-11-25 and later).

Capability for elicitation operations.

Clients must support at least one mode (form or url).


## ElicitationRequiredErrorData

Import as `mcp_types.ElicitationRequiredErrorData`  ·  defined at `mcp_types._types.ElicitationRequiredErrorData`

```python
class ElicitationRequiredErrorData(MCPModel)
```

**Also exported as** `mcp_types.ElicitationRequiredErrorData`

**Bases** `MCPModel`

**Declared members (1)**

- `elicitations: list[ElicitRequestURLParams]`  _instance-attribute_
  List of URL mode elicitations that must be completed.

Error data for the -32042 URL-elicitation-required error.

Servers return this when a request cannot be processed until one or more
URL mode elicitations are completed.

Removed in protocol 2026-07-28; sent/received on sessions negotiating 2025-11-25.


## EmbeddedResource

Import as `mcp_types.EmbeddedResource`  ·  defined at `mcp_types._types.EmbeddedResource`

```python
class EmbeddedResource(MCPModel)
```

**Also exported as** `mcp_types.EmbeddedResource`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (4)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.
- `resource: TextResourceContents | BlobResourceContents`  _instance-attribute_
- `type: Literal['resource'] = 'resource'`  _class-attribute, instance-attribute_

The contents of a resource, embedded into a prompt or tool call result.

It is up to the client how best to render embedded resources for the benefit
of the LLM and/or the user.


## EmptyResult

Import as `mcp_types.EmptyResult`  ·  defined at `mcp_types._types.EmptyResult`

```python
class EmptyResult(Result)
```

**Also exported as** `mcp_types.EmptyResult`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (1)**

- `result_type: ResultType | None = None`  _class-attribute, instance-attribute_
  None keeps the dump empty; see the class docstring.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A result that indicates success but carries no data.

`result_type` defaults to None so this dumps as `{}`: deployed TypeScript
and Rust SDK peers (clients and servers) validate empty results strictly
and reject extra keys. The 2026-07-28 schema requires `resultType`, so code
answering an empty result on a 2026-07-28+ session must pass
`result_type="complete"`.


## FormElicitationCapability

Import as `mcp_types.FormElicitationCapability`  ·  defined at `mcp_types._types.FormElicitationCapability`

```python
class FormElicitationCapability(MCPModel)
```

**Also exported as** `mcp_types.FormElicitationCapability`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

Capability for form mode elicitation.


## GetPromptRequest

Import as `mcp_types.GetPromptRequest`  ·  defined at `mcp_types._types.GetPromptRequest`

```python
class GetPromptRequest(Request[GetPromptRequestParams, Literal['prompts/get']])
```

**Also exported as** `mcp.GetPromptRequest`, `mcp_types.GetPromptRequest`

**Bases** `Request[GetPromptRequestParams, Literal['prompts/get']]`

**Declared members (2)**

- `method: Literal['prompts/get'] = 'prompts/get'`  _class-attribute, instance-attribute_
- `params: GetPromptRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Used by the client to get a prompt provided by the server.


## GetPromptRequestParams

Import as `mcp_types.GetPromptRequestParams`  ·  defined at `mcp_types._types.GetPromptRequestParams`

```python
class GetPromptRequestParams(InputResponseRequestParams)
```

**Also exported as** `mcp_types.GetPromptRequestParams`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `InputResponseRequestParams`

**Declared members (2)**

- `arguments: dict[str, str] | None = None`  _class-attribute, instance-attribute_
  Arguments to use for templating the prompt.
- `name: str`  _instance-attribute_
  The name of the prompt or prompt template.

**Inherited (3)**

- from `mcp_types._types.InputResponseRequestParams`: `input_responses`, `request_state`
- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## GetPromptResult

Import as `mcp_types.GetPromptResult`  ·  defined at `mcp_types._types.GetPromptResult`

```python
class GetPromptResult(Result)
```

**Also exported as** `mcp.GetPromptResult`, `mcp_types.GetPromptResult`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (3)**

- `description: str | None = None`  _class-attribute, instance-attribute_
  An optional description for the prompt.
- `messages: list[PromptMessage]`  _instance-attribute_
  The messages composing the prompt, in the order they should be presented.
- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server's response to a prompts/get request from the client.


## GetTaskPayloadRequest

Import as `mcp_types.GetTaskPayloadRequest`  ·  defined at `mcp_types._types.GetTaskPayloadRequest`

```python
class GetTaskPayloadRequest(Request[GetTaskPayloadRequestParams, Literal['tasks/result']])
```

**Also exported as** `mcp_types.GetTaskPayloadRequest`

**Bases** `Request[GetTaskPayloadRequestParams, Literal['tasks/result']]`

**Declared members (2)**

- `method: Literal['tasks/result'] = 'tasks/result'`  _class-attribute, instance-attribute_
- `params: GetTaskPayloadRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A request to retrieve the result of a completed task (2025-11-25 only).


## GetTaskPayloadRequestParams

Import as `mcp_types.GetTaskPayloadRequestParams`  ·  defined at `mcp_types._types.GetTaskPayloadRequestParams`

```python
class GetTaskPayloadRequestParams(RequestParams)
```

**Also exported as** `mcp_types.GetTaskPayloadRequestParams`

**Bases** `RequestParams`

**Declared members (1)**

- `task_id: str`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for a tasks/result request.


## GetTaskPayloadResult

Import as `mcp_types.GetTaskPayloadResult`  ·  defined at `mcp_types._types.GetTaskPayloadResult`

```python
class GetTaskPayloadResult(Result)
```

**Also exported as** `mcp_types.GetTaskPayloadResult`

**Bases** `Result`

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The response to a tasks/result request (2025-11-25 only).

The structure matches the result type of the original request. The payload
arrives as extra wire fields, which `MCPModel` does not retain; validate the
response into the original request's result type (e.g. `CallToolResult`)
instead of this class.


## GetTaskRequest

Import as `mcp_types.GetTaskRequest`  ·  defined at `mcp_types._types.GetTaskRequest`

```python
class GetTaskRequest(Request[GetTaskRequestParams, Literal['tasks/get']])
```

**Also exported as** `mcp_types.GetTaskRequest`

**Bases** `Request[GetTaskRequestParams, Literal['tasks/get']]`

**Declared members (2)**

- `method: Literal['tasks/get'] = 'tasks/get'`  _class-attribute, instance-attribute_
- `params: GetTaskRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A request to retrieve the state of a task (2025-11-25 only).


## GetTaskRequestParams

Import as `mcp_types.GetTaskRequestParams`  ·  defined at `mcp_types._types.GetTaskRequestParams`

```python
class GetTaskRequestParams(RequestParams)
```

**Also exported as** `mcp_types.GetTaskRequestParams`

**Bases** `RequestParams`

**Declared members (1)**

- `task_id: str`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## GetTaskResult

Import as `mcp_types.GetTaskResult`  ·  defined at `mcp_types._types.GetTaskResult`

```python
class GetTaskResult(Result, Task)
```

**Also exported as** `mcp_types.GetTaskResult`

**Bases** `Result`, `Task`

**Inherited (8)**

- from `mcp_types._types.Result`: `meta`
- from `mcp_types._types.Task`: `created_at`, `last_updated_at`, `poll_interval`, `status`, `status_message`, `task_id`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The response to a tasks/get request (2025-11-25 only).


## Icon

Import as `mcp_types.Icon`  ·  defined at `mcp_types._types.Icon`

```python
class Icon(MCPModel)
```

**Also exported as** `mcp.server.mcpserver.Icon`, `mcp_types.Icon`

_18 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (4)**

- `mime_type: str | None = None`  _class-attribute, instance-attribute_
  Optional MIME type override if the source MIME type is missing or generic.
- `sizes: list[str] | None = None`  _class-attribute, instance-attribute_
  Optional sizes this icon is available in: WxH (e.g. `"48x48"`) or `"any"`. If not provided, assume the icon can be used at any size.
- `src: str`  _instance-attribute_
  A standard URI pointing to an icon resource (`http(s):` or `data:`).
- `theme: IconTheme | None = None`  _class-attribute, instance-attribute_
  The theme this icon is designed for. If not provided, assume any theme.

An optionally-sized icon for display in a user interface (2025-11-25+).


## ImageContent

Import as `mcp_types.ImageContent`  ·  defined at `mcp_types._types.ImageContent`

```python
class ImageContent(MCPModel)
```

**Also exported as** `mcp_types.ImageContent`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (5)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `data: str`  _instance-attribute_
  The base64-encoded image data.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See the MCP specification's "General fields: _meta" section for notes on _meta usage.
- `mime_type: str`  _instance-attribute_
  The MIME type of the image. Different providers may support different image types.
- `type: Literal['image'] = 'image'`  _class-attribute, instance-attribute_

An image provided to or from an LLM.


## Implementation

Import as `mcp_types.Implementation`  ·  defined at `mcp_types._types.Implementation`

```python
class Implementation(BaseMetadata)
```

**Also exported as** `mcp.Implementation`, `mcp_types.Implementation`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseMetadata`

**Declared members (4)**

- `description: str | None = None`  _class-attribute, instance-attribute_
  An optional human-readable description of what this implementation does.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `version: str`  _instance-attribute_
- `website_url: str | None = None`  _class-attribute, instance-attribute_
  An optional URL of the website for this implementation.

**Inherited (2)**

- from `mcp_types._types.BaseMetadata`: `name`, `title`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Describes the name and version of an MCP implementation (`clientInfo` / `serverInfo`).


## InitializeRequest

Import as `mcp_types.InitializeRequest`  ·  defined at `mcp_types._types.InitializeRequest`

```python
class InitializeRequest(Request[InitializeRequestParams, Literal['initialize']])
```

**Also exported as** `mcp.InitializeRequest`, `mcp_types.InitializeRequest`

**Bases** `Request[InitializeRequestParams, Literal['initialize']]`

**Declared members (2)**

- `method: Literal['initialize'] = 'initialize'`  _class-attribute, instance-attribute_
- `params: InitializeRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

This request is sent from the client to the server when it first connects, asking it
to begin initialization.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.
On 2026-07-28 the handshake is `server/discover` plus per-request `_meta`.


## InitializeRequestParams

Import as `mcp_types.InitializeRequestParams`  ·  defined at `mcp_types._types.InitializeRequestParams`

```python
class InitializeRequestParams(RequestParams)
```

**Also exported as** `mcp_types.InitializeRequestParams`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (3)**

- `capabilities: ClientCapabilities`  _instance-attribute_
- `client_info: Implementation`  _instance-attribute_
- `protocol_version: str`  _instance-attribute_
  The latest version of the Model Context Protocol that the client supports.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for the `initialize` request.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.


## InitializeResult

Import as `mcp_types.InitializeResult`  ·  defined at `mcp_types._types.InitializeResult`

```python
class InitializeResult(Result)
```

**Also exported as** `mcp.InitializeResult`, `mcp_types.InitializeResult`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (4)**

- `capabilities: ServerCapabilities`  _instance-attribute_
- `instructions: str | None = None`  _class-attribute, instance-attribute_
  Instructions describing how to use the server and its features.
- `protocol_version: str`  _instance-attribute_
  The version of the Model Context Protocol that the server wants to use. If the client cannot support this version, it MUST disconnect.
- `server_info: Implementation`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

After receiving an initialize request from the client, the server sends this response.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.


## InitializedNotification

Import as `mcp_types.InitializedNotification`  ·  defined at `mcp_types._types.InitializedNotification`

```python
class InitializedNotification(Notification[NotificationParams | None, Literal['notifications/initialized']])
```

**Also exported as** `mcp.InitializedNotification`, `mcp_types.InitializedNotification`

**Bases** `Notification[NotificationParams | None, Literal['notifications/initialized']]`

**Declared members (2)**

- `method: Literal['notifications/initialized'] = 'notifications/initialized'`  _class-attribute, instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

This notification is sent from the client to the server after initialization has
finished.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.


## InputRequiredResult

Import as `mcp_types.InputRequiredResult`  ·  defined at `mcp_types._types.InputRequiredResult`

```python
class InputRequiredResult(Result)
```

**Also exported as** `mcp_types.InputRequiredResult`

_13 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (3)**

- `input_requests: InputRequests | None = None`  _class-attribute, instance-attribute_
  Requests the client must complete before retrying. Keys are server-assigned.
- `request_state: str | None = None`  _class-attribute, instance-attribute_
  Opaque state to pass back verbatim when the client retries the original request.
- `result_type: Literal['input_required'] = 'input_required'`  _class-attribute, instance-attribute_
  Discriminating tag for the dual-result response unions.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server needs additional input before the original request can complete (2026-07-28).

Returned in place of the normal result of an interactive client request
(`tools/call`, `prompts/get`, `resources/read`). The client fulfills
`input_requests` and retries the original request, carrying the responses
and the echoed `request_state`. At least one of those two fields is
present on the wire (spec MUST).


## InputResponseRequestParams

Import as `mcp_types.InputResponseRequestParams`  ·  defined at `mcp_types._types.InputResponseRequestParams`

```python
class InputResponseRequestParams(RequestParams)
```

**Also exported as** `mcp_types.InputResponseRequestParams`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (2)**

- `input_responses: InputResponses | None = None`  _class-attribute, instance-attribute_
  Responses to the server's `InputRequiredResult.input_requests`, keyed identically.
- `request_state: str | None = None`  _class-attribute, instance-attribute_
  Opaque state from the `InputRequiredResult`, passed back verbatim on retry.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Base params for client requests that can carry responses to a server's
input requests (2026-07-28 multi-round-trip flow).

When a request returns an `InputRequiredResult`, the client retries the
original request with these fields populated.


## ListPromptsRequest

Import as `mcp_types.ListPromptsRequest`  ·  defined at `mcp_types._types.ListPromptsRequest`

```python
class ListPromptsRequest(PaginatedRequest[Literal['prompts/list']])
```

**Also exported as** `mcp.ListPromptsRequest`, `mcp_types.ListPromptsRequest`

**Bases** `PaginatedRequest[Literal['prompts/list']]`

**Declared members (1)**

- `method: Literal['prompts/list'] = 'prompts/list'`  _class-attribute, instance-attribute_

**Inherited (2)**

- from `mcp_types._types.PaginatedRequest`: `params`
- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Sent from the client to request a list of prompts and prompt templates the server has.


## ListPromptsResult

Import as `mcp_types.ListPromptsResult`  ·  defined at `mcp_types._types.ListPromptsResult`

```python
class ListPromptsResult(PaginatedResult, CacheableResult)
```

**Also exported as** `mcp.ListPromptsResult`, `mcp_types.ListPromptsResult`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `PaginatedResult`, `CacheableResult`

**Declared members (2)**

- `prompts: list[Prompt]`  _instance-attribute_
- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.

**Inherited (4)**

- from `mcp_types._types.CacheableResult`: `cache_scope`, `ttl_ms`
- from `mcp_types._types.PaginatedResult`: `next_cursor`
- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server's response to a prompts/list request from the client.


## ListResourceTemplatesRequest

Import as `mcp_types.ListResourceTemplatesRequest`  ·  defined at `mcp_types._types.ListResourceTemplatesRequest`

```python
class ListResourceTemplatesRequest(PaginatedRequest[Literal['resources/templates/list']])
```

**Also exported as** `mcp_types.ListResourceTemplatesRequest`

**Bases** `PaginatedRequest[Literal['resources/templates/list']]`

**Declared members (1)**

- `method: Literal['resources/templates/list'] = 'resources/templates/list'`  _class-attribute, instance-attribute_

**Inherited (2)**

- from `mcp_types._types.PaginatedRequest`: `params`
- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Sent from the client to request a list of resource templates the server has.


## ListResourceTemplatesResult

Import as `mcp_types.ListResourceTemplatesResult`  ·  defined at `mcp_types._types.ListResourceTemplatesResult`

```python
class ListResourceTemplatesResult(PaginatedResult, CacheableResult)
```

**Also exported as** `mcp_types.ListResourceTemplatesResult`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `PaginatedResult`, `CacheableResult`

**Declared members (2)**

- `resource_templates: list[ResourceTemplate]`  _instance-attribute_
- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.

**Inherited (4)**

- from `mcp_types._types.CacheableResult`: `cache_scope`, `ttl_ms`
- from `mcp_types._types.PaginatedResult`: `next_cursor`
- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server's response to a resources/templates/list request from the client.


## ListResourcesRequest

Import as `mcp_types.ListResourcesRequest`  ·  defined at `mcp_types._types.ListResourcesRequest`

```python
class ListResourcesRequest(PaginatedRequest[Literal['resources/list']])
```

**Also exported as** `mcp.ListResourcesRequest`, `mcp_types.ListResourcesRequest`

**Bases** `PaginatedRequest[Literal['resources/list']]`

**Declared members (1)**

- `method: Literal['resources/list'] = 'resources/list'`  _class-attribute, instance-attribute_

**Inherited (2)**

- from `mcp_types._types.PaginatedRequest`: `params`
- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Sent from the client to request a list of resources the server has.


## ListResourcesResult

Import as `mcp_types.ListResourcesResult`  ·  defined at `mcp_types._types.ListResourcesResult`

```python
class ListResourcesResult(PaginatedResult, CacheableResult)
```

**Also exported as** `mcp.ListResourcesResult`, `mcp_types.ListResourcesResult`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `PaginatedResult`, `CacheableResult`

**Declared members (2)**

- `resources: list[Resource]`  _instance-attribute_
- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.

**Inherited (4)**

- from `mcp_types._types.CacheableResult`: `cache_scope`, `ttl_ms`
- from `mcp_types._types.PaginatedResult`: `next_cursor`
- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server's response to a resources/list request from the client.


## ListRootsRequest

Import as `mcp_types.ListRootsRequest`  ·  defined at `mcp_types._types.ListRootsRequest`

```python
class ListRootsRequest(Request[RequestParams | None, Literal['roots/list']])
```

**Also exported as** `mcp_types.ListRootsRequest`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Request[RequestParams | None, Literal['roots/list']]`

**Declared members (2)**

- `method: Literal['roots/list'] = 'roots/list'`  _class-attribute, instance-attribute_
- `params: RequestParams | None = None`  _class-attribute, instance-attribute_
  Stays optional on 2026-07-28 (reserved client `_meta` keys do not apply to server-to-client payloads).

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Sent from the server to request a list of root URIs from the client. Roots allow
servers to ask for specific directories or files to operate on. A common example
for roots is providing a set of repositories or directories a server should operate
on.

This request is typically used when the server needs to understand the file system
structure or access specific locations that the client has permission to read from.

A standalone JSON-RPC request through 2025-11-25; on 2026-07-28 it is
embedded in `InputRequiredResult.input_requests`. Deprecated in 2026-07-28 (SEP-2577).


## ListRootsResult

Import as `mcp_types.ListRootsResult`  ·  defined at `mcp_types._types.ListRootsResult`

```python
class ListRootsResult(Result)
```

**Also exported as** `mcp_types.ListRootsResult`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (1)**

- `roots: list[Root]`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The client's response to a roots/list request from the server.

This result contains an array of Root objects, each representing a root
directory or file that the server can operate on.

On 2026-07-28 this is carried as an `InputResponses` entry, not a JSON-RPC
result. Deprecated in 2026-07-28 (SEP-2577).


## ListTasksRequest

Import as `mcp_types.ListTasksRequest`  ·  defined at `mcp_types._types.ListTasksRequest`

```python
class ListTasksRequest(PaginatedRequest[Literal['tasks/list']])
```

**Also exported as** `mcp_types.ListTasksRequest`

**Bases** `PaginatedRequest[Literal['tasks/list']]`

**Declared members (1)**

- `method: Literal['tasks/list'] = 'tasks/list'`  _class-attribute, instance-attribute_

**Inherited (2)**

- from `mcp_types._types.PaginatedRequest`: `params`
- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A request to retrieve a list of tasks (2025-11-25 only).


## ListTasksResult

Import as `mcp_types.ListTasksResult`  ·  defined at `mcp_types._types.ListTasksResult`

```python
class ListTasksResult(PaginatedResult)
```

**Also exported as** `mcp_types.ListTasksResult`

**Bases** `PaginatedResult`

**Declared members (1)**

- `tasks: list[Task]`  _instance-attribute_

**Inherited (2)**

- from `mcp_types._types.PaginatedResult`: `next_cursor`
- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The response to a tasks/list request (2025-11-25 only).


## ListToolsRequest

Import as `mcp_types.ListToolsRequest`  ·  defined at `mcp_types._types.ListToolsRequest`

```python
class ListToolsRequest(PaginatedRequest[Literal['tools/list']])
```

**Also exported as** `mcp_types.ListToolsRequest`

**Bases** `PaginatedRequest[Literal['tools/list']]`

**Declared members (1)**

- `method: Literal['tools/list'] = 'tools/list'`  _class-attribute, instance-attribute_

**Inherited (2)**

- from `mcp_types._types.PaginatedRequest`: `params`
- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Sent from the client to request a list of tools the server has.


## ListToolsResult

Import as `mcp_types.ListToolsResult`  ·  defined at `mcp_types._types.ListToolsResult`

```python
class ListToolsResult(PaginatedResult, CacheableResult)
```

**Also exported as** `mcp.ListToolsResult`, `mcp_types.ListToolsResult`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `PaginatedResult`, `CacheableResult`

**Declared members (2)**

- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.
- `tools: list[Tool]`  _instance-attribute_

**Inherited (4)**

- from `mcp_types._types.CacheableResult`: `cache_scope`, `ttl_ms`
- from `mcp_types._types.PaginatedResult`: `next_cursor`
- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server's response to a tools/list request from the client.


## LoggingCapability

Import as `mcp_types.LoggingCapability`  ·  defined at `mcp_types._types.LoggingCapability`

```python
class LoggingCapability(MCPModel)
```

**Also exported as** `mcp_types.LoggingCapability`

**Bases** `MCPModel`

Capability for logging operations.


## LoggingMessageNotification

Import as `mcp_types.LoggingMessageNotification`  ·  defined at `mcp_types._types.LoggingMessageNotification`

```python
class LoggingMessageNotification(Notification[LoggingMessageNotificationParams, Literal['notifications/message']])
```

**Also exported as** `mcp.LoggingMessageNotification`, `mcp_types.LoggingMessageNotification`

**Bases** `Notification[LoggingMessageNotificationParams, Literal['notifications/message']]`

**Declared members (2)**

- `method: Literal['notifications/message'] = 'notifications/message'`  _class-attribute, instance-attribute_
- `params: LoggingMessageNotificationParams`  _instance-attribute_

Notification of a log message passed from server to client.

Through 2025-11-25 the client subscribes via `logging/setLevel`. On
2026-07-28 the client opts in per-request via `_meta` (`LOG_LEVEL_META_KEY`)
and the server MUST NOT send this without it. Deprecated in 2026-07-28 (SEP-2577).


## LoggingMessageNotificationParams

Import as `mcp_types.LoggingMessageNotificationParams`  ·  defined at `mcp_types._types.LoggingMessageNotificationParams`

```python
class LoggingMessageNotificationParams(NotificationParams)
```

**Also exported as** `mcp_types.LoggingMessageNotificationParams`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NotificationParams`

**Declared members (3)**

- `data: Any`  _instance-attribute_
  The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.
- `level: LoggingLevel`  _instance-attribute_
  The severity of this log message.
- `logger: str | None = None`  _class-attribute, instance-attribute_
  An optional name of the logger issuing this message.

**Inherited (1)**

- from `mcp_types._types.NotificationParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## MCPModel

`mcp_types._types.MCPModel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class MCPModel(BaseModel)
```

**Bases** `BaseModel`

Base class for all MCP protocol types.


## MissingRequiredClientCapabilityErrorData

Import as `mcp_types.MissingRequiredClientCapabilityErrorData`  ·  defined at `mcp_types._types.MissingRequiredClientCapabilityErrorData`

```python
class MissingRequiredClientCapabilityErrorData(MCPModel)
```

**Also exported as** `mcp_types.MissingRequiredClientCapabilityErrorData`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (1)**

- `required_capabilities: ClientCapabilities`  _instance-attribute_
  The capabilities the server requires from the client to process this request.

Error data for the -32021 missing-required-client-capability error (2026-07-28).


## ModelHint

Import as `mcp_types.ModelHint`  ·  defined at `mcp_types._types.ModelHint`

```python
class ModelHint(MCPModel)
```

**Also exported as** `mcp_types.ModelHint`

**Bases** `MCPModel`

**Declared members (1)**

- `name: str | None = None`  _class-attribute, instance-attribute_
  A hint for a model name.

Hints to use for model selection.

Keys not declared here are up to the client to interpret. Deprecated in
2026-07-28 (SEP-2577) with the rest of sampling.


## ModelPreferences

Import as `mcp_types.ModelPreferences`  ·  defined at `mcp_types._types.ModelPreferences`

```python
class ModelPreferences(MCPModel)
```

**Also exported as** `mcp_types.ModelPreferences`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (4)**

- `cost_priority: float | None = None`  _class-attribute, instance-attribute_
  How much to prioritize cost when selecting a model. A value of 0 means cost is not important, while a value of 1 means cost is the most important factor.
- `hints: list[ModelHint] | None = None`  _class-attribute, instance-attribute_
  Optional hints to use for model selection.
- `intelligence_priority: float | None = None`  _class-attribute, instance-attribute_
  How much to prioritize intelligence and capabilities when selecting a model. A value of 0 means intelligence is not important, while a value of 1 means intelligence is the most important factor.
- `speed_priority: float | None = None`  _class-attribute, instance-attribute_
  How much to prioritize sampling speed (latency) when selecting a model. A value of 0 means speed is not important, while a value of 1 means speed is the most important factor.

The server's preferences for model selection, requested of the client during
sampling.

Because LLMs can vary along multiple dimensions, choosing the "best" model is
rarely straightforward. Different models excel in different areas—some are
faster but less capable, others are more capable but more expensive, and so
on. This interface allows servers to express their priorities across multiple
dimensions to help clients make an appropriate selection for their use case.

These preferences are always advisory. The client MAY ignore them. It is also
up to the client to decide how to interpret these preferences and how to
balance them against other considerations.

Deprecated in 2026-07-28 (SEP-2577) with the rest of sampling.


## Notification

Import as `mcp_types.Notification`  ·  defined at `mcp_types._types.Notification`

```python
class Notification(MCPModel, Generic[NotificationParamsT, MethodT])
```

**Also exported as** `mcp.Notification`, `mcp_types.Notification`

**Bases** `MCPModel`, `Generic[NotificationParamsT, MethodT]`

**Declared members (2)**

- `method: MethodT`  _instance-attribute_
- `params: NotificationParamsT`  _instance-attribute_

Base class for JSON-RPC notifications.


## NotificationParams

Import as `mcp_types.NotificationParams`  ·  defined at `mcp_types._types.NotificationParams`

```python
class NotificationParams(MCPModel)
```

**Also exported as** `mcp_types.NotificationParams`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (1)**

- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.

## PaginatedRequest

Import as `mcp_types.PaginatedRequest`  ·  defined at `mcp_types._types.PaginatedRequest`

```python
class PaginatedRequest(Request[PaginatedRequestParams | None, MethodT], Generic[MethodT])
```

**Also exported as** `mcp_types.PaginatedRequest`

**Bases** `Request[PaginatedRequestParams | None, MethodT]`, `Generic[MethodT]`

**Declared members (1)**

- `params: PaginatedRequestParams | None = None`  _class-attribute, instance-attribute_
  Pagination params. Required on the 2026-07-28+ wire (because `_meta` is); the session layer materializes it there. Optional on earlier versions.

**Inherited (2)**

- from `mcp_types._types.Request`: `method`, `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Base class for paginated requests, matching the schema's PaginatedRequest interface.


## PaginatedRequestParams

Import as `mcp_types.PaginatedRequestParams`  ·  defined at `mcp_types._types.PaginatedRequestParams`

```python
class PaginatedRequestParams(RequestParams)
```

**Also exported as** `mcp_types.PaginatedRequestParams`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (1)**

- `cursor: str | None = None`  _class-attribute, instance-attribute_
  An opaque token representing the current pagination position.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## PaginatedResult

Import as `mcp_types.PaginatedResult`  ·  defined at `mcp_types._types.PaginatedResult`

```python
class PaginatedResult(Result)
```

**Also exported as** `mcp_types.PaginatedResult`

**Bases** `Result`

**Declared members (1)**

- `next_cursor: str | None = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## PingRequest

Import as `mcp_types.PingRequest`  ·  defined at `mcp_types._types.PingRequest`

```python
class PingRequest(Request[RequestParams | None, Literal['ping']])
```

**Also exported as** `mcp.PingRequest`, `mcp_types.PingRequest`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Request[RequestParams | None, Literal['ping']]`

**Declared members (2)**

- `method: Literal['ping'] = 'ping'`  _class-attribute, instance-attribute_
- `params: RequestParams | None = None`  _class-attribute, instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A ping, issued by either the server or the client, to check that the other party is
still alive. The receiver must promptly respond, or else may be disconnected.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.


## ProgressNotification

Import as `mcp_types.ProgressNotification`  ·  defined at `mcp_types._types.ProgressNotification`

```python
class ProgressNotification(Notification[ProgressNotificationParams, Literal['notifications/progress']])
```

**Also exported as** `mcp.ProgressNotification`, `mcp_types.ProgressNotification`

**Bases** `Notification[ProgressNotificationParams, Literal['notifications/progress']]`

**Declared members (2)**

- `method: Literal['notifications/progress'] = 'notifications/progress'`  _class-attribute, instance-attribute_
- `params: ProgressNotificationParams`  _instance-attribute_

An out-of-band notification used to inform the receiver of a progress update for a long-running request.


## ProgressNotificationParams

Import as `mcp_types.ProgressNotificationParams`  ·  defined at `mcp_types._types.ProgressNotificationParams`

```python
class ProgressNotificationParams(NotificationParams)
```

**Also exported as** `mcp_types.ProgressNotificationParams`

**Bases** `NotificationParams`

**Declared members (4)**

- `message: str | None = None`  _class-attribute, instance-attribute_
  Message related to progress.
- `progress: float`  _instance-attribute_
  The progress thus far. This should increase every time progress is made, even if the total is unknown.
- `progress_token: ProgressToken`  _instance-attribute_
  The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.
- `total: float | None = None`  _class-attribute, instance-attribute_
  Total number of items to process (or total progress required), if known.

**Inherited (1)**

- from `mcp_types._types.NotificationParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for progress notifications.


## Prompt

Import as `mcp_types.Prompt`  ·  defined at `mcp_types._types.Prompt`

```python
class Prompt(BaseMetadata)
```

**Also exported as** `fastmcp.prompts.base.SDKPrompt`, `mcp_types.Prompt`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseMetadata`

**Declared members (4)**

- `arguments: list[PromptArgument] | None = None`  _class-attribute, instance-attribute_
  A list of arguments to use for templating the prompt.
- `description: str | None = None`  _class-attribute, instance-attribute_
  An optional description of what this prompt provides.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  An optional list of icons for this prompt.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.

**Inherited (2)**

- from `mcp_types._types.BaseMetadata`: `name`, `title`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A prompt or prompt template that the server offers.


## PromptArgument

Import as `mcp_types.PromptArgument`  ·  defined at `mcp_types._types.PromptArgument`

```python
class PromptArgument(BaseMetadata)
```

**Also exported as** `fastmcp.prompts.base.SDKPromptArgument`, `mcp_types.PromptArgument`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseMetadata`

**Declared members (2)**

- `description: str | None = None`  _class-attribute, instance-attribute_
  A human-readable description of the argument.
- `required: bool | None = None`  _class-attribute, instance-attribute_
  Whether this argument must be provided.

**Inherited (2)**

- from `mcp_types._types.BaseMetadata`: `name`, `title`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Describes an argument that a prompt can accept.


## PromptListChangedNotification

Import as `mcp_types.PromptListChangedNotification`  ·  defined at `mcp_types._types.PromptListChangedNotification`

```python
class PromptListChangedNotification(Notification[NotificationParams | None, Literal['notifications/prompts/list_changed']])
```

**Also exported as** `mcp_types.PromptListChangedNotification`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Notification[NotificationParams | None, Literal['notifications/prompts/list_changed']]`

**Declared members (2)**

- `method: Literal['notifications/prompts/list_changed'] = 'notifications/prompts/list_changed'`  _class-attribute, instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list
of prompts it offers has changed.

May be sent spontaneously through 2025-11-25; on 2026-07-28 sessions the
client must opt in via `subscriptions/listen`.


## PromptMessage

Import as `mcp_types.PromptMessage`  ·  defined at `mcp_types._types.PromptMessage`

```python
class PromptMessage(MCPModel)
```

**Also exported as** `fastmcp.prompts.PromptMessage`, `mcp_types.PromptMessage`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (2)**

- `content: ContentBlock`  _instance-attribute_
- `role: Role`  _instance-attribute_

Describes a message returned as part of a prompt.

Similar to `SamplingMessage`, but also supports embedded resources.


## PromptReference

Import as `mcp_types.PromptReference`  ·  defined at `mcp_types._types.PromptReference`

```python
class PromptReference(MCPModel)
```

**Also exported as** `mcp_types.PromptReference`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (3)**

- `name: str`  _instance-attribute_
  The name of the prompt or prompt template.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Human-readable display title. If not provided, `name` should be used for display.
- `type: Literal['ref/prompt'] = 'ref/prompt'`  _class-attribute, instance-attribute_

Identifies a prompt.


## PromptsCapability

Import as `mcp_types.PromptsCapability`  ·  defined at `mcp_types._types.PromptsCapability`

```python
class PromptsCapability(MCPModel)
```

**Also exported as** `mcp.PromptsCapability`, `mcp_types.PromptsCapability`

**Bases** `MCPModel`

**Declared members (1)**

- `list_changed: bool | None = None`  _class-attribute, instance-attribute_
  Whether this server supports notifications for changes to the prompt list.

Capability for prompts operations.


## ReadResourceRequest

Import as `mcp_types.ReadResourceRequest`  ·  defined at `mcp_types._types.ReadResourceRequest`

```python
class ReadResourceRequest(Request[ReadResourceRequestParams, Literal['resources/read']])
```

**Also exported as** `mcp.ReadResourceRequest`, `mcp_types.ReadResourceRequest`

**Bases** `Request[ReadResourceRequestParams, Literal['resources/read']]`

**Declared members (2)**

- `method: Literal['resources/read'] = 'resources/read'`  _class-attribute, instance-attribute_
- `params: ReadResourceRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Sent from the client to the server, to read a specific resource URI.


## ReadResourceRequestParams

Import as `mcp_types.ReadResourceRequestParams`  ·  defined at `mcp_types._types.ReadResourceRequestParams`

```python
class ReadResourceRequestParams(InputResponseRequestParams)
```

**Also exported as** `mcp_types.ReadResourceRequestParams`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `InputResponseRequestParams`

**Declared members (1)**

- `uri: str`  _instance-attribute_
  The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it.

**Inherited (3)**

- from `mcp_types._types.InputResponseRequestParams`: `input_responses`, `request_state`
- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## ReadResourceResult

Import as `mcp_types.ReadResourceResult`  ·  defined at `mcp_types._types.ReadResourceResult`

```python
class ReadResourceResult(CacheableResult)
```

**Also exported as** `mcp.ReadResourceResult`, `mcp_types.ReadResourceResult`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `CacheableResult`

**Declared members (2)**

- `contents: list[TextResourceContents | BlobResourceContents]`  _instance-attribute_
  The contents of the resource or sub-resources that were read.
- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.

**Inherited (3)**

- from `mcp_types._types.CacheableResult`: `cache_scope`, `ttl_ms`
- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The server's response to a resources/read request from the client.


## RelatedTaskMetadata

Import as `mcp_types.RelatedTaskMetadata`  ·  defined at `mcp_types._types.RelatedTaskMetadata`

```python
class RelatedTaskMetadata(MCPModel)
```

**Also exported as** `mcp_types.RelatedTaskMetadata`

**Bases** `MCPModel`

**Declared members (1)**

- `task_id: str`  _instance-attribute_

Associates a message with a task, via `_meta["io.modelcontextprotocol/related-task"]` (2025-11-25 only).


## Request

Import as `mcp_types.Request`  ·  defined at `mcp_types._types.Request`

```python
class Request(MCPModel, Generic[RequestParamsT, MethodT])
```

**Also exported as** `mcp_types.Request`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`, `Generic[RequestParamsT, MethodT]`

**Declared members (3)**

- `method: MethodT`  _instance-attribute_
- `name_param: str | None = None`  _class-attribute_
  Wire-params key mirrored into the `Mcp-Name` header on sends; SEP-2663 requires it for tasks/*.
- `params: RequestParamsT`  _instance-attribute_

Base class for JSON-RPC requests.

The JSON-RPC envelope (`jsonrpc`, `id`) is attached by the session layer
(see `mcp_types.jsonrpc`), not carried here.


## RequestParams

Import as `mcp_types.RequestParams`  ·  defined at `mcp_types._types.RequestParams`

```python
class RequestParams(MCPModel)
```

**Also exported as** `mcp_types.RequestParams`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (1)**

- `meta: RequestParamsMeta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  Metadata reserved by MCP for protocol-level concerns (wire name `_meta`).

## RequestParamsMeta

Import as `mcp_types.RequestParamsMeta`  ·  defined at `mcp_types._types.RequestParamsMeta`

```python
class RequestParamsMeta(TypedDict)
```

**Also exported as** `mcp_types.RequestParamsMeta`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TypedDict`

**Declared members (1)**

- `progress_token: NotRequired[ProgressToken]`  _instance-attribute_
  If specified, the caller requests out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The rec…

The `_meta` object on request params (schema name: `RequestMetaObject`).

An open map: arbitrary keys round-trip via `extra_items=Any`. Read or set
the reserved `io.modelcontextprotocol/*` keys via the `*_META_KEY` constants.


## Resource

Import as `mcp_types.Resource`  ·  defined at `mcp_types._types.Resource`

```python
class Resource(BaseMetadata)
```

**Also exported as** `fastmcp.resources.base.SDKResource`, `mcp.Resource`, `mcp_types.Resource`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseMetadata`

**Declared members (7)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `description: str | None = None`  _class-attribute, instance-attribute_
  A description of what this resource represents.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See the MCP specification for notes on `_meta` usage.
- `mime_type: str | None = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `size: int | None = None`  _class-attribute, instance-attribute_
  The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.
- `uri: str`  _instance-attribute_
  The URI of this resource.

**Inherited (2)**

- from `mcp_types._types.BaseMetadata`: `name`, `title`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A known resource that the server is capable of reading.


## ResourceContents

Import as `mcp_types.ResourceContents`  ·  defined at `mcp_types._types.ResourceContents`

```python
class ResourceContents(MCPModel)
```

**Also exported as** `mcp_types.ResourceContents`

**Bases** `MCPModel`

**Declared members (3)**

- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.
- `mime_type: str | None = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `uri: str`  _instance-attribute_
  The URI of this resource.

The contents of a specific resource or sub-resource.


## ResourceLink

Import as `mcp_types.ResourceLink`  ·  defined at `mcp_types._types.ResourceLink`

```python
class ResourceLink(Resource)
```

**Also exported as** `mcp_types.ResourceLink`

**Bases** `Resource`

**Declared members (1)**

- `type: Literal['resource_link'] = 'resource_link'`  _class-attribute, instance-attribute_

**Inherited (9)**

- from `mcp_types._types.BaseMetadata`: `name`, `title`
- from `mcp_types._types.Resource`: `annotations`, `description`, `icons`, `meta`, `mime_type`, `size`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that the server is capable of reading, included in a prompt or tool call result.

Note: resource links returned by tools are not guaranteed to appear in the results of `resources/list` requests.


## ResourceListChangedNotification

Import as `mcp_types.ResourceListChangedNotification`  ·  defined at `mcp_types._types.ResourceListChangedNotification`

```python
class ResourceListChangedNotification(Notification[NotificationParams | None, Literal['notifications/resources/list_changed']])
```

**Also exported as** `mcp_types.ResourceListChangedNotification`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Notification[NotificationParams | None, Literal['notifications/resources/list_changed']]`

**Declared members (2)**

- `method: Literal['notifications/resources/list_changed'] = 'notifications/resources/list_changed'`  _class-attribute, instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list
of resources it can read from has changed.

May be sent spontaneously through 2025-11-25; on 2026-07-28 sessions the
client must opt in via `subscriptions/listen`.


## ResourceTemplate

Import as `mcp_types.ResourceTemplate`  ·  defined at `mcp_types._types.ResourceTemplate`

```python
class ResourceTemplate(BaseMetadata)
```

**Also exported as** `mcp_types.ResourceTemplate`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseMetadata`

**Declared members (6)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `description: str | None = None`  _class-attribute, instance-attribute_
  A description of what this template is for.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  An optional set of sized icons that the client can display in a user interface.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.
- `mime_type: str | None = None`  _class-attribute, instance-attribute_
  The MIME type for all resources that match this template.
- `uri_template: str`  _instance-attribute_
  A URI template (according to RFC 6570) that can be used to construct resource URIs.

**Inherited (2)**

- from `mcp_types._types.BaseMetadata`: `name`, `title`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A template description for resources available on the server.


## ResourceTemplateReference

Import as `mcp_types.ResourceTemplateReference`  ·  defined at `mcp_types._types.ResourceTemplateReference`

```python
class ResourceTemplateReference(MCPModel)
```

**Also exported as** `mcp_types.ResourceTemplateReference`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (2)**

- `type: Literal['ref/resource'] = 'ref/resource'`  _class-attribute, instance-attribute_
- `uri: str`  _instance-attribute_
  The URI or URI template of the resource.

A reference to a resource or resource template definition.


## ResourceUpdatedNotification

Import as `mcp_types.ResourceUpdatedNotification`  ·  defined at `mcp_types._types.ResourceUpdatedNotification`

```python
class ResourceUpdatedNotification(Notification[ResourceUpdatedNotificationParams, Literal['notifications/resources/updated']])
```

**Also exported as** `mcp.ResourceUpdatedNotification`, `mcp_types.ResourceUpdatedNotification`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Notification[ResourceUpdatedNotificationParams, Literal['notifications/resources/updated']]`

**Declared members (2)**

- `method: Literal['notifications/resources/updated'] = 'notifications/resources/updated'`  _class-attribute, instance-attribute_
- `params: ResourceUpdatedNotificationParams`  _instance-attribute_

A notification from the server to the client, informing it that a resource has
changed and may need to be read again.

Only sent if the client subscribed: via `resources/subscribe` through
2025-11-25, or `subscriptions/listen` on 2026-07-28.


## ResourceUpdatedNotificationParams

Import as `mcp_types.ResourceUpdatedNotificationParams`  ·  defined at `mcp_types._types.ResourceUpdatedNotificationParams`

```python
class ResourceUpdatedNotificationParams(NotificationParams)
```

**Also exported as** `mcp_types.ResourceUpdatedNotificationParams`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NotificationParams`

**Declared members (1)**

- `uri: str`  _instance-attribute_
  The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.

**Inherited (1)**

- from `mcp_types._types.NotificationParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## ResourcesCapability

Import as `mcp_types.ResourcesCapability`  ·  defined at `mcp_types._types.ResourcesCapability`

```python
class ResourcesCapability(MCPModel)
```

**Also exported as** `mcp.ResourcesCapability`, `mcp_types.ResourcesCapability`

**Bases** `MCPModel`

**Declared members (2)**

- `list_changed: bool | None = None`  _class-attribute, instance-attribute_
  Whether this server supports notifications for changes to the resource list.
- `subscribe: bool | None = None`  _class-attribute, instance-attribute_
  Whether this server supports subscribing to resource updates.

Capability for resources operations.


## Result

Import as `mcp_types.Result`  ·  defined at `mcp_types._types.Result`

```python
class Result(MCPModel)
```

**Also exported as** `mcp_types.Result`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (1)**

- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.

Base class for JSON-RPC results.

`result_type` is declared per concrete subclass, not here, because defaults
differ: most results default to "complete", `EmptyResult` defaults to None
(so it dumps as `{}`; some peer SDKs strict-validate empty results), and
`InputRequiredResult` carries a literal.


## Root

Import as `mcp_types.Root`  ·  defined at `mcp_types._types.Root`

```python
class Root(MCPModel)
```

**Also exported as** `mcp_types.Root`

**Bases** `MCPModel`

**Declared members (3)**

- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.
- `name: str | None = None`  _class-attribute, instance-attribute_
  An optional name for the root. This can be used to provide a human-readable identifier for the root, which may be useful for display purposes or for referencing the root in other parts of the application.
- `uri: FileUrl`  _instance-attribute_
  The URI identifying the root. This *must* start with file:// for now. This restriction may be relaxed in future versions of the protocol to allow other URI schemes.

Represents a root directory or file that the server can operate on.

Deprecated in 2026-07-28 (SEP-2577) with the rest of roots.


## RootsCapability

Import as `mcp_types.RootsCapability`  ·  defined at `mcp_types._types.RootsCapability`

```python
class RootsCapability(MCPModel)
```

**Also exported as** `mcp.RootsCapability`, `mcp_types.RootsCapability`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (1)**

- `list_changed: bool | None = None`  _class-attribute, instance-attribute_
  Whether the client supports notifications for changes to the roots list.

Capability for root operations.

Deprecated in protocol 2026-07-28 (SEP-2577) but still carried there as an
empty object (`list_changed` exists only through 2025-11-25).


## RootsListChangedNotification

Import as `mcp_types.RootsListChangedNotification`  ·  defined at `mcp_types._types.RootsListChangedNotification`

```python
class RootsListChangedNotification(Notification[NotificationParams | None, Literal['notifications/roots/list_changed']])
```

**Also exported as** `mcp_types.RootsListChangedNotification`

**Bases** `Notification[NotificationParams | None, Literal['notifications/roots/list_changed']]`

**Declared members (2)**

- `method: Literal['notifications/roots/list_changed'] = 'notifications/roots/list_changed'`  _class-attribute, instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

A notification from the client to the server, informing it that the list of
roots has changed.

This notification should be sent whenever the client adds, removes, or
modifies any root. The server should then request an updated list of roots
using the ListRootsRequest.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.


## SamplingCapability

Import as `mcp_types.SamplingCapability`  ·  defined at `mcp_types._types.SamplingCapability`

```python
class SamplingCapability(MCPModel)
```

**Also exported as** `mcp.SamplingCapability`, `mcp_types.SamplingCapability`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (2)**

- `context: SamplingContextCapability | None = None`  _class-attribute, instance-attribute_
  Present if the client supports non-'none' values for includeContext parameter. SOFT-DEPRECATED: New implementations should use tools parameter instead.
- `tools: SamplingToolsCapability | None = None`  _class-attribute, instance-attribute_
  Present if the client supports tools and toolChoice parameters in sampling requests. Presence indicates full tool calling support during sampling.

Sampling capability structure. Deprecated in 2026-07-28 (SEP-2577); shape unchanged.


## SamplingContextCapability

Import as `mcp_types.SamplingContextCapability`  ·  defined at `mcp_types._types.SamplingContextCapability`

```python
class SamplingContextCapability(MCPModel)
```

**Also exported as** `mcp.SamplingContextCapability`, `mcp_types.SamplingContextCapability`

**Bases** `MCPModel`

Capability for context inclusion during sampling.

Indicates support for non-'none' values in the includeContext parameter.
SOFT-DEPRECATED: New implementations should use tools parameter instead.


## SamplingMessage

Import as `mcp_types.SamplingMessage`  ·  defined at `mcp_types._types.SamplingMessage`

```python
class SamplingMessage(MCPModel)
```

**Also exported as** `fastmcp.client.sampling.SamplingMessage`, `mcp.SamplingMessage`, `mcp_types.SamplingMessage`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (4)**

- `content: SamplingMessageContentBlock | list[SamplingMessageContentBlock]`  _instance-attribute_
  Message content. Can be a single content block or an array of content blocks for multi-modal messages and tool interactions.
- `content_as_list: list[SamplingMessageContentBlock]`  _property_
  Returns the content as a list of content blocks, regardless of whether it was originally a single block or a list.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.
- `role: Role`  _instance-attribute_

Describes a message issued to or received from an LLM API.


## SamplingToolsCapability

Import as `mcp_types.SamplingToolsCapability`  ·  defined at `mcp_types._types.SamplingToolsCapability`

```python
class SamplingToolsCapability(MCPModel)
```

**Also exported as** `mcp.SamplingToolsCapability`, `mcp_types.SamplingToolsCapability`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

Capability indicating support for tool calling during sampling.

When present in ClientCapabilities.sampling, indicates that the client
supports the tools and toolChoice parameters in sampling requests.


## ServerCapabilities

Import as `mcp_types.ServerCapabilities`  ·  defined at `mcp_types._types.ServerCapabilities`

```python
class ServerCapabilities(MCPModel)
```

**Also exported as** `mcp.ServerCapabilities`, `mcp_types.ServerCapabilities`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (8)**

- `completions: CompletionsCapability | None = None`  _class-attribute, instance-attribute_
  Present if the server offers autocompletion suggestions for prompts and resources.
- `experimental: dict[str, dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
  Experimental, non-standard capabilities that the server supports.
- `extensions: dict[str, dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
  MCP extensions the server supports (2026-07-28). Keys are extension identifiers; values are per-extension settings (empty object = no settings).
- `logging: LoggingCapability | None = None`  _class-attribute, instance-attribute_
  Present if the server supports sending log messages to the client. Deprecated in 2026-07-28 (SEP-2577).
- `prompts: PromptsCapability | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any prompt templates.
- `resources: ResourcesCapability | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any resources to read.
- `tasks: ServerTasksCapability | None = None`  _class-attribute, instance-attribute_
  Present if the server supports task-augmented requests (2025-11-25 only).
- `tools: ToolsCapability | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any tools to call.

Capabilities that a server may support. Not a closed set.


## ServerTasksCapability

Import as `mcp_types.ServerTasksCapability`  ·  defined at `mcp_types._types.ServerTasksCapability`

```python
class ServerTasksCapability(MCPModel)
```

**Also exported as** `mcp_types.ServerTasksCapability`

**Bases** `MCPModel`

**Declared members (3)**

- `cancel: TasksCancelCapability | None = None`  _class-attribute, instance-attribute_
- `list: TasksListCapability | None = None`  _class-attribute, instance-attribute_
- `requests: ServerTasksRequestsCapability | None = None`  _class-attribute, instance-attribute_

Capability for server tasks operations (2025-11-25 only).


## ServerTasksRequestsCapability

Import as `mcp_types.ServerTasksRequestsCapability`  ·  defined at `mcp_types._types.ServerTasksRequestsCapability`

```python
class ServerTasksRequestsCapability(MCPModel)
```

**Also exported as** `mcp_types.ServerTasksRequestsCapability`

**Bases** `MCPModel`

**Declared members (1)**

- `tools: TasksToolsCapability | None = None`  _class-attribute, instance-attribute_

Specifies which request types the server can augment with tasks (2025-11-25 only).


## SetLevelRequest

Import as `mcp_types.SetLevelRequest`  ·  defined at `mcp_types._types.SetLevelRequest`

```python
class SetLevelRequest(Request[SetLevelRequestParams, Literal['logging/setLevel']])
```

**Also exported as** `mcp.SetLevelRequest`, `mcp_types.SetLevelRequest`

**Bases** `Request[SetLevelRequestParams, Literal['logging/setLevel']]`

**Declared members (2)**

- `method: Literal['logging/setLevel'] = 'logging/setLevel'`  _class-attribute, instance-attribute_
- `params: SetLevelRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A request from the client to the server, to enable or adjust logging.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.
On 2026-07-28 the client opts in per-request via `_meta` (`LOG_LEVEL_META_KEY`).


## SetLevelRequestParams

Import as `mcp_types.SetLevelRequestParams`  ·  defined at `mcp_types._types.SetLevelRequestParams`

```python
class SetLevelRequestParams(RequestParams)
```

**Also exported as** `mcp_types.SetLevelRequestParams`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (1)**

- `level: LoggingLevel`  _instance-attribute_
  The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (more severe).

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for setting the logging level.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.


## SubscribeRequest

Import as `mcp_types.SubscribeRequest`  ·  defined at `mcp_types._types.SubscribeRequest`

```python
class SubscribeRequest(Request[SubscribeRequestParams, Literal['resources/subscribe']])
```

**Also exported as** `mcp.SubscribeRequest`, `mcp_types.SubscribeRequest`

**Bases** `Request[SubscribeRequestParams, Literal['resources/subscribe']]`

**Declared members (2)**

- `method: Literal['resources/subscribe'] = 'resources/subscribe'`  _class-attribute, instance-attribute_
- `params: SubscribeRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Sent from the client to request resources/updated notifications from the server
whenever a particular resource changes.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.
On 2026-07-28 use `subscriptions/listen` instead.


## SubscribeRequestParams

Import as `mcp_types.SubscribeRequestParams`  ·  defined at `mcp_types._types.SubscribeRequestParams`

```python
class SubscribeRequestParams(RequestParams)
```

**Also exported as** `mcp_types.SubscribeRequestParams`

**Bases** `RequestParams`

**Declared members (1)**

- `uri: str`  _instance-attribute_
  The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for subscribing to a resource.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.


## SubscriptionFilter

Import as `mcp_types.SubscriptionFilter`  ·  defined at `mcp_types._types.SubscriptionFilter`

```python
class SubscriptionFilter(MCPModel)
```

**Also exported as** `mcp_types.SubscriptionFilter`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (4)**

- `prompts_list_changed: bool | None = None`  _class-attribute, instance-attribute_
  If true, receive notifications/prompts/list_changed.
- `resource_subscriptions: list[str] | None = None`  _class-attribute, instance-attribute_
  Subscribe to notifications/resources/updated for these resource URIs.
- `resources_list_changed: bool | None = None`  _class-attribute, instance-attribute_
  If true, receive notifications/resources/list_changed.
- `tools_list_changed: bool | None = None`  _class-attribute, instance-attribute_
  If true, receive notifications/tools/list_changed.

The set of notification types a client opts in to via `subscriptions/listen` (2026-07-28).

Each type is opt-in; the server MUST NOT send types not requested here.
Echoed back in `notifications/subscriptions/acknowledged` as the subset the
server agreed to honor. Extensions merge additional keys (e.g. `taskIds`),
so unknown keys round-trip.


## SubscriptionsAcknowledgedNotification

Import as `mcp_types.SubscriptionsAcknowledgedNotification`  ·  defined at `mcp_types._types.SubscriptionsAcknowledgedNotification`

```python
class SubscriptionsAcknowledgedNotification(Notification[SubscriptionsAcknowledgedNotificationParams, Literal['notifications/subscriptions/acknowledged']])
```

**Also exported as** `mcp_types.SubscriptionsAcknowledgedNotification`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Notification[SubscriptionsAcknowledgedNotificationParams, Literal['notifications/subscriptions/acknowledged']]`

**Declared members (2)**

- `method: Literal['notifications/subscriptions/acknowledged'] = 'notifications/subscriptions/acknowledged'`  _class-attribute, instance-attribute_
- `params: SubscriptionsAcknowledgedNotificationParams`  _instance-attribute_

First message on a `subscriptions/listen` stream: acknowledges the
subscription and reports which notification types the server will honor (2026-07-28).


## SubscriptionsAcknowledgedNotificationParams

Import as `mcp_types.SubscriptionsAcknowledgedNotificationParams`  ·  defined at `mcp_types._types.SubscriptionsAcknowledgedNotificationParams`

```python
class SubscriptionsAcknowledgedNotificationParams(NotificationParams)
```

**Also exported as** `mcp_types.SubscriptionsAcknowledgedNotificationParams`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NotificationParams`

**Declared members (1)**

- `notifications: SubscriptionFilter`  _instance-attribute_
  The subset of requested notification types the server agreed to honor. Unsupported types are omitted.

**Inherited (1)**

- from `mcp_types._types.NotificationParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## SubscriptionsListenRequest

Import as `mcp_types.SubscriptionsListenRequest`  ·  defined at `mcp_types._types.SubscriptionsListenRequest`

```python
class SubscriptionsListenRequest(Request[SubscriptionsListenRequestParams, Literal['subscriptions/listen']])
```

**Also exported as** `mcp_types.SubscriptionsListenRequest`

**Bases** `Request[SubscriptionsListenRequestParams, Literal['subscriptions/listen']]`

**Declared members (2)**

- `method: Literal['subscriptions/listen'] = 'subscriptions/listen'`  _class-attribute, instance-attribute_
- `params: SubscriptionsListenRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Opens a long-lived channel for receiving notifications outside the context
of a specific request (2026-07-28).


## SubscriptionsListenRequestParams

Import as `mcp_types.SubscriptionsListenRequestParams`  ·  defined at `mcp_types._types.SubscriptionsListenRequestParams`

```python
class SubscriptionsListenRequestParams(RequestParams)
```

**Also exported as** `mcp_types.SubscriptionsListenRequestParams`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RequestParams`

**Declared members (1)**

- `notifications: SubscriptionFilter`  _instance-attribute_
  The notifications the client opts in to on this stream.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## SubscriptionsListenResult

Import as `mcp_types.SubscriptionsListenResult`  ·  defined at `mcp_types._types.SubscriptionsListenResult`

```python
class SubscriptionsListenResult(Result)
```

**Also exported as** `mcp_types.SubscriptionsListenResult`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Result`

**Declared members (1)**

- `result_type: ResultType = 'complete'`  _class-attribute, instance-attribute_
  See `ResultType`. Always serialized; older peers ignore it.

**Inherited (1)**

- from `mcp_types._types.Result`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Signals that a `subscriptions/listen` stream has ended gracefully (2026-07-28).

Because the listen stream is long-lived, this result is sent only when the
server tears the subscription down (for example during shutdown); an abrupt
transport close carries no response. The body is otherwise empty: the
`_meta["io.modelcontextprotocol/subscriptionId"]` key is required on the
wire and equals the JSON-RPC id of the originating `subscriptions/listen`
request.


## Task

Import as `mcp_types.Task`  ·  defined at `mcp_types._types.Task`

```python
class Task(MCPModel)
```

**Also exported as** `mcp_types.Task`

**Bases** `MCPModel`

**Declared members (7)**

- `created_at: str`  _instance-attribute_
  ISO 8601 timestamp when the task was created.
- `last_updated_at: str`  _instance-attribute_
  ISO 8601 timestamp when the task was last updated.
- `poll_interval: int | None = None`  _class-attribute, instance-attribute_
  Suggested polling interval in milliseconds.
- `status: TaskStatus`  _instance-attribute_
- `status_message: str | None = None`  _class-attribute, instance-attribute_
  Optional human-readable message describing the current task state.
- `task_id: str`  _instance-attribute_
- `ttl: int | None`  _instance-attribute_
  Actual retention duration from creation in milliseconds, null for unlimited.

Data associated with a task (2025-11-25 only).


## TaskMetadata

Import as `mcp_types.TaskMetadata`  ·  defined at `mcp_types._types.TaskMetadata`

```python
class TaskMetadata(MCPModel)
```

**Also exported as** `mcp_types.TaskMetadata`

**Bases** `MCPModel`

**Declared members (1)**

- `ttl: int | None = None`  _class-attribute, instance-attribute_
  Requested duration in milliseconds to retain task from creation.

Metadata for augmenting a request with task execution (the `task` params field; 2025-11-25 only).


## TaskStatusNotification

Import as `mcp_types.TaskStatusNotification`  ·  defined at `mcp_types._types.TaskStatusNotification`

```python
class TaskStatusNotification(Notification[TaskStatusNotificationParams, Literal['notifications/tasks/status']])
```

**Also exported as** `mcp_types.TaskStatusNotification`

**Bases** `Notification[TaskStatusNotificationParams, Literal['notifications/tasks/status']]`

**Declared members (2)**

- `method: Literal['notifications/tasks/status'] = 'notifications/tasks/status'`  _class-attribute, instance-attribute_
- `params: TaskStatusNotificationParams`  _instance-attribute_

An optional notification informing the requestor that a task's status has changed (2025-11-25 only).


## TaskStatusNotificationParams

Import as `mcp_types.TaskStatusNotificationParams`  ·  defined at `mcp_types._types.TaskStatusNotificationParams`

```python
class TaskStatusNotificationParams(NotificationParams, Task)
```

**Also exported as** `mcp_types.TaskStatusNotificationParams`

**Bases** `NotificationParams`, `Task`

**Inherited (8)**

- from `mcp_types._types.NotificationParams`: `meta`
- from `mcp_types._types.Task`: `created_at`, `last_updated_at`, `poll_interval`, `status`, `status_message`, `task_id`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for a `notifications/tasks/status` notification.


## TasksCallCapability

Import as `mcp_types.TasksCallCapability`  ·  defined at `mcp_types._types.TasksCallCapability`

```python
class TasksCallCapability(MCPModel)
```

**Also exported as** `mcp_types.TasksCallCapability`

**Bases** `MCPModel`

Capability for task-augmented tools/call requests (2025-11-25 only).


## TasksCancelCapability

Import as `mcp_types.TasksCancelCapability`  ·  defined at `mcp_types._types.TasksCancelCapability`

```python
class TasksCancelCapability(MCPModel)
```

**Also exported as** `mcp_types.TasksCancelCapability`

**Bases** `MCPModel`

Capability for tasks cancel operations (2025-11-25 only).


## TasksCreateElicitationCapability

Import as `mcp_types.TasksCreateElicitationCapability`  ·  defined at `mcp_types._types.TasksCreateElicitationCapability`

```python
class TasksCreateElicitationCapability(MCPModel)
```

**Also exported as** `mcp_types.TasksCreateElicitationCapability`

**Bases** `MCPModel`

Capability for task-augmented elicitation/create requests (2025-11-25 only).


## TasksCreateMessageCapability

Import as `mcp_types.TasksCreateMessageCapability`  ·  defined at `mcp_types._types.TasksCreateMessageCapability`

```python
class TasksCreateMessageCapability(MCPModel)
```

**Also exported as** `mcp_types.TasksCreateMessageCapability`

**Bases** `MCPModel`

Capability for task-augmented sampling/createMessage requests (2025-11-25 only).


## TasksElicitationCapability

Import as `mcp_types.TasksElicitationCapability`  ·  defined at `mcp_types._types.TasksElicitationCapability`

```python
class TasksElicitationCapability(MCPModel)
```

**Also exported as** `mcp_types.TasksElicitationCapability`

**Bases** `MCPModel`

**Declared members (1)**

- `create: TasksCreateElicitationCapability | None = None`  _class-attribute, instance-attribute_

Capability for task-augmented elicitation operations (2025-11-25 only).


## TasksListCapability

Import as `mcp_types.TasksListCapability`  ·  defined at `mcp_types._types.TasksListCapability`

```python
class TasksListCapability(MCPModel)
```

**Also exported as** `mcp_types.TasksListCapability`

**Bases** `MCPModel`

Capability for tasks listing operations (2025-11-25 only).


## TasksSamplingCapability

Import as `mcp_types.TasksSamplingCapability`  ·  defined at `mcp_types._types.TasksSamplingCapability`

```python
class TasksSamplingCapability(MCPModel)
```

**Also exported as** `mcp_types.TasksSamplingCapability`

**Bases** `MCPModel`

**Declared members (1)**

- `create_message: TasksCreateMessageCapability | None = None`  _class-attribute, instance-attribute_

Capability for task-augmented sampling operations (2025-11-25 only).


## TasksToolsCapability

Import as `mcp_types.TasksToolsCapability`  ·  defined at `mcp_types._types.TasksToolsCapability`

```python
class TasksToolsCapability(MCPModel)
```

**Also exported as** `mcp_types.TasksToolsCapability`

**Bases** `MCPModel`

**Declared members (1)**

- `call: TasksCallCapability | None = None`  _class-attribute, instance-attribute_

Capability for task-augmented tool operations (2025-11-25 only).


## TextContent

Import as `mcp_types.TextContent`  ·  defined at `mcp_types._types.TextContent`

```python
class TextContent(MCPModel)
```

**Also exported as** `mcp_types.TextContent`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (4)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See [MCP specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/47339c03c143bb4ec01a26e721a1b8fe66634ebe/docs/specification/draft/basic/index.mdx#general-fields) for notes on _meta usage.
- `text: str`  _instance-attribute_
  The text content of the message.
- `type: Literal['text'] = 'text'`  _class-attribute, instance-attribute_

Text provided to or from an LLM.


## TextResourceContents

Import as `mcp_types.TextResourceContents`  ·  defined at `mcp_types._types.TextResourceContents`

```python
class TextResourceContents(ResourceContents)
```

**Also exported as** `mcp_types.TextResourceContents`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ResourceContents`

**Declared members (1)**

- `text: str`  _instance-attribute_
  The text of the item. This must only be set if the item can actually be represented as text (not binary data).

**Inherited (3)**

- from `mcp_types._types.ResourceContents`: `meta`, `mime_type`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Text contents of a resource.


## Tool

Import as `mcp_types.Tool`  ·  defined at `mcp_types._types.Tool`

```python
class Tool(BaseMetadata)
```

**Also exported as** `fastmcp.tools.base.MCPTool`, `mcp.Tool`, `mcp_types.Tool`

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseMetadata`

**Declared members (7)**

- `annotations: ToolAnnotations | None = None`  _class-attribute, instance-attribute_
  Optional additional tool information. Display-name precedence: `title`, `annotations.title`, then `name`.
- `description: str | None = None`  _class-attribute, instance-attribute_
  A human-readable description of the tool.
- `execution: ToolExecution | None = None`  _class-attribute, instance-attribute_
  Execution-related properties (2025-11-25 only; removed in 2026-07-28).
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons for display (2025-11-25+).
- `input_schema: dict[str, Any]`  _instance-attribute_
  A JSON Schema object defining the expected parameters for the tool.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  See the MCP specification for notes on `_meta` usage.
- `output_schema: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  An optional JSON Schema object defining the structure of the tool's output returned in the `structured_content` field of a `CallToolResult`.

**Inherited (2)**

- from `mcp_types._types.BaseMetadata`: `name`, `title`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Definition for a tool the client can call.


## ToolAnnotations

Import as `mcp_types.ToolAnnotations`  ·  defined at `mcp_types._types.ToolAnnotations`

```python
class ToolAnnotations(MCPModel)
```

**Also exported as** `mcp_types.ToolAnnotations`

_11 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (5)**

- `destructive_hint: bool | None = None`  _class-attribute, instance-attribute_
  If true, the tool may perform destructive updates to its environment. If false, the tool performs only additive updates. (This property is meaningful only when `read_only_hint == false`) Default: true
- `idempotent_hint: bool | None = None`  _class-attribute, instance-attribute_
  If true, calling the tool repeatedly with the same arguments will have no additional effect on its environment. (This property is meaningful only when `read_only_hint == false`) Default: false
- `open_world_hint: bool | None = None`  _class-attribute, instance-attribute_
  If true, this tool may interact with an "open world" of external entities. If false, the tool's domain of interaction is closed. For example, the world of a web search tool is open, whereas that of a memory tool is not. Default: true
- `read_only_hint: bool | None = None`  _class-attribute, instance-attribute_
  If true, the tool does not modify its environment. Default: false
- `title: str | None = None`  _class-attribute, instance-attribute_
  A human-readable title for the tool.

Additional properties describing a Tool to clients.

NOTE: all properties in ToolAnnotations are **hints**.
They are not guaranteed to provide a faithful description of
tool behavior (including descriptive properties like `title`).

Clients should never make tool use decisions based on ToolAnnotations
received from untrusted servers.


## ToolChoice

Import as `mcp_types.ToolChoice`  ·  defined at `mcp_types._types.ToolChoice`

```python
class ToolChoice(MCPModel)
```

**Also exported as** `mcp.ToolChoice`, `mcp_types.ToolChoice`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (1)**

- `mode: Literal['auto', 'required', 'none'] | None = None`  _class-attribute, instance-attribute_
  Controls the tool use ability of the model: - "auto": Model decides whether to use tools (default) - "required": Model MUST use at least one tool before completing - "none": Model MUST NOT use any tools

Controls tool selection behavior for sampling requests (2025-11-25+).

The client MUST return an error if this is received without the
`sampling.tools` capability. Absent means `{"mode": "auto"}`.


## ToolExecution

Import as `mcp_types.ToolExecution`  ·  defined at `mcp_types._types.ToolExecution`

```python
class ToolExecution(MCPModel)
```

**Also exported as** `mcp_types.ToolExecution`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (1)**

- `task_support: Literal['forbidden', 'optional', 'required'] | None = None`  _class-attribute, instance-attribute_
  Whether this tool supports task-augmented execution. Absent means "forbidden".

Execution-related properties for a tool (2025-11-25 only).


## ToolListChangedNotification

Import as `mcp_types.ToolListChangedNotification`  ·  defined at `mcp_types._types.ToolListChangedNotification`

```python
class ToolListChangedNotification(Notification[NotificationParams | None, Literal['notifications/tools/list_changed']])
```

**Also exported as** `mcp_types.ToolListChangedNotification`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Notification[NotificationParams | None, Literal['notifications/tools/list_changed']]`

**Declared members (2)**

- `method: Literal['notifications/tools/list_changed'] = 'notifications/tools/list_changed'`  _class-attribute, instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list
of tools it offers has changed.

May be sent spontaneously through 2025-11-25; on 2026-07-28 sessions the
client must opt in via `subscriptions/listen`.


## ToolResultContent

Import as `mcp_types.ToolResultContent`  ·  defined at `mcp_types._types.ToolResultContent`

```python
class ToolResultContent(MCPModel)
```

**Also exported as** `mcp.ToolResultContent`, `mcp_types.ToolResultContent`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (6)**

- `content: list[ContentBlock] = []`  _class-attribute, instance-attribute_
  The unstructured result content (same format as `CallToolResult.content`).
- `is_error: bool | None = None`  _class-attribute, instance-attribute_
  Whether the tool use resulted in an error. Absent is equivalent to false.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  Optional metadata. Clients SHOULD preserve this in subsequent sampling requests to enable caching optimizations.
- `structured_content: Any = None`  _class-attribute, instance-attribute_
  An optional structured result value. Any JSON value on 2026-07-28; restricted to a JSON object on 2025-11-25.
- `tool_use_id: str`  _instance-attribute_
  The `id` of the `ToolUseContent` this result corresponds to.
- `type: Literal['tool_result'] = 'tool_result'`  _class-attribute, instance-attribute_
  Discriminator for tool result content.

The result of a tool use, provided by the user back to the assistant (2025-11-25+).

Appears in sampling messages as a response to a `ToolUseContent` block.
Requires the `sampling.tools` client capability. Deprecated in 2026-07-28 (SEP-2577).


## ToolUseContent

Import as `mcp_types.ToolUseContent`  ·  defined at `mcp_types._types.ToolUseContent`

```python
class ToolUseContent(MCPModel)
```

**Also exported as** `mcp.ToolUseContent`, `mcp_types.ToolUseContent`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (5)**

- `id: str`  _instance-attribute_
  Unique identifier for this tool call, used to correlate with ToolResultContent.
- `input: dict[str, Any]`  _instance-attribute_
  Arguments to pass to the tool. Must conform to the tool's inputSchema.
- `meta: Meta | None = Field(alias='_meta', default=None)`  _class-attribute, instance-attribute_
  Optional metadata. Clients SHOULD preserve this in subsequent sampling requests to enable caching optimizations.
- `name: str`  _instance-attribute_
  The name of the tool to invoke. Must match a tool name from the request's tools array.
- `type: Literal['tool_use'] = 'tool_use'`  _class-attribute, instance-attribute_
  Discriminator for tool use content.

An assistant's request to invoke a tool during sampling (2025-11-25+).

Appears in `sampling/createMessage` results and replayed assistant messages.
The server should execute the tool and return a `ToolResultContent` in the
next user message. Deprecated in 2026-07-28 (SEP-2577).


## ToolsCapability

Import as `mcp_types.ToolsCapability`  ·  defined at `mcp_types._types.ToolsCapability`

```python
class ToolsCapability(MCPModel)
```

**Also exported as** `mcp.ToolsCapability`, `mcp_types.ToolsCapability`

**Bases** `MCPModel`

**Declared members (1)**

- `list_changed: bool | None = None`  _class-attribute, instance-attribute_
  Whether this server supports notifications for changes to the tool list.

Capability for tools operations.


## UnsubscribeRequest

Import as `mcp_types.UnsubscribeRequest`  ·  defined at `mcp_types._types.UnsubscribeRequest`

```python
class UnsubscribeRequest(Request[UnsubscribeRequestParams, Literal['resources/unsubscribe']])
```

**Also exported as** `mcp.UnsubscribeRequest`, `mcp_types.UnsubscribeRequest`

**Bases** `Request[UnsubscribeRequestParams, Literal['resources/unsubscribe']]`

**Declared members (2)**

- `method: Literal['resources/unsubscribe'] = 'resources/unsubscribe'`  _class-attribute, instance-attribute_
- `params: UnsubscribeRequestParams`  _instance-attribute_

**Inherited (1)**

- from `mcp_types._types.Request`: `name_param`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Sent from the client to request cancellation of resources/updated notifications
from the server. This should follow a previous resources/subscribe request.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.
On 2026-07-28 use `subscriptions/listen` instead.


## UnsubscribeRequestParams

Import as `mcp_types.UnsubscribeRequestParams`  ·  defined at `mcp_types._types.UnsubscribeRequestParams`

```python
class UnsubscribeRequestParams(RequestParams)
```

**Also exported as** `mcp_types.UnsubscribeRequestParams`

**Bases** `RequestParams`

**Declared members (1)**

- `uri: str`  _instance-attribute_
  The URI of the resource to unsubscribe from.

**Inherited (1)**

- from `mcp_types._types.RequestParams`: `meta`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for a resources/unsubscribe request.

Removed in protocol 2026-07-28; sent/received on sessions negotiating <= 2025-11-25.


## UnsupportedProtocolVersionErrorData

Import as `mcp_types.UnsupportedProtocolVersionErrorData`  ·  defined at `mcp_types._types.UnsupportedProtocolVersionErrorData`

```python
class UnsupportedProtocolVersionErrorData(MCPModel)
```

**Also exported as** `mcp_types.UnsupportedProtocolVersionErrorData`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPModel`

**Declared members (2)**

- `requested: str`  _instance-attribute_
- `supported: list[str]`  _instance-attribute_
  Protocol versions the server supports; the client should pick one and retry.

Error data for the -32022 unsupported-protocol-version error (2026-07-28).


## UrlElicitationCapability

Import as `mcp_types.UrlElicitationCapability`  ·  defined at `mcp_types._types.UrlElicitationCapability`

```python
class UrlElicitationCapability(MCPModel)
```

**Also exported as** `mcp_types.UrlElicitationCapability`

**Bases** `MCPModel`

Capability for URL mode elicitation (2025-11-25+).


