# `mcp.client.client`

Distribution: `mcp`

## ConnectMode

`mcp.client.client.ConnectMode`

```python
ConnectMode = Literal['legacy', 'auto'] | str
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'str'> ``` --- ``mode=`` value: ``"legacy"`` (initialize handshake), ``"auto"`` (discover, fall back to initialize), or a modern protocol-version string (adopt directly). The ``str`` arm is for forward-compat; ``Client.__post_init__`` rejects anything outside that set at construction.`

``mode=`` value: ``"legacy"`` (initialize handshake), ``"auto"`` (discover, fall back to
initialize), or a modern protocol-version string (adopt directly). The ``str`` arm is for
forward-compat; ``Client.__post_init__`` rejects anything outside that set at construction.


## _CacheableT

`mcp.client.client._CacheableT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CacheableT = TypeVar('_CacheableT', bound=CacheableResult)
```

## _Connector

`mcp.client.client._Connector`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_Connector = Callable[[AsyncExitStack, ConnectMode, bool], Awaitable['Dispatcher[Any]']]
```

Resolved at ``__post_init__`` from the shape of ``server`` alone: enter whatever resources
are needed onto the exit stack and hand back the ``Dispatcher`` ``ClientSession`` will drive.
``mode`` and ``raise_exceptions`` are passed at call time so they're read at the same moment
``__aenter__`` reads them for the handshake step.


## _ResultT

`mcp.client.client._ResultT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ResultT = TypeVar('_ResultT')
```

## _T

`mcp.client.client._T`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_T = TypeVar('_T')
```

## logger

`mcp.client.client.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## Client

Import as `mcp.Client`  ·  defined at `mcp.client.client.Client`

```python
class Client
```

**Also exported as** `mcp.Client`, `mcp.client.Client`

**Declared members (36)**

- `cache: CacheConfig | None = field(default_factory=CacheConfig)`  _class-attribute, instance-attribute_
  Client-side response caching for the SEP-2549 cacheable methods (2026-07-28).
- `async def call_tool(self, name: str, arguments: dict[str, Any] | None = None, read_timeout_seconds: float | None = None, progress_callback: ProgressFnT | None = None, input_responses: InputResponses | None = None, request_state: str | None = None, meta: RequestParamsMeta | None = None) -> CallToolResult`  _async_
  Call a tool on the server.
- `client_info: Implementation | None = None`  _class-attribute, instance-attribute_
  Client implementation info to send to server.
- `async def complete(self, ref: ResourceTemplateReference | PromptReference, argument: dict[str, str], context_arguments: dict[str, str] | None = None) -> CompleteResult`  _async_
  Get completions for a prompt or resource template argument.
- `elicitation_callback: ElicitationFnT | None = None`  _class-attribute, instance-attribute_
  Callback for handling elicitation requests.
- `extensions: Sequence[ClientExtension] | None = None`  _class-attribute, instance-attribute_
  Opt-in client extensions (SEP-2133).
- `async def get_prompt(self, name: str, arguments: dict[str, str] | None = None, input_responses: InputResponses | None = None, request_state: str | None = None, meta: RequestParamsMeta | None = None) -> GetPromptResult`  _async_
  Get a prompt from the server.
- `input_required_max_rounds: int = DEFAULT_INPUT_REQUIRED_MAX_ROUNDS`  _class-attribute, instance-attribute_
  Cap on `InputRequiredResult` retry rounds before `call_tool` / `get_prompt` / `read_resource` give up. Use `client.session.<method>(..., allow_input_required=True)` to drive the loop manually instead.
- `instructions: str | None`  _property_
  Server-provided instructions text, if any.
- `async def list_prompts(self, cursor: str | None = None, meta: RequestParamsMeta | None = None, cache_mode: CacheMode = 'use') -> ListPromptsResult`  _async_
  List available prompts from the server.
- `async def list_resource_templates(self, cursor: str | None = None, meta: RequestParamsMeta | None = None, cache_mode: CacheMode = 'use') -> ListResourceTemplatesResult`  _async_
  List available resource templates from the server.
- `async def list_resources(self, cursor: str | None = None, meta: RequestParamsMeta | None = None, cache_mode: CacheMode = 'use') -> ListResourcesResult`  _async_
  List available resources from the server.
- `list_roots_callback: ListRootsFnT | None = None`  _class-attribute, instance-attribute_
  Callback for handling list roots requests.
- `async def list_tools(self, cursor: str | None = None, meta: RequestParamsMeta | None = None, cache_mode: CacheMode = 'use') -> ListToolsResult`  _async_
  List available tools from the server.
- `def listen(self, tools_list_changed: bool = False, prompts_list_changed: bool = False, resources_list_changed: bool = False, resource_subscriptions: Sequence[str] = ()) -> AbstractAsyncContextManager[Subscription]`
  Open a `subscriptions/listen` stream of typed change events (2026-07-28 only).
- `log_level: LoggingLevel | None = None`  _class-attribute, instance-attribute_
  The log level to opt in to on 2026-07-28+ connections (deprecated logging feature, SEP-2577).
- `logging_callback: LoggingFnT | None = None`  _class-attribute, instance-attribute_
  Callback for handling logging notifications.
- `message_handler: MessageHandlerFnT | None = None`  _class-attribute, instance-attribute_
  Callback for handling raw messages.
- `mode: ConnectMode = 'auto'`  _class-attribute, instance-attribute_
  How to negotiate the protocol version.
- `prior_discover: types.DiscoverResult | None = None`  _class-attribute, instance-attribute_
  A previously-obtained DiscoverResult to install via .adopt() when mode is a version pin. Ignored when mode='legacy'.
- `protocol_version: str`  _property_
  Negotiated protocol version (set by initialize/discover/adopt during ``__aenter__``).
