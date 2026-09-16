# `fastmcp.server.middleware.middleware`

Distribution: `fastmcp`

## MiddlewarePhase

Import as `fastmcp.server.server.MiddlewarePhase`  ·  defined at `fastmcp.server.middleware.middleware.MiddlewarePhase`

```python
MiddlewarePhase = Literal['all', 'outer', 'typed']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["all", "outer", "typed"]'> ``` --- Which slice of a middleware's hooks to run in a single dispatch pass. - ``"all"`` runs the whole chain in one pass (``on_message`` -&gt; ``on_request`` / &nbsp;&nbsp;``on_notification`` -&gt; the typed per-method hook). This is what the interior &nbsp;&nbsp;component methods (``call_tool``, ``list_tools``, ...) run for the methods they &nbsp;&nbsp;serve, and what the ``initialize`` request runs at the dispatch root. - ``"outer"`` runs only ``on_message`` and ``on_request``/``on_notification``. &nbsp;&nbsp;The root dispatch (in the SDK's middleware layer) runs this pass for the messages the interior never &nbsp;&nbsp;dispatches (notifications, cancellations, unroutable/non-component requests, &nbsp;&nbsp;and pre-handler failures), so ``on_message`` observes *every* inbound message &nbsp;&nbsp;without double-firing for the component methods the interior already covers. - ``"typed"`` runs only the per-method hook. Reserved for a future full split; &nbsp;&nbsp;no current dispatch path uses it.`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Which slice of a middleware's hooks to run in a single dispatch pass.

- ``"all"`` runs the whole chain in one pass (``on_message`` -> ``on_request`` /
  ``on_notification`` -> the typed per-method hook). This is what the interior
  component methods (``call_tool``, ``list_tools``, ...) run for the methods they
  serve, and what the ``initialize`` request runs at the dispatch root.
- ``"outer"`` runs only ``on_message`` and ``on_request``/``on_notification``.
  The root dispatch (in the SDK's middleware layer) runs this pass for the messages the interior never
  dispatches (notifications, cancellations, unroutable/non-component requests,
  and pre-handler failures), so ``on_message`` observes *every* inbound message
  without double-firing for the component methods the interior already covers.
- ``"typed"`` runs only the per-method hook. Reserved for a future full split;
  no current dispatch path uses it.


## R

`fastmcp.server.middleware.middleware.R`

```python
R = TypeVar('R', covariant=True, default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## T

`fastmcp.server.middleware.middleware.T`

```python
T = TypeVar('T', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`fastmcp.server.middleware.middleware.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['CallNext', 'Middleware', 'MiddlewareContext']
```

## _dispatch_phase

`fastmcp.server.middleware.middleware._dispatch_phase`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_dispatch_phase: ContextVar[MiddlewarePhase] = ContextVar('fastmcp_dispatch_phase', default='all')
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The dispatch phase for the middleware chain currently running.

Set by ``FastMCP._run_middleware`` around each chain execution and read by
``Middleware.__call__``, so the phase never appears in the middleware call
signature — user middleware overriding the documented
``__call__(context, call_next)`` keeps working unchanged.


## _interior_dispatched

`fastmcp.server.middleware.middleware._interior_dispatched`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_interior_dispatched: ContextVar[bool] = ContextVar('fastmcp_interior_dispatched', default=False)
```

Set to True by an interior component dispatch when it runs its middleware chain.

The root dispatch reads this to tell whether the FastMCP middleware
chain already fired *inside* the wire request (so ``on_message``/``on_request``
were observed there — including any tool exception, exactly where the built-in
error/logging/timing middleware expect them). It is only consulted for the
component methods: if such a request fails *before* the interior runs (malformed
params, routing), the flag stays False and the root dispatch observes the failure itself.


## logger

`fastmcp.server.middleware.middleware.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## CallNext

Import as `fastmcp.server.middleware.CallNext`  ·  defined at `fastmcp.server.middleware.middleware.CallNext`

```python
class CallNext(Protocol[T, R])
```

**Also exported as** `fastmcp.server.middleware.CallNext`

_13 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol[T, R]`

## Middleware

Import as `fastmcp.server.middleware.Middleware`  ·  defined at `fastmcp.server.middleware.middleware.Middleware`

```python
class Middleware
```

**Also exported as** `fastmcp.server.middleware.Middleware`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (12)**

- `async def on_call_tool(self, context: MiddlewareContext[mt.CallToolRequestParams], call_next: CallNext[mt.CallToolRequestParams, ToolResult]) -> ToolResult`  _async_
- `async def on_discover(self, context: MiddlewareContext[mt.DiscoverRequest], call_next: CallNext[mt.DiscoverRequest, mt.DiscoverResult | dict[str, Any]]) -> mt.DiscoverResult | dict[str, Any]`  _async_
- `async def on_get_prompt(self, context: MiddlewareContext[mt.GetPromptRequestParams], call_next: CallNext[mt.GetPromptRequestParams, PromptResult]) -> PromptResult`  _async_
- `async def on_initialize(self, context: MiddlewareContext[mt.InitializeRequest], call_next: CallNext[mt.InitializeRequest, mt.InitializeResult | None]) -> mt.InitializeResult | None`  _async_
- `async def on_list_prompts(self, context: MiddlewareContext[mt.ListPromptsRequest], call_next: CallNext[mt.ListPromptsRequest, Sequence[Prompt]]) -> Sequence[Prompt]`  _async_
- `async def on_list_resource_templates(self, context: MiddlewareContext[mt.ListResourceTemplatesRequest], call_next: CallNext[mt.ListResourceTemplatesRequest, Sequence[ResourceTemplate]]) -> Sequence[ResourceTemplate]`  _async_
- `async def on_list_resources(self, context: MiddlewareContext[mt.ListResourcesRequest], call_next: CallNext[mt.ListResourcesRequest, Sequence[Resource]]) -> Sequence[Resource]`  _async_
- `async def on_list_tools(self, context: MiddlewareContext[mt.ListToolsRequest], call_next: CallNext[mt.ListToolsRequest, Sequence[Tool]]) -> Sequence[Tool]`  _async_
- `async def on_message(self, context: MiddlewareContext[Any], call_next: CallNext[Any, Any]) -> Any`  _async_
- `async def on_notification(self, context: MiddlewareContext[mt.Notification[Any, Any]], call_next: CallNext[mt.Notification[Any, Any], Any]) -> Any`  _async_
- `async def on_read_resource(self, context: MiddlewareContext[mt.ReadResourceRequestParams], call_next: CallNext[mt.ReadResourceRequestParams, ResourceResult]) -> ResourceResult`  _async_
- `async def on_request(self, context: MiddlewareContext[mt.Request[Any, Any]], call_next: CallNext[mt.Request[Any, Any], Any]) -> Any`  _async_

Base class for FastMCP middleware with dispatching hooks.


## MiddlewareContext

Import as `fastmcp.server.middleware.MiddlewareContext`  ·  defined at `fastmcp.server.middleware.middleware.MiddlewareContext`

```python
class MiddlewareContext(Generic[T])
```

**Also exported as** `fastmcp.server.middleware.MiddlewareContext`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[T]`

**Declared members (6)**

- `fastmcp_context: Context | None = None`  _class-attribute, instance-attribute_
- `message: T`  _instance-attribute_
- `method: str | None = None`  _class-attribute, instance-attribute_
- `source: Literal['client', 'server'] = 'client'`  _class-attribute, instance-attribute_
- `timestamp: datetime = field(default_factory=lambda: datetime.now(timezone.utc))`  _class-attribute, instance-attribute_
- `type: Literal['request', 'notification'] = 'request'`  _class-attribute, instance-attribute_

Unified context for all middleware operations.


## make_handler_wrapper

`fastmcp.server.middleware.middleware.make_handler_wrapper`

```python
def make_handler_wrapper(handler: Callable[..., Awaitable[Any]], call_next: CallNext[Any, Any]) -> CallNext[Any, Any]
```

## make_middleware_wrapper

`fastmcp.server.middleware.middleware.make_middleware_wrapper`

```python
def make_middleware_wrapper(middleware: Middleware, call_next: CallNext[T, R]) -> CallNext[T, R]
```

Create a wrapper that applies a single middleware to a context. The
closure bakes in the middleware and call_next function, so it can be
passed to other functions that expect a call_next function.


## mark_interior_dispatched

Import as `fastmcp.server.server.mark_interior_dispatched`  ·  defined at `fastmcp.server.middleware.middleware.mark_interior_dispatched`

```python
def mark_interior_dispatched() -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Record that an interior component middleware chain ran for this message.


