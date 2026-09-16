# Transports

## Server side

`mcp.run(transport=...)` accepts `stdio`, `http`, `sse` and `streamable-http`.
Host, port and path are **not** constructor arguments -- pass them to
`run_http_async()` / `http_app()`, or set `FASTMCP_*` environment variables.
See `catalogs/removed-api.md`.

| Method | Purpose |
|---|---|
| `run(...)` | synchronous wrapper |
| `run_async(...)` | the async entry point |
| `run_stdio_async(...)` | stdio explicitly |
| `run_http_async(...)` | HTTP, where host/port/path/security options live |
| `http_app(...)` | a Starlette app to mount in your own ASGI stack |

## Client side

11 transport classes descend from `fastmcp.client.ClientTransport`. `Client(...)`
infers one from what you pass -- a URL, a path, a dict, or a server instance.

| Transport | Import as |
|---|---|
| `MCPConfigTransport` | `fastmcp.client.client.MCPConfigTransport` |
| `StreamableHttpTransport` | `fastmcp.client.StreamableHttpTransport` |
| `FastMCPTransport` | `fastmcp.client.FastMCPTransport` |
| `SSETransport` | `fastmcp.client.SSETransport` |
| `FastMCPStdioTransport` | `fastmcp.client.transports.FastMCPStdioTransport` |
| `NodeStdioTransport` | `fastmcp.client.NodeStdioTransport` |
| `NpxStdioTransport` | `fastmcp.client.NpxStdioTransport` |
| `PythonStdioTransport` | `fastmcp.client.PythonStdioTransport` |
| `StdioTransport` | `fastmcp.client.StdioTransport` |
| `UvStdioTransport` | `fastmcp.client.UvStdioTransport` |
| `UvxStdioTransport` | `fastmcp.client.UvxStdioTransport` |

## Referenced how often

From `index/usage.tsv` — how much this library uses each of these itself. A low
count is not a warning: it measures internal use, not what callers need.

| Item | Callers | References |
|---|---:|---:|
| `fastmcp.client.transports.base.ClientTransport` | 7 | 39 |
| `fastmcp.client.transports.sse.SSETransport` | 6 | 21 |
| `fastmcp.client.transports.http.StreamableHttpTransport` | 6 | 21 |
| `fastmcp.client.transports.stdio.StdioTransport` | 6 | 19 |
| `fastmcp.client.transports.config.MCPConfigTransport` | 5 | 8 |
| `fastmcp.client.transports.memory.FastMCPTransport` | 4 | 19 |
| `fastmcp.client.transports.stdio.PythonStdioTransport` | 2 | 10 |
| `fastmcp.client.transports.stdio.NodeStdioTransport` | 2 | 10 |
| `fastmcp.client.transports.stdio.UvxStdioTransport` | 1 | 3 |
| `fastmcp.client.transports.stdio.UvStdioTransport` | 1 | 3 |
| `fastmcp.client.transports.stdio.NpxStdioTransport` | 1 | 3 |
| `fastmcp.client.transports.stdio.FastMCPStdioTransport` | 1 | 2 |
