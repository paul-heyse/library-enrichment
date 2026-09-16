# `fastmcp.server.providers.proxy`

Distribution: `fastmcp`

## ClientFactoryT

`fastmcp.server.providers.proxy.ClientFactoryT`

```python
ClientFactoryT = Callable[[], Client] | Callable[[], Awaitable[Client]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form '(() -> Client[Unknown]) | (() -> Awaitable[Client[Unknown]])'> ````

## PROXY_TRANSPORT_OPTIONS

`fastmcp.server.providers.proxy.PROXY_TRANSPORT_OPTIONS`

```python
PROXY_TRANSPORT_OPTIONS = TransportOptions(session_class=_ForwardingClientSession, forward_incoming_headers=True)
```

**Inferred type** (`ty`, not declared in the source): `TransportOptions`

## ProxyIdentity

`fastmcp.server.providers.proxy.ProxyIdentity`

```python
ProxyIdentity = Literal['proxy', 'upstream']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["proxy", "upstream"]'> ````

## _CONNECTION_META_KEYS

`fastmcp.server.providers.proxy._CONNECTION_META_KEYS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CONNECTION_META_KEYS = frozenset({mcp_types.PROTOCOL_VERSION_META_KEY, mcp_types.CLIENT_INFO_META_KEY, mcp_types.CLIENT_CAPABILITIES_META_KEY})
```

## _ComponentT

`fastmcp.server.providers.proxy._ComponentT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ComponentT = TypeVar('_ComponentT')
```

## _DEFAULT_CACHE_TTL

`fastmcp.server.providers.proxy._DEFAULT_CACHE_TTL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_CACHE_TTL: float = 300.0
```

## _PROXY_TRANSPORT_CAUSES

`fastmcp.server.providers.proxy._PROXY_TRANSPORT_CAUSES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PROXY_TRANSPORT_CAUSES: tuple[type[Exception], ...] = (TimeoutError, httpx2.HTTPError, anyio.ClosedResourceError, anyio.EndOfStream, anyio.BrokenResourceError)
```

## _PROXY_TRANSPORT_ERRORS

`fastmcp.server.providers.proxy._PROXY_TRANSPORT_ERRORS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PROXY_TRANSPORT_ERRORS: tuple[type[Exception], ...] = (RuntimeError, *_PROXY_TRANSPORT_CAUSES)
```

## logger

