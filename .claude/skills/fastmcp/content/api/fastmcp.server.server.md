# `fastmcp.server.server`

Distribution: `fastmcp`

## DuplicateBehavior

`fastmcp.server.server.DuplicateBehavior`

```python
DuplicateBehavior = Literal['warn', 'error', 'replace', 'ignore']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["warn", "error", "replace", "ignore"]'> ````

## F

`fastmcp.server.server.F`

```python
F = TypeVar('F', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## LifespanCallable

`fastmcp.server.server.LifespanCallable`

```python
LifespanCallable = Callable[['FastMCP[LifespanResultT]'], AbstractAsyncContextManager[LifespanResultT]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(FastMCP[LifespanResultT@LifespanCallable], /) -> AbstractAsyncContextManager[LifespanResultT@LifespanCallable, bool | None]'> ````

## Transport

Import as `fastmcp.server.mixins.transport.Transport`  ·  defined at `fastmcp.server.server.Transport`

```python
Transport = Literal['stdio', 'http', 'sse', 'streamable-http']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["stdio", "http", "sse", "streamable-http"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _REMOVED_KWARGS

`fastmcp.server.server._REMOVED_KWARGS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_REMOVED_KWARGS: dict[str, str] = {'host': 'Pass `host` to `run_http_async()`, or set FASTMCP_HOST.', 'port': 'Pass `port` to `run_http_async()`, or set FASTMCP_PORT.', 'sse_path': 'Pass `path` to `run_http_async()` or `http_app()`, or set FASTMCP_SSE_PATH.', 'message_path': 'Set FASTMCP_MESSAGE_PATH.', 'streamable_http_path': 'Pass `path` to `run_http_async()` or `http_app()`, or set FASTMCP_STREAMABLE_HTTP_PATH.', 'json_response': 'Pass `json_response` to `run_http_async()` or `http_app()`, or set FASTMCP_JSON_RESPONSE.', 'stateless_http': 'Pass `stateless_http` to `run_http_async()` or `http_app()`, or set FASTMCP_STATELESS_HTTP.', 'debug': 'Set FASTMCP_DEBUG.', 'log_level': 'Pass `log_level` to `run_http_async()`, or set FASTMCP_LOG_LEVEL.', 'on_duplicate_tools': 'Use `on_duplicate=` instead.', 'on_duplicate_resources': 'Use `on_duplicate=` instead.', 'on_duplicate_prompts': 'Use `on_duplicate=` instead.', 'tool_serializer': 'Return ToolResult from your tools instead. See https://gofastmcp.com/servers/tools#custom-serialization', 'include_tags': 'Use `server.enable(tags=..., only=True)` after creating the server.', 'exclude_tags': 'Use `server.disable(tags=...)` after creating the server.', 'tool_transformations': 'Use `server.add_transform(ToolTransform(...))` after creating the server.', 'sampling_handler': 'Server-initiated sampling is deprecated in MCP (SEP-2577) and the 2026-07-28 protocol has no back-channel for it (SEP-2322). Call an LLM directly from your tool.', 'sampling_handler_behavior': 'Server-initiated sampling is deprecated in MCP (SEP-2577) and the 2026-07-28 protocol has no back-channel for it (SEP-2322). Call an LLM directly from your tool.'}
```

## logger

`fastmcp.server.server.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FastMCP

Import as `fastmcp.FastMCP`  ·  defined at `fastmcp.server.server.FastMCP`

```python
class FastMCP(AggregateProvider, LifespanMixin, MCPOperationsMixin, TransportMixin, Generic[LifespanResultT])
```

**Also exported as** `fastmcp.FastMCP`, `fastmcp.server.FastMCP`

_26 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AggregateProvider`, `LifespanMixin`, `MCPOperationsMixin`, `TransportMixin`, `Generic[LifespanResultT]`

**Declared members (40)**

- `def add_completion_handler(self, handler: CompletionHandler) -> None`
  Register the server's argument-completion handler.
- `def add_extension(self, extension: ServerExtension) -> None`
  Register a server extension (SEP-2133).
- `def add_middleware(self, middleware: Middleware) -> None`
- `def add_prompt(self, prompt: Prompt | Callable[..., Any]) -> Prompt`
  Add a prompt to the server.
- `def add_provider(self, provider: Provider, namespace: str = '') -> None`
  Add a provider for dynamic tools, resources, and prompts.
- `def add_resource(self, resource: Resource | Callable[..., Any]) -> Resource | ResourceTemplate`
  Add a resource to the server.
- `def add_template(self, template: ResourceTemplate) -> ResourceTemplate`
  Add a resource template to the server.
- `def add_tool(self, tool: Tool | Callable[..., Any]) -> Tool`
  Add a tool to the server.
- `def add_transform(self, transform: Transform) -> None`
  Add a server-level transform.
- `auth: AuthProvider | None = auth`  _instance-attribute_
- `async def call_tool(self, name: str, arguments: dict[str, Any] | None = None, version: VersionSpec | None = None, run_middleware: bool = True) -> ToolResult`  _async_
  Call a tool by name.
- `client_log_level: mcp_types.LoggingLevel | None = client_log_level if client_log_level is not None else fastmcp.settings.client_log_level`  _instance-attribute_
- `def completion(self, handler: CompletionHandler | None = None) -> CompletionHandler | Callable[[CompletionHandler], CompletionHandler]`
  Decorator to register the server's argument-completion handler.
- `experimental_capabilities: dict[str, dict[str, Any]] = experimental_capabilities or {}`  _instance-attribute_
- `def from_fastapi(cls, app: Any, name: str | None = None, route_maps: list[RouteMap] | None = None, route_map_fn: OpenAPIRouteMapFn | None = None, mcp_component_fn: OpenAPIComponentFn | None = None, mcp_names: dict[str, str] | None = None, httpx_client_kwargs: dict[str, Any] | None = None, tags: set[str] | None = None, settings: Any = {}) -> Self`  _classmethod_
  Create a FastMCP server from a FastAPI application.
- `def from_openapi(cls, openapi_spec: dict[str, Any], client: httpx2.AsyncClient | None = None, name: str = 'OpenAPI Server', route_maps: list[RouteMap] | None = None, route_map_fn: OpenAPIRouteMapFn | None = None, mcp_component_fn: OpenAPIComponentFn | None = None, mcp_names: dict[str, str] | None = None, tags: set[str] | None = None, validate_output: bool = True, settings: Any = {}) -> Self`  _classmethod_
  Create a FastMCP server from an OpenAPI specification.
- `def generate_name(cls, name: str | None = None) -> str`  _classmethod_
- `async def get_prompt(self, name: str, version: VersionSpec | None = None) -> Prompt | None`  _async_
  Get a prompt by name, filtering disabled prompts.
- `async def get_resource(self, uri: str, version: VersionSpec | None = None) -> Resource | None`  _async_
  Get a resource by URI, filtering disabled resources.
- `async def get_resource_template(self, uri: str, version: VersionSpec | None = None) -> ResourceTemplate | None`  _async_
  Get a resource template by URI, filtering disabled templates.
- `async def get_tasks(self) -> Sequence[FastMCPComponent]`  _async_
  Get task-eligible components with all transforms applied.
- `async def get_tool(self, name: str, version: VersionSpec | None = None) -> Tool | None`  _async_
  Get a tool by name, filtering disabled tools.
- `icons: list[mcp_types.Icon]`  _property_
- `instructions: str | None`  _property, writable_
- `async def list_prompts(self, run_middleware: bool = True) -> Sequence[Prompt]`  _async_
  List all enabled prompts from providers.
- `async def list_resource_templates(self, run_middleware: bool = True) -> Sequence[ResourceTemplate]`  _async_
  List all enabled resource templates from providers.
- `async def list_resources(self, run_middleware: bool = True) -> Sequence[Resource]`  _async_
  List all enabled resources from providers.
- `async def list_tools(self, run_middleware: bool = True) -> Sequence[Tool]`  _async_
  List all enabled tools from providers.
- `local_provider: LocalProvider`  _property_
  The server's local provider, which stores directly-registered components.
- `middleware: list[Middleware] = list(middleware or [])`  _instance-attribute_
- `def mount(self, server: FastMCP[LifespanResultT], namespace: str | None = None, tool_names: dict[str, str] | None = None) -> None`
  Mount another FastMCP server on this server with an optional namespace.
- `name: str`  _property_
- `def prompt(self, name_or_fn: str | AnyFunction | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, tags: set[str] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> Callable[[AnyFunction], FunctionPrompt] | FunctionPrompt | partial[Callable[[AnyFunction], FunctionPrompt] | FunctionPrompt]`
  Decorator to register a prompt.
- `async def read_resource(self, uri: str, version: VersionSpec | None = None, run_middleware: bool = True) -> ResourceResult`  _async_
  Read a resource by URI.
- `async def render_prompt(self, name: str, arguments: dict[str, Any] | None = None, version: VersionSpec | None = None, run_middleware: bool = True) -> PromptResult`  _async_
  Render a prompt by name.
- `def resource(self, uri: str, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, mime_type: str | None = None, tags: set[str] | None = None, annotations: Annotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, app: AppConfig | dict[str, Any] | bool | None = None, auth: AuthCheck | list[AuthCheck] | None = None, security: ResourceSecurity | None | InheritSecurity = INHERIT_SECURITY) -> Callable[[F], F]`
  Decorator to register a function as a resource.
- `strict_input_validation: bool = strict_input_validation if strict_input_validation is not None else fastmcp.settings.strict_input_validation`  _instance-attribute_
- `def tool(self, name_or_fn: str | AnyFunction | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, tags: set[str] | None = None, output_schema: dict[str, Any] | NotSetT | None = NotSet, annotations: ToolAnnotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, app: AppConfig | dict[str, Any] | bool | None = None, task: bool | TaskConfig | None = None, timeout: float | None = None, auth: AuthCheck | list[AuthCheck] | None = None, run_in_thread: bool = True) -> Callable[[AnyFunction], FunctionTool] | FunctionTool | partial[Callable[[AnyFunction], FunctionTool] | FunctionTool]`
  Decorator to register a tool.
- `version: str | None`  _property_
- `website_url: str | None`  _property_

**Inherited (16)**

- from `fastmcp.server.mixins.lifespan.LifespanMixin`: `docket`
- from `fastmcp.server.mixins.transport.TransportMixin`: `custom_route`, `http_app`, `run`, `run_async`, `run_http_async`, `run_stdio_async`
- from `fastmcp.server.providers.aggregate.AggregateProvider`: `get_app_tool`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `disable`, `enable`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## StateValue

Import as `fastmcp.server.context.StateValue`  ·  defined at `fastmcp.server.server.StateValue`

```python
class StateValue(FastMCPBaseModel)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPBaseModel`

**Declared members (1)**

- `value: Any`  _instance-attribute_

Wrapper for stored context state values.


## _SuppressUnlistedToolWarning

`fastmcp.server.server._SuppressUnlistedToolWarning`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _SuppressUnlistedToolWarning(logging.Filter)
```

**Bases** `logging.Filter`

**Declared members (1)**

- `def filter(self, record: logging.LogRecord) -> bool`

## _check_removed_kwargs

`fastmcp.server.server._check_removed_kwargs`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _check_removed_kwargs(kwargs: dict[str, Any]) -> None
```

Raise helpful TypeErrors for kwargs FastMCP no longer accepts.


## _get_auth_context

`fastmcp.server.server._get_auth_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_auth_context() -> tuple[bool, Any]
```

Get auth context for the current request.

Returns a tuple of (skip_auth, token) where:
- skip_auth=True means auth checks should be skipped (STDIO transport)
- token is the access token for HTTP transports (may be None if unauthenticated)

Uses late import to avoid circular import with context.py.


## _lifespan_proxy

`fastmcp.server.server._lifespan_proxy`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _lifespan_proxy(fastmcp_server: FastMCP[LifespanResultT]) -> Callable[[LowLevelServer[LifespanResultT]], AbstractAsyncContextManager[LifespanResultT]]
```

## _tool_identity

`fastmcp.server.server._tool_identity`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _tool_identity(tool: Tool) -> str | None
```

Read a tool's stable identity hash, if it carries one.


## _version_request_meta

`fastmcp.server.server._version_request_meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _version_request_meta(version: VersionSpec | None) -> mcp_types.RequestParamsMeta | None
```

## create_proxy

Import as `fastmcp.server.create_proxy`  ·  defined at `fastmcp.server.server.create_proxy`

```python
def create_proxy(target: Client[ClientTransportT] | ClientTransport | FastMCP[Any] | SDKServer | AnyUrl | Path | MCPConfig | dict[str, Any] | str, mode: str | None = None, settings: Any = {}) -> FastMCPProxy
```

**Also exported as** `fastmcp.server.create_proxy`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create a FastMCP proxy server for the given target.

This is the recommended way to create a proxy server. For lower-level control,
use `FastMCPProxy` or `ProxyProvider` directly from `fastmcp.server.providers.proxy`.

Args:
    target: The backend to proxy to. Can be:
        - A Client instance (connected or disconnected)
        - A ClientTransport
        - A FastMCP server instance
        - A URL string or AnyUrl
        - A Path to a server script
        - An MCPConfig or dict
    mode: Protocol-era negotiation for auto-created proxy clients (a
        non-Client target). By default (``None``) the backend MIRRORS the
        front connection's negotiated era per request, so the whole chain
        speaks one era end-to-end: a modern front reaches a modern backend
        (a guard tool's `InputRequiredResult` (SEP-2322) round-trips) and a
        handshake front reaches a handshake backend (server-initiated
        sampling / elicitation / roots push-forwarding works). Pass an
        explicit mode (e.g. ``"auto"`` or a version string) to pin the
        backend era regardless of the front; this overrides mirroring and is
        appropriate when the backend only speaks one era. Ignored when
        `target` is already a `Client` (which carries its own mode).
    **settings: Additional settings passed to FastMCPProxy (name, etc.)

Returns:
    A FastMCPProxy server that proxies to the target.

Example:
    ```python
    from fastmcp.server import create_proxy

    # Create a proxy to a remote server
    proxy = create_proxy("http://remote-server/mcp")

    # Create a proxy to another FastMCP server
    proxy = create_proxy(other_server)
    ```


## default_lifespan

`fastmcp.server.server.default_lifespan`

```python
async def default_lifespan(server: FastMCP[LifespanResultT]) -> AsyncIterator[Any]
```

Default lifespan context manager that does nothing.

Args:
    server: The server instance this lifespan is managing

Returns:
    An empty dictionary as the lifespan result.


