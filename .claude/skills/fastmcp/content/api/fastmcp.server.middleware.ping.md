# `fastmcp.server.middleware.ping`

Distribution: `fastmcp`

## PingMiddleware

Import as `fastmcp.server.middleware.PingMiddleware`  ·  defined at `fastmcp.server.middleware.ping.PingMiddleware`

```python
class PingMiddleware(Middleware)
```

**Also exported as** `fastmcp.server.middleware.PingMiddleware`

**Bases** `Middleware`

**Declared members (2)**

- `interval_ms = interval_ms`  _instance-attribute_
- `async def on_message(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Start ping task on first message from a connection.

**Inherited (11)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that sends periodic pings to keep client connections alive.

Starts a background ping task on first message from each session. The task
sends server-to-client pings at the configured interval until the session
ends.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.middleware import PingMiddleware

    mcp = FastMCP("MyServer")
    mcp.add_middleware(PingMiddleware(interval_ms=5000))
    ```