- `raise_exceptions: bool = False`  _class-attribute, instance-attribute_
  Whether to raise exceptions from the server.
- `async def read_resource(self, uri: str, input_responses: InputResponses | None = None, request_state: str | None = None, meta: RequestParamsMeta | None = None, cache_mode: CacheMode = 'use') -> ReadResourceResult`  _async_
  Read a resource from the server.
- `read_timeout_seconds: float | None = None`  _class-attribute, instance-attribute_
  Timeout for read operations.
- `sampling_callback: SamplingFnT | None = None`  _class-attribute, instance-attribute_
  Callback for handling sampling requests.
- `sampling_capabilities: types.SamplingCapability | None = None`  _class-attribute, instance-attribute_
  Sampling sub-capabilities (e.g. tools) declared alongside `sampling_callback`; no effect without it.
- `async def send_ping(self, meta: RequestParamsMeta | None = None) -> EmptyResult`  _async_
  Send a ping request to the server.
- `async def send_progress_notification(self, progress_token: str | int, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
  Send a progress notification to the server.
- `async def send_roots_list_changed(self) -> None`  _async_
  Send a notification that the roots list has changed.
- `server: Server[Any] | MCPServer | Transport | StdioServerParameters | str`  _instance-attribute_
  The MCP server to connect to.
- `server_capabilities: ServerCapabilities`  _property_
  Server capabilities (set by initialize/discover/adopt during ``__aenter__``).
- `server_info: Implementation | None`  _property_
  Server name/version, or `None` when the server did not identify itself.
- `session: ClientSession`  _property_
  Get the underlying ClientSession.
- `async def set_logging_level(self, level: LoggingLevel, meta: RequestParamsMeta | None = None) -> EmptyResult`  _async_
  Set the logging level on the server.
- `async def subscribe_resource(self, uri: str, meta: RequestParamsMeta | None = None) -> EmptyResult`  _async_
  Subscribe to resource updates (2025-era servers only).
- `async def unsubscribe_resource(self, uri: str, meta: RequestParamsMeta | None = None) -> EmptyResult`  _async_
  Unsubscribe from resource updates (2025-era servers only).

A high-level MCP client for connecting to MCP servers.

Pass a URL string (Streamable HTTP), a `StdioServerParameters` (launch the command as a
subprocess and talk over its stdin/stdout), any `Transport`, or - in tests - a `Server` or
`MCPServer` instance to connect to it in-process.

Example:
    ```python
    import asyncio

    from mcp import Client

    async def main():
        async with Client("http://localhost:8000/mcp") as client:
            result = await client.call_tool("add", {"a": 1, "b": 2})

    asyncio.run(main())
    ```


## _FoldedExtensions

`mcp.client.client._FoldedExtensions`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _FoldedExtensions
```

**Declared members (4)**

- `ad: dict[str, dict[str, Any]] | None`  _instance-attribute_
- `bindings: tuple[NotificationBinding[Any], ...] | None`  _instance-attribute_
- `by_model: Mapping[type[Result], ResultClaim[Any]]`  _instance-attribute_
- `claims: dict[str, tuple[ResultClaim[Any], ...]] | None`  _instance-attribute_

`Client.extensions` instances folded into the shapes `ClientSession` consumes.


## _connect_inproc

`mcp.client.client._connect_inproc`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _connect_inproc(server: Server[Any]) -> _Connector
```

Connector for an in-process ``Server``: legacy mode drives the stream loop via
``InMemoryTransport``; any other mode drives the modern per-request path through a
``DirectDispatcher`` peer pair (no streams, no JSON-RPC framing, no initialize handshake).


## _connect_transport

`mcp.client.client._connect_transport`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _connect_transport(transport: Transport) -> _Connector
```

Connector for the stream-backed paths (URL, user-supplied ``Transport``).


## _connected

`mcp.client.client._connected`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _connected(value: _T | None) -> _T
```

Narrow a post-handshake session attribute from ``T | None`` to ``T``.

``Client.__aenter__`` only assigns ``_session`` after the handshake succeeds, so inside
``async with Client(...)`` these attributes are always populated; the ``.session`` gate
raises before this is reached otherwise. The guard exists for pyright, not runtime.


## _evicting_message_handler

`mcp.client.client._evicting_message_handler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _evicting_message_handler(cache: ClientResponseCache, user_handler: MessageHandlerFnT | None) -> MessageHandlerFnT
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Wrap the session message handler with cache eviction on server notifications.


## _fold_extensions

`mcp.client.client._fold_extensions`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _fold_extensions(extensions: Sequence[ClientExtension] | None) -> _FoldedExtensions
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Fold extension contributions at construction, naming both owners on duplicate tags or methods.


## _no_inbound_client_notifications

`mcp.client.client._no_inbound_client_notifications`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _no_inbound_client_notifications(_dctx: Any, _method: str, _params: Mapping[str, Any] | None) -> None
```

Server-side inbound ``OnNotify`` for the modern in-process path — receives nothing.

At 2026-07-28 the spec defines no client→server notifications: ``initialized`` and
``roots/list_changed`` are removed, and cancellation is structural (anyio scope cancel
through the direct await, not a notify). Server→client notifications (progress, log
messages) flow the other way via the per-request ``DispatchContext`` into the client's
callbacks, and are not seen here.


## _strip_userinfo

`mcp.client.client._strip_userinfo`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _strip_userinfo(url: str) -> str
```

Drop any userinfo from the URL's authority component; byte-exact otherwise.

Credentials must not enter cache-key material; any further normalization could merge distinct servers.


## _synthesize_discover

`mcp.client.client._synthesize_discover`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _synthesize_discover(protocol_version: str) -> types.DiscoverResult
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