`fastmcp.server.providers.proxy.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FastMCPProxy

Import as `fastmcp.server.server.FastMCPProxy`  ·  defined at `fastmcp.server.providers.proxy.FastMCPProxy`

```python
class FastMCPProxy(FastMCP)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCP`

**Declared members (2)**

- `client_factory = client_factory`  _instance-attribute_
- `provider_error_strategy = provider_error_strategy`  _instance-attribute_

**Inherited (55)**

- from `fastmcp.server.mixins.lifespan.LifespanMixin`: `docket`
- from `fastmcp.server.mixins.transport.TransportMixin`: `custom_route`, `http_app`, `run`, `run_async`, `run_http_async`, `run_stdio_async`
- from `fastmcp.server.providers.aggregate.AggregateProvider`: `get_app_tool`, `get_tool_by_hash`, `lifespan`, `providers`
- from `fastmcp.server.providers.base.Provider`: `disable`, `enable`, `transforms`, `wrap_transform`
- from `fastmcp.server.server.FastMCP`: `add_completion_handler`, `add_extension`, `add_middleware`, `add_prompt`, `add_provider`, `add_resource`, `add_template`, `add_tool`, `add_transform`, `auth`, `call_tool`, `client_log_level`, `completion`, `experimental_capabilities`, `from_fastapi`, `from_openapi`, `generate_name`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `icons`, `instructions`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `local_provider`, `middleware`, `mount`, `name`, `prompt`, `read_resource`, `render_prompt`, `resource`, `strict_input_validation`, `tool`, `version`, `website_url`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A FastMCP server that acts as a proxy to a remote MCP-compliant server.

This is a convenience wrapper that creates a FastMCP server with a
ProxyProvider. For more control, use FastMCP with add_provider(ProxyProvider(...)).

Example:
    ```python
    from fastmcp.server import create_proxy
    from fastmcp.server.providers.proxy import FastMCPProxy, ProxyClient

    # Create a proxy server using create_proxy (recommended)
    proxy = create_proxy("http://localhost:8000/mcp")

    # Or use FastMCPProxy directly with explicit client factory
    proxy = FastMCPProxy(client_factory=lambda: ProxyClient("http://localhost:8000/mcp"))
    ```


## ProxyClient

`fastmcp.server.providers.proxy.ProxyClient`

```python
class ProxyClient(Client[ClientTransportT])
```

**Bases** `Client[ClientTransportT]`

**Declared members (1)**

- `def new(self) -> ProxyClient[ClientTransportT]`

**Inherited (40)**

- from `fastmcp.client.client.Client`: `auto_initialize`, `cancel`, `close`, `complete`, `complete_mcp`, `generate_name`, `initialize`, `initialize_result`, `input_required_max_rounds`, `instructions`, `is_connected`, `mode`, `name`, `ping`, `prior_discover`, `progress`, `protocol_version`, `send_roots_list_changed`, `server_capabilities`, `server_info`, `session`, `set_elicitation_callback`, `set_logging_level`, `set_roots`, `set_sampling_callback`, `transport`
- from `fastmcp.client.mixins.prompts.ClientPromptsMixin`: `get_prompt`, `get_prompt_mcp`, `list_prompts`, `list_prompts_mcp`
- from `fastmcp.client.mixins.resources.ClientResourcesMixin`: `list_resource_templates`, `list_resource_templates_mcp`, `list_resources`, `list_resources_mcp`, `read_resource`, `read_resource_mcp`
- from `fastmcp.client.mixins.tools.ClientToolsMixin`: `call_tool`, `call_tool_mcp`, `list_tools`, `list_tools_mcp`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A proxy client that forwards advanced interactions between a remote MCP server and the proxy's connected clients.

Supports forwarding roots, sampling, elicitation, logging, and progress.

The default forwarding handlers must resolve the *proxy's* request context so
they relay server-initiated requests (roots/sampling/elicitation) back to the
proxy's own connected client, not to the upstream server they are talking to.
Under SDK v2 an in-memory backend runs in the same event loop as this client,
so a naive ``get_context()`` inside a handler can resolve to the backend's
context and forward the request straight back to the backend — an infinite
loop. To avoid that, ``ProxyTool.run`` (and the other proxy components) stash
the proxy-side ``RequestContext`` in ``_proxy_rc_ref`` before each backend
call, and the handlers are wrapped to restore it before forwarding.


## ProxyInitializeMiddleware

`fastmcp.server.providers.proxy.ProxyInitializeMiddleware`

```python
class ProxyInitializeMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (2)**

- `async def on_initialize(self, context: MiddlewareContext[mcp_types.InitializeRequest], call_next: CallNext[mcp_types.InitializeRequest, mcp_types.InitializeResult | None]) -> mcp_types.InitializeResult | None`  _async_
- `proxy = proxy`  _instance-attribute_

**Inherited (11)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_message`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Deprecated middleware for forwarding instructions during initialization.


## ProxyMetadataMiddleware

`fastmcp.server.providers.proxy.ProxyMetadataMiddleware`

```python
class ProxyMetadataMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (4)**

