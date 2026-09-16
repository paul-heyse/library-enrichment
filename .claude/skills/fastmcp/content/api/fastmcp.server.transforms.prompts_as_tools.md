# `fastmcp.server.transforms.prompts_as_tools`

Distribution: `fastmcp`

## PromptsAsTools

Import as `fastmcp.server.transforms.PromptsAsTools`  ·  defined at `fastmcp.server.transforms.prompts_as_tools.PromptsAsTools`

```python
class PromptsAsTools(Transform)
```

**Also exported as** `fastmcp.server.transforms.PromptsAsTools`

**Bases** `Transform`

**Declared members (2)**

- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
  Get a tool by name, including generated prompt tools.
- `async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
  Add prompt tools to the tool list.

**Inherited (6)**

- from `fastmcp.server.transforms.Transform`: `get_prompt`, `get_resource`, `get_resource_template`, `list_prompts`, `list_resource_templates`, `list_resources`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transform that adds tools for listing and getting prompts.

Generates two tools:
- `list_prompts`: Lists all prompts
- `get_prompt`: Gets a specific prompt with optional arguments

The generated tools route through the server at runtime, so auth,
middleware, and visibility apply automatically.

This transform should be applied to a FastMCP server instance, not
a raw Provider, because the generated tools need the server's
middleware chain for auth and visibility filtering.

Example:
    ```python
    mcp = FastMCP("Server")
    mcp.add_transform(PromptsAsTools(mcp))
    # Now has list_prompts and get_prompt tools
    ```


## _format_prompt_result

`fastmcp.server.transforms.prompts_as_tools._format_prompt_result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_prompt_result(result: Any) -> str
```

Format PromptResult for tool output.

Returns JSON with the messages array. Preserves embedded resources
as structured JSON objects.


