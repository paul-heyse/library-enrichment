# `fastmcp.server.low_level`

Distribution: `fastmcp`

## _INTERIOR_METHODS

`fastmcp.server.low_level._INTERIOR_METHODS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_INTERIOR_METHODS = frozenset({'tools/call', 'tools/list', 'resources/read', 'resources/list', 'resources/templates/list', 'prompts/get', 'prompts/list'})
```

## logger

`fastmcp.server.low_level.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FastMCPServerMiddleware

`fastmcp.server.low_level.FastMCPServerMiddleware`

```python
class FastMCPServerMiddleware
```

Root dispatch for the FastMCP middleware chain, in the SDK's middleware layer.

v2 no longer lets FastMCP subclass ``ServerSession`` (the runner constructs
it per request), so the old ``MiddlewareServerSession._received_request``
override is replaced by a ``ServerMiddleware`` — an ordinary entry in the
SDK's own middleware list. Sitting at the root of dispatch, this
is the single entry point through which *every* inbound message flows —
requests, notifications, cancellations, ``initialize``, and even malformed or
unroutable messages the SDK can still hand us. It binds the FastMCP
request-context ContextVar and re-applies the app-scoped ``SharedContext`` for
the whole chain, then runs the FastMCP ``Middleware`` chain so
``on_message`` / ``on_request`` / ``on_notification`` observe the message.

Dispatch shapes:

- Negotiation runs the *whole* FastMCP chain here: ``initialize`` dispatches
  through ``on_initialize`` and ``server/discover`` through ``on_discover``.
  Neither has an interior FastMCP handler adapter, and the SDK serializes both
  results before returning through its middleware seam, so this root adapter
  restores core results to typed models before FastMCP middleware observes them.
- The component methods (``tools/call``, ``tools/list``, ``resources/read``,
  ...) still run their FastMCP chain *interior*, in the handler adapter, where
  ``on_call_tool`` receives the typed component result and a tool exception
  propagates through ``on_message``/``on_request`` exactly where the built-in
  error/logging/timing middleware expect it. The root dispatch does not re-run the
  chain for these — it only steps in when such a request fails *before* the
  interior runs (malformed params, routing), so ``on_message`` still observes
  the failure.
- Every other message — all notifications (including ``notifications/cancelled``
  and ``notifications/initialized``), ``ping``, ``logging/setLevel``, and any
  unroutable/non-component request — has no interior FastMCP dispatch, so the
  root dispatch runs the ``"outer"`` pass (``on_message`` plus
  ``on_request``/``on_notification``) here, wrapping the real SDK dispatch.
  This closes the long-standing gap where these messages were invisible to
  FastMCP middleware.


## LowLevelServer

Import as `fastmcp.server.server.LowLevelServer`  ·  defined at `fastmcp.server.low_level.LowLevelServer`

```python
class LowLevelServer(_Server[LifespanResultT])
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_Server[LifespanResultT]`

**Declared members (5)**

- `def create_initialization_options(self, notification_options: NotificationOptions | None = None, experimental_capabilities: dict[str, dict[str, Any]] | None = None, extensions: dict[str, dict[str, Any]] | None = None) -> InitializationOptions`
- `fastmcp: FastMCP`  _property_
  Get the FastMCP instance.
- `def get_capabilities(self, notification_options: NotificationOptions | None = None, experimental_capabilities: dict[str, dict[str, Any]] | None = None, extensions: dict[str, dict[str, Any]] | None = None, protocol_version: str | None = None) -> mcp_types.ServerCapabilities`
  Override to advertise registered extensions and the MCP Apps UI extension.
- `middleware = [mw for mw in self.middleware if not isinstance(mw, OpenTelemetryMiddleware)]`  _instance-attribute_
- `notification_options = NotificationOptions(prompts_changed=True, resources_changed=True, tools_changed=True)`  _instance-attribute_

## _forward_ctx

`fastmcp.server.low_level._forward_ctx`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _forward_ctx(ctx: ServerRequestContext, mw_ctx: Any, original: Any) -> ServerRequestContext
```

Fold middleware edits to the *message* back into the SDK context.

The outer pass hands middleware a *copy* of the raw params (see
``_raw_message``), so a hook that follows the documented inspect/modify
contract — mutating ``context.message`` or passing ``context.copy(message=...)``
to ``call_next`` — would otherwise have its edits silently dropped when the
bridge dispatched the original context. Rewriting through
``dataclasses.replace`` is how the SDK documents altering what the handler
sees. An untouched message forwards the original context unchanged.

``ctx.method`` is deliberately *not* rewritable here. Dispatch has already
branched on the method to decide that this message has no interior handler,
so redirecting it now — say, turning a ``ping`` into a ``tools/list`` — would
hand it to a component handler that runs the FastMCP chain a second time,
firing ``on_message`` and raw ``__call__`` overrides twice for one message
and duplicating whatever side effects (rate limiting, authorization,
logging) they carry. Rewriting the method is not part of the documented
middleware contract; only the message is.


## _raw_message

`fastmcp.server.low_level._raw_message`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _raw_message(ctx: ServerRequestContext) -> Any
```

The message payload handed to the root dispatch's ``on_message``/``on_request`` pass.

The raw inbound params mapping is used verbatim rather than a validated,
typed request model. This is deliberate: the outer pass must observe *every*
message, including malformed or unroutable ones, and reconstructing a typed
model would raise on exactly those messages and hide them from the hooks.
The method and request/notification kind are carried on the
``MiddlewareContext`` itself, so observation middleware still has everything
it needs.


## client_supports_extension

Import as `fastmcp.server.context.client_supports_extension`  ·  defined at `fastmcp.server.low_level.client_supports_extension`

```python
def client_supports_extension(session: ServerSession, extension_id: str) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check whether the connected client supports a given MCP extension.

Inspects the ``extensions`` capability on ``ClientCapabilities`` sent by the
client during initialization. In v2 the client's initialize params are
reachable via ``session.client_params``.

SDK v2 declares ``extensions`` as a real field on ``ClientCapabilities``, so
a client sending ``ClientCapabilities(extensions={...})`` populates the field
directly. We read that field first and fall back to ``model_extra`` only for
legacy-serialized clients that carried ``extensions`` as an extra key.


