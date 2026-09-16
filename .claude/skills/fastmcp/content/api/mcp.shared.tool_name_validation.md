# `mcp.shared.tool_name_validation`

Distribution: `mcp`

## SEP_986_URL

`mcp.shared.tool_name_validation.SEP_986_URL`

```python
SEP_986_URL = 'https://modelcontextprotocol.io/specification/2025-11-25/server/tools#tool-names'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://modelcontextprotocol.io/specification/2025-11-25/server/tools#tool-names"]`

## TOOL_NAME_REGEX

`mcp.shared.tool_name_validation.TOOL_NAME_REGEX`

```python
TOOL_NAME_REGEX = re.compile('^[A-Za-z0-9._-]{1,128}$')
```

**Inferred type** (`ty`, not declared in the source): `Pattern[str]`

## logger

`mcp.shared.tool_name_validation.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ToolNameValidationResult

`mcp.shared.tool_name_validation.ToolNameValidationResult`

```python
class ToolNameValidationResult
```

**Declared members (2)**

- `is_valid: bool`  _instance-attribute_
- `warnings: list[str] = field(default_factory=lambda: [])`  _class-attribute, instance-attribute_

Result of tool name validation.

Attributes:
    is_valid: Whether the tool name conforms to SEP-986 requirements.
    warnings: List of warning messages for non-conforming aspects.


## issue_tool_name_warning

`mcp.shared.tool_name_validation.issue_tool_name_warning`

```python
def issue_tool_name_warning(name: str, warnings: list[str]) -> None
```

Log warnings for non-conforming tool names.

Args:
    name: The tool name that triggered the warnings.
    warnings: List of warning messages to log.


## validate_and_warn_tool_name

Import as `mcp.server.mcpserver.tools.base.validate_and_warn_tool_name`  ·  defined at `mcp.shared.tool_name_validation.validate_and_warn_tool_name`

```python
def validate_and_warn_tool_name(name: str) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate a tool name and issue warnings for non-conforming names.

This is the primary entry point for tool name validation. It validates
the name and logs any warnings via the logging module.

Args:
    name: The tool name to validate.

Returns:
    True if the name is valid, False otherwise.


## validate_tool_name

`mcp.shared.tool_name_validation.validate_tool_name`

```python
def validate_tool_name(name: str) -> ToolNameValidationResult
```

Validate a tool name according to the SEP-986 specification.

Args:
    name: The tool name to validate.

Returns:
    ToolNameValidationResult containing validation status and any warnings.


