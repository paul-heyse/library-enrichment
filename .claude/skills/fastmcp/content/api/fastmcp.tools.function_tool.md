# `fastmcp.tools.function_tool`

Distribution: `fastmcp`

## F

`fastmcp.tools.function_tool.F`

```python
F = TypeVar('F', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## logger

`fastmcp.tools.function_tool.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## DecoratedTool

`fastmcp.tools.function_tool.DecoratedTool`

```python
class DecoratedTool(Protocol)
```

**Bases** `Protocol`

Protocol for functions decorated with @tool.


## FunctionTool

Import as `fastmcp.tools.FunctionTool`  ·  defined at `fastmcp.tools.function_tool.FunctionTool`

```python
class FunctionTool(Tool)
```

**Also exported as** `fastmcp.tools.FunctionTool`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Tool`

**Declared members (4)**

- `fn: SkipJsonSchema[Callable[..., Any]]`  _instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], metadata: ToolMeta | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, annotations: ToolAnnotations | None = None, output_schema: dict[str, Any] | NotSetT | None = NotSet, meta: dict[str, Any] | None = None, task: bool | TaskConfig | None = None, timeout: float | None = None, auth: AuthCheck | list[AuthCheck] | None = None, run_in_thread: bool | None = None) -> FunctionTool`  _classmethod_
  Create a FunctionTool from a function.
- `async def run(self, arguments: dict[str, Any]) -> ToolResult`  _async_
  Run the tool with arguments.
- `run_in_thread: Annotated[bool, Field(description="Applies to sync tool functions only. When True (default), sync functions are dispatched to a worker thread so they don't block the event loop. Set to False to run the sync function inline on the event loop thread — useful for libraries with thread affinity (e.g. Windows COM, tkinter). Ignored for async functions, which always run on the event loop. Cannot be combined with `timeout` on a sync function: inline calls have no cancellation checkpoints, so the timeout would be a silent no-op.")] = True`  _class-attribute, instance-attribute_

**Inherited (25)**

- from `fastmcp.tools.base.Tool`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `execution`, `from_tool`, `get_span_attributes`, `output_schema`, `parameters`, `return_type`, `timeout`, `to_mcp_tool`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## ToolMeta

Import as `fastmcp.decorators.ToolMeta`  ·  defined at `fastmcp.tools.function_tool.ToolMeta`

```python
class ToolMeta
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (16)**

- `annotations: ToolAnnotations | None = None`  _class-attribute, instance-attribute_
- `app: Any = None`  _class-attribute, instance-attribute_
- `auth: AuthCheck | list[AuthCheck] | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `enabled: bool = True`  _class-attribute, instance-attribute_
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `name: str | None = None`  _class-attribute, instance-attribute_
- `output_schema: dict[str, Any] | NotSetT | None = NotSet`  _class-attribute, instance-attribute_
- `run_in_thread: bool = True`  _class-attribute, instance-attribute_
- `tags: set[str] | None = None`  _class-attribute, instance-attribute_
- `task: bool | TaskConfig | None = None`  _class-attribute, instance-attribute_
- `timeout: float | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_
- `type: Literal['tool'] = field(default='tool', init=False)`  _class-attribute, instance-attribute_
- `version: str | int | None = None`  _class-attribute, instance-attribute_

Metadata attached to functions by the @tool decorator.


## _ToolBodyError

`fastmcp.tools.function_tool._ToolBodyError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ToolBodyError(Exception)
```

**Bases** `Exception`

Marks a ``pydantic.ValidationError`` raised while executing a tool's body.

Pydantic validates a tool's arguments *before* invoking the body, so a bare
``pydantic.ValidationError`` surfacing from the call adapter is unambiguously
an argument-validation failure (a bad call). Errors a tool raises from its
own body — e.g. constructing a model from upstream data — are a different
class of problem (a server-side bug) that must not be reclassified as a bad
call. We wrap the body so those are tagged and can be told apart. See #4128.


## _resolve_param_hints

`fastmcp.tools.function_tool._resolve_param_hints`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolve_param_hints(fn: Callable[..., Any]) -> dict[str, Any]
```

Resolve a callable's parameter type hints, tolerating partials.

Depending on the Python version, ``get_type_hints`` either rejects
``functools.partial`` objects or returns no hints for them. The synchronous
TypeAdapter path handles partials natively. Resolve their hints against the
underlying function and keep only parameters in the partially-bound
signature.


## _strict_input_validation

`fastmcp.tools.function_tool._strict_input_validation`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _strict_input_validation() -> bool
```

Whether the running server enforces strict argument validation.

Reads ``strict_input_validation`` off the active request's ``FastMCP``
instance. Returns ``False`` outside a request context (e.g. a tool invoked
directly in tests), preserving the default coercing behavior.


## _wrap_body_errors

`fastmcp.tools.function_tool._wrap_body_errors`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _wrap_body_errors(fn: Callable[..., Any]) -> Callable[..., Any]
```

Wrap ``fn`` so a ``pydantic.ValidationError`` raised by its body is
re-raised as ``_ToolBodyError``.

The wrapper preserves ``fn``'s signature and annotations so the cached
``TypeAdapter`` validates arguments identically — only body execution is
affected. Argument validation happens before the wrapper is called, so it
keeps raising a bare ``pydantic.ValidationError``.


## tool

Import as `fastmcp.tools.tool`  ·  defined at `fastmcp.tools.function_tool.tool`

```python
def tool(name_or_fn: str | Callable[..., Any] | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, output_schema: dict[str, Any] | NotSetT | None = NotSet, annotations: ToolAnnotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, task: bool | TaskConfig | None = None, timeout: float | None = None, auth: AuthCheck | list[AuthCheck] | None = None, run_in_thread: bool = True) -> Any
```

**Overloads** (the signature above is the runtime dispatcher):

- `def tool(fn: F) -> F`
- `def tool(name_or_fn: str, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, output_schema: dict[str, Any] | NotSetT | None = NotSet, annotations: ToolAnnotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, task: bool | TaskConfig | None = None, timeout: float | None = None, auth: AuthCheck | list[AuthCheck] | None = None, run_in_thread: bool = True) -> Callable[[F], F]`
- `def tool(name_or_fn: None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, output_schema: dict[str, Any] | NotSetT | None = NotSet, annotations: ToolAnnotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, task: bool | TaskConfig | None = None, timeout: float | None = None, auth: AuthCheck | list[AuthCheck] | None = None, run_in_thread: bool = True) -> Callable[[F], F]`

**Also exported as** `fastmcp.tools.tool`

Standalone decorator to mark a function as an MCP tool.

Returns the original function with metadata attached. Register with a server
using mcp.add_tool().

Args:
    run_in_thread: Applies to sync tool functions only. When True (default),
        the sync function is dispatched to a worker thread so it does not
        block the event loop. Set to False to run the function inline on the
        event loop thread — useful for libraries with thread affinity
        (e.g. Windows COM via `uiautomation`/`comtypes`/`pywin32`, `tkinter`,
        some GPU/driver bindings). Ignored for async functions. Cannot be
        combined with `timeout` on a sync function: inline calls have no
        cancellation checkpoints, so the timeout would be a silent no-op.


