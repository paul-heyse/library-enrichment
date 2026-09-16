# `mcp_types._v2025_11_25`

Distribution: `mcp-types`

## ClientNotification

`mcp_types._v2025_11_25.ClientNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ClientNotification = CancelledNotification | InitializedNotification | ProgressNotification | TaskStatusNotification | RootsListChangedNotification
```

## ClientRequest

`mcp_types._v2025_11_25.ClientRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ClientRequest = InitializeRequest | PingRequest | ListResourcesRequest | ListResourceTemplatesRequest | ReadResourceRequest | SubscribeRequest | UnsubscribeRequest | ListPromptsRequest | GetPromptRequest | ListToolsRequest | CallToolRequest | GetTaskRequest | GetTaskPayloadRequest | CancelTaskRequest | ListTasksRequest | SetLevelRequest | CompleteRequest
```

## ClientResult

`mcp_types._v2025_11_25.ClientResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ClientResult = Result | GetTaskResult | GetTaskPayloadResult | CancelTaskResult | ListTasksResult | CreateMessageResult | ListRootsResult | ElicitResult
```

## ContentBlock

`mcp_types._v2025_11_25.ContentBlock`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ContentBlock = TextContent | ImageContent | AudioContent | ResourceLink | EmbeddedResource
```

## Cursor

`mcp_types._v2025_11_25.Cursor`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
Cursor = str
```

An opaque token used to represent a cursor for pagination.


## ElicitRequestParams

`mcp_types._v2025_11_25.ElicitRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ElicitRequestParams = ElicitRequestURLParams | ElicitRequestFormParams
```

The parameters for a request to elicit additional information from the user via the client.


## EmptyResult

`mcp_types._v2025_11_25.EmptyResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
EmptyResult = Result
```

## EnumSchema

`mcp_types._v2025_11_25.EnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
EnumSchema = UntitledSingleSelectEnumSchema | TitledSingleSelectEnumSchema | UntitledMultiSelectEnumSchema | TitledMultiSelectEnumSchema | LegacyTitledEnumSchema
```

## JSONRPCMessage

`mcp_types._v2025_11_25.JSONRPCMessage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
JSONRPCMessage = JSONRPCRequest | JSONRPCNotification | JSONRPCResultResponse | JSONRPCErrorResponse
```

Refers to any valid JSON-RPC object that can be decoded off the wire, or encoded to be sent.


## JSONRPCResponse

`mcp_types._v2025_11_25.JSONRPCResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
JSONRPCResponse = JSONRPCResultResponse | JSONRPCErrorResponse
```

A response to a request, containing either the result or error.


## LoggingLevel

`mcp_types._v2025_11_25.LoggingLevel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
LoggingLevel = Literal['alert', 'critical', 'debug', 'emergency', 'error', 'info', 'notice', 'warning']
```

The severity of a log message.

These map to syslog message severities, as specified in RFC-5424:
https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1


## MultiSelectEnumSchema

`mcp_types._v2025_11_25.MultiSelectEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
MultiSelectEnumSchema = UntitledMultiSelectEnumSchema | TitledMultiSelectEnumSchema
```

## PrimitiveSchemaDefinition

Import as `mcp.server.elicitation.PrimitiveSchemaDefinition`  ·  defined at `mcp_types._v2025_11_25.PrimitiveSchemaDefinition`

```python
PrimitiveSchemaDefinition = StringSchema | NumberSchema | BooleanSchema | UntitledSingleSelectEnumSchema | TitledSingleSelectEnumSchema | UntitledMultiSelectEnumSchema | TitledMultiSelectEnumSchema | LegacyTitledEnumSchema
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Restricted schema definitions that only allow primitive types
without nested objects or arrays.


## ProgressToken

`mcp_types._v2025_11_25.ProgressToken`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ProgressToken = str | int
```

A progress token, used to associate progress notifications with the original request.


## RequestId

`mcp_types._v2025_11_25.RequestId`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
RequestId = str | int
```

A uniquely identifying ID for a request in JSON-RPC.


## Role

`mcp_types._v2025_11_25.Role`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
Role = Literal['assistant', 'user']
```

The sender or recipient of messages and data in a conversation.


## SamplingMessageContentBlock

`mcp_types._v2025_11_25.SamplingMessageContentBlock`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
SamplingMessageContentBlock = TextContent | ImageContent | AudioContent | ToolUseContent | ToolResultContent
```

## ServerNotification

`mcp_types._v2025_11_25.ServerNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ServerNotification = CancelledNotification | ProgressNotification | ResourceListChangedNotification | ResourceUpdatedNotification | PromptListChangedNotification | ToolListChangedNotification | TaskStatusNotification | LoggingMessageNotification | ElicitationCompleteNotification
```

## ServerRequest

`mcp_types._v2025_11_25.ServerRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ServerRequest = PingRequest | GetTaskRequest | GetTaskPayloadRequest | CancelTaskRequest | ListTasksRequest | CreateMessageRequest | ListRootsRequest | ElicitRequest
```

## ServerResult

`mcp_types._v2025_11_25.ServerResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ServerResult = Result | InitializeResult | ListResourcesResult | ListResourceTemplatesResult | ReadResourceResult | ListPromptsResult | GetPromptResult | ListToolsResult | CallToolResult | GetTaskResult | GetTaskPayloadResult | CancelTaskResult | ListTasksResult | CompleteResult
```

## SingleSelectEnumSchema

`mcp_types._v2025_11_25.SingleSelectEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
SingleSelectEnumSchema = UntitledSingleSelectEnumSchema | TitledSingleSelectEnumSchema
```

## TaskStatus

`mcp_types._v2025_11_25.TaskStatus`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
TaskStatus = Literal['cancelled', 'completed', 'failed', 'input_required', 'working']
```

The status of a task.


## Annotations

