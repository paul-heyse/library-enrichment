# `fastmcp.tools.base`

Distribution: `fastmcp`

## _JSONABLE_ADAPTER

`fastmcp.tools.base._JSONABLE_ADAPTER`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_JSONABLE_ADAPTER = get_cached_typeadapter(Any)
```

## _PREFAB_TEXT_FALLBACK

`fastmcp.tools.base._PREFAB_TEXT_FALLBACK`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PREFAB_TEXT_FALLBACK = '[Rendered Prefab UI]'
```

## __all__

`fastmcp.tools.base.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['InputRequiredToolResult', 'Tool', 'ToolResult']
```

## logger

`fastmcp.tools.base.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## InputRequiredToolResult

Import as `fastmcp.tools.InputRequiredToolResult`  ·  defined at `fastmcp.tools.base.InputRequiredToolResult`

```python
class InputRequiredToolResult(ToolResult)
```

**Also exported as** `fastmcp.tools.InputRequiredToolResult`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ToolResult`

**Declared members (1)**

- `input_required: mcp_types.InputRequiredResult = Field(description='The client-input request this leg resolved to (SEP-2322)')`  _class-attribute, instance-attribute_

**Inherited (6)**

- from `fastmcp.tools.base.ToolResult`: `content`, `from_mcp_result`, `is_error`, `meta`, `structured_content`, `to_mcp_result`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The full result of a single multi-round-trip leg (SEP-2322).

The protocol is stateless: each MRTR leg is a complete request→response
cycle. When a guard tool returns an `InputRequiredResult` from its body to
ask the client for input, that ask is the *legitimate result* of this tool
call — not a pause, not an error, not a third control-flow outcome. FastMCP
wraps it in this `ToolResult` subclass so it flows through the middleware
chain as an ordinary return value: `call_next(...)` returns it, default
middleware completes normally on the leg, and middleware authors can
identify an ask with a simple `isinstance(result, InputRequiredToolResult)`
check.

Invariant: the wrapped `InputRequiredResult` is never serialized as tool
content. `content` is always empty; the wire handler (`_on_call_tool`)
reads `.input_required` and returns it to the runner as the
`input_required` result. Do not read `.content` / `.structured_content` on
this subclass — they carry nothing.


## Tool

Import as `fastmcp.tools.Tool`  ·  defined at `fastmcp.tools.base.Tool`

```python
class Tool(FastMCPComponent)
```

**Also exported as** `fastmcp.tools.Tool`

_37 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPComponent`

**Declared members (14)**

- `KEY_PREFIX: str = 'tool'`  _class-attribute_
- `annotations: Annotated[ToolAnnotations | None, Field(description='Additional annotations about the tool')] = None`  _class-attribute, instance-attribute_
- `auth: Annotated[SkipJsonSchema[AuthCheck | list[AuthCheck] | None], Field(description='Authorization checks for this tool', exclude=True)] = None`  _class-attribute, instance-attribute_
- `def convert_result(self, raw_value: Any) -> ToolResult`
  Convert a raw result to ToolResult.
- `execution: Annotated[ToolExecution | None, Field(description='Task execution configuration (SEP-1686)')] = None`  _class-attribute, instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, annotations: ToolAnnotations | None = None, output_schema: dict[str, Any] | NotSetT | None = NotSet, meta: dict[str, Any] | None = None, task: bool | TaskConfig | None = None, timeout: float | None = None, auth: AuthCheck | list[AuthCheck] | None = None, run_in_thread: bool | None = None) -> FunctionTool`  _classmethod_
  Create a Tool from a function.
- `def from_tool(cls, tool: Tool | Callable[..., Any], name: str | None = None, title: str | NotSetT | None = NotSet, description: str | NotSetT | None = NotSet, tags: set[str] | None = None, annotations: ToolAnnotations | NotSetT | None = NotSet, output_schema: dict[str, Any] | NotSetT | None = NotSet, meta: dict[str, Any] | NotSetT | None = NotSet, transform_args: dict[str, ArgTransform] | None = None, transform_fn: Callable[..., Any] | None = None) -> TransformedTool`  _classmethod_
- `def get_span_attributes(self) -> dict[str, Any]`
- `output_schema: Annotated[dict[str, Any] | None, Field(description='JSON schema for tool output')] = None`  _class-attribute, instance-attribute_
- `parameters: Annotated[dict[str, Any], Field(description='JSON schema for tool parameters')]`  _instance-attribute_
- `return_type: Annotated[SkipJsonSchema[Any], Field(exclude=True)] = None`  _class-attribute, instance-attribute_
- `async def run(self, arguments: dict[str, Any]) -> ToolResult`  _async_
  Run the tool with arguments.