- `identity = identity`  _instance-attribute_
- `async def on_discover(self, context: MiddlewareContext[mcp_types.DiscoverRequest], call_next: CallNext[mcp_types.DiscoverRequest, mcp_types.DiscoverResult | dict[str, Any]]) -> mcp_types.DiscoverResult | dict[str, Any]`  _async_
- `async def on_initialize(self, context: MiddlewareContext[mcp_types.InitializeRequest], call_next: CallNext[mcp_types.InitializeRequest, mcp_types.InitializeResult | None]) -> mcp_types.InitializeResult | None`  _async_
- `provider = provider`  _instance-attribute_

**Inherited (10)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_get_prompt`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_message`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Forward optional server metadata from a ``ProxyProvider`` backend.

Instructions and namespaced metadata are forwarded with frontend values
taking precedence. Protocol versions, capabilities, cache policy, and result
type are never copied from the backend. ``identity`` controls whether server
identity remains the gateway's or uses the backend's when available.


## ProxyPrompt

`fastmcp.server.providers.proxy.ProxyPrompt`

```python
class ProxyPrompt(Prompt)
```

**Bases** `Prompt`

**Declared members (4)**

- `def from_mcp_prompt(cls, client_factory: ClientFactoryT, mcp_prompt: mcp_types.Prompt) -> ProxyPrompt`  _classmethod_
  Factory method to create a ProxyPrompt from a raw MCP prompt schema.
- `def get_span_attributes(self) -> dict[str, Any]`
- `async def render(self, arguments: dict[str, Any]) -> PromptResult`  _async_
  Render the prompt by making a call through the client.
- `task_config: TaskConfig = TaskConfig(mode='forbidden')`  _class-attribute, instance-attribute_

**Inherited (18)**

- from `fastmcp.prompts.base.Prompt`: `KEY_PREFIX`, `arguments`, `auth`, `convert_result`, `from_function`, `to_mcp_prompt`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Prompt that represents and renders a prompt from a remote server.


## ProxyProvider

Import as `fastmcp.server.providers.ProxyProvider`  ·  defined at `fastmcp.server.providers.proxy.ProxyProvider`

```python
class ProxyProvider(Provider)
```

**Also exported as** `fastmcp.server.providers.ProxyProvider`

**Bases** `Provider`

**Declared members (3)**

- `client_factory = client_factory`  _instance-attribute_
- `async def get_tasks(self) -> Sequence[FastMCPComponent]`  _async_
  Return empty list since proxy components don't support tasks.
- `async def get_tool_by_hash(self, tool_hash: str, tool_name: str) -> Tool | None`  _async_
  Resolve an identity against the remote listing.

**Inherited (15)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `lifespan`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider that proxies to a remote MCP server via a client factory.

This provider fetches components from a remote server and returns Proxy*
component instances that forward execution to the remote server.

All components returned by this provider have task_config.mode="forbidden"
because tasks cannot be executed through a proxy.

