# `fastmcp.contrib.bulk_tool_caller.bulk_tool_caller`

Distribution: `fastmcp`

## BulkToolCaller

Import as `fastmcp.contrib.bulk_tool_caller.BulkToolCaller`  ·  defined at `fastmcp.contrib.bulk_tool_caller.bulk_tool_caller.BulkToolCaller`

```python
class BulkToolCaller(MCPMixin)
```

**Also exported as** `fastmcp.contrib.bulk_tool_caller.BulkToolCaller`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `MCPMixin`

**Declared members (3)**

- `async def call_tool_bulk(self, tool: str, tool_arguments: list[dict[str, str | int | float | bool | None]], continue_on_error: bool = True) -> list[CallToolRequestResult]`  _async_
  Call a single tool registered on this MCP server multiple times with a single request.  Each call can include different arguments. Useful for speeding up what would otherwise  take several individual tool calls.
- `async def call_tools_bulk(self, tool_calls: list[CallToolRequest], continue_on_error: bool = True) -> list[CallToolRequestResult]`  _async_
  Call multiple tools registered on this MCP server in a single request. Each call can  be for a different tool and can include different arguments. Useful for speeding up  what would otherwise take several individual tool calls.
- `def register_tools(self, mcp_server: FastMCP, prefix: str | None = None, separator: str = _DEFAULT_SEPARATOR_TOOL) -> None`
  Register the tools provided by this class with the given MCP server.

A class to provide a "bulk tool call" tool for a FastMCP server


## CallToolRequest

`fastmcp.contrib.bulk_tool_caller.bulk_tool_caller.CallToolRequest`

```python
class CallToolRequest(BaseModel)
```

**Bases** `BaseModel`

**Declared members (2)**

- `arguments: dict[str, Any] = Field(description='A dictionary containing the arguments for the tool call.')`  _class-attribute, instance-attribute_
- `tool: str = Field(description='The name of the tool to call.')`  _class-attribute, instance-attribute_

A class to represent a request to call a tool with specific arguments.


## CallToolRequestResult

`fastmcp.contrib.bulk_tool_caller.bulk_tool_caller.CallToolRequestResult`

```python
class CallToolRequestResult(CallToolResult)
```

**Bases** `CallToolResult`

**Declared members (3)**

- `arguments: dict[str, Any] = Field(description='The arguments used for the tool call.')`  _class-attribute, instance-attribute_
- `def from_call_tool_result(cls, result: CallToolResult, tool: str, arguments: dict[str, Any]) -> CallToolRequestResult`  _classmethod_
  Create a CallToolRequestResult from a CallToolResult.
- `tool: str = Field(description='The name of the tool that was called.')`  _class-attribute, instance-attribute_

A class to represent the result of a bulk tool call.
It extends CallToolResult to include information about the requested tool call.


