# `mcp.client._memory`

Distribution: `mcp`

## SERVER_SHUTDOWN_GRACE

`mcp.client._memory.SERVER_SHUTDOWN_GRACE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
SERVER_SHUTDOWN_GRACE = 2.0
```

Seconds to wait for the in-process server to exit on EOF before cancelling.


## InMemoryTransport

Import as `mcp.client.client.InMemoryTransport`  ·  defined at `mcp.client._memory.InMemoryTransport`

```python
class InMemoryTransport
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

In-memory transport for testing MCP servers without network overhead.

This transport starts the server in a background task and provides
streams for client-side communication. The server is automatically
stopped when the context manager exits.


