# ClientTransport

How a client reaches a server. Eleven ship; write one for a transport that does not.

Import as `fastmcp.client.ClientTransport`
Defined at `fastmcp.client.transports.base.ClientTransport`.

```python
class ClientTransport(abc.ABC)
```

## Required

You must write these. Nothing works until you do.

```python
async def connect_session(self, transport_options: TransportOptions | None = None, session_kwargs: Unpack[SessionKwargs] = {}) -> AsyncIterator[ClientSession]
```

## Provided

Defaulted, and this is where the capability hides. The default is almost always the conservative answer, so an implementation that overrides none of these works correctly and supplies nothing.

```python
async def close(self)
def get_session_id(self) -> str | None
legacy_only: bool = False
```

## Implementors (11)

Transitive. Read one before writing your own.

- `fastmcp.client.transports.config.MCPConfigTransport`
- `fastmcp.client.transports.http.StreamableHttpTransport`
- `fastmcp.client.transports.memory.FastMCPTransport`
- `fastmcp.client.transports.sse.SSETransport`
- `fastmcp.client.transports.stdio.FastMCPStdioTransport`
- `fastmcp.client.transports.stdio.NodeStdioTransport`
- `fastmcp.client.transports.stdio.NpxStdioTransport`
- `fastmcp.client.transports.stdio.PythonStdioTransport`
- `fastmcp.client.transports.stdio.StdioTransport`
- `fastmcp.client.transports.stdio.UvStdioTransport`
- `fastmcp.client.transports.stdio.UvxStdioTransport`

## Demonstrated by 6 upstream file(s)

Matched as syntax -- `class X(Base)` -- not by searching for the name.

- [`corpus/tests/client/client/test_client.py`](../corpus/tests/client/client/test_client.py)
- [`corpus/tests/client/client/test_mode_negotiation.py`](../corpus/tests/client/client/test_mode_negotiation.py)
- [`corpus/tests/client/client/test_session.py`](../corpus/tests/client/client/test_session.py)
- [`corpus/tests/client/group/test_client_group.py`](../corpus/tests/client/group/test_client_group.py)
- [`corpus/tests/client/transports/test_transports.py`](../corpus/tests/client/transports/test_transports.py)
- [`corpus/tests/test_mcp_config.py`](../corpus/tests/test_mcp_config.py)

## Documentation

Prose: [`api/fastmcp.client.transports.base.md`](../api/fastmcp.client.transports.base.md) · records: [`model/fastmcp.client.transports.base.json`](../model/fastmcp.client.transports.base.json)