`mcp_types._v2025_11_25.Annotations`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Annotations(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `audience: list[Role] | None = None`  _class-attribute, instance-attribute_
  Describes who the intended audience of this object or data is.
- `last_modified: Annotated[str | None, Field(alias='lastModified')] = None`  _class-attribute, instance-attribute_
  The moment the resource was last modified, as an ISO 8601 formatted string.
- `priority: Annotated[float | None, Field(ge=0.0, le=1.0)] = None`  _class-attribute, instance-attribute_
  Describes how important this data is for operating the server.

Optional annotations for the client. The client can use annotations to inform how objects are used or displayed


## AnyOfItem

`mcp_types._v2025_11_25.AnyOfItem`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class AnyOfItem(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `const: str`  _instance-attribute_
  The constant enum value.
- `title: str`  _instance-attribute_
  Display title for this option.

## Argument

`mcp_types._v2025_11_25.Argument`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Argument(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `name: str`  _instance-attribute_
  The name of the argument
- `value: str`  _instance-attribute_
  The value of the argument to use for completion matching.

The argument's information


## AudioContent

`mcp_types._v2025_11_25.AudioContent`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class AudioContent(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `data: str`  _instance-attribute_
  The base64-encoded audio data.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mime_type: Annotated[str, Field(alias='mimeType')]`  _instance-attribute_
  The MIME type of the audio. Different providers may support different audio types.
- `type: Literal['audio']`  _instance-attribute_

Audio provided to or from an LLM.


## BaseMetadata

`mcp_types._v2025_11_25.BaseMetadata`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class BaseMetadata(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.

Base interface for metadata with name (identifier) and title (display name) properties.


## BlobResourceContents

`mcp_types._v2025_11_25.BlobResourceContents`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class BlobResourceContents(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `blob: str`  _instance-attribute_
  A base64-encoded string representing the binary data of the item.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `uri: str`  _instance-attribute_
  The URI of this resource.

## BooleanSchema

`mcp_types._v2025_11_25.BooleanSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class BooleanSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `default: bool | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_
- `type: Literal['boolean']`  _instance-attribute_

## CallToolRequest

`mcp_types._v2025_11_25.CallToolRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CallToolRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['tools/call']`  _instance-attribute_
- `params: CallToolRequestParams`  _instance-attribute_

Used by the client to invoke a tool provided by the server.


## CallToolRequestParams

`mcp_types._v2025_11_25.CallToolRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CallToolRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `arguments: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Arguments to use for the tool call.
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `name: str`  _instance-attribute_
  The name of the tool.
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller is requesting task-augmented execution for this request. The request will return a CreateTaskResult immediately, and the actual result can be retrieved later via tasks/result.

Parameters for a `tools/call` request.


## CallToolResult

`mcp_types._v2025_11_25.CallToolResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CallToolResult(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `content: list[ContentBlock]`  _instance-attribute_
  A list of content objects that represent the unstructured result of the tool call.
- `is_error: Annotated[bool | None, Field(alias='isError')] = None`  _class-attribute, instance-attribute_
  Whether the tool call ended in an error.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `structured_content: Annotated[dict[str, Any] | None, Field(alias='structuredContent')] = None`  _class-attribute, instance-attribute_
  An optional JSON object that represents the structured result of the tool call.

The server's response to a tool call.


## CancelTaskRequest

`mcp_types._v2025_11_25.CancelTaskRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CancelTaskRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['tasks/cancel']`  _instance-attribute_
- `params: Params`  _instance-attribute_

A request to cancel a task.


## CancelTaskResult

`mcp_types._v2025_11_25.CancelTaskResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CancelTaskResult(Result, Task)
```

**Bases** `Result`, `Task`

**Inherited (8)**

- from `mcp_types._v2025_11_25.Result`: `meta`
- from `mcp_types._v2025_11_25.Task`: `created_at`, `last_updated_at`, `poll_interval`, `status`, `status_message`, `task_id`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The response to a tasks/cancel request.


## CancelledNotification

`mcp_types._v2025_11_25.CancelledNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CancelledNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/cancelled']`  _instance-attribute_
- `params: CancelledNotificationParams`  _instance-attribute_

This notification can be sent by either side to indicate that it is cancelling a previously-issued request.

The request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.

This notification indicates that the result will be unused, so any associated processing SHOULD cease.

A client MUST NOT attempt to cancel its `initialize` request.

For task cancellation, use the `tasks/cancel` request instead of this notification.


## CancelledNotificationParams

`mcp_types._v2025_11_25.CancelledNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CancelledNotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `reason: str | None = None`  _class-attribute, instance-attribute_
  An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.
- `request_id: Annotated[RequestId | None, Field(alias='requestId')] = None`  _class-attribute, instance-attribute_
  The ID of the request to cancel.

Parameters for a `notifications/cancelled` notification.


## ClientCapabilities

`mcp_types._v2025_11_25.ClientCapabilities`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ClientCapabilities(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `elicitation: Elicitation | None = None`  _class-attribute, instance-attribute_
  Present if the client supports elicitation from the server.
- `experimental: dict[str, dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
  Experimental, non-standard capabilities that the client supports.
- `roots: Roots | None = None`  _class-attribute, instance-attribute_
  Present if the client supports listing roots.
- `sampling: Sampling | None = None`  _class-attribute, instance-attribute_
  Present if the client supports sampling from an LLM.
- `tasks: Tasks | None = None`  _class-attribute, instance-attribute_
  Present if the client supports task-augmented requests.

Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities.


## CompleteRequest

`mcp_types._v2025_11_25.CompleteRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CompleteRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['completion/complete']`  _instance-attribute_
- `params: CompleteRequestParams`  _instance-attribute_

A request from the client to the server, to ask for completion options.


## CompleteRequestParams

`mcp_types._v2025_11_25.CompleteRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CompleteRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `argument: Argument`  _instance-attribute_
  The argument's information
- `context: Context | None = None`  _class-attribute, instance-attribute_
  Additional, optional context for completions
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `ref: PromptReference | ResourceTemplateReference`  _instance-attribute_

Parameters for a `completion/complete` request.


## CompleteResult

`mcp_types._v2025_11_25.CompleteResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CompleteResult(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `completion: Completion`  _instance-attribute_
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

The server's response to a completion/complete request


## Completion

`mcp_types._v2025_11_25.Completion`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Completion(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `has_more: Annotated[bool | None, Field(alias='hasMore')] = None`  _class-attribute, instance-attribute_
  Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown.
- `total: int | None = None`  _class-attribute, instance-attribute_
  The total number of completion options available. This can exceed the number of values actually sent in the response.
- `values: list[str]`  _instance-attribute_
  An array of completion values. Must not exceed 100 items.

## Context

`mcp_types._v2025_11_25.Context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Context(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `arguments: dict[str, str] | None = None`  _class-attribute, instance-attribute_
  Previously-resolved variables in a URI template or prompt.

Additional, optional context for completions


## CreateMessageRequest

`mcp_types._v2025_11_25.CreateMessageRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CreateMessageRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['sampling/createMessage']`  _instance-attribute_
- `params: CreateMessageRequestParams`  _instance-attribute_

A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it.


## CreateMessageRequestParams

`mcp_types._v2025_11_25.CreateMessageRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CreateMessageRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (12)**

- `include_context: Annotated[Literal['allServers', 'none', 'thisServer'] | None, Field(alias='includeContext')] = None`  _class-attribute, instance-attribute_
  A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.
- `max_tokens: Annotated[int, Field(alias='maxTokens')]`  _instance-attribute_
  The requested maximum number of tokens to sample (to prevent runaway completions).
- `messages: list[SamplingMessage]`  _instance-attribute_
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `metadata: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.
- `model_preferences: Annotated[ModelPreferences | None, Field(alias='modelPreferences')] = None`  _class-attribute, instance-attribute_
  The server's preferences for which model to select. The client MAY ignore these preferences.
- `stop_sequences: Annotated[list[str] | None, Field(alias='stopSequences')] = None`  _class-attribute, instance-attribute_
- `system_prompt: Annotated[str | None, Field(alias='systemPrompt')] = None`  _class-attribute, instance-attribute_
  An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller is requesting task-augmented execution for this request. The request will return a CreateTaskResult immediately, and the actual result can be retrieved later via tasks/result.
- `temperature: float | None = None`  _class-attribute, instance-attribute_
- `tool_choice: Annotated[ToolChoice | None, Field(alias='toolChoice')] = None`  _class-attribute, instance-attribute_
  Controls how the model uses tools. The client MUST return an error if this field is provided but ClientCapabilities.sampling.tools is not declared. Default is `{ mode: "auto" }`.
- `tools: list[Tool] | None = None`  _class-attribute, instance-attribute_
  Tools that the model may use during generation. The client MUST return an error if this field is provided but ClientCapabilities.sampling.tools is not declared.

Parameters for a `sampling/createMessage` request.


## CreateMessageResult

`mcp_types._v2025_11_25.CreateMessageResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CreateMessageResult(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `content: TextContent | ImageContent | AudioContent | ToolUseContent | ToolResultContent | list[SamplingMessageContentBlock]`  _instance-attribute_
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `model: str`  _instance-attribute_
  The name of the model that generated the message.
- `role: Role`  _instance-attribute_
- `stop_reason: Annotated[str | None, Field(alias='stopReason')] = None`  _class-attribute, instance-attribute_
  The reason why sampling stopped, if known.

The client's response to a sampling/createMessage request from the server.
The client should inform the user before returning the sampled message, to allow them
to inspect the response (human in the loop) and decide whether to allow the server to see it.


## CreateTaskResult

`mcp_types._v2025_11_25.CreateTaskResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CreateTaskResult(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `task: Task`  _instance-attribute_

A response to a task-augmented request.


## Data

`mcp_types._v2025_11_25.Data`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Data(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `elicitations: list[ElicitRequestURLParams]`  _instance-attribute_

Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).


## ElicitRequest

`mcp_types._v2025_11_25.ElicitRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['elicitation/create']`  _instance-attribute_
- `params: ElicitRequestParams`  _instance-attribute_

A request from the server to elicit additional information from the user via the client.


## ElicitRequestFormParams

`mcp_types._v2025_11_25.ElicitRequestFormParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitRequestFormParams(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `message: str`  _instance-attribute_
  The message to present to the user describing what information is being requested.
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mode: Literal['form'] = 'form'`  _class-attribute, instance-attribute_
  The elicitation mode.
- `requested_schema: Annotated[RequestedSchema, Field(alias='requestedSchema')]`  _instance-attribute_
  A restricted subset of JSON Schema. Only top-level properties are allowed, without nesting.
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller is requesting task-augmented execution for this request. The request will return a CreateTaskResult immediately, and the actual result can be retrieved later via tasks/result.

The parameters for a request to elicit non-sensitive information from the user via a form in the client.


## ElicitRequestURLParams

`mcp_types._v2025_11_25.ElicitRequestURLParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitRequestURLParams(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `elicitation_id: Annotated[str, Field(alias='elicitationId')]`  _instance-attribute_
  The ID of the elicitation, which must be unique within the context of the server. The client MUST treat this ID as an opaque value.
- `message: str`  _instance-attribute_
  The message to present to the user explaining why the interaction is needed.
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mode: Literal['url']`  _instance-attribute_
  The elicitation mode.
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller is requesting task-augmented execution for this request. The request will return a CreateTaskResult immediately, and the actual result can be retrieved later via tasks/result.
- `url: str`  _instance-attribute_
  The URL that the user should navigate to.

The parameters for a request to elicit information from the user via a URL in the client.


## ElicitResult

`mcp_types._v2025_11_25.ElicitResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `action: Literal['accept', 'cancel', 'decline']`  _instance-attribute_
  The user action in response to the elicitation. - "accept": User submitted the form/confirmed the action - "decline": User explicitly decline the action - "cancel": User dismissed without making an explicit choice
- `content: dict[str, list[str] | str | int | float | bool | None] | None = None`  _class-attribute, instance-attribute_
  The submitted form data, only present when action is "accept" and mode was "form". Contains values matching the requested schema. Omitted for out-of-band mode responses.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

The client's response to an elicitation request.


## Elicitation

`mcp_types._v2025_11_25.Elicitation`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Elicitation(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `form: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `url: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

Present if the client supports elicitation from the server.


## Elicitation1

`mcp_types._v2025_11_25.Elicitation1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Elicitation1(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `create: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Whether the client supports task-augmented elicitation/create requests.

Task support for elicitation-related requests.


## ElicitationCompleteNotification

`mcp_types._v2025_11_25.ElicitationCompleteNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitationCompleteNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/elicitation/complete']`  _instance-attribute_
- `params: Params1`  _instance-attribute_

An optional notification from the server to the client, informing it of a completion of a out-of-band elicitation request.


## EmbeddedResource

`mcp_types._v2025_11_25.EmbeddedResource`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class EmbeddedResource(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `resource: TextResourceContents | BlobResourceContents`  _instance-attribute_
- `type: Literal['resource']`  _instance-attribute_

The contents of a resource, embedded into a prompt or tool call result.

It is up to the client how best to render embedded resources for the benefit
of the LLM and/or the user.


## Error

`mcp_types._v2025_11_25.Error`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Error(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `code: int`  _instance-attribute_
  The error type that occurred.
- `data: Any | None = None`  _class-attribute, instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).
- `message: str`  _instance-attribute_
  A short description of the error. The message SHOULD be limited to a concise single sentence.

## Error1

`mcp_types._v2025_11_25.Error1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Error1(Error)
```

**Bases** `Error`

**Declared members (2)**

- `code: Literal[-32042]`  _instance-attribute_
  The error type that occurred.
- `data: Data`  _instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).

**Inherited (1)**

- from `mcp_types._v2025_11_25.Error`: `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## GetPromptRequest

`mcp_types._v2025_11_25.GetPromptRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetPromptRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['prompts/get']`  _instance-attribute_
- `params: GetPromptRequestParams`  _instance-attribute_

Used by the client to get a prompt provided by the server.


## GetPromptRequestParams

`mcp_types._v2025_11_25.GetPromptRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetPromptRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `arguments: dict[str, str] | None = None`  _class-attribute, instance-attribute_
  Arguments to use for templating the prompt.
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `name: str`  _instance-attribute_
  The name of the prompt or prompt template.

Parameters for a `prompts/get` request.


## GetPromptResult

`mcp_types._v2025_11_25.GetPromptResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetPromptResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `description: str | None = None`  _class-attribute, instance-attribute_
  An optional description for the prompt.
- `messages: list[PromptMessage]`  _instance-attribute_
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

The server's response to a prompts/get request from the client.


## GetTaskPayloadRequest

`mcp_types._v2025_11_25.GetTaskPayloadRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetTaskPayloadRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['tasks/result']`  _instance-attribute_
- `params: Params2`  _instance-attribute_

A request to retrieve the result of a completed task.


## GetTaskPayloadResult

`mcp_types._v2025_11_25.GetTaskPayloadResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetTaskPayloadResult(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

The response to a tasks/result request.
The structure matches the result type of the original request.
For example, a tools/call task would return the CallToolResult structure.


## GetTaskRequest

`mcp_types._v2025_11_25.GetTaskRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetTaskRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['tasks/get']`  _instance-attribute_
- `params: Params3`  _instance-attribute_

A request to retrieve the state of a task.


## GetTaskResult

`mcp_types._v2025_11_25.GetTaskResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetTaskResult(Result, Task)
```

**Bases** `Result`, `Task`

**Inherited (8)**

- from `mcp_types._v2025_11_25.Result`: `meta`
- from `mcp_types._v2025_11_25.Task`: `created_at`, `last_updated_at`, `poll_interval`, `status`, `status_message`, `task_id`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The response to a tasks/get request.


## Icon

`mcp_types._v2025_11_25.Icon`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Icon(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  Optional MIME type override if the source MIME type is missing or generic. For example: `"image/png"`, `"image/jpeg"`, or `"image/svg+xml"`.
- `sizes: list[str] | None = None`  _class-attribute, instance-attribute_
  Optional array of strings that specify sizes at which the icon can be used. Each string should be in WxH format (e.g., `"48x48"`, `"96x96"`) or `"any"` for scalable formats like SVG.
- `src: str`  _instance-attribute_
  A standard URI pointing to an icon resource. May be an HTTP/HTTPS URL or a `data:` URI with Base64-encoded image data.
- `theme: Literal['dark', 'light'] | None = None`  _class-attribute, instance-attribute_
  Optional specifier for the theme this icon is designed for. `light` indicates the icon is designed to be used with a light background, and `dark` indicates the icon is designed to be used with a dark background.

An optionally-sized icon that can be displayed in a user interface.


## Icons

`mcp_types._v2025_11_25.Icons`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Icons(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.

Base interface to add `icons` property.


## ImageContent

`mcp_types._v2025_11_25.ImageContent`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ImageContent(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `data: str`  _instance-attribute_
  The base64-encoded image data.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mime_type: Annotated[str, Field(alias='mimeType')]`  _instance-attribute_
  The MIME type of the image. Different providers may support different image types.
- `type: Literal['image']`  _instance-attribute_

An image provided to or from an LLM.


## Implementation

`mcp_types._v2025_11_25.Implementation`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Implementation(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `description: str | None = None`  _class-attribute, instance-attribute_
  An optional human-readable description of what this implementation does.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.
- `version: str`  _instance-attribute_
- `website_url: Annotated[str | None, Field(alias='websiteUrl')] = None`  _class-attribute, instance-attribute_
  An optional URL of the website for this implementation.

Describes the MCP implementation.


## InitializeRequest

`mcp_types._v2025_11_25.InitializeRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InitializeRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['initialize']`  _instance-attribute_
- `params: InitializeRequestParams`  _instance-attribute_

This request is sent from the client to the server when it first connects, asking it to begin initialization.


## InitializeRequestParams

`mcp_types._v2025_11_25.InitializeRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InitializeRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `capabilities: ClientCapabilities`  _instance-attribute_
- `client_info: Annotated[Implementation, Field(alias='clientInfo')]`  _instance-attribute_
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `protocol_version: Annotated[str, Field(alias='protocolVersion')]`  _instance-attribute_
  The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well.

Parameters for an `initialize` request.


## InitializeResult

`mcp_types._v2025_11_25.InitializeResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InitializeResult(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `capabilities: ServerCapabilities`  _instance-attribute_
- `instructions: str | None = None`  _class-attribute, instance-attribute_
  Instructions describing how to use the server and its features.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `protocol_version: Annotated[str, Field(alias='protocolVersion')]`  _instance-attribute_
  The version of the Model Context Protocol that the server wants to use. This may not match the version that the client requested. If the client cannot support this version, it MUST disconnect.
- `server_info: Annotated[Implementation, Field(alias='serverInfo')]`  _instance-attribute_

After receiving an initialize request from the client, the server sends this response.


## InitializedNotification

`mcp_types._v2025_11_25.InitializedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InitializedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/initialized']`  _instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

This notification is sent from the client to the server after initialization has finished.


## InputSchema

`mcp_types._v2025_11_25.InputSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InputSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `properties: dict[str, dict[str, Any] | bool] | None = None`  _class-attribute, instance-attribute_
- `required: list[str] | None = None`  _class-attribute, instance-attribute_
- `schema_: Annotated[str | None, Field(alias='$schema')] = None`  _class-attribute, instance-attribute_
- `type: Literal['object']`  _instance-attribute_

A JSON Schema object defining the expected parameters for the tool.


## Items

`mcp_types._v2025_11_25.Items`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Items(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `any_of: Annotated[list[AnyOfItem], Field(alias='anyOf')]`  _instance-attribute_
  Array of enum options with values and display labels.

Schema for array items with enum options and display labels.


## Items1

`mcp_types._v2025_11_25.Items1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Items1(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `enum: list[str]`  _instance-attribute_
  Array of enum values to choose from.
- `type: Literal['string']`  _instance-attribute_

Schema for the array items.


## JSONRPCErrorResponse

`mcp_types._v2025_11_25.JSONRPCErrorResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class JSONRPCErrorResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `error: Error`  _instance-attribute_
- `id: RequestId | None = None`  _class-attribute, instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_

A response to a request that indicates an error occurred.


## JSONRPCNotification

`mcp_types._v2025_11_25.JSONRPCNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class JSONRPCNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

A notification which does not expect a response.


## JSONRPCRequest

`mcp_types._v2025_11_25.JSONRPCRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class JSONRPCRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

A request that expects a response.


## JSONRPCResultResponse

`mcp_types._v2025_11_25.JSONRPCResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class JSONRPCResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: Result`  _instance-attribute_

A successful (non-error) response to a request.


## LegacyTitledEnumSchema

`mcp_types._v2025_11_25.LegacyTitledEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class LegacyTitledEnumSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `default: str | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `enum: list[str]`  _instance-attribute_
- `enum_names: Annotated[list[str] | None, Field(alias='enumNames')] = None`  _class-attribute, instance-attribute_
  (Legacy) Display names for enum values. Non-standard according to JSON schema 2020-12.
- `title: str | None = None`  _class-attribute, instance-attribute_
- `type: Literal['string']`  _instance-attribute_

Use TitledSingleSelectEnumSchema instead.
This interface will be removed in a future version.


## ListPromptsRequest

`mcp_types._v2025_11_25.ListPromptsRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListPromptsRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['prompts/list']`  _instance-attribute_
- `params: PaginatedRequestParams | None = None`  _class-attribute, instance-attribute_

Sent from the client to request a list of prompts and prompt templates the server has.


## ListPromptsResult

`mcp_types._v2025_11_25.ListPromptsResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListPromptsResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `prompts: list[Prompt]`  _instance-attribute_

The server's response to a prompts/list request from the client.


## ListResourceTemplatesRequest

`mcp_types._v2025_11_25.ListResourceTemplatesRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourceTemplatesRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['resources/templates/list']`  _instance-attribute_
- `params: PaginatedRequestParams | None = None`  _class-attribute, instance-attribute_

Sent from the client to request a list of resource templates the server has.


## ListResourceTemplatesResult

`mcp_types._v2025_11_25.ListResourceTemplatesResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourceTemplatesResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `resource_templates: Annotated[list[ResourceTemplate], Field(alias='resourceTemplates')]`  _instance-attribute_

The server's response to a resources/templates/list request from the client.


## ListResourcesRequest

`mcp_types._v2025_11_25.ListResourcesRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourcesRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['resources/list']`  _instance-attribute_
- `params: PaginatedRequestParams | None = None`  _class-attribute, instance-attribute_

Sent from the client to request a list of resources the server has.


## ListResourcesResult

`mcp_types._v2025_11_25.ListResourcesResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourcesResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `resources: list[Resource]`  _instance-attribute_

The server's response to a resources/list request from the client.


## ListRootsRequest

`mcp_types._v2025_11_25.ListRootsRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListRootsRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['roots/list']`  _instance-attribute_
- `params: RequestParams | None = None`  _class-attribute, instance-attribute_

Sent from the server to request a list of root URIs from the client. Roots allow
servers to ask for specific directories or files to operate on. A common example
for roots is providing a set of repositories or directories a server should operate
on.

This request is typically used when the server needs to understand the file system
structure or access specific locations that the client has permission to read from.


## ListRootsResult

`mcp_types._v2025_11_25.ListRootsResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListRootsResult(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `roots: list[Root]`  _instance-attribute_

The client's response to a roots/list request from the server.
This result contains an array of Root objects, each representing a root directory
or file that the server can operate on.


## ListTasksRequest

`mcp_types._v2025_11_25.ListTasksRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListTasksRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['tasks/list']`  _instance-attribute_
- `params: PaginatedRequestParams | None = None`  _class-attribute, instance-attribute_

A request to retrieve a list of tasks.


## ListTasksResult

`mcp_types._v2025_11_25.ListTasksResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListTasksResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `tasks: list[Task]`  _instance-attribute_

The response to a tasks/list request.


## ListToolsRequest

`mcp_types._v2025_11_25.ListToolsRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListToolsRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['tools/list']`  _instance-attribute_
- `params: PaginatedRequestParams | None = None`  _class-attribute, instance-attribute_

Sent from the client to request a list of tools the server has.


## ListToolsResult

`mcp_types._v2025_11_25.ListToolsResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListToolsResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `tools: list[Tool]`  _instance-attribute_

The server's response to a tools/list request from the client.


## LoggingMessageNotification

`mcp_types._v2025_11_25.LoggingMessageNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class LoggingMessageNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/message']`  _instance-attribute_
- `params: LoggingMessageNotificationParams`  _instance-attribute_

JSONRPCNotification of a log message passed from server to client. If no logging/setLevel request has been sent from the client, the server MAY decide which messages to send automatically.


## LoggingMessageNotificationParams

`mcp_types._v2025_11_25.LoggingMessageNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class LoggingMessageNotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `data: Any`  _instance-attribute_
  The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.
- `level: LoggingLevel`  _instance-attribute_
  The severity of this log message.
- `logger: str | None = None`  _class-attribute, instance-attribute_
  An optional name of the logger issuing this message.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

Parameters for a `notifications/message` notification.


## Meta

`mcp_types._v2025_11_25.Meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Meta(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `progress_token: Annotated[ProgressToken | None, Field(alias='progressToken')] = None`  _class-attribute, instance-attribute_
  If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. Th…

See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.


## ModelHint

`mcp_types._v2025_11_25.ModelHint`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ModelHint(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `name: str | None = None`  _class-attribute, instance-attribute_
  A hint for a model name.

Hints to use for model selection.

Keys not declared here are currently left unspecified by the spec and are up
to the client to interpret.


## ModelPreferences

`mcp_types._v2025_11_25.ModelPreferences`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ModelPreferences(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `cost_priority: Annotated[float | None, Field(alias='costPriority', ge=0.0, le=1.0)] = None`  _class-attribute, instance-attribute_
  How much to prioritize cost when selecting a model. A value of 0 means cost is not important, while a value of 1 means cost is the most important factor.
- `hints: list[ModelHint] | None = None`  _class-attribute, instance-attribute_
  Optional hints to use for model selection.
- `intelligence_priority: Annotated[float | None, Field(alias='intelligencePriority', ge=0.0, le=1.0)] = None`  _class-attribute, instance-attribute_
  How much to prioritize intelligence and capabilities when selecting a model. A value of 0 means intelligence is not important, while a value of 1 means intelligence is the most important factor.
- `speed_priority: Annotated[float | None, Field(alias='speedPriority', ge=0.0, le=1.0)] = None`  _class-attribute, instance-attribute_
  How much to prioritize sampling speed (latency) when selecting a model. A value of 0 means speed is not important, while a value of 1 means speed is the most important factor.

The server's preferences for model selection, requested of the client during sampling.

Because LLMs can vary along multiple dimensions, choosing the "best" model is
rarely straightforward.  Different models excel in different areas—some are
faster but less capable, others are more capable but more expensive, and so
on. This interface allows servers to express their priorities across multiple
dimensions to help clients make an appropriate selection for their use case.

These preferences are always advisory. The client MAY ignore them. It is also
up to the client to decide how to interpret these preferences and how to
balance them against other considerations.


## Notification

`mcp_types._v2025_11_25.Notification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Notification(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `method: str`  _instance-attribute_
- `params: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

## NotificationParams

`mcp_types._v2025_11_25.NotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class NotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

## NumberSchema

`mcp_types._v2025_11_25.NumberSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class NumberSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `default: int | float | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `maximum: int | float | None = None`  _class-attribute, instance-attribute_
- `minimum: int | float | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_
- `type: Literal['integer', 'number']`  _instance-attribute_

## OneOfItem

`mcp_types._v2025_11_25.OneOfItem`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class OneOfItem(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `const: str`  _instance-attribute_
  The enum value.
- `title: str`  _instance-attribute_
  Display label for this option.

## OutputSchema

`mcp_types._v2025_11_25.OutputSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class OutputSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `properties: dict[str, dict[str, Any] | bool] | None = None`  _class-attribute, instance-attribute_
- `required: list[str] | None = None`  _class-attribute, instance-attribute_
- `schema_: Annotated[str | None, Field(alias='$schema')] = None`  _class-attribute, instance-attribute_
- `type: Literal['object']`  _instance-attribute_

An optional JSON Schema object defining the structure of the tool's output returned in
the structuredContent field of a CallToolResult.

Defaults to JSON Schema 2020-12 when no explicit $schema is provided.
Currently restricted to type: "object" at the root level.


## PaginatedRequest

`mcp_types._v2025_11_25.PaginatedRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PaginatedRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params: PaginatedRequestParams | None = None`  _class-attribute, instance-attribute_

## PaginatedRequestParams

`mcp_types._v2025_11_25.PaginatedRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PaginatedRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `cursor: str | None = None`  _class-attribute, instance-attribute_
  An opaque token representing the current pagination position. If provided, the server should return results starting after this cursor.
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

Common parameters for paginated requests.


## PaginatedResult

`mcp_types._v2025_11_25.PaginatedResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PaginatedResult(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.

## Params

`mcp_types._v2025_11_25.Params`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Params(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `task_id: Annotated[str, Field(alias='taskId')]`  _instance-attribute_
  The task identifier to cancel.

## Params1

`mcp_types._v2025_11_25.Params1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Params1(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `elicitation_id: Annotated[str, Field(alias='elicitationId')]`  _instance-attribute_
  The ID of the elicitation that completed.

## Params2

`mcp_types._v2025_11_25.Params2`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Params2(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `task_id: Annotated[str, Field(alias='taskId')]`  _instance-attribute_
  The task identifier to retrieve results for.

## Params3

`mcp_types._v2025_11_25.Params3`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Params3(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `task_id: Annotated[str, Field(alias='taskId')]`  _instance-attribute_
  The task identifier to query.

## PingRequest

`mcp_types._v2025_11_25.PingRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PingRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['ping']`  _instance-attribute_
- `params: RequestParams | None = None`  _class-attribute, instance-attribute_

A ping, issued by either the server or the client, to check that the other party is still alive. The receiver must promptly respond, or else may be disconnected.


## ProgressNotification

`mcp_types._v2025_11_25.ProgressNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ProgressNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/progress']`  _instance-attribute_
- `params: ProgressNotificationParams`  _instance-attribute_

An out-of-band notification used to inform the receiver of a progress update for a long-running request.


## ProgressNotificationParams

`mcp_types._v2025_11_25.ProgressNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ProgressNotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `message: str | None = None`  _class-attribute, instance-attribute_
  An optional message describing the current progress.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `progress: float`  _instance-attribute_
  The progress thus far. This should increase every time progress is made, even if the total is unknown.
- `progress_token: Annotated[ProgressToken, Field(alias='progressToken')]`  _instance-attribute_
  The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.
- `total: float | None = None`  _class-attribute, instance-attribute_
  Total number of items to process (or total progress required), if known.

Parameters for a `notifications/progress` notification.


## Prompt

`mcp_types._v2025_11_25.Prompt`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Prompt(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `arguments: list[PromptArgument] | None = None`  _class-attribute, instance-attribute_
  A list of arguments to use for templating the prompt.
- `description: str | None = None`  _class-attribute, instance-attribute_
  An optional description of what this prompt provides
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.

A prompt or prompt template that the server offers.


## PromptArgument

`mcp_types._v2025_11_25.PromptArgument`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PromptArgument(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `description: str | None = None`  _class-attribute, instance-attribute_
  A human-readable description of the argument.
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `required: bool | None = None`  _class-attribute, instance-attribute_
  Whether this argument must be provided.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.

Describes an argument that a prompt can accept.


## PromptListChangedNotification

`mcp_types._v2025_11_25.PromptListChangedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PromptListChangedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/prompts/list_changed']`  _instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This may be issued by servers without any previous subscription from the client.


## PromptMessage

`mcp_types._v2025_11_25.PromptMessage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PromptMessage(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `content: ContentBlock`  _instance-attribute_
- `role: Role`  _instance-attribute_

Describes a message returned as part of a prompt.

This is similar to `SamplingMessage`, but also supports the embedding of
resources from the MCP server.


## PromptReference

`mcp_types._v2025_11_25.PromptReference`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PromptReference(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.
- `type: Literal['ref/prompt']`  _instance-attribute_

Identifies a prompt.


## Prompts

`mcp_types._v2025_11_25.Prompts`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Prompts(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `list_changed: Annotated[bool | None, Field(alias='listChanged')] = None`  _class-attribute, instance-attribute_
  Whether this server supports notifications for changes to the prompt list.

Present if the server offers any prompt templates.


## ReadResourceRequest

`mcp_types._v2025_11_25.ReadResourceRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ReadResourceRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['resources/read']`  _instance-attribute_
- `params: ReadResourceRequestParams`  _instance-attribute_

Sent from the client to the server, to read a specific resource URI.


## ReadResourceRequestParams

`mcp_types._v2025_11_25.ReadResourceRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ReadResourceRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `uri: str`  _instance-attribute_
  The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it.

Parameters for a `resources/read` request.


## ReadResourceResult

`mcp_types._v2025_11_25.ReadResourceResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ReadResourceResult(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `contents: list[TextResourceContents | BlobResourceContents]`  _instance-attribute_
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

The server's response to a resources/read request from the client.


## RelatedTaskMetadata

`mcp_types._v2025_11_25.RelatedTaskMetadata`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class RelatedTaskMetadata(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `task_id: Annotated[str, Field(alias='taskId')]`  _instance-attribute_
  The task identifier this message is associated with.

Metadata for associating messages with a task.
Include this in the `_meta` field under the key `io.modelcontextprotocol/related-task`.


## Request

`mcp_types._v2025_11_25.Request`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Request(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `method: str`  _instance-attribute_
- `params: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

## RequestParams

`mcp_types._v2025_11_25.RequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class RequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

Common params for any request.


## RequestedSchema

`mcp_types._v2025_11_25.RequestedSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class RequestedSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `properties: dict[str, Any]`  _instance-attribute_
- `required: list[str] | None = None`  _class-attribute, instance-attribute_
- `schema_: Annotated[str | None, Field(alias='$schema')] = None`  _class-attribute, instance-attribute_
- `type: Literal['object']`  _instance-attribute_

A restricted subset of JSON Schema.
Only top-level properties are allowed, without nesting.


## Requests

`mcp_types._v2025_11_25.Requests`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Requests(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `elicitation: Elicitation1 | None = None`  _class-attribute, instance-attribute_
  Task support for elicitation-related requests.
- `sampling: Sampling1 | None = None`  _class-attribute, instance-attribute_
  Task support for sampling-related requests.

Specifies which request types can be augmented with tasks.


## Requests1

`mcp_types._v2025_11_25.Requests1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Requests1(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `tools: Tools | None = None`  _class-attribute, instance-attribute_
  Task support for tool-related requests.

Specifies which request types can be augmented with tasks.


## Resource

`mcp_types._v2025_11_25.Resource`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Resource(WireModel)
```

**Bases** `WireModel`

**Declared members (9)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `description: str | None = None`  _class-attribute, instance-attribute_
  A description of what this resource represents.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `size: int | None = None`  _class-attribute, instance-attribute_
  The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.
- `uri: str`  _instance-attribute_
  The URI of this resource.

A known resource that the server is capable of reading.


## ResourceContents

`mcp_types._v2025_11_25.ResourceContents`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceContents(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `uri: str`  _instance-attribute_
  The URI of this resource.

The contents of a specific resource or sub-resource.


## ResourceLink

`mcp_types._v2025_11_25.ResourceLink`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceLink(WireModel)
```

**Bases** `WireModel`

**Declared members (10)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `description: str | None = None`  _class-attribute, instance-attribute_
  A description of what this resource represents.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `size: int | None = None`  _class-attribute, instance-attribute_
  The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.
- `type: Literal['resource_link']`  _instance-attribute_
- `uri: str`  _instance-attribute_
  The URI of this resource.

A resource that the server is capable of reading, included in a prompt or tool call result.

Note: resource links returned by tools are not guaranteed to appear in the results of `resources/list` requests.


## ResourceListChangedNotification

`mcp_types._v2025_11_25.ResourceListChangedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceListChangedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/resources/list_changed']`  _instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This may be issued by servers without any previous subscription from the client.


## ResourceRequestParams

`mcp_types._v2025_11_25.ResourceRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `uri: str`  _instance-attribute_
  The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it.

Common parameters when working with resources.


## ResourceTemplate

`mcp_types._v2025_11_25.ResourceTemplate`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceTemplate(WireModel)
```

**Bases** `WireModel`

**Declared members (8)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `description: str | None = None`  _class-attribute, instance-attribute_
  A description of what this template is for.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type for all resources that match this template. This should only be included if all resources matching this template have the same type.
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.
- `uri_template: Annotated[str, Field(alias='uriTemplate')]`  _instance-attribute_
  A URI template (according to RFC 6570) that can be used to construct resource URIs.

A template description for resources available on the server.


## ResourceTemplateReference

`mcp_types._v2025_11_25.ResourceTemplateReference`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceTemplateReference(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `type: Literal['ref/resource']`  _instance-attribute_
- `uri: str`  _instance-attribute_
  The URI or URI template of the resource.

A reference to a resource or resource template definition.


## ResourceUpdatedNotification

`mcp_types._v2025_11_25.ResourceUpdatedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceUpdatedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/resources/updated']`  _instance-attribute_
- `params: ResourceUpdatedNotificationParams`  _instance-attribute_

A notification from the server to the client, informing it that a resource has changed and may need to be read again. This should only be sent if the client previously sent a resources/subscribe request.


## ResourceUpdatedNotificationParams

`mcp_types._v2025_11_25.ResourceUpdatedNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceUpdatedNotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `uri: str`  _instance-attribute_
  The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.

Parameters for a `notifications/resources/updated` notification.


## Resources

`mcp_types._v2025_11_25.Resources`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Resources(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `list_changed: Annotated[bool | None, Field(alias='listChanged')] = None`  _class-attribute, instance-attribute_
  Whether this server supports notifications for changes to the resource list.
- `subscribe: bool | None = None`  _class-attribute, instance-attribute_
  Whether this server supports subscribing to resource updates.

Present if the server offers any resources to read.


## Result

`mcp_types._v2025_11_25.Result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Result(WireModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `WireModel`

**Declared members (1)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

## Root

`mcp_types._v2025_11_25.Root`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Root(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `name: str | None = None`  _class-attribute, instance-attribute_
  An optional name for the root. This can be used to provide a human-readable identifier for the root, which may be useful for display purposes or for referencing the root in other parts of the application.
- `uri: str`  _instance-attribute_
  The URI identifying the root. This *must* start with file:// for now. This restriction may be relaxed in future versions of the protocol to allow other URI schemes.

Represents a root directory or file that the server can operate on.


## Roots

`mcp_types._v2025_11_25.Roots`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Roots(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `list_changed: Annotated[bool | None, Field(alias='listChanged')] = None`  _class-attribute, instance-attribute_
  Whether the client supports notifications for changes to the roots list.

Present if the client supports listing roots.


## RootsListChangedNotification

`mcp_types._v2025_11_25.RootsListChangedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class RootsListChangedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/roots/list_changed']`  _instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

A notification from the client to the server, informing it that the list of roots has changed.
This notification should be sent whenever the client adds, removes, or modifies any root.
The server should then request an updated list of roots using the ListRootsRequest.


## Sampling

`mcp_types._v2025_11_25.Sampling`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Sampling(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `context: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Whether the client supports context inclusion via includeContext parameter. If not declared, servers SHOULD only use `includeContext: "none"` (or omit it).
- `tools: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Whether the client supports tool use via tools and toolChoice parameters.

Present if the client supports sampling from an LLM.


## Sampling1

`mcp_types._v2025_11_25.Sampling1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Sampling1(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `create_message: Annotated[dict[str, Any] | None, Field(alias='createMessage')] = None`  _class-attribute, instance-attribute_
  Whether the client supports task-augmented sampling/createMessage requests.

Task support for sampling-related requests.


## SamplingMessage

`mcp_types._v2025_11_25.SamplingMessage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SamplingMessage(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `content: TextContent | ImageContent | AudioContent | ToolUseContent | ToolResultContent | list[SamplingMessageContentBlock]`  _instance-attribute_
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `role: Role`  _instance-attribute_

Describes a message issued to or received from an LLM API.


## ServerCapabilities

`mcp_types._v2025_11_25.ServerCapabilities`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ServerCapabilities(WireModel)
```

**Bases** `WireModel`

**Declared members (7)**

- `completions: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Present if the server supports argument autocompletion suggestions.
- `experimental: dict[str, dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
  Experimental, non-standard capabilities that the server supports.
- `logging: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Present if the server supports sending log messages to the client.
- `prompts: Prompts | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any prompt templates.
- `resources: Resources | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any resources to read.
- `tasks: Tasks1 | None = None`  _class-attribute, instance-attribute_
  Present if the server supports task-augmented requests.
- `tools: Tools1 | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any tools to call.

Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities.


## SetLevelRequest

`mcp_types._v2025_11_25.SetLevelRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SetLevelRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['logging/setLevel']`  _instance-attribute_
- `params: SetLevelRequestParams`  _instance-attribute_

A request from the client to the server, to enable or adjust logging.


## SetLevelRequestParams

`mcp_types._v2025_11_25.SetLevelRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SetLevelRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `level: LoggingLevel`  _instance-attribute_
  The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/message.
- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.

Parameters for a `logging/setLevel` request.


## StringSchema

`mcp_types._v2025_11_25.StringSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class StringSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (7)**

- `default: str | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `format: Literal['date', 'date-time', 'email', 'uri'] | None = None`  _class-attribute, instance-attribute_
- `max_length: Annotated[int | None, Field(alias='maxLength')] = None`  _class-attribute, instance-attribute_
- `min_length: Annotated[int | None, Field(alias='minLength')] = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_
- `type: Literal['string']`  _instance-attribute_

## SubscribeRequest

`mcp_types._v2025_11_25.SubscribeRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscribeRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['resources/subscribe']`  _instance-attribute_
- `params: SubscribeRequestParams`  _instance-attribute_

Sent from the client to request resources/updated notifications from the server whenever a particular resource changes.


## SubscribeRequestParams

`mcp_types._v2025_11_25.SubscribeRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscribeRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `uri: str`  _instance-attribute_
  The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it.

Parameters for a `resources/subscribe` request.


## Task

`mcp_types._v2025_11_25.Task`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Task(WireModel)
```

**Bases** `WireModel`

**Declared members (7)**

- `created_at: Annotated[str, Field(alias='createdAt')]`  _instance-attribute_
  ISO 8601 timestamp when the task was created.
- `last_updated_at: Annotated[str, Field(alias='lastUpdatedAt')]`  _instance-attribute_
  ISO 8601 timestamp when the task was last updated.
- `poll_interval: Annotated[int | None, Field(alias='pollInterval')] = None`  _class-attribute, instance-attribute_
  Suggested polling interval in milliseconds.
- `status: TaskStatus`  _instance-attribute_
  Current task state.
- `status_message: Annotated[str | None, Field(alias='statusMessage')] = None`  _class-attribute, instance-attribute_
  Optional human-readable message describing the current task state. This can provide context for any status, including: - Reasons for "cancelled" status - Summaries for "completed" status - Diagnostic information for "failed" status (e.g.,…
- `task_id: Annotated[str, Field(alias='taskId')]`  _instance-attribute_
  The task identifier.
- `ttl: int | None`  _instance-attribute_
  Actual retention duration from creation in milliseconds, null for unlimited.

Data associated with a task.


## TaskAugmentedRequestParams

`mcp_types._v2025_11_25.TaskAugmentedRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TaskAugmentedRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `task: TaskMetadata | None = None`  _class-attribute, instance-attribute_
  If specified, the caller is requesting task-augmented execution for this request. The request will return a CreateTaskResult immediately, and the actual result can be retrieved later via tasks/result.

Common params for any task-augmented request.


## TaskMetadata

`mcp_types._v2025_11_25.TaskMetadata`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TaskMetadata(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `ttl: int | None = None`  _class-attribute, instance-attribute_
  Requested duration in milliseconds to retain task from creation.

Metadata for augmenting a request with task execution.
Include this in the `task` field of the request parameters.


## TaskStatusNotification

`mcp_types._v2025_11_25.TaskStatusNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TaskStatusNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/tasks/status']`  _instance-attribute_
- `params: TaskStatusNotificationParams`  _instance-attribute_

An optional notification from the receiver to the requestor, informing them that a task's status has changed. Receivers are not required to send these notifications.


## TaskStatusNotificationParams

`mcp_types._v2025_11_25.TaskStatusNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TaskStatusNotificationParams(NotificationParams, Task)
```

**Bases** `NotificationParams`, `Task`

**Inherited (8)**

- from `mcp_types._v2025_11_25.NotificationParams`: `meta`
- from `mcp_types._v2025_11_25.Task`: `created_at`, `last_updated_at`, `poll_interval`, `status`, `status_message`, `task_id`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Parameters for a `notifications/tasks/status` notification.


## Tasks

`mcp_types._v2025_11_25.Tasks`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Tasks(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `cancel: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Whether this client supports tasks/cancel.
- `list: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Whether this client supports tasks/list.
- `requests: Requests | None = None`  _class-attribute, instance-attribute_
  Specifies which request types can be augmented with tasks.

Present if the client supports task-augmented requests.


## Tasks1

`mcp_types._v2025_11_25.Tasks1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Tasks1(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `cancel: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Whether this server supports tasks/cancel.
- `list: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Whether this server supports tasks/list.
- `requests: Requests1 | None = None`  _class-attribute, instance-attribute_
  Specifies which request types can be augmented with tasks.

Present if the server supports task-augmented requests.


## TextContent

`mcp_types._v2025_11_25.TextContent`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TextContent(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `text: str`  _instance-attribute_
  The text content of the message.
- `type: Literal['text']`  _instance-attribute_

Text provided to or from an LLM.


## TextResourceContents

`mcp_types._v2025_11_25.TextResourceContents`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TextResourceContents(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `text: str`  _instance-attribute_
  The text of the item. This must only be set if the item can actually be represented as text (not binary data).
- `uri: str`  _instance-attribute_
  The URI of this resource.

## TitledMultiSelectEnumSchema

`mcp_types._v2025_11_25.TitledMultiSelectEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TitledMultiSelectEnumSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (7)**

- `default: list[str] | None = None`  _class-attribute, instance-attribute_
  Optional default value.
- `description: str | None = None`  _class-attribute, instance-attribute_
  Optional description for the enum field.
- `items: Items`  _instance-attribute_
  Schema for array items with enum options and display labels.
- `max_items: Annotated[int | None, Field(alias='maxItems')] = None`  _class-attribute, instance-attribute_
  Maximum number of items to select.
- `min_items: Annotated[int | None, Field(alias='minItems')] = None`  _class-attribute, instance-attribute_
  Minimum number of items to select.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Optional title for the enum field.
- `type: Literal['array']`  _instance-attribute_

Schema for multiple-selection enumeration with display titles for each option.


## TitledSingleSelectEnumSchema

`mcp_types._v2025_11_25.TitledSingleSelectEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TitledSingleSelectEnumSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `default: str | None = None`  _class-attribute, instance-attribute_
  Optional default value.
- `description: str | None = None`  _class-attribute, instance-attribute_
  Optional description for the enum field.
- `one_of: Annotated[list[OneOfItem], Field(alias='oneOf')]`  _instance-attribute_
  Array of enum options with values and display labels.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Optional title for the enum field.
- `type: Literal['string']`  _instance-attribute_

Schema for single-selection enumeration with display titles for each option.


## Tool

`mcp_types._v2025_11_25.Tool`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Tool(WireModel)
```

**Bases** `WireModel`

**Declared members (9)**

- `annotations: ToolAnnotations | None = None`  _class-attribute, instance-attribute_
  Optional additional tool information.
- `description: str | None = None`  _class-attribute, instance-attribute_
  A human-readable description of the tool.
- `execution: ToolExecution | None = None`  _class-attribute, instance-attribute_
  Execution-related properties for this tool.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `input_schema: Annotated[InputSchema, Field(alias='inputSchema')]`  _instance-attribute_
  A JSON Schema object defining the expected parameters for the tool.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `output_schema: Annotated[OutputSchema | None, Field(alias='outputSchema')] = None`  _class-attribute, instance-attribute_
  An optional JSON Schema object defining the structure of the tool's output returned in the structuredContent field of a CallToolResult.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.

Definition for a tool the client can call.


## ToolAnnotations

`mcp_types._v2025_11_25.ToolAnnotations`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ToolAnnotations(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `destructive_hint: Annotated[bool | None, Field(alias='destructiveHint')] = None`  _class-attribute, instance-attribute_
  If true, the tool may perform destructive updates to its environment. If false, the tool performs only additive updates.
- `idempotent_hint: Annotated[bool | None, Field(alias='idempotentHint')] = None`  _class-attribute, instance-attribute_
  If true, calling the tool repeatedly with the same arguments will have no additional effect on its environment.
- `open_world_hint: Annotated[bool | None, Field(alias='openWorldHint')] = None`  _class-attribute, instance-attribute_
  If true, this tool may interact with an "open world" of external entities. If false, the tool's domain of interaction is closed. For example, the world of a web search tool is open, whereas that of a memory tool is not.
- `read_only_hint: Annotated[bool | None, Field(alias='readOnlyHint')] = None`  _class-attribute, instance-attribute_
  If true, the tool does not modify its environment.
- `title: str | None = None`  _class-attribute, instance-attribute_
  A human-readable title for the tool.

Additional properties describing a Tool to clients.

NOTE: all properties in ToolAnnotations are **hints**.
They are not guaranteed to provide a faithful description of
tool behavior (including descriptive properties like `title`).

Clients should never make tool use decisions based on ToolAnnotations
received from untrusted servers.


## ToolChoice

`mcp_types._v2025_11_25.ToolChoice`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ToolChoice(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `mode: Literal['auto', 'none', 'required'] | None = None`  _class-attribute, instance-attribute_
  Controls the tool use ability of the model: - "auto": Model decides whether to use tools (default) - "required": Model MUST use at least one tool before completing - "none": Model MUST NOT use any tools

Controls tool selection behavior for sampling requests.


## ToolExecution

`mcp_types._v2025_11_25.ToolExecution`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ToolExecution(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `task_support: Annotated[Literal['forbidden', 'optional', 'required'] | None, Field(alias='taskSupport')] = None`  _class-attribute, instance-attribute_
  Indicates whether this tool supports task-augmented execution. This allows clients to handle long-running operations through polling the task system.

Execution-related properties for a tool.


## ToolListChangedNotification

`mcp_types._v2025_11_25.ToolListChangedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ToolListChangedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/tools/list_changed']`  _instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list of tools it offers has changed. This may be issued by servers without any previous subscription from the client.


## ToolResultContent

`mcp_types._v2025_11_25.ToolResultContent`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ToolResultContent(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `content: list[ContentBlock]`  _instance-attribute_
  The unstructured result content of the tool use.
- `is_error: Annotated[bool | None, Field(alias='isError')] = None`  _class-attribute, instance-attribute_
  Whether the tool use resulted in an error.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  Optional metadata about the tool result. Clients SHOULD preserve this field when including tool results in subsequent sampling requests to enable caching optimizations.
- `structured_content: Annotated[dict[str, Any] | None, Field(alias='structuredContent')] = None`  _class-attribute, instance-attribute_
  An optional structured result object.
- `tool_use_id: Annotated[str, Field(alias='toolUseId')]`  _instance-attribute_
  The ID of the tool use this result corresponds to.
- `type: Literal['tool_result']`  _instance-attribute_

The result of a tool use, provided by the user back to the assistant.


## ToolUseContent

`mcp_types._v2025_11_25.ToolUseContent`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ToolUseContent(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `id: str`  _instance-attribute_
  A unique identifier for this tool use.
- `input: dict[str, Any]`  _instance-attribute_
  The arguments to pass to the tool, conforming to the tool's input schema.
- `meta: Annotated[dict[str, Any] | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  Optional metadata about the tool use. Clients SHOULD preserve this field when including tool uses in subsequent sampling requests to enable caching optimizations.
- `name: str`  _instance-attribute_
  The name of the tool to call.
- `type: Literal['tool_use']`  _instance-attribute_

A request from the assistant to call a tool.


## Tools

`mcp_types._v2025_11_25.Tools`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Tools(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `call: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Whether the server supports task-augmented tools/call requests.

Task support for tool-related requests.


## Tools1

`mcp_types._v2025_11_25.Tools1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Tools1(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `list_changed: Annotated[bool | None, Field(alias='listChanged')] = None`  _class-attribute, instance-attribute_
  Whether this server supports notifications for changes to the tool list.

Present if the server offers any tools to call.


## URLElicitationRequiredError

`mcp_types._v2025_11_25.URLElicitationRequiredError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class URLElicitationRequiredError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `error: Error1`  _instance-attribute_
- `id: RequestId | None = None`  _class-attribute, instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_

An error response that indicates that the server requires the client to provide additional information via an elicitation request.


## UnsubscribeRequest

`mcp_types._v2025_11_25.UnsubscribeRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class UnsubscribeRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['resources/unsubscribe']`  _instance-attribute_
- `params: UnsubscribeRequestParams`  _instance-attribute_

Sent from the client to request cancellation of resources/updated notifications from the server. This should follow a previous resources/subscribe request.


## UnsubscribeRequestParams

`mcp_types._v2025_11_25.UnsubscribeRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class UnsubscribeRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[Meta | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  See [General fields: `_meta`](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on `_meta` usage.
- `uri: str`  _instance-attribute_
  The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it.

Parameters for a `resources/unsubscribe` request.


## UntitledMultiSelectEnumSchema

`mcp_types._v2025_11_25.UntitledMultiSelectEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class UntitledMultiSelectEnumSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (7)**

- `default: list[str] | None = None`  _class-attribute, instance-attribute_
  Optional default value.
- `description: str | None = None`  _class-attribute, instance-attribute_
  Optional description for the enum field.
- `items: Items1`  _instance-attribute_
  Schema for the array items.
- `max_items: Annotated[int | None, Field(alias='maxItems')] = None`  _class-attribute, instance-attribute_
  Maximum number of items to select.
- `min_items: Annotated[int | None, Field(alias='minItems')] = None`  _class-attribute, instance-attribute_
  Minimum number of items to select.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Optional title for the enum field.
- `type: Literal['array']`  _instance-attribute_

Schema for multiple-selection enumeration without display titles for options.


## UntitledSingleSelectEnumSchema

`mcp_types._v2025_11_25.UntitledSingleSelectEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class UntitledSingleSelectEnumSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `default: str | None = None`  _class-attribute, instance-attribute_
  Optional default value.
- `description: str | None = None`  _class-attribute, instance-attribute_
  Optional description for the enum field.
- `enum: list[str]`  _instance-attribute_
  Array of enum values to choose from.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Optional title for the enum field.
- `type: Literal['string']`  _instance-attribute_

Schema for single-selection enumeration without display titles for options.


