# `fastmcp.client.transports.memory`

Distribution: `fastmcp`

## FastMCPTransport

Import as `fastmcp.client.FastMCPTransport`  ·  defined at `fastmcp.client.transports.memory.FastMCPTransport`

```python
class FastMCPTransport(ClientTransport)
```

**Also exported as** `fastmcp.client.FastMCPTransport`, `fastmcp.client.transports.FastMCPTransport`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ClientTransport`

**Declared members (3)**

- `async def connect_session(self, transport_options: TransportOptions | None = None, session_kwargs: Unpack[SessionKwargs] = {}) -> AsyncIterator[ClientSession]`  _async_
- `raise_exceptions = raise_exceptions`  _instance-attribute_
- `server = mcp`  _instance-attribute_

**Inherited (3)**

- from `fastmcp.client.transports.base.ClientTransport`: `close`, `get_session_id`, `legacy_only`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

In-memory transport for FastMCP servers.

This transport connects directly to a FastMCP server instance in the same
Python process. It works with both FastMCP servers and the SDK's own
high-level `MCPServer` from the low-level MCP SDK. This is particularly
useful for unit tests or scenarios where client and server run in the same
runtime.


## _enter_server_lifespan

`fastmcp.client.transports.memory._enter_server_lifespan`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _enter_server_lifespan(server: FastMCP[Any] | SDKServer) -> AsyncIterator[None]
```

Enters the server's lifespan context for FastMCP servers and does nothing for the SDK's own high-level servers.


## _lowlevel_of

`fastmcp.client.transports.memory._lowlevel_of`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _lowlevel_of(server: FastMCP[Any] | SDKServer) -> Server
```

Resolve the underlying lowlevel MCP `Server` for either server type.

The SDK's own high-level `MCPServer` exposes its lowlevel server as
`_lowlevel_server` and its own `run()` is synchronous, so we always drive
the async lowlevel `Server.run` here. FastMCP servers expose the same
lowlevel server as `_mcp_server`.


