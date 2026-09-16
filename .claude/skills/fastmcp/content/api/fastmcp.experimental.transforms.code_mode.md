# `fastmcp.experimental.transforms.code_mode`

Distribution: `fastmcp`

## DiscoveryToolFactory

`fastmcp.experimental.transforms.code_mode.DiscoveryToolFactory`

```python
DiscoveryToolFactory = Callable[[GetToolCatalog], Tool]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '((Context, /) -> Awaitable[Sequence[Tool]], /) -> Tool'> ``` --- Factory that receives catalog access and returns a synthetic Tool.`

Factory that receives catalog access and returns a synthetic Tool.


## GetToolCatalog

`fastmcp.experimental.transforms.code_mode.GetToolCatalog`

```python
GetToolCatalog = Callable[[Context], Awaitable[Sequence[Tool]]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(Context, /) -> Awaitable[Sequence[Tool]]'> ``` --- Async callable that returns the auth-filtered tool catalog.`

Async callable that returns the auth-filtered tool catalog.


## SearchFn

`fastmcp.experimental.transforms.code_mode.SearchFn`

```python
SearchFn = Callable[[Sequence[Tool], str], Awaitable[Sequence[Tool]]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( Sequence[Tool], str, / ) -> Awaitable[Sequence[Tool]]'> ``` --- Async callable that searches a tool sequence by query string.`

Async callable that searches a tool sequence by query string.


## ToolDetailLevel

`fastmcp.experimental.transforms.code_mode.ToolDetailLevel`

```python
ToolDetailLevel = Literal['brief', 'detailed', 'full']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["brief", "detailed", "full"]'> ``` --- Detail level for discovery tool output. - ``"brief"``: tool names and one-line descriptions - ``"detailed"``: compact markdown with parameter names, types, and required markers - ``"full"``: complete JSON schema`

Detail level for discovery tool output.

- ``"brief"``: tool names and one-line descriptions
- ``"detailed"``: compact markdown with parameter names, types, and required markers
- ``"full"``: complete JSON schema


## _DEFAULT_LIMITS

`fastmcp.experimental.transforms.code_mode._DEFAULT_LIMITS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_LIMITS: ResourceLimits = {'max_duration_secs': 30.0, 'max_memory': 100000000}
```

Baseline limits applied when ``MontySandboxProvider`` is constructed
without an explicit ``limits`` argument. Pass ``limits=None`` to opt out
entirely, or a dict to override.


## _UNSET

`fastmcp.experimental.transforms.code_mode._UNSET`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_UNSET = _UnsetType()
```

## __all__

`fastmcp.experimental.transforms.code_mode.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['CodeMode', 'GetSchemas', 'GetTags', 'GetToolCatalog', 'ListTools', 'MontySandboxProvider', 'SandboxProvider', 'Search']
```

## CodeMode

`fastmcp.experimental.transforms.code_mode.CodeMode`

```python
class CodeMode(CatalogTransform)
```

**Bases** `CatalogTransform`

**Declared members (6)**

- `execute_description = execute_description`  _instance-attribute_
- `execute_tool_name = execute_tool_name`  _instance-attribute_
- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
- `max_tool_calls = max_tool_calls`  _instance-attribute_
- `sandbox_provider = sandbox_provider or MontySandboxProvider()`  _instance-attribute_
- `async def transform_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_

**Inherited (14)**

- from `fastmcp.server.transforms.Transform`: `get_prompt`, `get_resource`, `get_resource_template`
- from `fastmcp.server.transforms.catalog.CatalogTransform`: `get_prompt_catalog`, `get_resource_catalog`, `get_resource_template_catalog`, `get_tool_catalog`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transform_prompts`, `transform_resource_templates`, `transform_resources`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transform that collapses all tools into discovery + execute meta-tools.

Discovery tools are composable via the ``discovery_tools`` parameter.
Each is a callable that receives catalog access and returns a ``Tool``.
By default, ``Search`` and ``GetSchemas`` are included for
progressive disclosure: search finds candidates, get_schema retrieves
parameter details, and execute runs code.

The ``execute`` tool is always present and provides a sandboxed Python
environment with ``call_tool(name, params)`` in scope.


## GetSchemas

`fastmcp.experimental.transforms.code_mode.GetSchemas`

```python
class GetSchemas
```

Discovery tool factory that returns schemas for tools by name.

Args:
    name: Name of the synthetic tool exposed to the LLM.
    default_detail: Default detail level for schema results.
        ``"brief"`` returns tool names and descriptions only.
        ``"detailed"`` renders compact markdown with parameter names,
        types, and required markers.
        ``"full"`` returns the complete JSON schema.


## GetTags

`fastmcp.experimental.transforms.code_mode.GetTags`

```python
class GetTags
```

Discovery tool factory that lists tool tags from the catalog.

Reads ``tool.tags`` from the catalog and groups tools by tag. Tools
without tags appear under ``"untagged"``.

Args:
    name: Name of the synthetic tool exposed to the LLM.
    default_detail: Default detail level.
        ``"brief"`` returns tag names with tool counts.
        ``"full"`` lists all tools under each tag.


## ListTools

`fastmcp.experimental.transforms.code_mode.ListTools`

```python
class ListTools
```

Discovery tool factory that lists all tools in the catalog.

Args:
    name: Name of the synthetic tool exposed to the LLM.
    default_detail: Default detail level.
        ``"brief"`` returns tool names and one-line descriptions.
        ``"detailed"`` returns compact markdown with parameter schemas.
        ``"full"`` returns the complete JSON schema.


## MontySandboxProvider

`fastmcp.experimental.transforms.code_mode.MontySandboxProvider`

```python
class MontySandboxProvider
```

**Declared members (2)**

- `limits: ResourceLimits | None = _DEFAULT_LIMITS.copy() if isinstance(limits, _UnsetType) else limits`  _instance-attribute_
- `async def run(self, code: str, inputs: dict[str, Any] | None = None, external_functions: dict[str, Callable[..., Any]] | None = None) -> Any`  _async_

Sandbox provider backed by `pydantic-monty`.

Args:
    limits: Resource limits for sandbox execution. Supported keys:
        ``max_duration_secs`` (float), ``max_allocations`` (int),
        ``max_memory`` (int), ``max_recursion_depth`` (int),
        ``gc_interval`` (int).  All are optional; omit a key to
        leave that limit uncapped.

        When the argument is omitted entirely, a conservative baseline
        is applied (``max_duration_secs=30``, ``max_memory=100 MB``) so
        the out-of-box configuration is not unbounded. Pass
        ``limits=None`` to explicitly run without any limits, or a dict
        to set your own.


## SandboxProvider

`fastmcp.experimental.transforms.code_mode.SandboxProvider`

```python
class SandboxProvider(Protocol)
```

**Bases** `Protocol`

**Declared members (1)**

- `async def run(self, code: str, inputs: dict[str, Any] | None = None, external_functions: dict[str, Callable[..., Any]] | None = None) -> Any`  _async_

Interface for executing LLM-generated Python code in a sandbox.

WARNING: The ``code`` parameter passed to ``run`` contains untrusted,
LLM-generated Python.  Implementations MUST execute it in an isolated
sandbox — never with plain ``exec()``.  Use ``MontySandboxProvider``
(backed by ``pydantic-monty``) for production workloads.


## Search

`fastmcp.experimental.transforms.code_mode.Search`

```python
class Search
```

Discovery tool factory that searches the catalog by query.

Args:
    search_fn: Async callable ``(tools, query) -> matching_tools``.
        Defaults to BM25 ranking.
    name: Name of the synthetic tool exposed to the LLM.
    default_detail: Default detail level for search results.
        ``"brief"`` returns tool names and descriptions only.
        ``"detailed"`` returns compact markdown with parameter schemas.
        ``"full"`` returns complete JSON tool definitions.
    default_limit: Maximum number of results to return.
        The LLM can override this per call.  ``None`` means no limit.


## _UnsetType

`fastmcp.experimental.transforms.code_mode._UnsetType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _UnsetType
```

Sentinel distinguishing "argument omitted" from an explicit value.


## _default_discovery_tools

`fastmcp.experimental.transforms.code_mode._default_discovery_tools`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _default_discovery_tools() -> list[DiscoveryToolFactory]
```

## _ensure_async

`fastmcp.experimental.transforms.code_mode._ensure_async`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _ensure_async(fn: Callable[..., Any]) -> Callable[..., Any]
```

## _render_tools

`fastmcp.experimental.transforms.code_mode._render_tools`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _render_tools(tools: Sequence[Tool], detail: ToolDetailLevel) -> str
```

Render tools at the requested detail level.

The same detail value produces the same output format regardless of
which discovery tool calls this, so ``detail="detailed"`` on Search
gives identical formatting to ``detail="detailed"`` on GetSchemas.


## _unwrap_tool_result

`fastmcp.experimental.transforms.code_mode._unwrap_tool_result`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _unwrap_tool_result(result: ToolResult) -> dict[str, Any] | str
```

Convert a ToolResult for use in the sandbox.

- Output schema present → structured_content dict (matches the schema)
- Otherwise → concatenated text content as a string