Component lists (tools, resources, templates, prompts) are cached so that
individual lookups (e.g. during ``call_tool``) can resolve from the cache
instead of opening a new backend connection.  The cache stores the
backend's raw component metadata and is shared across all sessions;
per-session visibility and auth filtering are applied after cache lookup
by the server layer.  The cache is refreshed whenever a ``list_*`` call
is made, and entries expire after ``cache_ttl`` seconds (default 300).
Set ``cache_ttl=0`` to disable caching.  Disabling is recommended for
backends whose component lists change dynamically.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.providers.proxy import ProxyProvider, ProxyClient

    # Create a proxy provider for a remote server
    proxy = ProxyProvider(lambda: ProxyClient("http://localhost:8000/mcp"))

    mcp = FastMCP("Proxy Server")
    mcp.add_provider(proxy)

    # Can also add with a namespace
    mcp.add_provider(proxy, namespace="remote")
    ```


## ProxyResource

`fastmcp.server.providers.proxy.ProxyResource`

```python
class ProxyResource(Resource)
```

**Bases** `Resource`

**Declared members (4)**

- `def from_mcp_resource(cls, client_factory: ClientFactoryT, mcp_resource: mcp_types.Resource) -> ProxyResource`  _classmethod_
  Factory method to create a ProxyResource from a raw MCP resource schema.
- `def get_span_attributes(self) -> dict[str, Any]`
- `async def read(self) -> ResourceResult`  _async_
  Read the resource content from the remote server.
- `task_config: TaskConfig = TaskConfig(mode='forbidden')`  _class-attribute, instance-attribute_

**Inherited (22)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `key`, `mime_type`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Resource that represents and reads a resource from a remote server.


## ProxyTemplate

`fastmcp.server.providers.proxy.ProxyTemplate`

```python
class ProxyTemplate(ResourceTemplate)
```

**Bases** `ResourceTemplate`

**Declared members (4)**

- `async def create_resource(self, uri: str, params: dict[str, Any], context: Context | None = None) -> ProxyResource`  _async_
  Create a resource from the template by calling the remote server.
- `def from_mcp_template(cls, client_factory: ClientFactoryT, mcp_template: mcp_types.ResourceTemplate) -> ProxyTemplate`  _classmethod_
  Factory method to create a ProxyTemplate from a raw MCP template schema.
- `def get_span_attributes(self) -> dict[str, Any]`
- `task_config: TaskConfig = TaskConfig(mode='forbidden')`  _class-attribute, instance-attribute_

**Inherited (26)**

- from `fastmcp.resources.template.ResourceTemplate`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `key`, `matches`, `mime_type`, `parameters`, `read`, `resolve_security`, `security`, `set_default_mime_type`, `to_mcp_template`, `uri_template`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `name`, `tags`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A ResourceTemplate that represents and creates resources from a remote server template.


## ProxyTool

`fastmcp.server.providers.proxy.ProxyTool`

```python
class ProxyTool(Tool)
```

**Bases** `Tool`

**Declared members (4)**

- `def from_mcp_tool(cls, client_factory: ClientFactoryT, mcp_tool: mcp_types.Tool) -> ProxyTool`  _classmethod_
  Factory method to create a ProxyTool from a raw MCP tool schema.
- `def get_span_attributes(self) -> dict[str, Any]`
- `async def run(self, arguments: dict[str, Any], context: Context | None = None) -> ToolResult`  _async_
  Executes the tool by making a call through the client.
- `task_config: TaskConfig = TaskConfig(mode='forbidden')`  _class-attribute, instance-attribute_

**Inherited (24)**

- from `fastmcp.tools.base.Tool`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `execution`, `from_function`, `from_tool`, `output_schema`, `parameters`, `return_type`, `timeout`, `to_mcp_tool`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Tool that represents and executes a tool on a remote server.


## StatefulProxyClient

`fastmcp.server.providers.proxy.StatefulProxyClient`

```python
class StatefulProxyClient(ProxyClient[ClientTransportT])
```

**Bases** `ProxyClient[ClientTransportT]`

**Declared members (3)**

- `async def clear(self)`  _async_
  Clear all cached clients and force disconnect them.
- `def new(self) -> StatefulProxyClient[ClientTransportT]`
- `def new_stateful(self) -> Client[ClientTransportT]`
  Create a new stateful proxy client instance with the same configuration.

**Inherited (40)**

- from `fastmcp.client.client.Client`: `auto_initialize`, `cancel`, `close`, `complete`, `complete_mcp`, `generate_name`, `initialize`, `initialize_result`, `input_required_max_rounds`, `instructions`, `is_connected`, `mode`, `name`, `ping`, `prior_discover`, `progress`, `protocol_version`, `send_roots_list_changed`, `server_capabilities`, `server_info`, `session`, `set_elicitation_callback`, `set_logging_level`, `set_roots`, `set_sampling_callback`, `transport`
- from `fastmcp.client.mixins.prompts.ClientPromptsMixin`: `get_prompt`, `get_prompt_mcp`, `list_prompts`, `list_prompts_mcp`
- from `fastmcp.client.mixins.resources.ClientResourcesMixin`: `list_resource_templates`, `list_resource_templates_mcp`, `list_resources`, `list_resources_mcp`, `read_resource`, `read_resource_mcp`
- from `fastmcp.client.mixins.tools.ClientToolsMixin`: `call_tool`, `call_tool_mcp`, `list_tools`, `list_tools_mcp`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A proxy client that provides a stateful client factory for the proxy server.

The stateful proxy client bound its copy to the server session.
And it will be disconnected when the session is exited.

This is useful to proxy a stateful mcp server such as the Playwright MCP server.
Note that it is essential to ensure that the proxy server itself is also stateful.

The base ``ProxyClient`` already installs the context-restoring handlers
(see its docstring); this subclass additionally caches one client per stable
``Connection`` and forces disconnect when the connection is torn down.


## _CacheEntry

`fastmcp.server.providers.proxy._CacheEntry`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CacheEntry(Generic[_ComponentT])
```

