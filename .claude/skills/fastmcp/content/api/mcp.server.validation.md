# `mcp.server.validation`

Distribution: `mcp`

## check_sampling_tools_capability

`mcp.server.validation.check_sampling_tools_capability`

```python
def check_sampling_tools_capability(client_caps: ClientCapabilities | None) -> bool
```

Check if the client supports sampling tools capability.

Args:
    client_caps: The client's declared capabilities

Returns:
    True if client supports sampling.tools, False otherwise


## validate_sampling_tools

Import as `mcp.server.session.validate_sampling_tools`  ·  defined at `mcp.server.validation.validate_sampling_tools`

```python
def validate_sampling_tools(client_caps: ClientCapabilities | None, tools: list[Tool] | None, tool_choice: ToolChoice | None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate that the client supports sampling tools if tools are being used.

Args:
    client_caps: The client's declared capabilities
    tools: The tools list, if provided
    tool_choice: The tool choice setting, if provided

Raises:
    MCPError: If tools/tool_choice are provided but client doesn't support them


## validate_tool_use_result_messages

Import as `mcp.server.session.validate_tool_use_result_messages`  ·  defined at `mcp.server.validation.validate_tool_use_result_messages`

```python
def validate_tool_use_result_messages(messages: list[SamplingMessage]) -> None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate tool_use/tool_result message structure per SEP-1577.

This validation ensures:
1. Messages with tool_result content contain ONLY tool_result content
2. tool_result messages are preceded by a message with tool_use
3. tool_result IDs match the tool_use IDs from the previous message

See: https://github.com/modelcontextprotocol/modelcontextprotocol/issues/1577

Args:
    messages: The list of sampling messages to validate

Raises:
    ValueError: If the message structure is invalid


## wants_sampling_tools

Import as `mcp.server.session.wants_sampling_tools`  ·  defined at `mcp.server.validation.wants_sampling_tools`

```python
def wants_sampling_tools(tools: list[Tool] | None, tool_choice: ToolChoice | None) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Whether a sampling request is tools-mode: `sampling.tools` gated, array-capable answer.


