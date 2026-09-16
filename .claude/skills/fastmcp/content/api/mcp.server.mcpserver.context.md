# `mcp.server.mcpserver.context`

Distribution: `mcp`

## Context

Import as `mcp.server.mcpserver.Context`  ·  defined at `mcp.server.mcpserver.context.Context`

```python
class Context(BaseModel, Generic[LifespanContextT, RequestT])
```

**Also exported as** `mcp.server.mcpserver.Context`

_10 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`, `Generic[LifespanContextT, RequestT]`

**Declared members (24)**

- `client_capabilities: ClientCapabilities | None`  _property_
  The client's declared capabilities for this connection.
- `async def close_sse_stream(self) -> None`  _async_
  Close the SSE stream to trigger client reconnection.
- `async def close_standalone_sse_stream(self) -> None`  _async_
  Close the standalone GET SSE stream to trigger client reconnection.
- `async def debug(self, data: Any, logger_name: str | None = None) -> None`  _async_
  Send a debug log message.
- `async def elicit(self, message: str, schema: type[ElicitSchemaModelT]) -> ElicitationResult[ElicitSchemaModelT]`  _async_
  Elicit information from the client/user.
- `async def elicit_url(self, message: str, url: str, elicitation_id: str) -> UrlElicitationResult`  _async_
  Request URL mode elicitation from the client.
- `async def error(self, data: Any, logger_name: str | None = None) -> None`  _async_
  Send an error log message.
- `headers: Mapping[str, str] | None`  _property_
  Request headers carried by this message, when the transport has them.
- `async def info(self, data: Any, logger_name: str | None = None) -> None`  _async_
  Send an info log message.
- `input_responses: InputResponses | None`  _property_
  Client responses to a prior `InputRequiredResult.input_requests`.
- `async def log(self, level: LoggingLevel, data: Any, logger_name: str | None = None) -> None`  _async_
  Send a log message to the client.
- `mcp_server: MCPServer`  _property_
  Access to the MCPServer instance.
- `async def notify_prompts_changed(self) -> None`  _async_
  Publish a prompts list-changed event to `subscriptions/listen` subscribers.
- `async def notify_resource_updated(self, uri: str | AnyUrl) -> None`  _async_
  Publish a resource-updated event for `uri` to `subscriptions/listen` subscribers.
- `async def notify_resources_changed(self) -> None`  _async_
  Publish a resources list-changed event to `subscriptions/listen` subscribers.
- `async def notify_tools_changed(self) -> None`  _async_
  Publish a tools list-changed event to `subscriptions/listen` subscribers.
- `protocol_version: str | None`  _property_
  The negotiated protocol version, or `None` outside of an active request.
- `async def read_resource(self, uri: str | AnyUrl) -> Iterable[ReadResourceContents]`  _async_
  Read a resource by URI.
- `async def report_progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
  Report progress for the current operation.
- `request_context: ServerRequestContext[LifespanContextT, RequestT]`  _property_
  Access to the underlying request context.
- `request_id: str`  _property_
  Get the unique ID for this request.
- `request_state: str | None`  _property_
  Opaque state echoed from a prior `InputRequiredResult.request_state`.
- `session`  _property_
  Access to the underlying session for advanced usage.
- `async def warning(self, data: Any, logger_name: str | None = None) -> None`  _async_
  Send a warning log message.

Context object providing access to MCP capabilities.

This provides a cleaner interface to MCP's RequestContext functionality.
It gets injected into tool and resource functions that request it via type hints.

To use context in a tool function, add a parameter with the Context type annotation:

```python
@server.tool()
async def my_tool(x: int, ctx: Context) -> str:
    # Log messages to the client
    await ctx.info(f"Processing {x}")
    await ctx.debug("Debug info")
    await ctx.warning("Warning message")
    await ctx.error("Error message")

    # Report progress
    await ctx.report_progress(50, 100)

    # Access resources
    data = await ctx.read_resource("resource://data")

    # Get request info
    request_id = ctx.request_id

    return str(x)
```

The context parameter name can be anything as long as it's annotated with Context.
The context is optional - tools that don't need it can omit the parameter.


