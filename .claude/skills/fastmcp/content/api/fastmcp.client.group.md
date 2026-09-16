# `fastmcp.client.group`

Distribution: `fastmcp`

## ClientGroup

Import as `fastmcp.ClientGroup`  ·  defined at `fastmcp.client.group.ClientGroup`

```python
class ClientGroup
```

**Also exported as** `fastmcp.ClientGroup`

**Declared members (7)**

- `async def call_tool(self, name: str, arguments: dict[str, Any] | None = None, version: str | None = None, timeout: datetime.timedelta | float | int | None = None, progress_handler: ProgressHandler | None = None, raise_on_error: bool = True, meta: dict[str, Any] | None = None) -> CallToolResult`  _async_
  Call a namespaced tool through the client that advertised it.
- `async def call_tool_mcp(self, name: str, arguments: dict[str, Any] | None = None, timeout: datetime.timedelta | float | int | None = None, progress_handler: ProgressHandler | None = None, meta: dict[str, Any] | None = None) -> mcp_types.CallToolResult`  _async_
  Call a namespaced tool and return its raw MCP result.
- `clients: Mapping[str, Client[Any]]`  _property_
  The group's clients, keyed by server name.
- `def from_config(cls, config: MCPConfig | dict[str, Any], default_mode: ConnectMode = 'auto') -> ClientGroup`  _classmethod_
  Create one independent client for each configured server.
- `async def list_tools(self, cache_mode: CacheMode = 'refresh') -> list[mcp_types.Tool]`  _async_
  List tools from every client with namespaced names.
- `protocol_versions: dict[str, str | None]`  _property_
- `async def resolve_tool(self, name: str) -> ToolRoute`  _async_
  Resolve a public tool name to its client and upstream identity.

Coordinate independent clients without introducing a proxy server.

Each client retains its own transport, session, capabilities, and protocol
version. The group only combines tool discovery and routes tool calls.

Callers may manage the clients' connections themselves or use the group as
a convenience context manager. The group's context is reentrant in the
same way a client's is: entries are reference counted, the first entry
connects every client, and the last exit disconnects them. Entering an
already-connected FastMCP client is likewise safe because client contexts
are reference counted.


## ToolRoute

`fastmcp.client.group.ToolRoute`

```python
class ToolRoute
```

**Declared members (3)**

- `client: Client[Any]`  _instance-attribute_
- `server_name: str`  _instance-attribute_
- `upstream_name: str`  _instance-attribute_

The client and upstream name behind a public group tool name.


