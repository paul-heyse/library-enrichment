# `fastmcp.server.transforms.search.base`

Distribution: `fastmcp`

## SearchResultSerializer

Import as `fastmcp.server.transforms.search.SearchResultSerializer`  ·  defined at `fastmcp.server.transforms.search.base.SearchResultSerializer`

```python
SearchResultSerializer = Callable[[Sequence[Tool]], Any | Awaitable[Any]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(Sequence[Tool], /) -> Any | Awaitable[Any]'> ````

**Also exported as** `fastmcp.server.transforms.search.SearchResultSerializer`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## BaseSearchTransform

Import as `fastmcp.server.transforms.search.bm25.BaseSearchTransform`  ·  defined at `fastmcp.server.transforms.search.base.BaseSearchTransform`

```python
class BaseSearchTransform(CatalogTransform)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `CatalogTransform`

**Declared members (2)**

- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
  Intercept synthetic tool names; delegate everything else.
- `async def transform_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
  Replace the catalog with pinned + synthetic search/call tools.

**Inherited (14)**

- from `fastmcp.server.transforms.Transform`: `get_prompt`, `get_resource`, `get_resource_template`
- from `fastmcp.server.transforms.catalog.CatalogTransform`: `get_prompt_catalog`, `get_resource_catalog`, `get_resource_template_catalog`, `get_tool_catalog`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transform_prompts`, `transform_resource_templates`, `transform_resources`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Replace the tool listing with a search interface.

When this transform is active, ``list_tools()`` returns only:

* Any tools listed in ``always_visible`` (pinned).
* A **search tool** that finds tools matching a query.
* A **call_tool** proxy that executes tools discovered via search.

Hidden tools remain callable — ``get_tool()`` delegates unknown
names downstream, so direct calls and the call-tool proxy both work.

Search results respect the full auth pipeline: middleware, visibility
transforms, and component-level auth checks all apply.

Args:
    max_results: Maximum number of tools returned per search.
    always_visible: Tool names that stay in the ``list_tools``
        output alongside the synthetic search/call tools.
    search_tool_name: Name of the generated search tool.
    call_tool_name: Name of the generated call-tool proxy.


## _extract_searchable_text

`fastmcp.server.transforms.search.base._extract_searchable_text`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _extract_searchable_text(tool: Tool) -> str
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Combine tool name, description, and parameter info into searchable text.


## _invoke_serializer

`fastmcp.server.transforms.search.base._invoke_serializer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _invoke_serializer(serializer: SearchResultSerializer, tools: Sequence[Tool]) -> Any
```

Call a serializer and await the result if it returns a coroutine.


## _schema_section

`fastmcp.server.transforms.search.base._schema_section`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _schema_section(schema: dict[str, Any] | None, title: str) -> list[str]
```

## _schema_type

`fastmcp.server.transforms.search.base._schema_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _schema_type(schema: Any) -> str
```

## _union_type

`fastmcp.server.transforms.search.base._union_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _union_type(branches: list[Any]) -> str
```

## serialize_tools_for_output_json

Import as `fastmcp.server.transforms.search.serialize_tools_for_output_json`  ·  defined at `fastmcp.server.transforms.search.base.serialize_tools_for_output_json`

```python
def serialize_tools_for_output_json(tools: Sequence[Tool]) -> list[dict[str, Any]]
```

**Also exported as** `fastmcp.server.transforms.search.serialize_tools_for_output_json`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Serialize tools to the same dict format as ``list_tools`` output.


## serialize_tools_for_output_markdown

Import as `fastmcp.server.transforms.search.serialize_tools_for_output_markdown`  ·  defined at `fastmcp.server.transforms.search.base.serialize_tools_for_output_markdown`

```python
def serialize_tools_for_output_markdown(tools: Sequence[Tool]) -> str
```

**Also exported as** `fastmcp.server.transforms.search.serialize_tools_for_output_markdown`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Serialize tools to compact markdown, using ~65-70% fewer tokens than JSON.


