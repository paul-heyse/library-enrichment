# `fastmcp.apps.app`

Distribution: `fastmcp`

## F

`fastmcp.apps.app.F`

```python
F = TypeVar('F', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## logger

`fastmcp.apps.app.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FastMCPApp

Import as `fastmcp.FastMCPApp`  ·  defined at `fastmcp.apps.app.FastMCPApp`

```python
class FastMCPApp(Provider)
```

**Also exported as** `fastmcp.FastMCPApp`, `fastmcp.apps.FastMCPApp`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Provider`

**Declared members (6)**

- `def add_tool(self, tool: Tool | Callable[..., Any]) -> Tool`
  Add a tool to this app programmatically.
- `async def lifespan(self) -> AsyncIterator[None]`  _async_
- `name = name`  _instance-attribute_
- `def run(self, transport: Literal['stdio', 'http', 'sse', 'streamable-http'] | None = None, kwargs: Any = {}) -> None`
  Create a temporary FastMCP server and run this app standalone.
- `def tool(self, name_or_fn: str | AnyFunction | None = None, name: str | None = None, description: str | None = None, model: bool = False, auth: AuthCheck | list[AuthCheck] | None = None, timeout: float | None = None) -> Any`
  Register a backend tool that the UI calls via CallTool.
- `def ui(self, name_or_fn: str | AnyFunction | None = None, name: str | None = None, description: str | None = None, title: str | None = None, tags: set[str] | None = None, icons: list[Icon] | None = None, annotations: ToolAnnotations | None = None, auth: AuthCheck | list[AuthCheck] | None = None, timeout: float | None = None) -> Any`
  Register a UI entry-point tool that the model calls.

**Inherited (16)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `get_tool_by_hash`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Provider that represents an MCP application.

Binds together entry-point tools (``@app.ui``), backend tools
(``@app.tool``), and the Prefab renderer resource.  Backend tools
are tagged with ``meta["fastmcp"]["app"]`` so ``Provider.get_tool``
can find them by original name even when transforms have been applied.


## _dispatch_decorator

`fastmcp.apps.app._dispatch_decorator`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _dispatch_decorator(name_or_fn: str | AnyFunction | None, name: str | None, register: Callable[[Any, str | None], Any], decorator_name: str) -> Any
```

Shared dispatch logic for @app.tool() and @app.ui() calling patterns.


## _make_resolver

`fastmcp.apps.app._make_resolver`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_resolver(app_name: str | None = None) -> Any
```

Create a CallTool resolver that addresses peer tools by identity.

``app_name`` is the FastMCPApp's name, known at serialization time from
the tool's ``meta["fastmcp"]["app"]`` tag. Serialization happens deep
inside whatever composition the server has, so nothing here can know
what these tools will be *called* by the time the payload reaches a
host. References therefore start out identity-addressed, as
``<hash>_<local_name>``.

Each FastMCP server rewrites those references on the way out to the
name it lists that tool under, so what a renderer finally receives is
an ordinary tool name (see ``server.providers.prefab_payload``). A
reference no server could resolve keeps this form, which the dispatcher
still routes via ``get_tool_by_hash``.


