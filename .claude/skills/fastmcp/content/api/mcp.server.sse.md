# `mcp.server.sse`

Distribution: `mcp`

## logger

`mcp.server.sse.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## SseServerTransport

Import as `mcp.server.mcpserver.server.SseServerTransport`  ·  defined at `mcp.server.sse.SseServerTransport`

```python
class SseServerTransport
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `async def connect_sse(self, scope: Scope, receive: Receive, send: Send)`  _async_
- `async def handle_post_message(self, scope: Scope, receive: Receive, send: Send) -> None`  _async_
  ASGI application for the message endpoint.

SSE server transport for MCP. This class provides two ASGI applications,
suitable for use with a framework like Starlette and a server like Hypercorn:

    1. connect_sse() is an ASGI application which receives incoming GET requests,
       and sets up a new SSE stream to send server messages to the client.
    2. handle_post_message() is an ASGI application which receives incoming POST
       requests, which should contain client messages that link to a
       previously-established SSE session.