**Bases** `Generic[_ComponentT]`

**Declared members (3)**

- `def is_fresh(self, ttl: float) -> bool`
- `items = items`  _instance-attribute_
- `timestamp = timestamp`  _instance-attribute_

A cached sequence of components with a monotonic timestamp.


## _ForwardingClientSession

`fastmcp.server.providers.proxy._ForwardingClientSession`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ForwardingClientSession(ClientSession)
```

**Bases** `ClientSession`

**Declared members (1)**

- `async def validate_tool_result(self, name: str, result: mcp_types.CallToolResult) -> None`  _async_

A session that does not enforce the backend's declared output schema.

`ClientSession.call_tool` normally validates a tool's structured content
against the output schema the backend advertised, raising if they disagree.
That check belongs to whoever consumes the result. A proxy only relays it,
and the end client runs the same check for itself, so enforcing it mid-path
turns a backend's schema bug into a proxy error and hides the real response.


## _UpstreamServerMetadata

`fastmcp.server.providers.proxy._UpstreamServerMetadata`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _UpstreamServerMetadata
```

**Declared members (6)**

- `def from_client(cls, client: Client) -> _UpstreamServerMetadata | None`  _classmethod_
- `def from_discover(cls, result: mcp_types.DiscoverResult) -> _UpstreamServerMetadata`  _classmethod_
- `def from_result(cls, result: mcp_types.InitializeResult | mcp_types.DiscoverResult, server_info: mcp_types.Implementation | None) -> _UpstreamServerMetadata`  _classmethod_
  Detach forwarded values from the backend session's adopted result.
- `instructions: str | None`  _instance-attribute_
- `meta: dict[str, Any]`  _instance-attribute_
- `server_info: mcp_types.Implementation | None`  _instance-attribute_

## _create_client_factory

`fastmcp.server.providers.proxy._create_client_factory`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_client_factory(target: Client[ClientTransportT] | ClientTransport | FastMCP[Any] | SDKServer | AnyUrl | Path | MCPConfig | dict[str, Any] | str, mode: str | None = None) -> ClientFactoryT
```

Create a client factory from the given target.

Internal helper that handles the session strategy based on the target type:
- Connected Client: reuses existing session (with warning about context mixing)
- Disconnected Client: creates fresh sessions per request
- Other targets: creates ProxyClient and fresh sessions per request


## _forwardable_request_meta

`fastmcp.server.providers.proxy._forwardable_request_meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _forwardable_request_meta(ctx: Context | None) -> dict[str, Any] | None
```

Frontend request metadata that may cross onto the backend connection.

This is the proxy's one sanctioned read of the inbound request's `_meta`:
progress tokens, tracing, task, and application metadata pass through,
while connection-owned keys (`_CONNECTION_META_KEYS`) are dropped because
they describe the frontend connection, not the backend one.


## _forwardable_server_meta

`fastmcp.server.providers.proxy._forwardable_server_meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _forwardable_server_meta(meta: dict[str, Any] | None) -> dict[str, Any]
```

Backend result metadata that may cross onto the frontend connection.


## _has_transport_cause

`fastmcp.server.providers.proxy._has_transport_cause`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_transport_cause(error: RuntimeError) -> bool
```

