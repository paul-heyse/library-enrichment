# `fastmcp.server.middleware.caching`

Distribution: `fastmcp`

## ANONYMOUS_AUTH_KEY

`fastmcp.server.middleware.caching.ANONYMOUS_AUTH_KEY`

```python
ANONYMOUS_AUTH_KEY = '__anonymous__'
```

**Inferred type** (`ty`, not declared in the source): `Literal["__anonymous__"]`

## BaseModelT

`fastmcp.server.middleware.caching.BaseModelT`

```python
BaseModelT = TypeVar('BaseModelT', bound=FastMCPBaseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## DEFAULT_VERSION_CACHE_KEY

`fastmcp.server.middleware.caching.DEFAULT_VERSION_CACHE_KEY`

```python
DEFAULT_VERSION_CACHE_KEY = '__default__'
```

**Inferred type** (`ty`, not declared in the source): `Literal["__default__"]`

## FIVE_MINUTES_IN_SECONDS

`fastmcp.server.middleware.caching.FIVE_MINUTES_IN_SECONDS`

```python
FIVE_MINUTES_IN_SECONDS = 300
```

**Inferred type** (`ty`, not declared in the source): `Literal[300]`

## ONE_HOUR_IN_SECONDS

`fastmcp.server.middleware.caching.ONE_HOUR_IN_SECONDS`

```python
ONE_HOUR_IN_SECONDS = 3600
```

**Inferred type** (`ty`, not declared in the source): `Literal[3600]`

## ONE_MB_IN_BYTES

`fastmcp.server.middleware.caching.ONE_MB_IN_BYTES`

```python
ONE_MB_IN_BYTES = 1024 * 1024
```

**Inferred type** (`ty`, not declared in the source): `Literal[1048576]`

## logger

`fastmcp.server.middleware.caching.logger`

```python
logger: Logger = get_logger(name=__name__)
```

## CacheableMessage

`fastmcp.server.middleware.caching.CacheableMessage`

```python
class CacheableMessage(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (2)**

- `content: mcp_types.TextContent | mcp_types.ImageContent | mcp_types.AudioContent | mcp_types.EmbeddedResource`  _instance-attribute_
- `role: str`  _instance-attribute_

A wrapper for Message that can be cached.


## CacheablePromptResult

`fastmcp.server.middleware.caching.CacheablePromptResult`

```python
class CacheablePromptResult(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (6)**

- `description: str | None = None`  _class-attribute, instance-attribute_
- `def get_size(self) -> int`
- `messages: list[CacheableMessage]`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `def unwrap(self) -> PromptResult`
- `def wrap(cls, value: PromptResult) -> Self`  _classmethod_

A wrapper for PromptResult that can be cached.


## CacheableResourceContent

`fastmcp.server.middleware.caching.CacheableResourceContent`

```python
class CacheableResourceContent(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (3)**

- `content: str | bytes`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `mime_type: str | None = None`  _class-attribute, instance-attribute_

A wrapper for ResourceContent that can be cached.


## CacheableResourceResult

`fastmcp.server.middleware.caching.CacheableResourceResult`

```python
class CacheableResourceResult(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (5)**

- `contents: list[CacheableResourceContent]`  _instance-attribute_
- `def get_size(self) -> int`
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `def unwrap(self) -> ResourceResult`
- `def wrap(cls, value: ResourceResult) -> Self`  _classmethod_

A wrapper for ResourceResult that can be cached.


## CacheableToolResult

`fastmcp.server.middleware.caching.CacheableToolResult`

```python
class CacheableToolResult(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (6)**

- `content: list[mcp_types.ContentBlock]`  _instance-attribute_
- `is_error: bool = False`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None`  _instance-attribute_
- `structured_content: dict[str, Any] | None`  _instance-attribute_
- `def unwrap(self) -> ToolResult`
- `def wrap(cls, value: ToolResult) -> Self`  _classmethod_

## CallToolSettings

`fastmcp.server.middleware.caching.CallToolSettings`

```python
class CallToolSettings(SharedMethodSettings)
```

**Bases** `SharedMethodSettings`

**Declared members (2)**

- `excluded_tools: NotRequired[list[str]]`  _instance-attribute_
- `included_tools: NotRequired[list[str]]`  _instance-attribute_

**Inherited (2)**

- from `fastmcp.server.middleware.caching.SharedMethodSettings`: `enabled`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Configuration options for Tool-related caching.


## GetPromptSettings

`fastmcp.server.middleware.caching.GetPromptSettings`

```python
class GetPromptSettings(SharedMethodSettings)
```

**Bases** `SharedMethodSettings`

**Inherited (2)**

- from `fastmcp.server.middleware.caching.SharedMethodSettings`: `enabled`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Configuration options for Prompt-related caching.


## ListPromptsSettings

`fastmcp.server.middleware.caching.ListPromptsSettings`

```python
class ListPromptsSettings(SharedMethodSettings)
```

**Bases** `SharedMethodSettings`

**Inherited (2)**

- from `fastmcp.server.middleware.caching.SharedMethodSettings`: `enabled`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Configuration options for Prompt-related caching.


## ListResourcesSettings

`fastmcp.server.middleware.caching.ListResourcesSettings`

```python
class ListResourcesSettings(SharedMethodSettings)
```

**Bases** `SharedMethodSettings`

**Inherited (2)**

- from `fastmcp.server.middleware.caching.SharedMethodSettings`: `enabled`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Configuration options for Resource-related caching.


## ListToolsSettings

`fastmcp.server.middleware.caching.ListToolsSettings`

```python
class ListToolsSettings(SharedMethodSettings)
```

**Bases** `SharedMethodSettings`

**Inherited (2)**

- from `fastmcp.server.middleware.caching.SharedMethodSettings`: `enabled`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Configuration options for Tool-related caching.


## ReadResourceSettings

`fastmcp.server.middleware.caching.ReadResourceSettings`

```python
class ReadResourceSettings(SharedMethodSettings)
```

**Bases** `SharedMethodSettings`

**Inherited (2)**

- from `fastmcp.server.middleware.caching.SharedMethodSettings`: `enabled`, `ttl`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Configuration options for Resource-related caching.


## ResponseCachingMiddleware

`fastmcp.server.middleware.caching.ResponseCachingMiddleware`

```python
class ResponseCachingMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (7)**

- `async def on_call_tool(self, context: MiddlewareContext[mcp_types.CallToolRequestParams], call_next: CallNext[mcp_types.CallToolRequestParams, ToolResult]) -> ToolResult`  _async_
  Call a tool from the cache, if caching is enabled, and the result is in the cache. Otherwise, otherwise call the next middleware and store the result in the cache if caching is enabled.
- `async def on_get_prompt(self, context: MiddlewareContext[mcp_types.GetPromptRequestParams], call_next: CallNext[mcp_types.GetPromptRequestParams, PromptResult]) -> PromptResult`  _async_
  Get a prompt from the cache, if caching is enabled, and the result is in the cache. Otherwise, otherwise call the next middleware and store the result in the cache if caching is enabled.
- `async def on_list_prompts(self, context: MiddlewareContext[mcp_types.ListPromptsRequest], call_next: CallNext[mcp_types.ListPromptsRequest, Sequence[Prompt]]) -> Sequence[Prompt]`  _async_
  List prompts from the cache, if caching is enabled, and the result is in the cache. Otherwise, otherwise call the next middleware and store the result in the cache if caching is enabled.
- `async def on_list_resources(self, context: MiddlewareContext[mcp_types.ListResourcesRequest], call_next: CallNext[mcp_types.ListResourcesRequest, Sequence[Resource]]) -> Sequence[Resource]`  _async_
  List resources from the cache, if caching is enabled, and the result is in the cache. Otherwise, otherwise call the next middleware and store the result in the cache if caching is enabled.
- `async def on_list_tools(self, context: MiddlewareContext[mcp_types.ListToolsRequest], call_next: CallNext[mcp_types.ListToolsRequest, Sequence[Tool]]) -> Sequence[Tool]`  _async_
  List tools from the cache, if caching is enabled, and the result is in the cache. Otherwise, otherwise call the next middleware and store the result in the cache if caching is enabled.
- `async def on_read_resource(self, context: MiddlewareContext[mcp_types.ReadResourceRequestParams], call_next: CallNext[mcp_types.ReadResourceRequestParams, ResourceResult]) -> ResourceResult`  _async_
  Read a resource from the cache, if caching is enabled, and the result is in the cache. Otherwise, otherwise call the next middleware and store the result in the cache if caching is enabled.
- `def statistics(self) -> ResponseCachingStatistics`
  Get the statistics for the cache.

**Inherited (6)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_discover`, `on_initialize`, `on_list_resource_templates`, `on_message`, `on_notification`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The response caching middleware offers a simple way to cache responses to mcp methods. The Middleware
supports cache invalidation via notifications from the server. The Middleware implements TTL-based caching
but cache implementations may offer additional features like LRU eviction, size limits, and more.

When items are retrieved from the cache they will no longer be the original objects, but rather no-op objects
this means that response caching may not be compatible with other middleware that expects original subclasses.

Notes:
- Caches `tools/call`, `resources/read`, `prompts/get`, `tools/list`, `resources/list`, and `prompts/list` requests.
- Cache keys are derived from the method name, requested component version,
  arguments, and the caller's access token. Entries are partitioned per-token
  so that responses filtered by per-component authorization (e.g.
  `auth=require_scopes(...)`) cannot leak across users with different
  permissions. Unauthenticated callers (including STDIO) share a single
  anonymous partition.


## ResponseCachingStatistics

`fastmcp.server.middleware.caching.ResponseCachingStatistics`

```python
class ResponseCachingStatistics(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (6)**

- `call_tool: KVStoreCollectionStatistics | None = Field(default=None)`  _class-attribute, instance-attribute_
- `get_prompt: KVStoreCollectionStatistics | None = Field(default=None)`  _class-attribute, instance-attribute_
- `list_prompts: KVStoreCollectionStatistics | None = Field(default=None)`  _class-attribute, instance-attribute_
- `list_resources: KVStoreCollectionStatistics | None = Field(default=None)`  _class-attribute, instance-attribute_
- `list_tools: KVStoreCollectionStatistics | None = Field(default=None)`  _class-attribute, instance-attribute_
- `read_resource: KVStoreCollectionStatistics | None = Field(default=None)`  _class-attribute, instance-attribute_

## SharedMethodSettings

`fastmcp.server.middleware.caching.SharedMethodSettings`

```python
class SharedMethodSettings(TypedDict)
```

**Bases** `TypedDict`

**Declared members (2)**

- `enabled: NotRequired[bool]`  _instance-attribute_
- `ttl: NotRequired[int]`  _instance-attribute_

Shared config for a cache method.


## _get_arguments_str

`fastmcp.server.middleware.caching._get_arguments_str`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_arguments_str(arguments: dict[str, Any] | None) -> str
```

Get a canonical string representation of the arguments.


## _get_auth_partition_key

`fastmcp.server.middleware.caching._get_auth_partition_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_auth_partition_key() -> str
```

Return a stable, hashed identifier for the current access token.

Cache entries are partitioned by access token so that responses filtered
by per-component authorization (e.g. `auth=require_scopes(...)`) are not
leaked across users with different permissions. Unauthenticated callers
(including STDIO) share a single anonymous partition.


## _get_component_version_cache_key

`fastmcp.server.middleware.caching._get_component_version_cache_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_component_version_cache_key(msg: mcp_types.RequestParams) -> str
```

Return a cache partition for the requested component version.


## _hash_cache_key

`fastmcp.server.middleware.caching._hash_cache_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _hash_cache_key(value: str) -> str
```

Build a fixed-length SHA-256 cache key from request-derived input.


## _is_continuation_leg

`fastmcp.server.middleware.caching._is_continuation_leg`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_continuation_leg(context: MiddlewareContext[Any]) -> bool
```

Whether this request is answering a previous round's ask (SEP-2322).

A continuation must bypass the cache entirely. Cache keys are built from the
component's identity and arguments alone, so a continuation shares its key
with a fresh call: reading could serve a prior flow's final answer to this
leg, and writing would serve THIS flow's final answer to a later fresh call,
which would then never be asked the questions at all.

Either signal marks a continuation. A state-only round (one that carried
`request_state` without asking anything) retries with `input_responses`
still `None`.


## _make_call_tool_cache_key

`fastmcp.server.middleware.caching._make_call_tool_cache_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_call_tool_cache_key(msg: mcp_types.CallToolRequestParams, auth_key: str = ANONYMOUS_AUTH_KEY) -> str
```

Make a cache key for a tool call using name, version, and arguments.


## _make_get_prompt_cache_key

`fastmcp.server.middleware.caching._make_get_prompt_cache_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_get_prompt_cache_key(msg: mcp_types.GetPromptRequestParams, auth_key: str = ANONYMOUS_AUTH_KEY) -> str
```

Make a cache key for a prompt get using name, version, and arguments.


## _make_read_resource_cache_key

`fastmcp.server.middleware.caching._make_read_resource_cache_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_read_resource_cache_key(msg: mcp_types.ReadResourceRequestParams, auth_key: str = ANONYMOUS_AUTH_KEY) -> str
```

Make a cache key for a resource read using version and URI.


## _to_base_model

`fastmcp.server.middleware.caching._to_base_model`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _to_base_model(value: FastMCPBaseModel, model_type: type[BaseModelT]) -> BaseModelT
```

Validate a component's public base fields without serializing its subclass.


