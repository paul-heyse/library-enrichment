# `mcp.server.extension`

Distribution: `mcp`

## RequestHandler

Import as `mcp.server.mcpserver.server.RequestHandler`  ·  defined at `mcp.server.extension.RequestHandler`

```python
RequestHandler = Callable[[ServerRequestContext[Any, Any], Any], Awaitable[HandlerResult]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( ServerRequestContext[Any, Any], Any, / ) -> Awaitable[BaseModel | dict[str, Any] | None]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## Extension

Import as `mcp.server.mcpserver.Extension`  ·  defined at `mcp.server.extension.Extension`

```python
class Extension
```

**Also exported as** `mcp.server.mcpserver.Extension`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `identifier: str`  _instance-attribute_
- `async def intercept_tool_call(self, params: CallToolRequestParams, ctx: ServerRequestContext[Any, Any], call_next: CallNext) -> HandlerResult`  _async_
  Wrap `tools/call`. Default: pass through unchanged.
- `def methods(self) -> Sequence[MethodBinding]`
  New request methods this extension serves (additive).
- `def resources(self) -> Sequence[ResourceBinding]`
  Resources this extension contributes (additive).
- `def settings(self) -> dict[str, Any]`
  Per-extension settings advertised at `capabilities.extensions[identifier]`.
- `def tools(self) -> Sequence[ToolBinding]`
  Tools this extension contributes (additive).

Base class for an opt-in MCP extension. Override only the methods you need.

Subclass and set `identifier`, then override the contribution methods that
apply. Every method has a default, so a minimal extension overrides nothing
but `identifier` and one of `tools`/`resources`/`methods`. `identifier` is
enforced at subclass-definition time.


## MethodBinding

Import as `mcp.server.mcpserver.MethodBinding`  ·  defined at `mcp.server.extension.MethodBinding`

```python
class MethodBinding
```

**Also exported as** `mcp.server.mcpserver.MethodBinding`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `handler: RequestHandler`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params_type: type[BaseModel]`  _instance-attribute_
- `protocol_versions: frozenset[str] | None = None`  _class-attribute, instance-attribute_

A new request method an extension serves, e.g. `tasks/get`.

`params_type` validates incoming params before `handler` runs; it should
subclass `RequestParams` so `_meta` parses uniformly. `protocol_versions`,
when set, restricts the method to those wire versions - a request for the
method at any other version is rejected as `METHOD_NOT_FOUND`, mirroring the
spec's `(method, version)` boundary table. `None` (the default) admits the
method at every version.

Extension methods are additive: `method` must not name a spec-defined
request method (`tools/list`, `completion/complete`, ...) — those handlers
belong to the server, and an extension binding one would silently shadow or
be shadowed by it. Both constraints are enforced at construction. To
re-provide a spec method the 2026 revision removed (e.g. `logging/setLevel`
for legacy clients), use the lowlevel `Server.add_request_handler` API
instead — the runner's per-version surface gate would never route such a
method to an extension handler anyway.


## ResourceBinding

Import as `mcp.server.mcpserver.ResourceBinding`  ·  defined at `mcp.server.extension.ResourceBinding`

```python
class ResourceBinding
```

**Also exported as** `mcp.server.mcpserver.ResourceBinding`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `resource: Resource`  _instance-attribute_

A pre-built resource an extension contributes.


## ToolBinding

Import as `mcp.server.mcpserver.ToolBinding`  ·  defined at `mcp.server.extension.ToolBinding`

```python
class ToolBinding
```

**Also exported as** `mcp.server.mcpserver.ToolBinding`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `fn: Callable[..., Any]`  _instance-attribute_
- `kwargs: dict[str, Any] = field(default_factory=lambda: {})`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

A tool an extension contributes, plus the `_meta` to stamp on it.


## _bind_interceptor

`mcp.server.extension._bind_interceptor`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _bind_interceptor(extension: Extension, params: CallToolRequestParams, call_next: CallNext) -> CallNext
```

## compose_tool_call_handler

Import as `mcp.server.mcpserver.server.compose_tool_call_handler`  ·  defined at `mcp.server.extension.compose_tool_call_handler`

```python
def compose_tool_call_handler(extensions: Sequence[Extension], handler: RequestHandler) -> RequestHandler
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Fold every extension's `intercept_tool_call` around the `tools/call` handler.

The returned handler nests the interceptors (first extension outermost) and
replaces the plain `tools/call` registration. Interception happens at the
handler layer, below the runner's outbound envelope pass, so a
short-circuiting interceptor's result is sieved and stamped exactly like
the wrapped handler's would be.


