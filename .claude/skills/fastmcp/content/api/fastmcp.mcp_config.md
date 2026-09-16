# `fastmcp.mcp_config`

Distribution: `fastmcp`

## CanonicalMCPServerTypes

`fastmcp.mcp_config.CanonicalMCPServerTypes`

```python
CanonicalMCPServerTypes = StdioMCPServer | RemoteMCPServer
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'StdioMCPServer | RemoteMCPServer'> ````

## MCPServerTypes

Import as `fastmcp.cli.discovery.MCPServerTypes`  ·  defined at `fastmcp.mcp_config.MCPServerTypes`

```python
MCPServerTypes = TransformingMCPServerTypes | CanonicalMCPServerTypes
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'StdioMCPServer | RemoteMCPServer'> ````

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## TransformingMCPServerTypes

`fastmcp.mcp_config.TransformingMCPServerTypes`

```python
TransformingMCPServerTypes = TransformingStdioMCPServer | TransformingRemoteMCPServer
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'TransformingStdioMCPServer | TransformingRemoteMCPServer'> ````

## CanonicalMCPConfig

`fastmcp.mcp_config.CanonicalMCPConfig`

```python
class CanonicalMCPConfig(MCPConfig)
```

**Bases** `MCPConfig`

**Declared members (2)**

- `def add_server(self, name: str, server: CanonicalMCPServerTypes) -> None`
  Add or update a server in the configuration.
- `mcpServers: dict[str, CanonicalMCPServerTypes] = Field(default_factory=dict)`  _class-attribute, instance-attribute_

**Inherited (5)**

- from `fastmcp.mcp_config.MCPConfig`: `from_dict`, `from_file`, `to_dict`, `wrap_servers_at_root`, `write_to_file`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Canonical MCP configuration format.

This defines the standard configuration format for Model Context Protocol servers.
The format is designed to be client-agnostic and extensible for future use cases.


## MCPConfig

Import as `fastmcp.client.group.MCPConfig`  ·  defined at `fastmcp.mcp_config.MCPConfig`

```python
class MCPConfig(BaseModel)
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (7)**

- `def add_server(self, name: str, server: MCPServerTypes) -> None`
  Add or update a server in the configuration.
- `def from_dict(cls, config: dict[str, Any]) -> Self`  _classmethod_
  Parse MCP configuration from dictionary format.
- `def from_file(cls, file_path: Path) -> Self`  _classmethod_
  Load configuration from JSON file.
- `mcpServers: dict[str, MCPServerTypes] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `def to_dict(self) -> dict[str, Any]`
  Convert MCPConfig to dictionary format, preserving all fields.
- `def wrap_servers_at_root(cls, values: dict[str, Any]) -> dict[str, Any]`  _classmethod_
  If there's no mcpServers key but there are server configs at root, wrap them.
- `def write_to_file(self, file_path: Path) -> None`
  Write configuration to JSON file.

A configuration object for MCP Servers that conforms to the canonical MCP configuration format
while adding additional fields for enabling FastMCP-specific features like tool transformations
and filtering by tags.

For an MCPConfig that is strictly canonical, see the `CanonicalMCPConfig` class.


## RemoteMCPServer

Import as `fastmcp.cli.discovery.RemoteMCPServer`  ·  defined at `fastmcp.mcp_config.RemoteMCPServer`

```python
class RemoteMCPServer(BaseModel)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (10)**

- `auth: Annotated[str | Literal['oauth'] | httpx2.Auth | None, Field(description='Either a string representing a Bearer token, the literal "oauth" to use OAuth authentication, or an httpx2.Auth instance for custom authentication.')] = None`  _class-attribute, instance-attribute_
- `authentication: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `headers: dict[str, str] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `icon: str | None = None`  _class-attribute, instance-attribute_
- `sse_read_timeout: datetime.timedelta | int | float | None = None`  _class-attribute, instance-attribute_
- `timeout: int | None = None`  _class-attribute, instance-attribute_
- `def to_transport(self) -> StreamableHttpTransport | SSETransport | FastMCPTransport`
- `transport: Literal['http', 'streamable-http', 'sse'] | None = None`  _class-attribute, instance-attribute_
- `url: str`  _instance-attribute_

MCP server configuration for HTTP/SSE transport.