## _make_restoring_handler

`fastmcp.server.providers.proxy._make_restoring_handler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_restoring_handler(handler: Callable, rc_ref: list[Any]) -> Callable
```

Wrap a proxy handler to restore request_ctx before delegating.


## _mirror_front_era_mode

`fastmcp.server.providers.proxy._mirror_front_era_mode`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _mirror_front_era_mode() -> str | None
```

Return the backend connect ``mode`` that mirrors the front connection's era.

A proxy is a server on its front and a client on its back. The two protocol
eras have mutually exclusive interaction models on a single session, so the
whole chain must speak one era end-to-end: a modern front must reach a modern
backend (a guard tool's `InputRequiredResult` round-trips), and a handshake
front must reach a handshake backend (server-initiated sampling / elicitation
/ roots push-forwarding works). Rather than pin its own era, the proxy speaks
on its back whatever era was negotiated on its front.

Reads the negotiated protocol version from the active front request context:

- modern front → that exact version, so the backend negotiates the same era
  (pinning the version rather than ``"auto"`` makes the eras truly match).
- handshake front → ``"legacy"``.
- no request context (e.g. proxy construction before any request) → ``None``,
  leaving the factory's configured default mode in place.


## _proxy_upstream_error

`fastmcp.server.providers.proxy._proxy_upstream_error`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _proxy_upstream_error(error: Exception) -> MCPError
```

Report an unreachable backend the same way however the failure arrived.

Depending on where the dead connection is noticed, the proxy sees either
FastMCP's own `RuntimeError("Client failed to connect: ...")` or the raw
transport error underneath it. Both describe one thing — the proxy could
not reach its upstream — so both are presented identically rather than
leaking the race into the message the front client reads.


## _relay_read_resource

`fastmcp.server.providers.proxy._relay_read_resource`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _relay_read_resource(client: Client, uri: str, ctx: Context | None) -> list[mcp_types.TextResourceContents | mcp_types.BlobResourceContents] | mcp_types.InputRequiredResult
```

Read a backend resource, surfacing a guard ask rather than driving it.

Mirrors `ProxyTool.run`: on a modern backend the low-level session is used
so an `InputRequiredResult` (SEP-2322) comes back as a result for the parent
to forward, instead of the high-level client trying to answer it here — the
proxy has no back-channel to the real user, so driving it fails outright.
The inbound request's continuation state travels down so the backend guard
sees the client's answers on its own `ctx.input_responses`.


## _restore_request_context

`fastmcp.server.providers.proxy._restore_request_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _restore_request_context(rc_ref: list[Any]) -> None
```

Set the ``request_ctx``, ``_current_context`` and ``_current_server``
ContextVars from stashed values so a proxy forwarding handler relays to the
proxy's own client rather than the upstream server.

Called at the start of every proxy handler invocation. The stashed proxy
``RequestContext`` is the correct forwarding target, so we restore it unless
it is already active. This covers two cases:

- Stateful proxy: the reused receive-loop task carries a stale ContextVar
  from an earlier request (same session, different request_id).
- In-memory backend (SDK v2): the backend runs in this event loop, so the
  handler may inherit the *backend's* request_ctx (a different session).

We stash a ``(RequestContext, weakref[FastMCP])`` tuple — never a
``Context`` instance — because ``Context`` properties are themselves
ContextVar-dependent and would resolve stale values in the receive
loop.  Instead we construct a fresh ``Context`` here after restoring
``request_ctx``, so its property accesses read the correct values.

This is a set-only repair of a long-lived task's ContextVars, not a
scope: we never ``reset()`` because the prior values are stale and
the loop keeps running.  ``_current_server`` is restored alongside
``_current_context`` so handlers that resolve the server via
dependency injection (e.g. ``get_server()``) see the right instance;
it is set directly rather than via ``Context.__aenter__`` to avoid
opening a context-manager lifecycle on an unscoped path.


## _session_request_meta

`fastmcp.server.providers.proxy._session_request_meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _session_request_meta(meta: dict[str, Any] | None) -> mcp_types.RequestParamsMeta | None
```

Adapt forwardable metadata for a direct backend-session call.

Direct session calls bypass the high-level client mixins, so trace context
is injected here, matching what the mixins do on the legacy client paths.


## _stash_proxy_request_context

`fastmcp.server.providers.proxy._stash_proxy_request_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _stash_proxy_request_context(client: Client, ctx: Context) -> None
```

Stash the proxy's ``RequestContext`` on a ``ProxyClient`` before a backend call.

Every proxy component (tool, resource, template, prompt) must call this
before relaying to its backend so the forwarding handlers can restore the
proxy's request context before relaying a server-initiated request
(roots/sampling/elicitation) back to the proxy's client. Required for every
proxy client: under SDK v2 an in-memory backend shares this event loop, so a
handler's ``get_context()`` would otherwise resolve to the backend context
and the server-initiated request would hang until timeout.

We stash a ``(RequestContext, weakref[FastMCP])`` tuple — never a ``Context``
instance — because ``Context`` properties are themselves ContextVar-dependent
and would resolve stale values in the receive loop.


## _with_proxy_transport_options

`fastmcp.server.providers.proxy._with_proxy_transport_options`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _with_proxy_transport_options(options: TransportOptions | None) -> TransportOptions
```

