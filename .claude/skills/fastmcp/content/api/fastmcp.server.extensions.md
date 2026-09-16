# `fastmcp.server.extensions`

Distribution: `fastmcp`

## ExtensionRequestHandler

`fastmcp.server.extensions.ExtensionRequestHandler`

```python
ExtensionRequestHandler: TypeAlias = Callable[[ServerRequestContext[Any, Any], Any], Awaitable[BaseModel | dict[str, Any] | None]]
```

## ToolCallContinuation

`fastmcp.server.extensions.ToolCallContinuation`

```python
ToolCallContinuation: TypeAlias = Callable[[], Awaitable['ToolCallOutcome']]
```

## ToolCallOutcome

`fastmcp.server.extensions.ToolCallOutcome`

```python
ToolCallOutcome: TypeAlias = 'ToolResult | BaseModel'
```

## __all__

`fastmcp.server.extensions.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['MethodBinding', 'ServerExtension', 'read_client_extension_settings']
```

## MethodBinding

`fastmcp.server.extensions.MethodBinding`

```python
class MethodBinding
```

**Declared members (4)**

- `handler: ExtensionRequestHandler`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params_type: type[BaseModel]`  _instance-attribute_
- `protocol_versions: frozenset[str] | None = None`  _class-attribute, instance-attribute_

A new request method an extension serves, e.g. `tasks/get`.

`params_type` validates incoming params before `handler` runs; it should
subclass `RequestParams` so `_meta` parses uniformly. `protocol_versions`,
when set, restricts the method to those wire versions — a request at any
other version is rejected as `METHOD_NOT_FOUND`, mirroring the spec's
`(method, version)` boundary. `None` (the default) admits every version.

Extension methods are additive: `method` must not name a spec-defined
request method (`tools/call`, `completion/complete`, ...). Binding one would
silently shadow the server's own handler. Both constraints are enforced at
construction.


## ServerExtension

Import as `fastmcp.server.server.ServerExtension`  ·  defined at `fastmcp.server.extensions.ServerExtension`

```python
class ServerExtension
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (7)**

- `def client_settings(self, ctx: ServerRequestContext[Any, Any]) -> dict[str, Any] | None`
  This extension's per-request opt-in settings declared by the client.
- `identifier: str`  _instance-attribute_
- `async def intercept_tool_call(self, params: CallToolRequestParams, context: Context, call_next: ToolCallContinuation) -> ToolCallOutcome`  _async_
  Wrap `tools/call`. Default: pass through unchanged.
- `def lifespan(self) -> AbstractAsyncContextManager[None]`
  A context manager entered with the server's lifespan, exited on shutdown.
- `def methods(self) -> Sequence[MethodBinding]`
  New request methods this extension serves (additive).
- `server: FastMCP`  _property_
  The FastMCP server this extension is registered on.
- `def settings(self) -> dict[str, Any]`
  Per-extension settings advertised at `capabilities.extensions[identifier]`.

Base class for an opt-in FastMCP server extension (SEP-2133).

Subclass, set `identifier`, and override the contribution methods that
apply. Every method has a default, so a minimal extension overrides only
`identifier` and one contribution. `identifier` is validated at
subclass-definition time when set as a class attribute, and again at
registration (which covers per-instance identifiers assigned in `__init__`).

Register an instance with `FastMCP.add_extension(...)`, which binds the
extension to the server so `self.server`, `intercept_tool_call`, and method
handlers can reach FastMCP-level constructs.


## _extract_client_extension_settings

`fastmcp.server.extensions._extract_client_extension_settings`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _extract_client_extension_settings(meta: Mapping[str, Any] | None, identifier: str) -> dict[str, Any] | None
```

Pull `_meta[clientCapabilities][extensions][identifier]` from a lifted meta block.


## build_method_handler

`fastmcp.server.extensions.build_method_handler`

```python
def build_method_handler(binding: MethodBinding) -> ExtensionRequestHandler
```

Wrap a `MethodBinding` into a low-level request handler.

The adapter enforces `protocol_versions` gating (rejecting other versions as
`METHOD_NOT_FOUND`, since `add_request_handler` registers unconditionally)
and binds the FastMCP request context so the handler can use `get_context()`,
auth, and other request-scoped dependencies.


## read_client_extension_settings

`fastmcp.server.extensions.read_client_extension_settings`

```python
def read_client_extension_settings(ctx: ServerRequestContext[Any, Any], identifier: str) -> dict[str, Any] | None
```

Read a client's per-request extension opt-in from the request `_meta`.

SEP-2133 extensions negotiate per request: the client repeats its extension
capabilities in each request's `_meta` under
`io.modelcontextprotocol/clientCapabilities` → `extensions` → `identifier`.
Returns the declared settings dict (possibly empty) when the extension was
opted in for this request, or `None` when it was not.


## wrap_tool_call_interceptor

`fastmcp.server.extensions.wrap_tool_call_interceptor`

```python
def wrap_tool_call_interceptor(extension: ServerExtension, call_next: Callable[[Any], Awaitable[Any]]) -> Callable[[Any], Awaitable[Any]]
```

Fold one extension's `intercept_tool_call` around a middleware `call_next`.

The returned wrapper is a FastMCP `CallNext`: it hands the extension the
validated `tools/call` params, the FastMCP `Context`, and a zero-arg
continuation that runs the rest of the chain and, finally, the tool body.


