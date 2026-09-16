# `fastmcp.server.providers.local_provider.decorators.tools`

Distribution: `fastmcp`

## DuplicateBehavior

`fastmcp.server.providers.local_provider.decorators.tools.DuplicateBehavior`

```python
DuplicateBehavior = Literal['error', 'warn', 'replace', 'ignore']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["error", "warn", "replace", "ignore"]'> ````

## F

`fastmcp.server.providers.local_provider.decorators.tools.F`

```python
F = TypeVar('F', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## PREFAB_RENDERER_URI

`fastmcp.server.providers.local_provider.decorators.tools.PREFAB_RENDERER_URI`

```python
PREFAB_RENDERER_URI = 'ui://prefab/renderer.html'
```

**Inferred type** (`ty`, not declared in the source): `Literal["ui://prefab/renderer.html"]`

## ToolDecoratorMixin

Import as `fastmcp.server.providers.local_provider.decorators.ToolDecoratorMixin`  ·  defined at `fastmcp.server.providers.local_provider.decorators.tools.ToolDecoratorMixin`

```python
class ToolDecoratorMixin
```

**Also exported as** `fastmcp.server.providers.local_provider.decorators.ToolDecoratorMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `def add_tool(self: LocalProvider, tool: Tool | Callable[..., Any]) -> Tool`
  Add a tool to this provider's storage.
- `def tool(self: LocalProvider, name_or_fn: str | AnyFunction | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, tags: set[str] | None = None, output_schema: dict[str, Any] | NotSetT | None = NotSet, annotations: ToolAnnotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, enabled: bool = True, task: bool | TaskConfig | None = None, timeout: float | None = None, auth: AuthCheck | list[AuthCheck] | None = None, run_in_thread: bool = True) -> Callable[[AnyFunction], FunctionTool] | FunctionTool | partial[Callable[[AnyFunction], FunctionTool] | FunctionTool]`
  Decorator to register a tool.

Mixin class providing tool decorator functionality for LocalProvider.

This mixin contains all methods related to:
- Tool registration via add_tool()
- Tool decorator (@provider.tool)


## _has_prefab_return_type

`fastmcp.server.providers.local_provider.decorators.tools._has_prefab_return_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_prefab_return_type(tool: Tool) -> bool
```

Check if a FunctionTool's return type annotation is a prefab type.


## _is_prefab_type

`fastmcp.server.providers.local_provider.decorators.tools._is_prefab_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_prefab_type(tp: Any) -> bool
```

Check if *tp* is or contains a prefab type, recursing through unions and Annotated.


## _maybe_apply_prefab_ui

`fastmcp.server.providers.local_provider.decorators.tools._maybe_apply_prefab_ui`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _maybe_apply_prefab_ui(provider: LocalProvider, tool: Tool) -> None
```

Mark a tool as a Prefab tool if its config or return type implies it.

Per-tool renderer resources are synthesized lazily at list/read time;
here we only normalize the tool's meta so the synthesis pass can spot
it. ``app=True``, return-type inference, and ``PrefabAppConfig`` all
funnel through the same placeholder marker.


## _stamp_prefab_marker

`fastmcp.server.providers.local_provider.decorators.tools._stamp_prefab_marker`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _stamp_prefab_marker(tool: Tool) -> None
```

Mark a tool as needing a Prefab renderer resource.

Sets ``meta["ui"]["resourceUri"]`` to a placeholder URI. The server
recognizes the placeholder at list_tools / list_resources / read_resource
time and synthesizes a per-tool resource on the fly with a hashed URI
derived from the tool's mount-point address. Nothing is stored — the
renderer HTML and CSP are generated on demand from the tool's own meta.


