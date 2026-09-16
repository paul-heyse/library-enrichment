# Removed and relocated API

18 keyword arguments that `FastMCP()` accepted before 4.0 now raise
`TypeError`. Each row is the replacement upstream names in its own error message.

These matter more than a renamed import: the call still looks correct, still passes a
type checker in most setups, and fails only when the server is constructed.

| Removed keyword | What to do instead |
|---|---|
| `debug` | Set FASTMCP_DEBUG. |
| `exclude_tags` | Use `server.disable(tags=...)` after creating the server. |
| `host` | Pass `host` to `run_http_async()`, or set FASTMCP_HOST. |
| `include_tags` | Use `server.enable(tags=..., only=True)` after creating the server. |
| `json_response` | Pass `json_response` to `run_http_async()` or `http_app()`, or set FASTMCP_JSON_RESPONSE. |
| `log_level` | Pass `log_level` to `run_http_async()`, or set FASTMCP_LOG_LEVEL. |
| `message_path` | Set FASTMCP_MESSAGE_PATH. |
| `on_duplicate_prompts` | Use `on_duplicate=` instead. |
| `on_duplicate_resources` | Use `on_duplicate=` instead. |
| `on_duplicate_tools` | Use `on_duplicate=` instead. |
| `port` | Pass `port` to `run_http_async()`, or set FASTMCP_PORT. |
| `sampling_handler` | Server-initiated sampling is deprecated in MCP (SEP-2577) and the 2026-07-28 protocol has no back-channel for it (SEP-2322). Call an LLM directly from your tool. |
| `sampling_handler_behavior` | Server-initiated sampling is deprecated in MCP (SEP-2577) and the 2026-07-28 protocol has no back-channel for it (SEP-2322). Call an LLM directly from your tool. |
| `sse_path` | Pass `path` to `run_http_async()` or `http_app()`, or set FASTMCP_SSE_PATH. |
| `stateless_http` | Pass `stateless_http` to `run_http_async()` or `http_app()`, or set FASTMCP_STATELESS_HTTP. |
| `streamable_http_path` | Pass `path` to `run_http_async()` or `http_app()`, or set FASTMCP_STREAMABLE_HTTP_PATH. |
| `tool_serializer` | Return ToolResult from your tools instead. See https://gofastmcp.com/servers/tools#custom-serialization |
| `tool_transformations` | Use `server.add_transform(ToolTransform(...))` after creating the server. |

## Methods that no longer exist

| Removed | What to do instead |
|---|---|
| `FastMCP.import_server(...)` | Use `mount()` or add a provider; composition is now the Provider architecture. |
| `FastMCP.as_proxy(...)` | Use `fastmcp.server.create_proxy(target, ...)`. |

## The other FastMCP

`mcp.server.fastmcp` is a tombstone module: importing it raises `ModuleNotFoundError`
with a migration message. The official SDK renamed its own server class to
`mcp.server.mcpserver.MCPServer`. That is a different library from `fastmcp.FastMCP`
with a different feature set -- not an alias, not a fork to fall back on.
