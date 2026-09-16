# `fastmcp.server.middleware.dereference`

Distribution: `fastmcp`

## DereferenceRefsMiddleware

`fastmcp.server.middleware.dereference.DereferenceRefsMiddleware`

```python
class DereferenceRefsMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (2)**

- `async def on_list_resource_templates(self, context: MiddlewareContext[mt.ListResourceTemplatesRequest], call_next: CallNext[mt.ListResourceTemplatesRequest, Sequence[ResourceTemplate]]) -> Sequence[ResourceTemplate]`  _async_
- `async def on_list_tools(self, context: MiddlewareContext[mt.ListToolsRequest], call_next: CallNext[mt.ListToolsRequest, Sequence[Tool]]) -> Sequence[Tool]`  _async_

**Inherited (10)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resources`, `on_message`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Dereferences $ref in component schemas before sending to clients.

Some MCP clients (e.g., VS Code Copilot) don't handle JSON Schema $ref
properly. This middleware inlines all $ref definitions so schemas are
self-contained. Enabled by default via ``FastMCP(dereference_schemas=True)``.


## _dereference_resource_template

`fastmcp.server.middleware.dereference._dereference_resource_template`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _dereference_resource_template(template: ResourceTemplate) -> ResourceTemplate
```

Return a copy of the template with dereferenced schemas.


## _dereference_tool

`fastmcp.server.middleware.dereference._dereference_tool`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _dereference_tool(tool: Tool) -> Tool
```

Return a copy of the tool with dereferenced schemas.


## _has_ref

`fastmcp.server.middleware.dereference._has_ref`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_ref(schema: dict[str, Any]) -> bool
```

Check if a schema contains any $ref.


