# `fastmcp.server.middleware.authorization`

Distribution: `fastmcp`

## logger

`fastmcp.server.middleware.authorization.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AuthMiddleware

Import as `fastmcp.server.middleware.AuthMiddleware`  ·  defined at `fastmcp.server.middleware.authorization.AuthMiddleware`

```python
class AuthMiddleware(Middleware)
```

**Also exported as** `fastmcp.server.middleware.AuthMiddleware`

**Bases** `Middleware`

**Declared members (8)**

- `auth = auth`  _instance-attribute_
- `async def on_call_tool(self, context: MiddlewareContext[mt.CallToolRequestParams], call_next: CallNext[mt.CallToolRequestParams, ToolResult]) -> ToolResult`  _async_
  Check auth before tool execution.
- `async def on_get_prompt(self, context: MiddlewareContext[mt.GetPromptRequestParams], call_next: CallNext[mt.GetPromptRequestParams, PromptResult]) -> PromptResult`  _async_
  Check auth before prompt render.
- `async def on_list_prompts(self, context: MiddlewareContext[mt.ListPromptsRequest], call_next: CallNext[mt.ListPromptsRequest, Sequence[Prompt]]) -> Sequence[Prompt]`  _async_
  Filter prompts/list response based on auth checks.
- `async def on_list_resource_templates(self, context: MiddlewareContext[mt.ListResourceTemplatesRequest], call_next: CallNext[mt.ListResourceTemplatesRequest, Sequence[ResourceTemplate]]) -> Sequence[ResourceTemplate]`  _async_
  Filter resource templates/list response based on auth checks.
- `async def on_list_resources(self, context: MiddlewareContext[mt.ListResourcesRequest], call_next: CallNext[mt.ListResourcesRequest, Sequence[Resource]]) -> Sequence[Resource]`  _async_
  Filter resources/list response based on auth checks.
- `async def on_list_tools(self, context: MiddlewareContext[mt.ListToolsRequest], call_next: CallNext[mt.ListToolsRequest, Sequence[Tool]]) -> Sequence[Tool]`  _async_
  Filter tools/list response based on auth checks.
- `async def on_read_resource(self, context: MiddlewareContext[mt.ReadResourceRequestParams], call_next: CallNext[mt.ReadResourceRequestParams, ResourceResult]) -> ResourceResult`  _async_
  Check auth before resource read.

**Inherited (5)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_discover`, `on_initialize`, `on_message`, `on_notification`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Global authorization middleware using callable checks.

This middleware applies auth checks to all components (tools, resources,
prompts) on the server. It uses the same callable API as component-level
auth checks.

The middleware:
- Filters tools/resources/prompts from list responses based on auth checks
- Checks auth before tool execution, resource read, and prompt render
- Skips all auth checks for STDIO transport (no OAuth concept)

Args:
    auth: A single auth check function or list of check functions.
        All checks must pass for authorization to succeed (AND logic).

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth import require_scopes

    # Require specific scope for all components
    mcp = FastMCP(middleware=[AuthMiddleware(auth=require_scopes("api"))])

    # Multiple scopes (AND logic)
    mcp = FastMCP(middleware=[
        AuthMiddleware(auth=require_scopes("read", "api"))
    ])
    ```


## _requested_version

`fastmcp.server.middleware.authorization._requested_version`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _requested_version(meta: Mapping[str, Any] | None) -> VersionSpec | None
```

