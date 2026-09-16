# `fastmcp.server.transforms.catalog`

Distribution: `fastmcp`

## _instance_counter

`fastmcp.server.transforms.catalog._instance_counter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_instance_counter = itertools.count()
```

## CatalogTransform

Import as `fastmcp.experimental.transforms.code_mode.CatalogTransform`  ·  defined at `fastmcp.server.transforms.catalog.CatalogTransform`

```python
class CatalogTransform(Transform)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Transform`

**Declared members (12)**

- `async def get_prompt_catalog(self, ctx: Context, run_middleware: bool = True) -> Sequence[Prompt]`  _async_
  Fetch the real prompt catalog, bypassing this transform.
- `async def get_resource_catalog(self, ctx: Context, run_middleware: bool = True) -> Sequence[Resource]`  _async_
  Fetch the real resource catalog, bypassing this transform.
- `async def get_resource_template_catalog(self, ctx: Context, run_middleware: bool = True) -> Sequence[ResourceTemplate]`  _async_
  Fetch the real resource template catalog, bypassing this transform.
- `async def get_tool_catalog(self, ctx: Context, run_middleware: bool = True) -> Sequence[Tool]`  _async_
  Fetch the real tool catalog, bypassing this transform.
- `async def list_prompts(self, prompts: Sequence[Prompt]) -> Sequence[Prompt]`  _async_
- `async def list_resource_templates(self, templates: Sequence[ResourceTemplate]) -> Sequence[ResourceTemplate]`  _async_
- `async def list_resources(self, resources: Sequence[Resource]) -> Sequence[Resource]`  _async_
- `async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
- `async def transform_prompts(self, prompts: Sequence[Prompt]) -> Sequence[Prompt]`  _async_
  Transform the prompt catalog.
- `async def transform_resource_templates(self, templates: Sequence[ResourceTemplate]) -> Sequence[ResourceTemplate]`  _async_
  Transform the resource template catalog.
- `async def transform_resources(self, resources: Sequence[Resource]) -> Sequence[Resource]`  _async_
  Transform the resource catalog.
- `async def transform_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
  Transform the tool catalog.

**Inherited (4)**

- from `fastmcp.server.transforms.Transform`: `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transform that needs access to the real component catalog.

Subclasses override ``transform_tools()`` / ``transform_resources()``
/ ``transform_prompts()`` / ``transform_resource_templates()``
instead of the ``list_*()`` methods.  The base class owns
``list_*()`` and handles re-entrant bypass automatically — subclasses
never see re-entrant calls from ``get_*_catalog()``.

The ``get_*_catalog()`` methods fetch the real (auth-filtered) catalog
by temporarily setting a bypass flag so that this transform's
``list_*()`` passes through without calling the subclass hook.


