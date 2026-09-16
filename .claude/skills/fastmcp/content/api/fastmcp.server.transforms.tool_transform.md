# `fastmcp.server.transforms.tool_transform`

Distribution: `fastmcp`

## ToolTransform

Import as `fastmcp.server.transforms.ToolTransform`  ·  defined at `fastmcp.server.transforms.tool_transform.ToolTransform`

```python
class ToolTransform(Transform)
```

**Also exported as** `fastmcp.server.transforms.ToolTransform`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Transform`

**Declared members (2)**

- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
  Get tool by transformed name.
- `async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
  Apply transforms to matching tools.

**Inherited (6)**

- from `fastmcp.server.transforms.Transform`: `get_prompt`, `get_resource`, `get_resource_template`, `list_prompts`, `list_resource_templates`, `list_resources`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Applies tool transformations to modify tool schemas.

Wraps ToolTransformConfig to apply argument renames, schema changes,
hidden arguments, and other transformations at the transform level.

Example:
    ```python
    transform = ToolTransform({
        "my_tool": ToolTransformConfig(
            name="renamed_tool",
            arguments={"old_arg": ArgTransformConfig(name="new_arg")}
        )
    })
    ```


