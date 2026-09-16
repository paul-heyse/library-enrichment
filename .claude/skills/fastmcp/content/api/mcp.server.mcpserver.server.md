# `mcp.server.mcpserver.server`

Distribution: `mcp`

## _CallableT

`mcp.server.mcpserver.server._CallableT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CallableT = TypeVar('_CallableT', bound=Callable[..., Any])
```

## _MISSING_AUDIENCE

`mcp.server.mcpserver.server._MISSING_AUDIENCE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MISSING_AUDIENCE = 'request_state_security is configured but this server has no name. Sealed\nrequestState carries the server name as an audience claim, so state minted by\nanother service that shares the same keys is rejected; unnamed servers would\nall stamp the same placeholder and the check would mean nothing. Name the\nserver (MCPServer("my-service", ...)) or set RequestStateSecurity(audience=...).'
```

## logger

`mcp.server.mcpserver.server.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## MCPServer

Import as `mcp.server.MCPServer`  ·  defined at `mcp.server.mcpserver.server.MCPServer`

```python
class MCPServer(Generic[LifespanResultT])
```

**Also exported as** `mcp.server.MCPServer`, `mcp.server.mcpserver.MCPServer`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[LifespanResultT]`

**Declared members (34)**

- `def add_prompt(self, prompt: Prompt) -> None`
  Add a prompt to the server.
- `def add_resource(self, resource: Resource) -> None`
  Add a resource to the server.
- `def add_tool(self, fn: Callable[..., Any], name: str | None = None, title: str | None = None, description: str | None = None, annotations: ToolAnnotations | None = None, icons: list[Icon] | None = None, meta: dict[str, Any] | None = None, structured_output: bool | None = None) -> None`
  Add a tool to the server.
- `async def call_tool(self, name: str, arguments: dict[str, Any], context: Context[LifespanResultT, Any] | None = None) -> CallToolResult | InputRequiredResult`  _async_
  Call a tool by name with arguments.
- `def completion(self)`
  Decorator to register a completion handler.
- `def custom_route(self, path: str, methods: list[str], name: str | None = None, include_in_schema: bool = True)`
  Decorator to register a custom HTTP route on the MCP server.
- `dependencies = self.settings.dependencies`  _instance-attribute_
- `description: str | None`  _property_
- `async def get_prompt(self, name: str, arguments: dict[str, Any] | None = None, context: Context[LifespanResultT, Any] | None = None) -> GetPromptResult | InputRequiredResult`  _async_
  Get a prompt by name with arguments.
- `icons: list[Icon] | None`  _property_
- `instructions: str | None`  _property_
- `async def list_prompts(self) -> list[MCPPrompt]`  _async_
  List all available prompts.
- `async def list_resource_templates(self) -> list[MCPResourceTemplate]`  _async_
- `async def list_resources(self) -> list[MCPResource]`  _async_
  List all available resources.
- `async def list_tools(self) -> list[MCPTool]`  _async_
  List all available tools.
- `middleware: list[ServerMiddleware[Any]]`  _property_
  The middleware chain wrapping every inbound message, outermost-first.
- `name: str`  _property_
- `def prompt(self, name: str | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None) -> Callable[[_CallableT], _CallableT]`
  Decorator to register a prompt.
- `async def read_resource(self, uri: AnyUrl | str, context: Context[LifespanResultT, Any] | None = None) -> Iterable[ReadResourceContents] | InputRequiredResult`  _async_
  Read a resource by URI.
- `def remove_prompt(self, name: str) -> None`
  Remove a prompt from the server by name.
- `def remove_tool(self, name: str) -> None`
  Remove a tool from the server by name.
- `def resource(self, uri: str, name: str | None = None, title: str | None = None, description: str | None = None, mime_type: str | None = None, icons: list[Icon] | None = None, annotations: Annotations | None = None, meta: dict[str, Any] | None = None, security: ResourceSecurity | None = None) -> Callable[[_CallableT], _CallableT]`
  Decorator to register a function as a resource.
- `def run(self, transport: Literal['stdio', 'sse', 'streamable-http'] = 'stdio', kwargs: Any = {}) -> None`
  Run the MCP server. Note this is a synchronous function.
- `async def run_sse_async(self, host: str = '127.0.0.1', port: int = 8000, sse_path: str = '/sse', message_path: str = '/messages/', max_request_body_size: int = DEFAULT_MAX_REQUEST_BODY_SIZE, transport_security: TransportSecuritySettings | None = None) -> None`  _async_
  Run the server using SSE transport.
- `async def run_stdio_async(self) -> None`  _async_
  Run the server using stdio transport.