Layer proxy-owned settings onto options supplied by another client layer.


## default_proxy_elicitation_handler

`fastmcp.server.providers.proxy.default_proxy_elicitation_handler`

```python
async def default_proxy_elicitation_handler(message: str, response_type: type, params: mcp_types.ElicitRequestParams, context: ServerRequestContext[Any, Any]) -> ElicitResult
```

Forward elicitation request from remote server to proxy's connected clients.


## default_proxy_log_handler

`fastmcp.server.providers.proxy.default_proxy_log_handler`

```python
async def default_proxy_log_handler(message: LogMessage) -> None
```

Forward log notification from remote server to proxy's connected clients.


## default_proxy_progress_handler

`fastmcp.server.providers.proxy.default_proxy_progress_handler`

```python
async def default_proxy_progress_handler(progress: float, total: float | None, message: str | None) -> None
```

Forward progress notification from remote server to proxy's connected clients.


## default_proxy_roots_handler

`fastmcp.server.providers.proxy.default_proxy_roots_handler`

```python
async def default_proxy_roots_handler(context: ServerRequestContext[Any, Any]) -> RootsList
```

Forward list roots request from remote server to proxy's connected clients.

A handshake-era backend can still issue `roots/list`, and the proxy is that
backend's client, so it relays the request onto its own front session. This
reaches the wire through the SDK session rather than a `Context` method:
`ctx.list_roots()` is not part of FastMCP's server-authoring API, because
SEP-2322 removed server-initiated requests from the modern protocol. The
relay exists only for handshake-era interop on both legs.


## default_proxy_sampling_handler

`fastmcp.server.providers.proxy.default_proxy_sampling_handler`

```python
async def default_proxy_sampling_handler(messages: list[mcp_types.SamplingMessage], params: mcp_types.CreateMessageRequestParams, context: ServerRequestContext[Any, Any]) -> mcp_types.CreateMessageResult
```

Forward sampling request from remote server to proxy's connected clients.

Relays through the SDK session for the same reason as
`default_proxy_roots_handler`: server-initiated sampling is not part of
FastMCP's server-authoring API, and this path only ever runs when both legs
of the proxy speak the handshake era.


