# `fastmcp.server.context`

Distribution: `fastmcp`

## T

`fastmcp.server.context.T`

```python
T = TypeVar('T', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TransportType

`fastmcp.server.context.TransportType`

```python
TransportType = Literal['stdio', 'sse', 'streamable-http']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["stdio", "sse", "streamable-http"]'> ````

## _ELICIT_MODERN_ERROR

`fastmcp.server.context._ELICIT_MODERN_ERROR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ELICIT_MODERN_ERROR = 'elicitation via server-initiated requests is unavailable on 2026-07-28 connections.'
```

## _MCP_LEVEL_SEVERITY

`fastmcp.server.context._MCP_LEVEL_SEVERITY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MCP_LEVEL_SEVERITY: dict[LoggingLevel, int] = {'debug': 0, 'info': 1, 'notice': 2, 'warning': 3, 'error': 4, 'critical': 5, 'alert': 6, 'emergency': 7}
```

## _TASK_ELICIT_ERROR

`fastmcp.server.context._TASK_ELICIT_ERROR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TASK_ELICIT_ERROR = 'Imperative ctx.elicit() is not supported inside a background task. Gather input with the guard pattern instead: return an InputRequiredResult from the tool (with input_requests), and read ctx.input_responses / ctx.request_state when the task re-runs after the client answers.'
```

## _current_context

`fastmcp.server.context._current_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_current_context: ContextVar[Context | None] = ContextVar('context', default=None)
```

## _current_transport

`fastmcp.server.context._current_transport`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_current_transport: ContextVar[TransportType | None] = ContextVar('transport', default=None)
```

## _mcp_level_to_python_level

`fastmcp.server.context._mcp_level_to_python_level`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_mcp_level_to_python_level = {'debug': logging.DEBUG, 'info': logging.INFO, 'notice': logging.INFO, 'warning': logging.WARNING, 'error': logging.ERROR, 'critical': logging.CRITICAL, 'alert': logging.CRITICAL, 'emergency': logging.CRITICAL}
```

## logger

`fastmcp.server.context.logger`

```python
logger: Logger = get_logger(name=__name__)
```

## to_client_logger

`fastmcp.server.context.to_client_logger`

```python
to_client_logger: Logger = logger.getChild(suffix='to_client')
```

## Context

Import as `fastmcp.Context`  ·  defined at `fastmcp.server.context.Context`

```python
class Context
```

**Also exported as** `fastmcp.Context`, `fastmcp.server.Context`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (34)**

- `def client_extension_settings(self, identifier: str) -> dict[str, Any] | None`
  This request's per-request opt-in settings for an MCP extension.
- `client_id: str | None`  _property_
  Get the client ID if available.
- `def client_supports_extension(self, extension_id: str) -> bool`
  Check whether the connected client supports a given MCP extension.
- `async def close_sse_stream(self) -> None`  _async_
  Close the current response stream to trigger client reconnection.
- `async def debug(self, message: str, logger_name: str | None = None, extra: Mapping[str, Any] | None = None) -> None`  _async_
  Send a `DEBUG`-level message to the connected MCP Client.
- `async def delete_state(self, key: str) -> None`  _async_
  Delete a value from the state store.
- `async def disable_components(self, names: set[str] | None = None, keys: set[str] | None = None, version: VersionSpec | None = None, tags: set[str] | None = None, components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None, match_all: bool = False) -> None`  _async_
  Disable components matching criteria for this session only.
- `async def elicit(self, message: str, response_type: type[T] | list[str] | dict[str, dict[str, str]] | list[list[str]] | list[dict[str, dict[str, str]]], response_title: str | None = None, response_description: str | None = None) -> AcceptedElicitation[T] | AcceptedElicitation[dict[str, Any]] | AcceptedElicitation[str] | AcceptedElicitation[list[str]] | DeclinedElicitation | CancelledElicitation`  _async_
  Send an elicitation request to the client and await the response.
- `async def enable_components(self, names: set[str] | None = None, keys: set[str] | None = None, version: VersionSpec | None = None, tags: set[str] | None = None, components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None, match_all: bool = False) -> None`  _async_
  Enable components matching criteria for this session only.
- `async def error(self, message: str, logger_name: str | None = None, extra: Mapping[str, Any] | None = None) -> None`  _async_
  Send a `ERROR`-level message to the connected MCP Client.
- `fastmcp: FastMCP`  _property_
  Get the FastMCP instance.
- `async def get_prompt(self, name: str, arguments: dict[str, Any] | None = None) -> GetPromptResult`  _async_
  Get a prompt by name with optional arguments.
- `async def get_state(self, key: str) -> Any`  _async_
  Get a value from the state store.
- `async def info(self, message: str, logger_name: str | None = None, extra: Mapping[str, Any] | None = None) -> None`  _async_
  Send a `INFO`-level message to the connected MCP Client.
- `input_responses: mcp_types.InputResponses | None`  _property_
  Client responses to a prior `InputRequiredResult.input_requests`.
- `is_background_task: bool`  _property_
  True when this context is running in a background task (Docket worker).
- `lifespan_context: dict[str, Any]`  _property_
  Access the server's lifespan context.
