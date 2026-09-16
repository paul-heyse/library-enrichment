# `fastmcp.utilities.inspect`

Distribution: `fastmcp`

## FastMCPInfo

`fastmcp.utilities.inspect.FastMCPInfo`

```python
class FastMCPInfo
```

**Declared members (13)**

- `capabilities: dict[str, Any]`  _instance-attribute_
- `fastmcp_version: str`  _instance-attribute_
- `icons: list[dict[str, Any]] | None`  _instance-attribute_
- `instructions: str | None`  _instance-attribute_
- `mcp_version: str`  _instance-attribute_
- `name: str`  _instance-attribute_
- `prompts: list[PromptInfo]`  _instance-attribute_
- `resources: list[ResourceInfo]`  _instance-attribute_
- `server_generation: int`  _instance-attribute_
- `templates: list[TemplateInfo]`  _instance-attribute_
- `tools: list[ToolInfo]`  _instance-attribute_
- `version: str | None`  _instance-attribute_
- `website_url: str | None`  _instance-attribute_

Information extracted from a FastMCP instance.


## InspectFormat

Import as `fastmcp.cli.cli.InspectFormat`  ·  defined at `fastmcp.utilities.inspect.InspectFormat`

```python
class InspectFormat(str, Enum)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `str`, `Enum`

**Declared members (2)**

- `FASTMCP = 'fastmcp'`  _class-attribute, instance-attribute_
- `MCP = 'mcp'`  _class-attribute, instance-attribute_

Output format for inspect command.


## PromptInfo

`fastmcp.utilities.inspect.PromptInfo`

```python
class PromptInfo
```

**Declared members (8)**

- `arguments: list[dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
- `description: str | None`  _instance-attribute_
- `icons: list[dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
- `key: str`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `name: str`  _instance-attribute_
- `tags: list[str] | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_

Information about a prompt.


## ResourceInfo

`fastmcp.utilities.inspect.ResourceInfo`

```python
class ResourceInfo
```

**Declared members (10)**

- `annotations: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `description: str | None`  _instance-attribute_
- `icons: list[dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
- `key: str`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `mime_type: str | None = None`  _class-attribute, instance-attribute_
- `name: str | None`  _instance-attribute_
- `tags: list[str] | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_
- `uri: str`  _instance-attribute_

Information about a resource.


## TemplateInfo

`fastmcp.utilities.inspect.TemplateInfo`

```python
class TemplateInfo
```

**Declared members (11)**

- `annotations: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `description: str | None`  _instance-attribute_
- `icons: list[dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
- `key: str`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `mime_type: str | None = None`  _class-attribute, instance-attribute_
- `name: str | None`  _instance-attribute_
- `parameters: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `tags: list[str] | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_
- `uri_template: str`  _instance-attribute_

Information about a resource template.


## ToolInfo

`fastmcp.utilities.inspect.ToolInfo`

```python
class ToolInfo
```

**Declared members (10)**

- `annotations: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `description: str | None`  _instance-attribute_
- `icons: list[dict[str, Any]] | None = None`  _class-attribute, instance-attribute_
- `input_schema: dict[str, Any]`  _instance-attribute_
- `key: str`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `name: str`  _instance-attribute_
- `output_schema: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `tags: list[str] | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_

Information about a tool.


## format_fastmcp_info

`fastmcp.utilities.inspect.format_fastmcp_info`

```python
def format_fastmcp_info(info: FastMCPInfo) -> bytes
```

Format FastMCPInfo as FastMCP-specific JSON.

This includes FastMCP-specific fields like tags, enabled, annotations, etc.


## format_info

Import as `fastmcp.cli.cli.format_info`  ·  defined at `fastmcp.utilities.inspect.format_info`

```python
async def format_info(mcp: FastMCP[Any] | SDKServer, format: InspectFormat | Literal['fastmcp', 'mcp'], info: FastMCPInfo | None = None) -> bytes
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Format server information according to the specified format.

Args:
    mcp: The FastMCP instance
    format: Output format ("fastmcp" or "mcp")
    info: Pre-extracted FastMCPInfo (optional, will be extracted if not provided)

Returns:
    JSON bytes in the requested format


## format_mcp_info

`fastmcp.utilities.inspect.format_mcp_info`

```python
async def format_mcp_info(mcp: FastMCP[Any] | SDKServer) -> bytes
```

Format server info as standard MCP protocol JSON.

Uses Client to get the standard MCP protocol format with camelCase fields.
Includes version metadata at the top level.


## inspect_fastmcp

Import as `fastmcp.cli.cli.inspect_fastmcp`  ·  defined at `fastmcp.utilities.inspect.inspect_fastmcp`

```python
async def inspect_fastmcp(mcp: FastMCP[Any] | SDKServer) -> FastMCPInfo
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract information from a FastMCP instance into a dataclass.

This function automatically detects whether the instance is FastMCP v1.x or v2.x
and uses the appropriate extraction method.

Args:
    mcp: The FastMCP instance to inspect (v1.x or v2.x)

Returns:
    FastMCPInfo dataclass containing the extracted information


## inspect_fastmcp_v1

`fastmcp.utilities.inspect.inspect_fastmcp_v1`

```python
async def inspect_fastmcp_v1(mcp: SDKServer) -> FastMCPInfo
```

Extract information from a FastMCP v1.x instance using a Client.

Args:
    mcp: The FastMCP v1.x instance to inspect

Returns:
    FastMCPInfo dataclass containing the extracted information


## inspect_fastmcp_v2

`fastmcp.utilities.inspect.inspect_fastmcp_v2`

```python
async def inspect_fastmcp_v2(mcp: FastMCP[Any]) -> FastMCPInfo
```

Extract information from a FastMCP v2.x instance.

Args:
    mcp: The FastMCP v2.x instance to inspect

Returns:
    FastMCPInfo dataclass containing the extracted information


