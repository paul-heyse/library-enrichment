# `fastmcp.client.client`

Distribution: `fastmcp`

## CacheableT

`fastmcp.client.client.CacheableT`

```python
CacheableT = TypeVar('CacheableT', bound=mcp_types.CacheableResult)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## ConnectMode

Import as `fastmcp.client.group.ConnectMode`  ·  defined at `fastmcp.client.client.ConnectMode`

```python
ConnectMode = Literal['legacy', 'auto'] | str
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'str'> ``` --- How the client negotiates the protocol era at connect time. - ``"auto"`` (the default): probe ``server/discover`` at the newest modern version and &nbsp;&nbsp;adopt it, falling back to the initialize handshake for any server that is not positive &nbsp;&nbsp;evidence of a modern peer (a denylist fallback — see the SDK's ``negotiate_auto``). - ``"legacy"``: the classic initialize handshake, byte-identical to pre-v4 behavior for &nbsp;&nbsp;handshake-era servers. Opt into this to force the old handshake. - a modern protocol-version string (e.g. ``"2026-07-28"``): adopt that version directly &nbsp;&nbsp;without probing, synthesizing a minimal ``DiscoverResult`` when none is supplied. The ``str`` arm is only for the version-pin case; ``Client.__init__`` rejects any other value.`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

How the client negotiates the protocol era at connect time.