- `async def list_prompts(self) -> list[SDKPrompt]`  _async_
  List all available prompts from the server.
- `async def list_resources(self) -> list[SDKResource]`  _async_
  List all available resources from the server.
- `async def log(self, message: str, level: LoggingLevel | None = None, logger_name: str | None = None, extra: Mapping[str, Any] | None = None) -> None`  _async_
  Send a log message to the client.
- `origin_request_id: str | None`  _property_
  Get the request ID that originated this execution, if available.
- `async def read_resource(self, uri: str | AnyUrl) -> ResourceResult`  _async_
  Read a resource by URI.
- `async def report_progress(self, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
  Report progress for the current operation.
- `request_context: FastMCPRequestContext | None`  _property_
  Access to the underlying request context.
- `request_id: str`  _property_
  Get the unique ID for this request.
- `request_state: str | None`  _property_
  Opaque state echoed from a prior `InputRequiredResult.request_state`.
- `async def reset_visibility(self) -> None`  _async_
  Clear all session visibility rules.
- `async def send_notification(self, notification: mcp_types.ServerNotification) -> None`  _async_
  Send a notification to the client immediately.
- `session: ServerSession`  _property_
  Access to the underlying session for advanced usage.
- `session_id: str`  _property_
  Get the MCP session ID for ALL transports.
- `async def set_state(self, key: str, value: Any, serializable: bool = True) -> None`  _async_
  Set a value in the state store.
- `task_id: str | None`  _property_
  Get the background task ID if running in a background task.
- `transport: TransportType | None`  _property_
  Get the current transport type.
- `async def warning(self, message: str, logger_name: str | None = None, extra: Mapping[str, Any] | None = None) -> None`  _async_
  Send a `WARNING`-level message to the connected MCP Client.

Context object providing access to MCP capabilities.

This provides a cleaner interface to MCP's RequestContext functionality.
It gets injected into tool and resource functions that request it via type hints.

To use context in a tool function, add a parameter with the Context type annotation:

```python
@server.tool
async def my_tool(x: int, ctx: Context) -> str:
    # Log messages to the client
    await ctx.info(f"Processing {x}")
    await ctx.debug("Debug info")
    await ctx.warning("Warning message")
    await ctx.error("Error message")

    # Report progress
    await ctx.report_progress(50, 100, "Processing")

    # Access resources
    data = await ctx.read_resource("resource://data")

    # Get request info
    request_id = ctx.request_id
    client_id = ctx.client_id

    # Manage state across the session (persists across requests)
    await ctx.set_state("key", "value")
    value = await ctx.get_state("key")

    # Store non-serializable values for the current request only
    await ctx.set_state("client", http_client, serializable=False)

    return str(x)
```

State Management:
Context provides session-scoped state that persists across requests within
the same MCP session. State is automatically keyed by session, ensuring
isolation between different clients.

State set during `on_initialize` middleware will persist to subsequent tool
calls when using the same session object (STDIO, SSE, single-server HTTP).
For distributed/serverless HTTP deployments where different machines handle
the init and tool calls, state is isolated by the mcp-session-id header.

The context parameter name can be anything as long as it's annotated with Context.
The context is optional - tools that don't need it can omit the parameter.


## LogData

`fastmcp.server.context.LogData`

```python
class LogData
```

**Declared members (2)**

- `extra: Mapping[str, Any] | None = None`  _class-attribute, instance-attribute_
- `msg: str`  _instance-attribute_

Data object for passing log arguments to client-side handlers.

This provides an interface to match the Python standard library logging,
for compatibility with structured logging.


## _detached_request_context

`fastmcp.server.context._detached_request_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _detached_request_context(context: Context) -> ServerRequestContext
```

Build a minimal SDK request context for internal handler invocation.

Used by ``Context._paginate_list`` when no request context is active (e.g.
introspection outside a live request), so the ``_on_*`` list handlers have a
context to bind. The list handlers only read ``self`` (the FastMCP server)
to enumerate components, so a session-less context is sufficient.


## _log_level_session_key

`fastmcp.server.context._log_level_session_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _log_level_session_key(session: ServerSession) -> str
```

Derive the per-session key used for logging/setLevel gating.

v2 constructs sessions per-request, so the stable identity is the
connection session id (stateful HTTP). stdio/in-memory has no session id,
so a sentinel key is used — all such connections share one gate, matching
the single-connection nature of those transports.


## _log_to_server_and_client

`fastmcp.server.context._log_to_server_and_client`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _log_to_server_and_client(data: LogData, session: ServerSession, level: LoggingLevel, logger_name: str | None = None, related_request_id: str | None = None, min_level: LoggingLevel | None = None) -> None
```

Log a message to the server and client.


## reset_transport

`fastmcp.server.context.reset_transport`

```python
def reset_transport(token: Token[TransportType | None]) -> None
```

Reset transport to previous value.


## set_context

`fastmcp.server.context.set_context`

```python
def set_context(context: Context) -> Generator[Context, None, None]
```

## set_transport

`fastmcp.server.context.set_transport`

```python
def set_transport(transport: TransportType) -> Token[TransportType | None]
```

Set the current transport type. Returns token for reset.


