# `mcp_types._v2026_07_28`

Distribution: `mcp-types`

## AnyCallToolResult

`mcp_types._v2026_07_28.AnyCallToolResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
AnyCallToolResult = CallToolResult | InputRequiredResult
```

## AnyGetPromptResult

`mcp_types._v2026_07_28.AnyGetPromptResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
AnyGetPromptResult = GetPromptResult | InputRequiredResult
```

## AnyReadResourceResult

`mcp_types._v2026_07_28.AnyReadResourceResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
AnyReadResourceResult = ReadResourceResult | InputRequiredResult
```

## ClientRequest

`mcp_types._v2026_07_28.ClientRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ClientRequest = DiscoverRequest | ListResourcesRequest | ListResourceTemplatesRequest | ReadResourceRequest | SubscriptionsListenRequest | ListPromptsRequest | GetPromptRequest | ListToolsRequest | CallToolRequest | CompleteRequest
```

## ClientResult

`mcp_types._v2026_07_28.ClientResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ClientResult = Result
```

Common result fields.


## ContentBlock

`mcp_types._v2026_07_28.ContentBlock`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ContentBlock = TextContent | ImageContent | AudioContent | ResourceLink | EmbeddedResource
```

## Cursor

`mcp_types._v2026_07_28.Cursor`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
Cursor = str
```

An opaque token used to represent a cursor for pagination.


## ElicitRequestParams

`mcp_types._v2026_07_28.ElicitRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ElicitRequestParams = ElicitRequestFormParams | ElicitRequestURLParams
```

The parameters for a request to elicit additional information from the user via the client.


## EmptyResult

`mcp_types._v2026_07_28.EmptyResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
EmptyResult = Result
```

Common result fields.


## EnumSchema

`mcp_types._v2026_07_28.EnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
EnumSchema = UntitledSingleSelectEnumSchema | TitledSingleSelectEnumSchema | UntitledMultiSelectEnumSchema | TitledMultiSelectEnumSchema | LegacyTitledEnumSchema
```

## InputRequest

`mcp_types._v2026_07_28.InputRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
InputRequest = CreateMessageRequest | ListRootsRequest | ElicitRequest
```

## InputRequests

`mcp_types._v2026_07_28.InputRequests`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
InputRequests = dict[str, InputRequest]
```

A map of server-initiated requests that the client must fulfill.
Keys are server-assigned identifiers; values are the request objects.


## InputResponse

`mcp_types._v2026_07_28.InputResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
InputResponse = CreateMessageResult | ListRootsResult | ElicitResult
```

## InputResponses

`mcp_types._v2026_07_28.InputResponses`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
InputResponses = dict[str, InputResponse]
```

A map of client responses to server-initiated requests.
Keys correspond to the keys in the {@link InputRequests} map;
values are the client's result for each request.


## JSONArray

`mcp_types._v2026_07_28.JSONArray`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
JSONArray = list[JSONValue]
```

## JSONObject

`mcp_types._v2026_07_28.JSONObject`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
JSONObject = dict[str, JSONValue]
```

## JSONRPCMessage

`mcp_types._v2026_07_28.JSONRPCMessage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
JSONRPCMessage = JSONRPCRequest | JSONRPCNotification | JSONRPCResultResponse | JSONRPCErrorResponse
```

Refers to any valid JSON-RPC object that can be decoded off the wire, or encoded to be sent.


## JSONRPCResponse

`mcp_types._v2026_07_28.JSONRPCResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
JSONRPCResponse = JSONRPCResultResponse | JSONRPCErrorResponse
```

A response to a request, containing either the result or error.


## JSONValue

`mcp_types._v2026_07_28.JSONValue`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
JSONValue = TypeAliasType('JSONValue', Union[dict[str, 'JSONValue'], list['JSONValue'], str | int | float | bool | None])
```

## LoggingLevel

`mcp_types._v2026_07_28.LoggingLevel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
LoggingLevel = Literal['alert', 'critical', 'debug', 'emergency', 'error', 'info', 'notice', 'warning']
```

The severity of a log message.

These map to syslog message severities, as specified in RFC-5424:
https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1


## MultiSelectEnumSchema

`mcp_types._v2026_07_28.MultiSelectEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
MultiSelectEnumSchema = UntitledMultiSelectEnumSchema | TitledMultiSelectEnumSchema
```

## PrimitiveSchemaDefinition

`mcp_types._v2026_07_28.PrimitiveSchemaDefinition`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
PrimitiveSchemaDefinition = StringSchema | NumberSchema | BooleanSchema | UntitledSingleSelectEnumSchema | TitledSingleSelectEnumSchema | UntitledMultiSelectEnumSchema | TitledMultiSelectEnumSchema | LegacyTitledEnumSchema
```

Restricted schema definitions that only allow primitive types
without nested objects or arrays.


## ProgressToken

`mcp_types._v2026_07_28.ProgressToken`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ProgressToken = str | int
```

A progress token, used to associate progress notifications with the original request.


## RequestId

`mcp_types._v2026_07_28.RequestId`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
RequestId = str | int
```

A uniquely identifying ID for a request in JSON-RPC.


## ResultType

`mcp_types._v2026_07_28.ResultType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ResultType = str
```

Indicates the type of a {@link Result} object, allowing the client to
determine how to parse the response.

complete - the request completed successfully and the result contains the final content.
input_required - the request requires additional input and the result contains an {@link InputRequiredResult} object with instructions for the client to provide additional input before retrying the original request.


## Role

`mcp_types._v2026_07_28.Role`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
Role = Literal['assistant', 'user']
```

The sender or recipient of messages and data in a conversation.


## SamplingMessageContentBlock

`mcp_types._v2026_07_28.SamplingMessageContentBlock`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
SamplingMessageContentBlock = TextContent | ImageContent | AudioContent | ToolUseContent | ToolResultContent
```

## ServerNotification

`mcp_types._v2026_07_28.ServerNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ServerNotification = CancelledNotification | ProgressNotification | ResourceListChangedNotification | SubscriptionsAcknowledgedNotification | ResourceUpdatedNotification | PromptListChangedNotification | ToolListChangedNotification | LoggingMessageNotification
```

## ServerResult

`mcp_types._v2026_07_28.ServerResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ServerResult = Result | InputRequiredResult | DiscoverResult | ListResourcesResult | ListResourceTemplatesResult | ReadResourceResult | SubscriptionsListenResult | ListPromptsResult | GetPromptResult | ListToolsResult | CallToolResult | CompleteResult
```

## SingleSelectEnumSchema

`mcp_types._v2026_07_28.SingleSelectEnumSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
SingleSelectEnumSchema = UntitledSingleSelectEnumSchema | TitledSingleSelectEnumSchema
```

## Annotations

`mcp_types._v2026_07_28.Annotations`

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

`mcp_types._v2026_07_28.AnyOfItem`

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

`mcp_types._v2026_07_28.Argument`

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

`mcp_types._v2026_07_28.AudioContent`

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
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `mime_type: Annotated[str, Field(alias='mimeType')]`  _instance-attribute_
  The MIME type of the audio. Different providers may support different audio types.
- `type: Literal['audio']`  _instance-attribute_

Audio provided to or from an LLM.


## BaseMetadata

`mcp_types._v2026_07_28.BaseMetadata`

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

