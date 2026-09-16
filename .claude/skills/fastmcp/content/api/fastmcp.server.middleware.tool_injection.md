# `fastmcp.server.middleware.tool_injection`

Distribution: `fastmcp`

## logger

`fastmcp.server.middleware.tool_injection.logger`

```python
logger: Logger = get_logger(name=__name__)
```

## ToolInjectionMiddleware

`fastmcp.server.middleware.tool_injection.ToolInjectionMiddleware`

```python
class ToolInjectionMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (2)**

- `async def on_call_tool(self, context: MiddlewareContext[mcp_types.CallToolRequestParams], call_next: CallNext[mcp_types.CallToolRequestParams, ToolResult]) -> ToolResult`  _async_
  Intercept tool calls to injected tools.
- `async def on_list_tools(self, context: MiddlewareContext[mcp_types.ListToolsRequest], call_next: CallNext[mcp_types.ListToolsRequest, Sequence[Tool]]) -> Sequence[Tool]`  _async_
  Inject tools into the response.

**Inherited (10)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_message`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A middleware for injecting tools into the context.