- ``"auto"`` (the default): probe ``server/discover`` at the newest modern version and
  adopt it, falling back to the initialize handshake for any server that is not positive
  evidence of a modern peer (a denylist fallback — see the SDK's ``negotiate_auto``).
- ``"legacy"``: the classic initialize handshake, byte-identical to pre-v4 behavior for
  handshake-era servers. Opt into this to force the old handshake.
- a modern protocol-version string (e.g. ``"2026-07-28"``): adopt that version directly
  without probing, synthesizing a minimal ``DiscoverResult`` when none is supplied.

The ``str`` arm is only for the version-pin case; ``Client.__init__`` rejects any other value.


## ResultT

`fastmcp.client.client.ResultT`

```python
ResultT = TypeVar('ResultT')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## T

`fastmcp.client.client.T`

```python
T = TypeVar('T', bound='ClientTransport')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`fastmcp.client.client.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['Client', 'ElicitationHandler', 'LogHandler', 'MessageHandler', 'ProgressHandler', 'RootsHandler', 'RootsList', 'SamplingHandler', 'SessionKwargs']
```

## logger

`fastmcp.client.client.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## CallToolResult

Import as `fastmcp.cli.client.CallToolResult`  ·  defined at `fastmcp.client.client.CallToolResult`

```python
class CallToolResult
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `content: list[mcp_types.ContentBlock]`  _instance-attribute_
- `data: Any = None`  _class-attribute, instance-attribute_
- `is_error: bool = False`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None`  _instance-attribute_
- `structured_content: dict[str, Any] | None`  _instance-attribute_

Parsed result from a tool call.


## Client

Import as `fastmcp.Client`  ·  defined at `fastmcp.client.client.Client`

```python
class Client(Generic[ClientTransportT], ClientResourcesMixin, ClientPromptsMixin, ClientToolsMixin)
```

**Also exported as** `fastmcp.Client`, `fastmcp.client.Client`

_11 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[ClientTransportT]`, `ClientResourcesMixin`, `ClientPromptsMixin`, `ClientToolsMixin`

**Declared members (27)**

- `auto_initialize = auto_initialize`  _instance-attribute_
- `async def cancel(self, request_id: str | int, reason: str | None = None) -> None`  _async_
  Send a cancellation notification for an in-progress request.
- `async def close(self)`  _async_
- `async def complete(self, ref: mcp_types.ResourceTemplateReference | mcp_types.PromptReference, argument: dict[str, str], context_arguments: dict[str, Any] | None = None) -> mcp_types.Completion`  _async_
  Send a completion request to the server.
- `async def complete_mcp(self, ref: mcp_types.ResourceTemplateReference | mcp_types.PromptReference, argument: dict[str, str], context_arguments: dict[str, Any] | None = None) -> mcp_types.CompleteResult`  _async_
  Send a completion request and return the complete MCP protocol result.
- `def generate_name(cls, name: str | None = None) -> str`  _classmethod_
- `async def initialize(self, timeout: datetime.timedelta | float | int | None = None) -> mcp_types.InitializeResult`  _async_
  Send an initialize request to the server.
- `initialize_result: mcp_types.InitializeResult | None`  _property_
  Get the result of the initialization request.
- `input_required_max_rounds = input_required_max_rounds`  _instance-attribute_
- `instructions: str | None`  _property_
  The server's instructions, or `None` when absent or disconnected.
- `def is_connected(self) -> bool`
  Check if the client is currently connected.
- `mode: ConnectMode = mode`  _instance-attribute_
- `name = name or self.generate_name()`  _instance-attribute_
- `def new(self) -> Client[ClientTransportT]`
  Create a new client instance with the same configuration but fresh session state.
- `async def ping(self) -> bool`  _async_
  Send a ping request.
- `prior_discover: mcp_types.DiscoverResult | None`  _property_
  The configured result to adopt when `mode` pins a modern version.
- `async def progress(self, progress_token: str | int, progress: float, total: float | None = None, message: str | None = None) -> None`  _async_
  Send a progress notification.
- `protocol_version: str | None`  _property_
  The negotiated protocol version, or `None` when disconnected.
- `async def send_roots_list_changed(self) -> None`  _async_
  Send a roots/list_changed notification.
- `server_capabilities: mcp_types.ServerCapabilities | None`  _property_
  The server's advertised capabilities, or `None` when disconnected.
- `server_info: mcp_types.Implementation | None`  _property_
  The session's server identity, or `None` when disconnected.
- `session: ClientSession`  _property_
  Get the current active session. Raises RuntimeError if not connected.
- `def set_elicitation_callback(self, elicitation_callback: ElicitationHandler) -> None`
  Set the elicitation callback for the client.
- `async def set_logging_level(self, level: mcp_types.LoggingLevel) -> None`  _async_
  Send a logging/setLevel request.
- `def set_roots(self, roots: RootsList | RootsHandler) -> None`
  Set the roots for the client. This does not automatically call `send_roots_list_changed`.
- `def set_sampling_callback(self, sampling_callback: SamplingHandler, sampling_capabilities: mcp_types.SamplingCapability | None = None) -> None`
  Set the sampling callback for the client.
- `transport = cast(ClientTransportT, infer_transport(transport))`  _instance-attribute_

**Inherited (14)**

- from `fastmcp.client.mixins.prompts.ClientPromptsMixin`: `get_prompt`, `get_prompt_mcp`, `list_prompts`, `list_prompts_mcp`
- from `fastmcp.client.mixins.resources.ClientResourcesMixin`: `list_resource_templates`, `list_resource_templates_mcp`, `list_resources`, `list_resources_mcp`, `read_resource`, `read_resource_mcp`
- from `fastmcp.client.mixins.tools.ClientToolsMixin`: `call_tool`, `call_tool_mcp`, `list_tools`, `list_tools_mcp`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

MCP client that delegates connection management to a Transport instance.

The Client class is responsible for MCP protocol logic, while the Transport
handles connection establishment and management. Client provides methods for
working with resources, prompts, tools and other MCP capabilities.

This client supports reentrant context managers (multiple concurrent
`async with client:` blocks) using reference counting and background session
management. This allows efficient session reuse in any scenario with
nested or concurrent client usage.

MCP SDK 1.10 introduced automatic list_tools() calls during call_tool()
execution. This created a race condition where events could be reset while
other tasks were waiting on them, causing deadlocks. The issue was exposed
in proxy scenarios but affects any reentrant usage.

The solution uses reference counting to track active context managers,
a background task to manage the session lifecycle, events to coordinate
between tasks, and ensures all session state changes happen within a lock.
Events are only created when needed, never reset outside locks.

This design prevents race conditions where tasks wait on events that get
replaced by other tasks, ensuring reliable coordination in concurrent scenarios.

Args:
    transport:
        Connection source specification, which can be:

            - ClientTransport: Direct transport instance
            - FastMCP: In-process FastMCP server
            - AnyUrl or str: URL to connect to
            - Path: File path for local socket
            - MCPConfig: MCP server configuration
            - dict: Transport configuration

    roots: Optional RootsList or RootsHandler for filesystem access
    sampling_handler: Optional handler for sampling requests
    log_handler: Optional handler for log messages
    message_handler: Optional handler for protocol messages
    progress_handler: Optional handler for progress notifications
    timeout: Optional timeout for requests (seconds or timedelta)
    init_timeout: Optional timeout for initial connection (seconds or timedelta).
        Set to 0 to disable. If None, uses the value in the FastMCP global settings.
    mode: Protocol-era negotiation at connect time. `"auto"` (the default) probes
        `server/discover` and negotiates the modern era, denylist-falling-back to the
        initialize handshake for any server that is not positive evidence of a modern
        peer — safe against a mixed fleet of legacy and modern servers. `"legacy"`
        forces the initialize handshake, byte-identical to pre-v4 behavior; opt into it
        to pin the old handshake. A modern version string (e.g. `"2026-07-28"`) adopts
        that version directly without a probe.
    prior_discover: A previously obtained `DiscoverResult` to adopt when `mode` is a
        version pin, reused instead of synthesizing a minimal one. Ignored otherwise.
    input_required_max_rounds: Cap on `InputRequiredResult` (SEP-2322) retry rounds
        for `call_tool` / `get_prompt` / `read_resource` before the driver gives up.
        Only reachable on 2026-era servers that emit `InputRequiredResult`.
    cache: Client-side response caching (SEP-2549), opt-in. `None` (default) and
        `False` disable it; `True` enables the default in-memory store honoring
        server `ttlMs`/`cacheScope` hints; a `CacheConfig` customizes it. Honoring is
        modern-only, so a cache is inert on legacy connections. A custom `CacheConfig`
        store requires `target_id`, since FastMCP transports expose no server URL to
        derive a shared-store identity from.
    extensions: Opt-in client extensions (SEP-2133), a sequence of
        `mcp.client.extension.ClientExtension` instances. Each contributes its
        capability advertisement, its result claims, and its notification bindings,
        all of which are threaded into the underlying session. User-supplied
        notification bindings compose with FastMCP's internal task-status binding
        rather than replacing it. A claimed `call_tool` result is resolved
        transparently through the owning extension's resolver. For an advertise-only
        entry, use `mcp.client.advertise(identifier, settings)`.
    result_claims: Additional `ResultClaim`s (SEP-2133) keyed by the identifier of
        an extension already advertised through `extensions`, merged with that
        extension's own claims. Rarely needed directly; prefer declaring claims on
        the extension itself. Claimed shapes are modern-only and inert on a legacy
        connection.

Examples:
    ```python
    # Connect to FastMCP server
    client = Client("http://localhost:8080")

    async with client:
        # List available resources
        resources = await client.list_resources()

        # Call a tool
        result = await client.call_tool("my_tool", {"param": "value"})
    ```


## ClientSessionState

`fastmcp.client.client.ClientSessionState`

```python
class ClientSessionState
```

**Declared members (7)**

- `initialize_result: mcp_types.InitializeResult | None = None`  _class-attribute, instance-attribute_
- `lock: anyio.Lock = field(default_factory=anyio.Lock)`  _class-attribute, instance-attribute_
- `nesting_counter: int = 0`  _class-attribute, instance-attribute_
- `ready_event: anyio.Event = field(default_factory=anyio.Event)`  _class-attribute, instance-attribute_
- `session: ClientSession | None = None`  _class-attribute, instance-attribute_
- `session_task: asyncio.Task | None = None`  _class-attribute, instance-attribute_
- `stop_event: anyio.Event = field(default_factory=anyio.Event)`  _class-attribute, instance-attribute_

Holds all session-related state for a Client instance.

This allows clean separation of configuration (which is copied) from
session state (which should be fresh for each new client instance).


## _FoldedExtensions

`fastmcp.client.client._FoldedExtensions`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _FoldedExtensions
```

**Declared members (4)**

- `ad: dict[str, dict[str, Any]]`  _instance-attribute_
- `bindings: list[NotificationBinding[Any]]`  _instance-attribute_
- `by_model: dict[type[mcp_types.Result], ResultClaim[Any]]`  _instance-attribute_
- `claims: dict[str, tuple[ResultClaim[Any], ...]]`  _instance-attribute_

`Client(extensions=...)` folded into the shapes `ClientSession` consumes.

`ad` maps each extension identifier to its advertised settings (the SEP-2133
capability ad), `claims` maps each identifier to its `ResultClaim`s, `bindings`
is the flat list of `NotificationBinding`s the extensions observe, and `by_model`
indexes every claim by its result model so a claimed `tools/call` result can be
routed back to the owning resolver.


## _conformant_discover_only

`fastmcp.client.client._conformant_discover_only`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _conformant_discover_only(session: ClientSession) -> AsyncIterator[None]
```

Hold ``session.send_discover`` to the same wire schema every later reply must meet.

``negotiate_auto`` accepts a probe that parses as the version-free
``DiscoverResult``, whose ``resultType``/``ttlMs``/``cacheScope`` all carry
SDK-side defaults. Every request *after* adoption is instead checked against
the strict per-version surface (``validate_server_result``), where those same
three fields are required. A server that answers ``server/discover`` without
them therefore passes the probe and then fails every subsequent call — the
connection is adopted into an era the peer cannot actually serve.

Closing that gap means judging the probe by the rule that will govern the rest
of the connection. A result that would be rejected later is not positive
evidence of a modern peer, so it is reported as an ordinary probe failure and
``negotiate_auto`` falls back to the initialize handshake, exactly as it does
for a server with no ``server/discover`` at all.


## _connection_failure

`fastmcp.client.client._connection_failure`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _connection_failure(exception: BaseException) -> BaseException
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Present a dead session the same way wherever it is noticed.

A failed session surfaces from two places: `_connect`, when the connection
never comes up, and `_await_with_session_monitoring`, when the session task
dies while a request is in flight. Which one wins is a matter of timing, so
both report the failure identically — otherwise the same dead backend
reaches callers as either a `RuntimeError` naming the connection or the raw
transport error, depending on the race. Types callers reasonably branch on
are passed through untouched.


