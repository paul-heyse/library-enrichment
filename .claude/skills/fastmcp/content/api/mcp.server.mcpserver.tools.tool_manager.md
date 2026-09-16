# `mcp.server.mcpserver.tools.tool_manager`

Distribution: `mcp`

## logger

`mcp.server.mcpserver.tools.tool_manager.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ToolManager

Import as `mcp.server.mcpserver.tools.ToolManager`  ·  defined at `mcp.server.mcpserver.tools.tool_manager.ToolManager`

```python
class ToolManager
```

**Also exported as** `mcp.server.mcpserver.tools.ToolManager`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `def add_tool(self, fn: Callable[..., Any], name: str | None = None, title: str | None = None, description: str | None = None, annotations: ToolAnnotations | None = None, icons: list[Icon] | None = None, meta: dict[str, Any] | None = None, structured_output: bool | None = None) -> Tool`
  Add a tool to the server.
- `async def call_tool(self, name: str, arguments: dict[str, Any], context: Context[LifespanContextT, RequestT], convert_result: bool = False) -> Any`  _async_
  Call a tool by name with arguments.
- `def get_tool(self, name: str) -> Tool | None`
  Get tool by name.
- `def list_tools(self) -> list[Tool]`
  List all registered tools.
- `def remove_tool(self, name: str) -> None`
  Remove a tool by name.
- `warn_on_duplicate_tools = warn_on_duplicate_tools`  _instance-attribute_

Manages MCPServer tools.


