# `fastmcp.client.mixins.tools`

Distribution: `fastmcp`

## AUTO_PAGINATION_MAX_PAGES

`fastmcp.client.mixins.tools.AUTO_PAGINATION_MAX_PAGES`

```python
AUTO_PAGINATION_MAX_PAGES = 250
```

**Inferred type** (`ty`, not declared in the source): `Literal[250]`

## logger

`fastmcp.client.mixins.tools.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ClientToolsMixin

Import as `fastmcp.client.mixins.ClientToolsMixin`  ·  defined at `fastmcp.client.mixins.tools.ClientToolsMixin`

```python
class ClientToolsMixin
```

**Also exported as** `fastmcp.client.mixins.ClientToolsMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `async def call_tool(self: Client, name: str, arguments: dict[str, Any] | None = None, version: str | None = None, timeout: datetime.timedelta | float | int | None = None, progress_handler: ProgressHandler | None = None, raise_on_error: bool = True, meta: dict[str, Any] | None = None) -> CallToolResult`  _async_
  Call a tool on the server.
- `async def call_tool_mcp(self: Client, name: str, arguments: dict[str, Any], progress_handler: ProgressHandler | None = None, timeout: datetime.timedelta | float | int | None = None, meta: dict[str, Any] | None = None) -> mcp_types.CallToolResult`  _async_
  Send a tools/call request and return the complete MCP protocol result.
- `async def list_tools(self: Client, max_pages: int = AUTO_PAGINATION_MAX_PAGES, cache_mode: CacheMode = 'use') -> list[mcp_types.Tool]`  _async_
  Retrieve all tools available on the server.
- `async def list_tools_mcp(self: Client, cursor: str | None = None, cache_mode: CacheMode = 'use') -> mcp_types.ListToolsResult`  _async_
  Send a tools/list request and return the complete MCP protocol result.

Mixin providing tool-related methods for Client.


## _parse_call_tool_result

`fastmcp.client.mixins.tools._parse_call_tool_result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _parse_call_tool_result(name: str, result: mcp_types.CallToolResult, tool_output_schemas: dict[str, dict[str, Any] | None], list_tools_fn: Any, client_name: str | None = None, raise_on_error: bool = False) -> CallToolResult
```

Parse an mcp_types.CallToolResult into our CallToolResult dataclass.

Args:
    name: Tool name (for schema lookup)
    result: Raw MCP protocol result
    tool_output_schemas: Dictionary mapping tool names to their output schemas
    list_tools_fn: Async function to refresh tool schemas if needed
    client_name: Optional client name for logging
    raise_on_error: Whether to raise ToolError on errors

Returns:
    CallToolResult: Parsed result with structured data