- `async def run_streamable_http_async(self, host: str = '127.0.0.1', port: int = 8000, streamable_http_path: str = '/mcp', json_response: bool = False, stateless_http: bool = False, event_store: EventStore | None = None, retry_interval: int | None = None, max_request_body_size: int = DEFAULT_MAX_REQUEST_BODY_SIZE, session_idle_timeout: float | None = DEFAULT_SESSION_IDLE_TIMEOUT, max_sessions: int | None = DEFAULT_MAX_SESSIONS, transport_security: TransportSecuritySettings | None = None) -> None`  _async_
  Run the server using StreamableHTTP transport.
- `session_manager: StreamableHTTPSessionManager`  _property_
  Get the StreamableHTTP session manager.
- `settings = Settings(debug=debug, log_level=log_level, warn_on_duplicate_resources=warn_on_duplicate_resources, warn_on_duplicate_tools=warn_on_duplicate_tools, warn_on_duplicate_prompts=warn_on_duplicate_prompts, dependencies=dependencies or [], lifespan=lifespan, auth=auth)`  _instance-attribute_
- `def sse_app(self, sse_path: str = '/sse', message_path: str = '/messages/', max_request_body_size: int = DEFAULT_MAX_REQUEST_BODY_SIZE, transport_security: TransportSecuritySettings | None = None, host: str = '127.0.0.1') -> Starlette`
  Return an instance of the SSE server app.
- `def streamable_http_app(self, streamable_http_path: str = '/mcp', json_response: bool = False, stateless_http: bool = False, event_store: EventStore | None = None, retry_interval: int | None = None, max_request_body_size: int = DEFAULT_MAX_REQUEST_BODY_SIZE, session_idle_timeout: float | None = DEFAULT_SESSION_IDLE_TIMEOUT, max_sessions: int | None = DEFAULT_MAX_SESSIONS, transport_security: TransportSecuritySettings | None = None, host: str = '127.0.0.1') -> Starlette`
  Return an instance of the StreamableHTTP server app.
- `title: str | None`  _property_
- `def tool(self, name: str | None = None, title: str | None = None, description: str | None = None, annotations: ToolAnnotations | None = None, icons: list[Icon] | None = None, meta: dict[str, Any] | None = None, structured_output: bool | None = None) -> Callable[[_CallableT], _CallableT]`
  Decorator to register a tool.
- `version: str`  _property_
- `website_url: str | None`  _property_

## Settings

`mcp.server.mcpserver.server.Settings`

```python
class Settings(BaseModel, Generic[LifespanResultT])
```

**Bases** `BaseModel`, `Generic[LifespanResultT]`

**Declared members (8)**

- `auth: AuthSettings | None`  _instance-attribute_
- `debug: bool`  _instance-attribute_
- `dependencies: list[str]`  _instance-attribute_
  List of dependencies to install in the server environment. Used by the `mcp install` and `mcp dev` CLI.
- `lifespan: Callable[[MCPServer[LifespanResultT]], AbstractAsyncContextManager[LifespanResultT]] | None`  _instance-attribute_
  An async context manager that will be called when the server is started.
- `log_level: Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL']`  _instance-attribute_
- `warn_on_duplicate_prompts: bool`  _instance-attribute_
- `warn_on_duplicate_resources: bool`  _instance-attribute_
- `warn_on_duplicate_tools: bool`  _instance-attribute_

MCPServer settings, as passed to the `MCPServer` constructor.


## _version_gated

`mcp.server.mcpserver.server._version_gated`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _version_gated(method: MethodBinding) -> RequestHandler
```

Wrap a method handler so a request at a disallowed protocol version is rejected.

The low-level `_request_handlers` dict is keyed by method only, so per-version
scoping is enforced here rather than at the runner's boundary table.


## lifespan_wrapper

`mcp.server.mcpserver.server.lifespan_wrapper`

```python
def lifespan_wrapper(app: MCPServer[LifespanResultT], lifespan: Callable[[MCPServer[LifespanResultT]], AbstractAsyncContextManager[LifespanResultT]]) -> Callable[[Server[LifespanResultT]], AbstractAsyncContextManager[LifespanResultT]]
```

## require_client_extension

Import as `mcp.server.mcpserver.require_client_extension`  ·  defined at `mcp.server.mcpserver.server.require_client_extension`

```python
def require_client_extension(ctx: ServerRequestContext[Any, Any], identifier: str) -> None
```

**Also exported as** `mcp.server.mcpserver.require_client_extension`

Assert the connected client declared support for `identifier`.

Call this from an extension's handler or `intercept_tool_call` before
offering extension-specific behaviour. Raises `MCPError` with the
`-32021` (missing required client capability) code and a
`requiredCapabilities` payload when the client did not declare the
extension, per SEP-2133.

Args:
    ctx: The current request context.
    identifier: The extension identifier the client must have declared.

Raises:
    MCPError: With code `MISSING_REQUIRED_CLIENT_CAPABILITY` if the client
        did not advertise `identifier`.


