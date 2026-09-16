# `fastmcp.server.middleware.response_limiting`

Distribution: `fastmcp`

## __all__

`fastmcp.server.middleware.response_limiting.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ResponseLimitingMiddleware']
```

## logger

`fastmcp.server.middleware.response_limiting.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ResponseLimitingMiddleware

`fastmcp.server.middleware.response_limiting.ResponseLimitingMiddleware`

```python
class ResponseLimitingMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (5)**

- `max_size = max_size`  _instance-attribute_
- `async def on_call_tool(self, context: MiddlewareContext[mt.CallToolRequestParams], call_next: CallNext[mt.CallToolRequestParams, ToolResult]) -> ToolResult`  _async_
  Intercept tool calls and limit response size.
- `async def on_list_tools(self, context: MiddlewareContext[mt.ListToolsRequest], call_next: CallNext[mt.ListToolsRequest, Sequence[Tool]]) -> Sequence[Tool]`  _async_
  Hide schemas for tools whose response shape may be truncated to text.
- `tools = set(tools) if tools is not None else None`  _instance-attribute_
- `truncation_suffix = truncation_suffix`  _instance-attribute_

**Inherited (10)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_message`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that limits the response size of tool calls.

Intercepts tool call responses and enforces size limits. If a response
exceeds the limit, it extracts text content, truncates it, and returns
a single TextContent block.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.middleware.response_limiting import (
        ResponseLimitingMiddleware,
    )

    mcp = FastMCP("MyServer")

    # Limit all tool responses to 500KB
    mcp.add_middleware(ResponseLimitingMiddleware(max_size=500_000))

    # Limit only specific tools
    mcp.add_middleware(
        ResponseLimitingMiddleware(
            max_size=100_000,
            tools=["search", "fetch_data"],
        )
    )
    ```