- `timeout: Annotated[float | None, Field(description='Execution timeout in seconds. If None, no timeout is applied.')] = None`  _class-attribute, instance-attribute_
- `def to_mcp_tool(self, overrides: Any = {}) -> MCPTool`
  Convert the FastMCP tool to an MCP tool.

**Inherited (13)**

- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Internal tool registration info.


## ToolResult

Import as `fastmcp.tools.ToolResult`  ·  defined at `fastmcp.tools.base.ToolResult`

```python
class ToolResult(BaseModel)
```

**Also exported as** `fastmcp.tools.ToolResult`

_16 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (6)**

- `content: list[ContentBlock] = Field(description='List of content blocks for the tool result')`  _class-attribute, instance-attribute_
- `def from_mcp_result(cls, result: CallToolResult) -> ToolResult`  _classmethod_
  Wrap a protocol result while preserving its exact wire representation.
- `is_error: bool = Field(default=False, description='Whether this result represents a tool execution error. When True, it maps to CallToolResult.is_error so the error is returned to the client rather than raised.')`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None = Field(default=None, description='Runtime metadata about the tool execution')`  _class-attribute, instance-attribute_
- `structured_content: dict[str, Any] | None = Field(default=None, description="Structured content matching the tool's output schema")`  _class-attribute, instance-attribute_
- `def to_mcp_result(self) -> list[ContentBlock] | tuple[list[ContentBlock], dict[str, Any]] | CallToolResult`

## _convert_to_content

`fastmcp.tools.base._convert_to_content`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _convert_to_content(result: Any) -> list[ContentBlock]
```

Convert a result to a sequence of content objects.


## _convert_to_single_content_block

`fastmcp.tools.base._convert_to_single_content_block`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _convert_to_single_content_block(item: Any) -> ContentBlock
```

## _default_title

`fastmcp.tools.base._default_title`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _default_title(name: str) -> str
```

Derive a display title from a tool name.

The MCP spec says clients should fall back to `name` for display when
`title` is absent, but some clients (e.g. ChatGPT) instead drop the tool
entirely. Always emitting a title avoids depending on that fallback.


## _get_fastmcp_app_name

`fastmcp.tools.base._get_fastmcp_app_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_fastmcp_app_name(tool: Tool) -> str | None
```

Read the FastMCPApp name from a tool's metadata, if present.


## _get_tool_resolver

`fastmcp.tools.base._get_tool_resolver`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_tool_resolver(app_name: str | None = None) -> Callable[..., str] | None
```

Get the Prefab peer-reference resolver bound to an app name.


## _prefab_to_json

`fastmcp.tools.base._prefab_to_json`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _prefab_to_json(app: Any, fastmcp_app_name: str | None = None) -> dict[str, Any]
```

Serialize a PrefabApp, addressing its peer-tool references by identity.

The resolver writes each reference as ``<hash>_<local_name>``, and the
identity behind it is recorded in the payload's meta so that servers
can re-address the reference on the way out without losing track of
what it points at.


## _prefab_to_tool_result

`fastmcp.tools.base._prefab_to_tool_result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _prefab_to_tool_result(app: Any, fastmcp_app_name: str | None = None) -> ToolResult
```

Convert a PrefabApp to a FastMCP ToolResult.


## _serialize_to_jsonable

`fastmcp.tools.base._serialize_to_jsonable`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _serialize_to_jsonable(data: Any, annotation: Any = Any) -> Any
```

Serialize through Pydantic, falling back for unsupported annotations.


## default_serializer

`fastmcp.tools.base.default_serializer`

```python
def default_serializer(data: Any) -> str
```

