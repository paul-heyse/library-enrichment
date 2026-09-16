# `mcp.server.lowlevel.server`

Distribution: `mcp`

## LifespanResultT

Import as `mcp.server.mcpserver.server.LifespanResultT`  ·  defined at `mcp.server.lowlevel.server.LifespanResultT`

```python
LifespanResultT = TypeVar('LifespanResultT', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## NotificationHandler

`mcp.server.lowlevel.server.NotificationHandler`

```python
NotificationHandler = Callable[[ServerRequestContext[LifespanResultT], _ParamsT], Awaitable[None]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( ServerRequestContext[LifespanResultT@NotificationHandler, Any], _ParamsT@NotificationHandler, / ) -> Awaitable[None]'> ``` --- A registered notification handler: `(ctx, params) -> None`.`

A registered notification handler: `(ctx, params) -> None`.


## RequestHandler

`mcp.server.lowlevel.server.RequestHandler`

```python
RequestHandler = Callable[[ServerRequestContext[LifespanResultT], _ParamsT], Awaitable[HandlerResult]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( ServerRequestContext[LifespanResultT@RequestHandler, Any], _ParamsT@RequestHandler, / ) -> Awaitable[BaseModel | dict[str, Any] | None]'> ``` --- A registered request handler: `(ctx, params) -> result`.`

A registered request handler: `(ctx, params) -> result`.


## _ParamsT

`mcp.server.lowlevel.server._ParamsT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ParamsT = TypeVar('_ParamsT', bound=BaseModel, default=BaseModel)
```

## logger

`mcp.server.lowlevel.server.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## HandlerEntry

`mcp.server.lowlevel.server.HandlerEntry`

```python
class HandlerEntry(Generic[LifespanResultT])
```

**Bases** `Generic[LifespanResultT]`

**Declared members (2)**

- `handler: RequestHandler[LifespanResultT, Any]`  _instance-attribute_
- `params_type: type[BaseModel]`  _instance-attribute_

A registered handler and the params model to validate incoming params against.

Stored in `Server._request_handlers` / `_notification_handlers` and consumed
by `ServerRunner` to validate, build `Context`, and invoke. The handler's
second-argument type is erased to `Any` in storage (each entry has a
different concrete params type and `Callable` parameters are contravariant);
the precise type is recoverable via `params_type`. The correlation is
enforced at registration time by `Server.add_request_handler`.


## NotificationOptions

Import as `mcp.server.NotificationOptions`  ·  defined at `mcp.server.lowlevel.server.NotificationOptions`

```python
class NotificationOptions
```

**Also exported as** `mcp.server.NotificationOptions`, `mcp.server.lowlevel.NotificationOptions`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `prompts_changed = prompts_changed`  _instance-attribute_
- `resources_changed = resources_changed`  _instance-attribute_
- `tools_changed = tools_changed`  _instance-attribute_

## Server

Import as `mcp.server.Server`  ·  defined at `mcp.server.lowlevel.server.Server`

```python
class Server(Generic[LifespanResultT])
```

**Also exported as** `mcp.server.Server`, `mcp.server.lowlevel.Server`

_10 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[LifespanResultT]`

**Declared members (22)**

- `def add_notification_handler(self, method: str, params_type: type[_ParamsT], handler: NotificationHandler[LifespanResultT, _ParamsT]) -> None`
  Register a notification handler for `method`.
- `def add_request_handler(self, method: str, params_type: type[_ParamsT], handler: RequestHandler[LifespanResultT, _ParamsT]) -> None`
  Register a request handler for `method`.
- `cache_hints: dict[str, CacheHint] = validate_cache_hints(cache_hints)`  _instance-attribute_
- `def create_initialization_options(self, notification_options: NotificationOptions | None = None, experimental_capabilities: dict[str, dict[str, Any]] | None = None, extensions: dict[str, dict[str, Any]] | None = None) -> InitializationOptions`
  Create initialization options from this server instance.
- `description = description`  _instance-attribute_
- `extensions: dict[str, dict[str, Any]] = {}`  _instance-attribute_
- `def get_capabilities(self, notification_options: NotificationOptions | None = None, experimental_capabilities: dict[str, dict[str, Any]] | None = None, extensions: dict[str, dict[str, Any]] | None = None, protocol_version: str | None = None) -> types.ServerCapabilities`
  Convert existing handlers to a ServerCapabilities object.
- `def get_notification_handler(self, method: str) -> HandlerEntry[LifespanResultT] | None`
  Return the registered entry for a notification method, or `None`.
- `def get_request_handler(self, method: str) -> HandlerEntry[LifespanResultT] | None`
  Return the registered entry for a request method, or `None`.
- `icons = icons`  _instance-attribute_
- `instructions = instructions`  _instance-attribute_
- `lifespan = lifespan`  _instance-attribute_
- `middleware: list[ServerMiddleware[LifespanResultT]] = [OpenTelemetryMiddleware()]`  _instance-attribute_
- `name = name`  _instance-attribute_
- `async def run(self, read_stream: ReadStream[SessionMessage | Exception], write_stream: WriteStream[SessionMessage], initialization_options: InitializationOptions, raise_exceptions: bool = False) -> None`  _async_
  Serve a single connection over the given streams until the read side closes.
- `server_info: types.Implementation`  _property_
  The `serverInfo` block describing this implementation.
- `server_info_stamp: dict[str, Any]`  _property_
  A fresh wire dump of `server_info`; callers own the returned dict.
- `session_manager: StreamableHTTPSessionManager`  _property_
  Get the StreamableHTTP session manager.
- `def streamable_http_app(self, streamable_http_path: str = '/mcp', json_response: bool = False, stateless_http: bool = False, event_store: EventStore | None = None, retry_interval: int | None = None, max_request_body_size: int = DEFAULT_MAX_REQUEST_BODY_SIZE, session_idle_timeout: float | None = DEFAULT_SESSION_IDLE_TIMEOUT, max_sessions: int | None = DEFAULT_MAX_SESSIONS, transport_security: TransportSecuritySettings | None = None, host: str = '127.0.0.1', auth: AuthSettings | None = None, token_verifier: TokenVerifier | None = None, auth_server_provider: OAuthAuthorizationServerProvider[Any, Any, Any] | None = None, custom_starlette_routes: list[Route] | None = None, debug: bool = False) -> Starlette`
  Return an instance of the StreamableHTTP server app.
- `title = title`  _instance-attribute_
- `version = version`  _instance-attribute_
- `website_url = website_url`  _instance-attribute_

## _ping_handler

`mcp.server.lowlevel.server._ping_handler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _ping_handler(ctx: ServerRequestContext[Any], params: types.RequestParams | None) -> types.EmptyResult
```

## lifespan

Import as `mcp.server.mcpserver.server.default_lifespan`  ·  defined at `mcp.server.lowlevel.server.lifespan`

```python
async def lifespan(_: Server[Any]) -> AsyncIterator[dict[str, Any]]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Default lifespan context manager that does nothing.

Returns:
    An empty context object