This is the canonical configuration format for MCP servers using remote transports.


## StdioMCPServer

Import as `fastmcp.cli.discovery.StdioMCPServer`  ·  defined at `fastmcp.mcp_config.StdioMCPServer`

```python
class StdioMCPServer(BaseModel)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (12)**

- `args: list[str] = Field(default_factory=list)`  _class-attribute, instance-attribute_
- `authentication: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `command: str`  _instance-attribute_
- `cwd: str | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `env: dict[str, Any] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `icon: str | None = None`  _class-attribute, instance-attribute_
- `keep_alive: bool | None = None`  _class-attribute, instance-attribute_
- `timeout: int | None = None`  _class-attribute, instance-attribute_
- `def to_transport(self) -> StdioTransport | FastMCPTransport`
- `transport: Literal['stdio'] = 'stdio'`  _class-attribute, instance-attribute_
- `type: Literal['stdio'] | None = None`  _class-attribute, instance-attribute_

MCP server configuration for stdio transport.

This is the canonical configuration format for MCP servers using stdio transport.


## TransformingRemoteMCPServer

Import as `fastmcp.client.transports.config.TransformingRemoteMCPServer`  ·  defined at `fastmcp.mcp_config.TransformingRemoteMCPServer`

```python
class TransformingRemoteMCPServer(_TransformingMCPServerMixin, RemoteMCPServer)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_TransformingMCPServerMixin`, `RemoteMCPServer`

**Inherited (13)**

- from `fastmcp.mcp_config.RemoteMCPServer`: `auth`, `authentication`, `description`, `headers`, `icon`, `sse_read_timeout`, `timeout`, `transport`, `url`
- from `fastmcp.mcp_config._TransformingMCPServerMixin`: `exclude_tags`, `include_tags`, `to_transport`, `tools`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Remote server with tool transforms.


## TransformingStdioMCPServer

Import as `fastmcp.client.transports.config.TransformingStdioMCPServer`  ·  defined at `fastmcp.mcp_config.TransformingStdioMCPServer`

```python
class TransformingStdioMCPServer(_TransformingMCPServerMixin, StdioMCPServer)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_TransformingMCPServerMixin`, `StdioMCPServer`

**Inherited (15)**

- from `fastmcp.mcp_config.StdioMCPServer`: `args`, `authentication`, `command`, `cwd`, `description`, `env`, `icon`, `keep_alive`, `timeout`, `transport`, `type`
- from `fastmcp.mcp_config._TransformingMCPServerMixin`: `exclude_tags`, `include_tags`, `to_transport`, `tools`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Stdio server with tool transforms.


## _TransformingMCPServerMixin

`fastmcp.mcp_config._TransformingMCPServerMixin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _TransformingMCPServerMixin(BaseModel)
```

**Bases** `BaseModel`

**Declared members (4)**

- `exclude_tags: set[str] | None = Field(default=None, description='The tags to exclude in the proxy.')`  _class-attribute, instance-attribute_
- `include_tags: set[str] | None = Field(default=None, description='The tags to include in the proxy.')`  _class-attribute, instance-attribute_
- `def to_transport(self) -> FastMCPTransport`
  Get the transport for the transforming MCP server.
- `tools: dict[str, Any] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
  The multi-tool transform to apply to the tools.

A mixin that enables wrapping an MCP Server with tool transforms.


## _coerce_tool_transform_configs

`fastmcp.mcp_config._coerce_tool_transform_configs`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _coerce_tool_transform_configs(tools: dict[str, Any]) -> dict[str, Any]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## infer_transport_type_from_url

Import as `fastmcp.client.transports.inference.infer_transport_type_from_url`  ·  defined at `fastmcp.mcp_config.infer_transport_type_from_url`

```python
def infer_transport_type_from_url(url: str | AnyUrl) -> Literal['http', 'sse']
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Infer the appropriate transport type from the given URL.


## update_config_file

Import as `fastmcp.cli.install.cursor.update_config_file`  ·  defined at `fastmcp.mcp_config.update_config_file`

```python
def update_config_file(file_path: Path, server_name: str, server_config: CanonicalMCPServerTypes) -> None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Update an MCP configuration file from a server object, preserving existing fields.

This is used for updating the mcpServer configurations of third-party tools so we do not
worry about transforming server objects here.


