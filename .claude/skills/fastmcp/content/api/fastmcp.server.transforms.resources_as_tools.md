# `fastmcp.server.transforms.resources_as_tools`

Distribution: `fastmcp`

## _DEFAULT_ANNOTATIONS

`fastmcp.server.transforms.resources_as_tools._DEFAULT_ANNOTATIONS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_ANNOTATIONS = ToolAnnotations(read_only_hint=True)
```

## ResourcesAsTools

Import as `fastmcp.server.transforms.ResourcesAsTools`  ·  defined at `fastmcp.server.transforms.resources_as_tools.ResourcesAsTools`

```python
class ResourcesAsTools(Transform)
```

**Also exported as** `fastmcp.server.transforms.ResourcesAsTools`

**Bases** `Transform`

**Declared members (2)**

- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
  Get a tool by name, including generated resource tools.
- `async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
  Add resource tools to the tool list.

**Inherited (6)**

- from `fastmcp.server.transforms.Transform`: `get_prompt`, `get_resource`, `get_resource_template`, `list_prompts`, `list_resource_templates`, `list_resources`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transform that adds tools for listing and reading resources.

Generates two tools:
- `list_resources`: Lists all resources and templates
- `read_resource`: Reads a resource by URI

The generated tools route through the server at runtime, so auth,
middleware, and visibility apply automatically.

This transform should be applied to a FastMCP server instance, not
a raw Provider, because the generated tools need the server's
middleware chain for auth and visibility filtering.

Example:
    ```python
    mcp = FastMCP("Server")
    mcp.add_transform(ResourcesAsTools(mcp))
    # Now has list_resources and read_resource tools
    ```


## _format_result

`fastmcp.server.transforms.resources_as_tools._format_result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_result(result: Any) -> str
```

Format ResourceResult for tool output.

Single text content is returned as-is. Single binary content is
base64-encoded. Multiple contents are JSON-encoded.


