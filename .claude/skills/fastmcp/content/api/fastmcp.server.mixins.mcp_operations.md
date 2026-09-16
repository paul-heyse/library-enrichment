# `fastmcp.server.mixins.mcp_operations`

Distribution: `fastmcp`

## PaginateT

`fastmcp.server.mixins.mcp_operations.PaginateT`

```python
PaginateT = TypeVar('PaginateT')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## logger

`fastmcp.server.mixins.mcp_operations.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## MCPOperationsMixin

Import as `fastmcp.server.mixins.MCPOperationsMixin`  ·  defined at `fastmcp.server.mixins.mcp_operations.MCPOperationsMixin`

```python
class MCPOperationsMixin
```

**Also exported as** `fastmcp.server.mixins.MCPOperationsMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Mixin providing MCP protocol handler setup and wire-format handlers.

Handlers are registered via ``add_request_handler(method, params_type,
handler)`` on the low-level SDK server. Each adapter takes
``(ctx: ServerRequestContext, params)`` and returns the bare SDK result
model (no ``ServerResult`` wrapping — the SDK v2 runner serializes the
result itself).


## _apply_pagination

`fastmcp.server.mixins.mcp_operations._apply_pagination`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _apply_pagination(items: Sequence[PaginateT], cursor: str | None, page_size: int | None) -> tuple[list[PaginateT], str | None]
```

Apply pagination to items, raising MCPError for invalid cursors.

If page_size is None, returns all items without pagination.


## _normalize_call_tool_result

`fastmcp.server.mixins.mcp_operations._normalize_call_tool_result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_call_tool_result(result: Any) -> mcp_types.CallToolResult
```

Normalize a tool's ``to_mcp_result()`` output into a ``CallToolResult``.

``ToolResult.to_mcp_result()`` returns one of three shapes for backward
compatibility: a ``CallToolResult`` (error/meta case), a bare
``list[ContentBlock]`` (unstructured), or a ``(content, structured)`` tuple.
The SDK v2 runner requires a ``BaseModel`` result, so wrap the shorthand
forms here (the SDK's old ``call_tool`` decorator used to do this).


## _version_from_ctx

`fastmcp.server.mixins.mcp_operations._version_from_ctx`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _version_from_ctx(ctx: ServerRequestContext) -> VersionSpec | None
```

Extract the FastMCP component version from the request's lifted _meta.