`mcp_types._v2026_07_28.BlobResourceContents`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class BlobResourceContents(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `blob: str`  _instance-attribute_
  A base64-encoded string representing the binary data of the item.
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `uri: str`  _instance-attribute_
  The URI of this resource.

## BooleanSchema

`mcp_types._v2026_07_28.BooleanSchema`

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

## CacheableResult

`mcp_types._v2026_07_28.CacheableResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CacheableResult(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `cache_scope: Annotated[Literal['private', 'public'], Field(alias='cacheScope')]`  _instance-attribute_
  Indicates the intended scope of the cached response, analogous to HTTP `Cache-Control: public` vs `Cache-Control: private`.
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.
- `ttl_ms: Annotated[int, Field(alias='ttlMs', ge=0)]`  _instance-attribute_
  A hint from the server indicating how long (in milliseconds) the client MAY cache this response before re-fetching. Semantics are analogous to HTTP Cache-Control max-age.

A result that supports a time-to-live (TTL) hint for client-side caching.


## CallToolRequest

`mcp_types._v2026_07_28.CallToolRequest`

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

`mcp_types._v2026_07_28.CallToolRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CallToolRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `arguments: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Arguments to use for the tool call.
- `input_responses: Annotated[InputResponses | None, Field(alias='inputResponses')] = None`  _class-attribute, instance-attribute_
- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_
- `name: str`  _instance-attribute_
  The name of the tool.
- `request_state: Annotated[str | None, Field(alias='requestState')] = None`  _class-attribute, instance-attribute_

Parameters for a `tools/call` request.


## CallToolResult

`mcp_types._v2026_07_28.CallToolResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CallToolResult(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `content: list[ContentBlock]`  _instance-attribute_
  A list of content objects that represent the unstructured result of the tool call.
- `is_error: Annotated[bool | None, Field(alias='isError')] = None`  _class-attribute, instance-attribute_
  Whether the tool call ended in an error.
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.
- `structured_content: Annotated[Any | None, Field(alias='structuredContent')] = None`  _class-attribute, instance-attribute_
  An optional JSON value that represents the structured result of the tool call.

The result returned by the server for a {@link CallToolRequesttools/call} request.


## CallToolResultResponse

`mcp_types._v2026_07_28.CallToolResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CallToolResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: InputRequiredResult | CallToolResult`  _instance-attribute_

A successful response from the server for a {@link CallToolRequesttools/call} request.


## CancelledNotification

`mcp_types._v2026_07_28.CancelledNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CancelledNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/cancelled']`  _instance-attribute_
- `params: CancelledNotificationParams`  _instance-attribute_

This notification is sent by the client to indicate that it is cancelling a request it previously issued.

On stdio, the server also sends this notification, solely to terminate a {@link SubscriptionsListenRequestsubscriptions/listen} stream: it references the ID of the `subscriptions/listen` request that opened the stream. Servers MUST NOT use this notification to cancel any other request.

The request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.

This notification indicates that the result will be unused, so any associated processing SHOULD cease.


## CancelledNotificationParams

`mcp_types._v2026_07_28.CancelledNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CancelledNotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[NotificationMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `reason: str | None = None`  _class-attribute, instance-attribute_
  An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.
- `request_id: Annotated[RequestId, Field(alias='requestId')]`  _instance-attribute_
  The ID of the request to cancel.

Parameters for a `notifications/cancelled` notification.


## ClientCapabilities

`mcp_types._v2026_07_28.ClientCapabilities`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ClientCapabilities(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `elicitation: Elicitation | None = None`  _class-attribute, instance-attribute_
  Present if the client supports elicitation from the server.
- `experimental: dict[str, JSONObject] | None = None`  _class-attribute, instance-attribute_
  Experimental, non-standard capabilities that the client supports.
- `extensions: dict[str, JSONObject] | None = None`  _class-attribute, instance-attribute_
  Optional MCP extensions that the client supports. Keys are extension identifiers (e.g., "io.modelcontextprotocol/oauth-client-credentials"), and values are per-extension settings objects. An empty object indicates support with no settings.
- `roots: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
  Present if the client supports listing roots.
- `sampling: Sampling | None = None`  _class-attribute, instance-attribute_
  Present if the client supports sampling from an LLM.

Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities.


## ClientNotification

`mcp_types._v2026_07_28.ClientNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ClientNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/cancelled']`  _instance-attribute_
- `params: CancelledNotificationParams`  _instance-attribute_

This notification is sent by the client to indicate that it is cancelling a request it previously issued.

On stdio, the server also sends this notification, solely to terminate a {@link SubscriptionsListenRequestsubscriptions/listen} stream: it references the ID of the `subscriptions/listen` request that opened the stream. Servers MUST NOT use this notification to cancel any other request.

The request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.

This notification indicates that the result will be unused, so any associated processing SHOULD cease.


## CompleteRequest

`mcp_types._v2026_07_28.CompleteRequest`

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

`mcp_types._v2026_07_28.CompleteRequestParams`

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
- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_
- `ref: PromptReference | ResourceTemplateReference`  _instance-attribute_

Parameters for a `completion/complete` request.


## CompleteResult

`mcp_types._v2026_07_28.CompleteResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CompleteResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `completion: Completion`  _instance-attribute_
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.

The result returned by the server for a {@link CompleteRequestcompletion/complete} request.


## CompleteResultResponse

`mcp_types._v2026_07_28.CompleteResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CompleteResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: CompleteResult`  _instance-attribute_

A successful response from the server for a {@link CompleteRequestcompletion/complete} request.


## Completion

`mcp_types._v2026_07_28.Completion`

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
- `values: Annotated[list[str], Field(max_length=100)]`  _instance-attribute_
  An array of completion values. Must not exceed 100 items.

## Context

`mcp_types._v2026_07_28.Context`

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

`mcp_types._v2026_07_28.CreateMessageRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CreateMessageRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `method: Literal['sampling/createMessage']`  _instance-attribute_
- `params: CreateMessageRequestParams`  _instance-attribute_

A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it.


## CreateMessageRequestParams

`mcp_types._v2026_07_28.CreateMessageRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CreateMessageRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (10)**

- `include_context: Annotated[Literal['allServers', 'none', 'thisServer'] | None, Field(alias='includeContext')] = None`  _class-attribute, instance-attribute_
  A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.
- `max_tokens: Annotated[int, Field(alias='maxTokens')]`  _instance-attribute_
  The requested maximum number of tokens to sample (to prevent runaway completions).
- `messages: list[SamplingMessage]`  _instance-attribute_
- `metadata: JSONObject | None = None`  _class-attribute, instance-attribute_
  Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.
- `model_preferences: Annotated[ModelPreferences | None, Field(alias='modelPreferences')] = None`  _class-attribute, instance-attribute_
  The server's preferences for which model to select. The client MAY ignore these preferences.
- `stop_sequences: Annotated[list[str] | None, Field(alias='stopSequences')] = None`  _class-attribute, instance-attribute_
- `system_prompt: Annotated[str | None, Field(alias='systemPrompt')] = None`  _class-attribute, instance-attribute_
  An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.
- `temperature: float | None = None`  _class-attribute, instance-attribute_
- `tool_choice: Annotated[ToolChoice | None, Field(alias='toolChoice')] = None`  _class-attribute, instance-attribute_
  Controls how the model uses tools. The client MUST return an error if this field is provided but {@link ClientCapabilities.sampling.tools} is not declared. Default is `{ mode: "auto" }`.
- `tools: list[Tool] | None = None`  _class-attribute, instance-attribute_
  Tools that the model may use during generation. The client MUST return an error if this field is provided but {@link ClientCapabilities.sampling.tools} is not declared.

Parameters for a `sampling/createMessage` request.


## CreateMessageResult

`mcp_types._v2026_07_28.CreateMessageResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CreateMessageResult(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `content: TextContent | ImageContent | AudioContent | ToolUseContent | ToolResultContent | list[SamplingMessageContentBlock]`  _instance-attribute_
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `model: str`  _instance-attribute_
  The name of the model that generated the message.
- `role: Role`  _instance-attribute_
- `stop_reason: Annotated[str | None, Field(alias='stopReason')] = None`  _class-attribute, instance-attribute_
  The reason why sampling stopped, if known.

The result returned by the client for a {@link CreateMessageRequestsampling/createMessage} request.
The client should inform the user before returning the sampled message, to allow them
to inspect the response (human in the loop) and decide whether to allow the server to see it.


## Data

`mcp_types._v2026_07_28.Data`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Data(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `required_capabilities: Annotated[ClientCapabilities, Field(alias='requiredCapabilities')]`  _instance-attribute_
  The capabilities the server requires from the client to process this request.

Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).


## Data1

`mcp_types._v2026_07_28.Data1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Data1(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `requested: str`  _instance-attribute_
  The protocol version that was requested by the client.
- `supported: list[str]`  _instance-attribute_
  Protocol versions the server supports. The client should choose a mutually supported version from this list and retry.

Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).


## DiscoverRequest

`mcp_types._v2026_07_28.DiscoverRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class DiscoverRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['server/discover']`  _instance-attribute_
- `params: RequestParams`  _instance-attribute_

A request from the client asking the server to advertise its supported
protocol versions, capabilities, and other metadata. Servers **MUST**
implement `server/discover`. Clients **MAY** call it but are not required
to — version negotiation can also happen inline via per-request `_meta`.


## DiscoverResult

`mcp_types._v2026_07_28.DiscoverResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class DiscoverResult(WireModel)
```

**Bases** `WireModel`

**Declared members (7)**

- `cache_scope: Annotated[Literal['private', 'public'], Field(alias='cacheScope')]`  _instance-attribute_
  Indicates the intended scope of the cached response, analogous to HTTP `Cache-Control: public` vs `Cache-Control: private`.
- `capabilities: ServerCapabilities`  _instance-attribute_
  The capabilities of the server.
- `instructions: str | None = None`  _class-attribute, instance-attribute_
  Natural-language guidance describing the server and its features.
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.
- `supported_versions: Annotated[list[str], Field(alias='supportedVersions')]`  _instance-attribute_
  MCP Protocol Versions this server supports. The client should choose a version from this list for use in subsequent requests.
- `ttl_ms: Annotated[int, Field(alias='ttlMs', ge=0)]`  _instance-attribute_
  A hint from the server indicating how long (in milliseconds) the client MAY cache this response before re-fetching. Semantics are analogous to HTTP Cache-Control max-age.

The result returned by the server for a {@link DiscoverRequestserver/discover} request.


## DiscoverResultResponse

`mcp_types._v2026_07_28.DiscoverResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class DiscoverResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: DiscoverResult`  _instance-attribute_

A successful response from the server for a {@link DiscoverRequestserver/discover} request.


## ElicitRequest

`mcp_types._v2026_07_28.ElicitRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `method: Literal['elicitation/create']`  _instance-attribute_
- `params: ElicitRequestParams`  _instance-attribute_

A request from the server to elicit additional information from the user via the client.


## ElicitRequestFormParams

`mcp_types._v2026_07_28.ElicitRequestFormParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitRequestFormParams(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `message: str`  _instance-attribute_
  The message to present to the user describing what information is being requested.
- `mode: Literal['form'] = 'form'`  _class-attribute, instance-attribute_
  The elicitation mode.
- `requested_schema: Annotated[RequestedSchema, Field(alias='requestedSchema')]`  _instance-attribute_
  A restricted subset of JSON Schema. Only top-level properties are allowed, without nesting.

The parameters for a request to elicit non-sensitive information from the user via a form in the client.


## ElicitRequestURLParams

`mcp_types._v2026_07_28.ElicitRequestURLParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitRequestURLParams(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `message: str`  _instance-attribute_
  The message to present to the user explaining why the interaction is needed.
- `mode: Literal['url']`  _instance-attribute_
  The elicitation mode.
- `url: str`  _instance-attribute_
  The URL that the user should navigate to.

The parameters for a request to elicit information from the user via a URL in the client.


## ElicitResult

`mcp_types._v2026_07_28.ElicitResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ElicitResult(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `action: Literal['accept', 'cancel', 'decline']`  _instance-attribute_
  The user action in response to the elicitation. - `"accept"`: User submitted the form/confirmed the action - `"decline"`: User explicitly declined the action - `"cancel"`: User dismissed without making an explicit choice
- `content: dict[str, list[str] | str | int | float | bool | None] | None = None`  _class-attribute, instance-attribute_
  The submitted form data, only present when action is `"accept"` and mode was `"form"`. Contains values matching the requested schema. Omitted for out-of-band mode responses.

The result returned by the client for an {@link ElicitRequestelicitation/create} request.


## Elicitation

`mcp_types._v2026_07_28.Elicitation`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Elicitation(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `form: JSONObject | None = None`  _class-attribute, instance-attribute_
- `url: JSONObject | None = None`  _class-attribute, instance-attribute_

Present if the client supports elicitation from the server.


## EmbeddedResource

`mcp_types._v2026_07_28.EmbeddedResource`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class EmbeddedResource(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `resource: TextResourceContents | BlobResourceContents`  _instance-attribute_
- `type: Literal['resource']`  _instance-attribute_

The contents of a resource, embedded into a prompt or tool call result.

It is up to the client how best to render embedded resources for the benefit
of the LLM and/or the user.


## Error

`mcp_types._v2026_07_28.Error`

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

`mcp_types._v2026_07_28.Error1`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Error1(Error)
```

**Bases** `Error`

**Declared members (1)**

- `code: Literal[-32020]`  _instance-attribute_
  The error type that occurred.

**Inherited (2)**

- from `mcp_types._v2026_07_28.Error`: `data`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## Error2

`mcp_types._v2026_07_28.Error2`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Error2(Error)
```

**Bases** `Error`

**Declared members (2)**

- `code: Literal[-32021]`  _instance-attribute_
  The error type that occurred.
- `data: Data`  _instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).

**Inherited (1)**

- from `mcp_types._v2026_07_28.Error`: `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## Error3

`mcp_types._v2026_07_28.Error3`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Error3(Error)
```

**Bases** `Error`

**Declared members (2)**

- `code: Literal[-32022]`  _instance-attribute_
  The error type that occurred.
- `data: Data1`  _instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).

**Inherited (1)**

- from `mcp_types._v2026_07_28.Error`: `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## GetPromptRequest

`mcp_types._v2026_07_28.GetPromptRequest`

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

`mcp_types._v2026_07_28.GetPromptRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetPromptRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `arguments: dict[str, str] | None = None`  _class-attribute, instance-attribute_
  Arguments to use for templating the prompt.
- `input_responses: Annotated[InputResponses | None, Field(alias='inputResponses')] = None`  _class-attribute, instance-attribute_
- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_
- `name: str`  _instance-attribute_
  The name of the prompt or prompt template.
- `request_state: Annotated[str | None, Field(alias='requestState')] = None`  _class-attribute, instance-attribute_

Parameters for a `prompts/get` request.


## GetPromptResult

`mcp_types._v2026_07_28.GetPromptResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetPromptResult(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `description: str | None = None`  _class-attribute, instance-attribute_
  An optional description for the prompt.
- `messages: list[PromptMessage]`  _instance-attribute_
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.

The result returned by the server for a {@link GetPromptRequestprompts/get} request.


## GetPromptResultResponse

`mcp_types._v2026_07_28.GetPromptResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class GetPromptResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: InputRequiredResult | GetPromptResult`  _instance-attribute_

A successful response from the server for a {@link GetPromptRequestprompts/get} request.


## HeaderMismatchError

`mcp_types._v2026_07_28.HeaderMismatchError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class HeaderMismatchError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `error: Error1`  _instance-attribute_
- `id: RequestId | None = None`  _class-attribute, instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_

Returned when a server rejects a request because the values in the HTTP
headers do not match the corresponding values in the request body, or
because required headers are missing or malformed. For HTTP, the response
status code MUST be `400 Bad Request`.


## Icon

`mcp_types._v2026_07_28.Icon`

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
  Optional specifier for the theme this icon is designed for. `"light"` indicates the icon is designed to be used with a light background, and `"dark"` indicates the icon is designed to be used with a dark background.

An optionally-sized icon that can be displayed in a user interface.


## Icons

`mcp_types._v2026_07_28.Icons`

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

`mcp_types._v2026_07_28.ImageContent`

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
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `mime_type: Annotated[str, Field(alias='mimeType')]`  _instance-attribute_
  The MIME type of the image. Different providers may support different image types.
- `type: Literal['image']`  _instance-attribute_

An image provided to or from an LLM.


## Implementation

`mcp_types._v2026_07_28.Implementation`

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
  The version of this implementation.
- `website_url: Annotated[str | None, Field(alias='websiteUrl')] = None`  _class-attribute, instance-attribute_
  An optional URL of the website for this implementation.

Describes the MCP implementation.


## InputRequiredResult

`mcp_types._v2026_07_28.InputRequiredResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InputRequiredResult(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `input_requests: Annotated[InputRequests | None, Field(alias='inputRequests')] = None`  _class-attribute, instance-attribute_
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `request_state: Annotated[str | None, Field(alias='requestState')] = None`  _class-attribute, instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.

An InputRequiredResult sent by the server to indicate that additional input is needed
before the request can be completed.

At least one of `inputRequests` or `requestState` MUST be present.


## InputResponseRequestParams

`mcp_types._v2026_07_28.InputResponseRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InputResponseRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `input_responses: Annotated[InputResponses | None, Field(alias='inputResponses')] = None`  _class-attribute, instance-attribute_
- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_
- `request_state: Annotated[str | None, Field(alias='requestState')] = None`  _class-attribute, instance-attribute_

## InputSchema

`mcp_types._v2026_07_28.InputSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InputSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `schema_: Annotated[str | None, Field(alias='$schema')] = None`  _class-attribute, instance-attribute_
- `type: Literal['object']`  _instance-attribute_

A JSON Schema object defining the expected parameters for the tool.

Tool arguments are always JSON objects, so `type: "object"` is required at the root.
Beyond that, any JSON Schema 2020-12 keyword may appear alongside `type` — including
composition keywords (`oneOf`, `anyOf`, `allOf`, `not`), conditional keywords
(`if`/`then`/`else`), reference keywords (`$ref`, `$defs`, `$anchor`), and any other
standard validation or annotation keywords.

Property schemas may carry an `x-mcp-header` annotation to mirror the
argument value into an HTTP header on the Streamable HTTP transport. See
the Streamable HTTP transport specification for the validity and
extraction rules.

Defaults to JSON Schema 2020-12 when no explicit `$schema` is provided.


## InternalError

`mcp_types._v2026_07_28.InternalError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InternalError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `code: Literal[-32603]`  _instance-attribute_
  The error type that occurred.
- `data: Any | None = None`  _class-attribute, instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).
- `message: str`  _instance-attribute_
  A short description of the error. The message SHOULD be limited to a concise single sentence.

A JSON-RPC error indicating that an internal error occurred on the receiver. This error is returned when the receiver encounters an unexpected condition that prevents it from fulfilling the request.


## InvalidParamsError

`mcp_types._v2026_07_28.InvalidParamsError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InvalidParamsError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `code: Literal[-32602]`  _instance-attribute_
  The error type that occurred.
- `data: Any | None = None`  _class-attribute, instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).
- `message: str`  _instance-attribute_
  A short description of the error. The message SHOULD be limited to a concise single sentence.

A JSON-RPC error indicating that the method parameters are invalid or malformed.

In MCP, this error is returned in various contexts when request parameters fail validation:

- **Tools**: Unknown tool name or invalid tool arguments
- **Prompts**: Unknown prompt name or missing required arguments
- **Pagination**: Invalid or expired cursor values
- **Logging**: Invalid log level
- **Elicitation**: Server requests an elicitation mode not declared in client capabilities
- **Sampling**: Missing tool result or tool results mixed with other content


## InvalidRequestError

`mcp_types._v2026_07_28.InvalidRequestError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class InvalidRequestError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `code: Literal[-32600]`  _instance-attribute_
  The error type that occurred.
- `data: Any | None = None`  _class-attribute, instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).
- `message: str`  _instance-attribute_
  A short description of the error. The message SHOULD be limited to a concise single sentence.

A JSON-RPC error indicating that the request is not a valid request object. This error is returned when the message structure does not conform to the JSON-RPC 2.0 specification requirements for a request (e.g., missing required fields like `jsonrpc` or `method`, or using invalid types for these fields).


## Items

`mcp_types._v2026_07_28.Items`

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

`mcp_types._v2026_07_28.Items1`

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

`mcp_types._v2026_07_28.JSONRPCErrorResponse`

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

`mcp_types._v2026_07_28.JSONRPCNotification`

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

`mcp_types._v2026_07_28.JSONRPCRequest`

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

`mcp_types._v2026_07_28.JSONRPCResultResponse`

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

`mcp_types._v2026_07_28.LegacyTitledEnumSchema`

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

Use {@link TitledSingleSelectEnumSchema} instead.
This interface will be removed in a future version.


## ListPromptsRequest

`mcp_types._v2026_07_28.ListPromptsRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListPromptsRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['prompts/list']`  _instance-attribute_
- `params: PaginatedRequestParams`  _instance-attribute_

Sent from the client to request a list of prompts and prompt templates the server has.


## ListPromptsResult

`mcp_types._v2026_07_28.ListPromptsResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListPromptsResult(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `cache_scope: Annotated[Literal['private', 'public'], Field(alias='cacheScope')]`  _instance-attribute_
  Indicates the intended scope of the cached response, analogous to HTTP `Cache-Control: public` vs `Cache-Control: private`.
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `prompts: list[Prompt]`  _instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.
- `ttl_ms: Annotated[int, Field(alias='ttlMs', ge=0)]`  _instance-attribute_
  A hint from the server indicating how long (in milliseconds) the client MAY cache this response before re-fetching. Semantics are analogous to HTTP Cache-Control max-age.

The result returned by the server for a {@link ListPromptsRequestprompts/list} request.


## ListPromptsResultResponse

`mcp_types._v2026_07_28.ListPromptsResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListPromptsResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: ListPromptsResult`  _instance-attribute_

A successful response from the server for a {@link ListPromptsRequestprompts/list} request.


## ListResourceTemplatesRequest

`mcp_types._v2026_07_28.ListResourceTemplatesRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourceTemplatesRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['resources/templates/list']`  _instance-attribute_
- `params: PaginatedRequestParams`  _instance-attribute_

Sent from the client to request a list of resource templates the server has.


## ListResourceTemplatesResult

`mcp_types._v2026_07_28.ListResourceTemplatesResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourceTemplatesResult(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `cache_scope: Annotated[Literal['private', 'public'], Field(alias='cacheScope')]`  _instance-attribute_
  Indicates the intended scope of the cached response, analogous to HTTP `Cache-Control: public` vs `Cache-Control: private`.
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `resource_templates: Annotated[list[ResourceTemplate], Field(alias='resourceTemplates')]`  _instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.
- `ttl_ms: Annotated[int, Field(alias='ttlMs', ge=0)]`  _instance-attribute_
  A hint from the server indicating how long (in milliseconds) the client MAY cache this response before re-fetching. Semantics are analogous to HTTP Cache-Control max-age.

The result returned by the server for a {@link ListResourceTemplatesRequestresources/templates/list} request.


## ListResourceTemplatesResultResponse

`mcp_types._v2026_07_28.ListResourceTemplatesResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourceTemplatesResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: ListResourceTemplatesResult`  _instance-attribute_

A successful response from the server for a {@link ListResourceTemplatesRequestresources/templates/list} request.


## ListResourcesRequest

`mcp_types._v2026_07_28.ListResourcesRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourcesRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['resources/list']`  _instance-attribute_
- `params: PaginatedRequestParams`  _instance-attribute_

Sent from the client to request a list of resources the server has.


## ListResourcesResult

`mcp_types._v2026_07_28.ListResourcesResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourcesResult(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `cache_scope: Annotated[Literal['private', 'public'], Field(alias='cacheScope')]`  _instance-attribute_
  Indicates the intended scope of the cached response, analogous to HTTP `Cache-Control: public` vs `Cache-Control: private`.
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `resources: list[Resource]`  _instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.
- `ttl_ms: Annotated[int, Field(alias='ttlMs', ge=0)]`  _instance-attribute_
  A hint from the server indicating how long (in milliseconds) the client MAY cache this response before re-fetching. Semantics are analogous to HTTP Cache-Control max-age.

The result returned by the server for a {@link ListResourcesRequestresources/list} request.


## ListResourcesResultResponse

`mcp_types._v2026_07_28.ListResourcesResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListResourcesResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: ListResourcesResult`  _instance-attribute_

A successful response from the server for a {@link ListResourcesRequestresources/list} request.


## ListRootsRequest

`mcp_types._v2026_07_28.ListRootsRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListRootsRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `method: Literal['roots/list']`  _instance-attribute_
- `params: Params | None = None`  _class-attribute, instance-attribute_

Sent from the server to request a list of root URIs from the client. Roots allow
servers to ask for specific directories or files to operate on. A common example
for roots is providing a set of repositories or directories a server should operate
on.

This request is typically used when the server needs to understand the file system
structure or access specific locations that the client has permission to read from.


## ListRootsResult

`mcp_types._v2026_07_28.ListRootsResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListRootsResult(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `roots: list[Root]`  _instance-attribute_

The result returned by the client for a {@link ListRootsRequestroots/list} request.
This result contains an array of {@link Root} objects, each representing a root directory
or file that the server can operate on.


## ListToolsRequest

`mcp_types._v2026_07_28.ListToolsRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListToolsRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['tools/list']`  _instance-attribute_
- `params: PaginatedRequestParams`  _instance-attribute_

Sent from the client to request a list of tools the server has.


## ListToolsResult

`mcp_types._v2026_07_28.ListToolsResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListToolsResult(WireModel)
```

**Bases** `WireModel`

**Declared members (6)**

- `cache_scope: Annotated[Literal['private', 'public'], Field(alias='cacheScope')]`  _instance-attribute_
  Indicates the intended scope of the cached response, analogous to HTTP `Cache-Control: public` vs `Cache-Control: private`.
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.
- `tools: list[Tool]`  _instance-attribute_
- `ttl_ms: Annotated[int, Field(alias='ttlMs', ge=0)]`  _instance-attribute_
  A hint from the server indicating how long (in milliseconds) the client MAY cache this response before re-fetching. Semantics are analogous to HTTP Cache-Control max-age.

The result returned by the server for a {@link ListToolsRequesttools/list} request.


## ListToolsResultResponse

`mcp_types._v2026_07_28.ListToolsResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ListToolsResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: ListToolsResult`  _instance-attribute_

A successful response from the server for a {@link ListToolsRequesttools/list} request.


## LoggingMessageNotification

`mcp_types._v2026_07_28.LoggingMessageNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class LoggingMessageNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/message']`  _instance-attribute_
- `params: LoggingMessageNotificationParams`  _instance-attribute_

JSONRPCNotification of a log message passed from server to client. The client opts in by setting `"io.modelcontextprotocol/logLevel"` in a request's `_meta`.


## LoggingMessageNotificationParams

`mcp_types._v2026_07_28.LoggingMessageNotificationParams`

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
- `meta: Annotated[NotificationMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_

Parameters for a `notifications/message` notification.


## MetaObject

`mcp_types._v2026_07_28.MetaObject`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class MetaObject(WireModel)
```

**Bases** `WireModel`

Represents the contents of a `_meta` field, which clients and servers use to attach additional metadata to their interactions.

Certain key names are reserved by MCP for protocol-level metadata; implementations MUST NOT make assumptions about values at these keys. Additionally, specific schema definitions may reserve particular names for purpose-specific metadata, as declared in those definitions.

Valid keys have two segments:

**Prefix:**
- Optional — if specified, MUST be a series of _labels_ separated by dots (`.`), followed by a slash (`/`).
- Labels MUST start with a letter and end with a letter or digit. Interior characters may be letters, digits, or hyphens (`-`).
- Implementations SHOULD use reverse DNS notation (e.g., `com.example/` rather than `example.com/`).
- Any prefix where the second label is `modelcontextprotocol` or `mcp` is **reserved** for MCP use. For example: `io.modelcontextprotocol/`, `dev.mcp/`, `org.modelcontextprotocol.api/`, and `com.mcp.tools/` are all reserved. However, `com.example.mcp/` is NOT reserved, as the second label is `example`.

**Name:**
- Unless empty, MUST start and end with an alphanumeric character (`[a-z0-9A-Z]`).
- Interior characters may be alphanumeric, hyphens (`-`), underscores (`_`), or dots (`.`).


## MethodNotFoundError

`mcp_types._v2026_07_28.MethodNotFoundError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class MethodNotFoundError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `code: Literal[-32601]`  _instance-attribute_
  The error type that occurred.
- `data: Any | None = None`  _class-attribute, instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).
- `message: str`  _instance-attribute_
  A short description of the error. The message SHOULD be limited to a concise single sentence.

A JSON-RPC error indicating that the requested method does not exist or is not available.

In MCP, a server returns this error when a client invokes a method the server does not implement — either a genuinely unknown method, or one gated behind a server capability the server did not advertise (e.g., calling `prompts/list` when the `prompts` capability was not advertised).

A request that requires a client capability the client did not declare is signalled instead by {@link MissingRequiredClientCapabilityError} (`-32021`).


## MissingRequiredClientCapabilityError

`mcp_types._v2026_07_28.MissingRequiredClientCapabilityError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class MissingRequiredClientCapabilityError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `error: Error2`  _instance-attribute_
- `id: RequestId | None = None`  _class-attribute, instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_

Returned when processing a request requires a capability the client did not
declare in `clientCapabilities`. For HTTP, the response status code MUST be
`400 Bad Request`.


## ModelHint

`mcp_types._v2026_07_28.ModelHint`

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

`mcp_types._v2026_07_28.ModelPreferences`

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

`mcp_types._v2026_07_28.Notification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Notification(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `method: str`  _instance-attribute_
- `params: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

## NotificationMetaObject

`mcp_types._v2026_07_28.NotificationMetaObject`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class NotificationMetaObject(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `io_modelcontextprotocol_subscription_id: Annotated[RequestId | None, Field(alias='io.modelcontextprotocol/subscriptionId')] = None`  _class-attribute, instance-attribute_
  Identifies the subscription stream a notification was delivered on. The server MUST include this key on every notification delivered via a {@link SubscriptionsListenRequestsubscriptions/listen} stream, so the client can correlate the notif…

Extends {@link MetaObject} with additional notification-specific fields. All key naming rules from `MetaObject` apply.


## NotificationParams

`mcp_types._v2026_07_28.NotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class NotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `meta: Annotated[NotificationMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_

Common params for any notification.


## NumberSchema

`mcp_types._v2026_07_28.NumberSchema`

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

`mcp_types._v2026_07_28.OneOfItem`

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

`mcp_types._v2026_07_28.OutputSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class OutputSchema(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `schema_: Annotated[str | None, Field(alias='$schema')] = None`  _class-attribute, instance-attribute_

An optional JSON Schema object defining the structure of the tool's output returned in
the structuredContent field of a {@link CallToolResult}. This can be any valid JSON Schema 2020-12.

Defaults to JSON Schema 2020-12 when no explicit `$schema` is provided.


## PaginatedRequest

`mcp_types._v2026_07_28.PaginatedRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PaginatedRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params: PaginatedRequestParams`  _instance-attribute_

## PaginatedRequestParams

`mcp_types._v2026_07_28.PaginatedRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PaginatedRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `cursor: str | None = None`  _class-attribute, instance-attribute_
  An opaque token representing the current pagination position. If provided, the server should return results starting after this cursor.
- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_

Common params for paginated requests.


## PaginatedResult

`mcp_types._v2026_07_28.PaginatedResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PaginatedResult(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `next_cursor: Annotated[str | None, Field(alias='nextCursor')] = None`  _class-attribute, instance-attribute_
  An opaque token representing the pagination position after the last returned result. If present, there may be more results available.
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.

## Params

`mcp_types._v2026_07_28.Params`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Params(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_

## ParseError

`mcp_types._v2026_07_28.ParseError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ParseError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `code: Literal[-32700]`  _instance-attribute_
  The error type that occurred.
- `data: Any | None = None`  _class-attribute, instance-attribute_
  Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).
- `message: str`  _instance-attribute_
  A short description of the error. The message SHOULD be limited to a concise single sentence.

A JSON-RPC error indicating that invalid JSON was received by the server. This error is returned when the server cannot parse the JSON text of a message.


## ProgressNotification

`mcp_types._v2026_07_28.ProgressNotification`

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

`mcp_types._v2026_07_28.ProgressNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ProgressNotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `message: str | None = None`  _class-attribute, instance-attribute_
  An optional message describing the current progress.
- `meta: Annotated[NotificationMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `progress: float`  _instance-attribute_
  The progress thus far. This should increase every time progress is made, even if the total is unknown.
- `progress_token: Annotated[ProgressToken, Field(alias='progressToken')]`  _instance-attribute_
  The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.
- `total: float | None = None`  _class-attribute, instance-attribute_
  Total number of items to process (or total progress required), if known.

Parameters for a {@link ProgressNotificationnotifications/progress} notification.


## Prompt

`mcp_types._v2026_07_28.Prompt`

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
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.

A prompt or prompt template that the server offers.


## PromptArgument

`mcp_types._v2026_07_28.PromptArgument`

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

`mcp_types._v2026_07_28.PromptListChangedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PromptListChangedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/prompts/list_changed']`  _instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This is only delivered on a {@link SubscriptionsListenRequestsubscriptions/listen} stream when the client requested it via the `promptsListChanged` filter field.


## PromptMessage

`mcp_types._v2026_07_28.PromptMessage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PromptMessage(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `content: ContentBlock`  _instance-attribute_
- `role: Role`  _instance-attribute_

Describes a message returned as part of a prompt.

This is similar to {@link SamplingMessage}, but also supports the embedding of
resources from the MCP server.


## PromptReference

`mcp_types._v2026_07_28.PromptReference`

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

`mcp_types._v2026_07_28.Prompts`

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

`mcp_types._v2026_07_28.ReadResourceRequest`

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

`mcp_types._v2026_07_28.ReadResourceRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ReadResourceRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `input_responses: Annotated[InputResponses | None, Field(alias='inputResponses')] = None`  _class-attribute, instance-attribute_
- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_
- `request_state: Annotated[str | None, Field(alias='requestState')] = None`  _class-attribute, instance-attribute_
- `uri: str`  _instance-attribute_
  The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it.

Parameters for a `resources/read` request.


## ReadResourceResult

`mcp_types._v2026_07_28.ReadResourceResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ReadResourceResult(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `cache_scope: Annotated[Literal['private', 'public'], Field(alias='cacheScope')]`  _instance-attribute_
  Indicates the intended scope of the cached response, analogous to HTTP `Cache-Control: public` vs `Cache-Control: private`.
- `contents: list[TextResourceContents | BlobResourceContents]`  _instance-attribute_
- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.
- `ttl_ms: Annotated[int, Field(alias='ttlMs', ge=0)]`  _instance-attribute_
  A hint from the server indicating how long (in milliseconds) the client MAY cache this response before re-fetching. Semantics are analogous to HTTP Cache-Control max-age.

The result returned by the server for a {@link ReadResourceRequestresources/read} request.


## ReadResourceResultResponse

`mcp_types._v2026_07_28.ReadResourceResultResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ReadResourceResultResponse(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: InputRequiredResult | ReadResourceResult`  _instance-attribute_

A successful response from the server for a {@link ReadResourceRequestresources/read} request.


## Request

`mcp_types._v2026_07_28.Request`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Request(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `method: str`  _instance-attribute_
- `params: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

## RequestMetaObject

`mcp_types._v2026_07_28.RequestMetaObject`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class RequestMetaObject(WireModel)
```

**Bases** `WireModel`

**Declared members (5)**

- `io_modelcontextprotocol_client_capabilities: Annotated[ClientCapabilities, Field(alias='io.modelcontextprotocol/clientCapabilities')]`  _instance-attribute_
  The client's capabilities for this specific request. Required.
- `io_modelcontextprotocol_client_info: Annotated[Implementation | None, Field(alias='io.modelcontextprotocol/clientInfo')] = None`  _class-attribute, instance-attribute_
  Identifies the client software making the request. Clients SHOULD include this field on every request unless specifically configured not to do so.
- `io_modelcontextprotocol_log_level: Annotated[LoggingLevel | None, Field(alias='io.modelcontextprotocol/logLevel')] = None`  _class-attribute, instance-attribute_
  The desired log level for this request. Optional.
- `io_modelcontextprotocol_protocol_version: Annotated[str, Field(alias='io.modelcontextprotocol/protocolVersion')]`  _instance-attribute_
  The MCP Protocol Version being used for this request. Required.
- `progress_token: Annotated[ProgressToken | None, Field(alias='progressToken')] = None`  _class-attribute, instance-attribute_
  If specified, the caller is requesting out-of-band progress notifications for this request (as represented by {@link ProgressNotificationnotifications/progress}). The value of this parameter is an opaque token that will be attached to any…

Extends {@link MetaObject} with additional request-specific fields. All key naming rules from `MetaObject` apply.


## RequestParams

`mcp_types._v2026_07_28.RequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class RequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_

Common params for any request.


## RequestedSchema

`mcp_types._v2026_07_28.RequestedSchema`

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


## Resource

`mcp_types._v2026_07_28.Resource`

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
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
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

`mcp_types._v2026_07_28.ResourceContents`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceContents(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `uri: str`  _instance-attribute_
  The URI of this resource.

The contents of a specific resource or sub-resource.


## ResourceLink

`mcp_types._v2026_07_28.ResourceLink`

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
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
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

Note: resource links returned by tools are not guaranteed to appear in the results of {@link ListResourcesRequestresources/list} requests.


## ResourceListChangedNotification

`mcp_types._v2026_07_28.ResourceListChangedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceListChangedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/resources/list_changed']`  _instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This is only delivered on a {@link SubscriptionsListenRequestsubscriptions/listen} stream when the client requested it via the `resourcesListChanged` filter field.


## ResourceRequestParams

`mcp_types._v2026_07_28.ResourceRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_
- `uri: str`  _instance-attribute_
  The URI of the resource. The URI can use any protocol; it is up to the server how to interpret it.

Common params for resource-related requests.


## ResourceTemplate

`mcp_types._v2026_07_28.ResourceTemplate`

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
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
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

`mcp_types._v2026_07_28.ResourceTemplateReference`

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

`mcp_types._v2026_07_28.ResourceUpdatedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceUpdatedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/resources/updated']`  _instance-attribute_
- `params: ResourceUpdatedNotificationParams`  _instance-attribute_

A notification from the server to the client, informing it that a resource has changed and may need to be read again. This is only sent for resources the client opted in to via the `resourceSubscriptions` field of a {@link SubscriptionsListenRequestsubscriptions/listen} request.


## ResourceUpdatedNotificationParams

`mcp_types._v2026_07_28.ResourceUpdatedNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResourceUpdatedNotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[NotificationMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `uri: str`  _instance-attribute_
  The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.

Parameters for a `notifications/resources/updated` notification.


## Resources

`mcp_types._v2026_07_28.Resources`

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

`mcp_types._v2026_07_28.Result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Result(WireModel)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[ResultMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.

Common result fields.


## ResultMetaObject

`mcp_types._v2026_07_28.ResultMetaObject`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ResultMetaObject(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `io_modelcontextprotocol_server_info: Annotated[Any | None, Field(alias='io.modelcontextprotocol/serverInfo')] = None`  _class-attribute, instance-attribute_
  Identifies the server software producing the response. Servers SHOULD include this field on every response unless specifically configured not to do so.

Extends {@link MetaObject} with additional result-specific fields. All key naming rules from `MetaObject` apply.


## Root

`mcp_types._v2026_07_28.Root`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Root(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `name: str | None = None`  _class-attribute, instance-attribute_
  An optional name for the root. This can be used to provide a human-readable identifier for the root, which may be useful for display purposes or for referencing the root in other parts of the application.
- `uri: str`  _instance-attribute_
  The URI identifying the root. This *must* start with `file://` for now. This restriction may be relaxed in future versions of the protocol to allow other URI schemes.

Represents a root directory or file that the server can operate on.


## Sampling

`mcp_types._v2026_07_28.Sampling`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Sampling(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `context: JSONObject | None = None`  _class-attribute, instance-attribute_
  Whether the client supports context inclusion via `includeContext` parameter. If not declared, servers SHOULD only use `includeContext: "none"` (or omit it).
- `tools: JSONObject | None = None`  _class-attribute, instance-attribute_
  Whether the client supports tool use via `tools` and `toolChoice` parameters.

Present if the client supports sampling from an LLM.


## SamplingMessage

`mcp_types._v2026_07_28.SamplingMessage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SamplingMessage(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `content: TextContent | ImageContent | AudioContent | ToolUseContent | ToolResultContent | list[SamplingMessageContentBlock]`  _instance-attribute_
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `role: Role`  _instance-attribute_

Describes a message issued to or received from an LLM API.


## ServerCapabilities

`mcp_types._v2026_07_28.ServerCapabilities`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ServerCapabilities(WireModel)
```

**Bases** `WireModel`

**Declared members (7)**

- `completions: JSONObject | None = None`  _class-attribute, instance-attribute_
  Present if the server supports argument autocompletion suggestions.
- `experimental: dict[str, JSONObject] | None = None`  _class-attribute, instance-attribute_
  Experimental, non-standard capabilities that the server supports.
- `extensions: dict[str, JSONObject] | None = None`  _class-attribute, instance-attribute_
  Optional MCP extensions that the server supports. Keys are extension identifiers (e.g., "io.modelcontextprotocol/tasks"), and values are per-extension settings objects. An empty object indicates support with no settings.
- `logging: JSONObject | None = None`  _class-attribute, instance-attribute_
  Present if the server supports sending log messages to the client.
- `prompts: Prompts | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any prompt templates.
- `resources: Resources | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any resources to read.
- `tools: Tools | None = None`  _class-attribute, instance-attribute_
  Present if the server offers any tools to call.

Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities.


## StringSchema

`mcp_types._v2026_07_28.StringSchema`

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

## SubscriptionFilter

`mcp_types._v2026_07_28.SubscriptionFilter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscriptionFilter(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `prompts_list_changed: Annotated[bool | None, Field(alias='promptsListChanged')] = None`  _class-attribute, instance-attribute_
  If true, receive {@link PromptListChangedNotificationnotifications/prompts/list_changed}.
- `resource_subscriptions: Annotated[list[str] | None, Field(alias='resourceSubscriptions')] = None`  _class-attribute, instance-attribute_
  Subscribe to {@link ResourceUpdatedNotificationnotifications/resources/updated} for these resource URIs. Replaces the former `resources/subscribe` RPC.
- `resources_list_changed: Annotated[bool | None, Field(alias='resourcesListChanged')] = None`  _class-attribute, instance-attribute_
  If true, receive {@link ResourceListChangedNotificationnotifications/resources/list_changed}.
- `tools_list_changed: Annotated[bool | None, Field(alias='toolsListChanged')] = None`  _class-attribute, instance-attribute_
  If true, receive {@link ToolListChangedNotificationnotifications/tools/list_changed}.

The set of notification types a client may opt in to on a
{@link SubscriptionsListenRequestsubscriptions/listen} request.

Each notification type is **opt-in**; the server **MUST NOT** send
notification types the client has not explicitly requested here.


## SubscriptionsAcknowledgedNotification

`mcp_types._v2026_07_28.SubscriptionsAcknowledgedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscriptionsAcknowledgedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/subscriptions/acknowledged']`  _instance-attribute_
- `params: SubscriptionsAcknowledgedNotificationParams`  _instance-attribute_

Sent by the server to acknowledge that a
{@link SubscriptionsListenRequestsubscriptions/listen} subscription has been
established and to report which notification types it agreed to honor.

This notification MUST be the first message the server sends carrying the
subscription's ID in `io.modelcontextprotocol/subscriptionId`. The server MUST
NOT send any notification on the subscription before acknowledging it. On
stdio, where every subscription shares one channel, this ordering is defined
per subscription ID and not per channel: messages belonging to other
subscriptions MAY be interleaved before it.


## SubscriptionsAcknowledgedNotificationParams

`mcp_types._v2026_07_28.SubscriptionsAcknowledgedNotificationParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscriptionsAcknowledgedNotificationParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[NotificationMetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `notifications: SubscriptionFilter`  _instance-attribute_
  The subset of requested notification types the server agreed to honor. Only includes notification types the server actually supports; if the client requested an unsupported type (e.g., `promptsListChanged` when the server has no prompts),…

Parameters for a {@link SubscriptionsAcknowledgedNotificationnotifications/subscriptions/acknowledged} notification.


## SubscriptionsListenRequest

`mcp_types._v2026_07_28.SubscriptionsListenRequest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscriptionsListenRequest(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['subscriptions/listen']`  _instance-attribute_
- `params: SubscriptionsListenRequestParams`  _instance-attribute_

Sent from the client to open a long-lived channel for receiving notifications
outside the context of a specific request. Replaces the previous HTTP GET
endpoint and ensures consistent behavior between HTTP and STDIO.


## SubscriptionsListenRequestParams

`mcp_types._v2026_07_28.SubscriptionsListenRequestParams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscriptionsListenRequestParams(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[RequestMetaObject, Field(alias='_meta')]`  _instance-attribute_
- `notifications: SubscriptionFilter`  _instance-attribute_
  The notifications the client opts in to on this stream. The server **MUST NOT** send notification types the client has not explicitly requested.

Parameters for a {@link SubscriptionsListenRequestsubscriptions/listen} request.


## SubscriptionsListenResult

`mcp_types._v2026_07_28.SubscriptionsListenResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscriptionsListenResult(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `meta: Annotated[SubscriptionsListenResultMeta, Field(alias='_meta')]`  _instance-attribute_
- `result_type: Annotated[str, Field(alias='resultType')]`  _instance-attribute_
  Indicates the type of the result, which allows the client to determine how to parse the result object.

The response to a {@link SubscriptionsListenRequestsubscriptions/listen}
request, signalling that the subscription has ended gracefully (for example,
during server shutdown). Because the listen stream is long-lived, this result
is sent only when the server tears the subscription down; an abrupt transport
close carries no response. The result body is otherwise empty.


## SubscriptionsListenResultMeta

`mcp_types._v2026_07_28.SubscriptionsListenResultMeta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SubscriptionsListenResultMeta(WireModel)
```

**Bases** `WireModel`

**Declared members (2)**

- `io_modelcontextprotocol_server_info: Annotated[Any | None, Field(alias='io.modelcontextprotocol/serverInfo')] = None`  _class-attribute, instance-attribute_
  Identifies the server software producing the response. Servers SHOULD include this field on every response unless specifically configured not to do so.
- `io_modelcontextprotocol_subscription_id: Annotated[RequestId, Field(alias='io.modelcontextprotocol/subscriptionId')]`  _instance-attribute_
  Identifies the subscription stream this response closes, so the client can correlate it with the originating subscription — mirroring the same key on the stream's notifications. The value is the JSON-RPC ID of the `subscriptions/listen` re…

Extends {@link ResultMetaObject} with the subscription-stream identifier carried by a
{@link SubscriptionsListenResult}. All key naming rules from `MetaObject` apply.


## TextContent

`mcp_types._v2026_07_28.TextContent`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TextContent(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
  Optional annotations for the client.
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `text: str`  _instance-attribute_
  The text content of the message.
- `type: Literal['text']`  _instance-attribute_

Text provided to or from an LLM.


## TextResourceContents

`mcp_types._v2026_07_28.TextResourceContents`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class TextResourceContents(WireModel)
```

**Bases** `WireModel`

**Declared members (4)**

- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `mime_type: Annotated[str | None, Field(alias='mimeType')] = None`  _class-attribute, instance-attribute_
  The MIME type of this resource, if known.
- `text: str`  _instance-attribute_
  The text of the item. This must only be set if the item can actually be represented as text (not binary data).
- `uri: str`  _instance-attribute_
  The URI of this resource.

## TitledMultiSelectEnumSchema

`mcp_types._v2026_07_28.TitledMultiSelectEnumSchema`

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

`mcp_types._v2026_07_28.TitledSingleSelectEnumSchema`

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

`mcp_types._v2026_07_28.Tool`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Tool(WireModel)
```

**Bases** `WireModel`

**Declared members (8)**

- `annotations: ToolAnnotations | None = None`  _class-attribute, instance-attribute_
  Optional additional tool information.
- `description: str | None = None`  _class-attribute, instance-attribute_
  A human-readable description of the tool.
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
  Optional set of sized icons that the client can display in a user interface.
- `input_schema: Annotated[InputSchema, Field(alias='inputSchema')]`  _instance-attribute_
  A JSON Schema object defining the expected parameters for the tool.
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
- `name: str`  _instance-attribute_
  Intended for programmatic or logical use, but used as a display name in past specs or fallback (if title isn't present).
- `output_schema: Annotated[OutputSchema | None, Field(alias='outputSchema')] = None`  _class-attribute, instance-attribute_
  An optional JSON Schema object defining the structure of the tool's output returned in the structuredContent field of a {@link CallToolResult}. This can be any valid JSON Schema 2020-12.
- `title: str | None = None`  _class-attribute, instance-attribute_
  Intended for UI and end-user contexts — optimized to be human-readable and easily understood, even by those unfamiliar with domain-specific terminology.

Definition for a tool the client can call.


## ToolAnnotations

`mcp_types._v2026_07_28.ToolAnnotations`

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

Additional properties describing a {@link Tool} to clients.

NOTE: all properties in `ToolAnnotations` are **hints**.
They are not guaranteed to provide a faithful description of
tool behavior (including descriptive properties like `title`).

Clients should never make tool use decisions based on `ToolAnnotations`
received from untrusted servers.


## ToolChoice

`mcp_types._v2026_07_28.ToolChoice`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ToolChoice(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `mode: Literal['auto', 'none', 'required'] | None = None`  _class-attribute, instance-attribute_
  Controls the tool use ability of the model: - `"auto"`: Model decides whether to use tools (default) - `"required"`: Model MUST use at least one tool before completing - `"none"`: Model MUST NOT use any tools

Controls tool selection behavior for sampling requests.


## ToolListChangedNotification

`mcp_types._v2026_07_28.ToolListChangedNotification`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ToolListChangedNotification(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: Literal['notifications/tools/list_changed']`  _instance-attribute_
- `params: NotificationParams | None = None`  _class-attribute, instance-attribute_

An optional notification from the server to the client, informing it that the list of tools it offers has changed. This is only delivered on a {@link SubscriptionsListenRequestsubscriptions/listen} stream when the client requested it via the `toolsListChanged` filter field.


## ToolResultContent

`mcp_types._v2026_07_28.ToolResultContent`

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
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  Optional metadata about the tool result. Clients SHOULD preserve this field when including tool results in subsequent sampling requests to enable caching optimizations.
- `structured_content: Annotated[Any | None, Field(alias='structuredContent')] = None`  _class-attribute, instance-attribute_
  An optional structured result value.
- `tool_use_id: Annotated[str, Field(alias='toolUseId')]`  _instance-attribute_
  The ID of the tool use this result corresponds to.
- `type: Literal['tool_result']`  _instance-attribute_

The result of a tool use, provided by the user back to the assistant.


## ToolUseContent

`mcp_types._v2026_07_28.ToolUseContent`

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
- `meta: Annotated[MetaObject | None, Field(alias='_meta')] = None`  _class-attribute, instance-attribute_
  Optional metadata about the tool use. Clients SHOULD preserve this field when including tool uses in subsequent sampling requests to enable caching optimizations.
- `name: str`  _instance-attribute_
  The name of the tool to call.
- `type: Literal['tool_use']`  _instance-attribute_

A request from the assistant to call a tool.


## Tools

`mcp_types._v2026_07_28.Tools`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Tools(WireModel)
```

**Bases** `WireModel`

**Declared members (1)**

- `list_changed: Annotated[bool | None, Field(alias='listChanged')] = None`  _class-attribute, instance-attribute_
  Whether this server supports notifications for changes to the tool list.

Present if the server offers any tools to call.


## UnsupportedProtocolVersionError

`mcp_types._v2026_07_28.UnsupportedProtocolVersionError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class UnsupportedProtocolVersionError(WireModel)
```

**Bases** `WireModel`

**Declared members (3)**

- `error: Error3`  _instance-attribute_
- `id: RequestId | None = None`  _class-attribute, instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_

Returned when the request's protocol version is unknown to the server or
unsupported (e.g., a known experimental or draft version the server has
chosen not to implement). For HTTP, the response status code MUST be
`400 Bad Request`.


## UntitledMultiSelectEnumSchema

`mcp_types._v2026_07_28.UntitledMultiSelectEnumSchema`

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

`mcp_types._v2026_07_28.UntitledSingleSelectEnumSchema`

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


