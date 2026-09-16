# Transform

Rewriting the component catalog, observably. The system can see what you changed, which is what separates this from a Provider that lies.

Import as `fastmcp.server.server.Transform`
Defined at `fastmcp.server.transforms.Transform`.

```python
class Transform
```

## Required

Nothing. Every member has a default, so a subclass that overrides none of them is valid -- and does nothing useful.

## Provided

Defaulted, and this is where the capability hides. The default is almost always the conservative answer, so an implementation that overrides none of these works correctly and supplies nothing.

```python
async def get_prompt(self, name: str, call_next: GetPromptNext, version: VersionSpec | None = None) -> Prompt | None
async def get_resource(self, uri: str, call_next: GetResourceNext, version: VersionSpec | None = None) -> Resource | None
async def get_resource_template(self, uri: str, call_next: GetResourceTemplateNext, version: VersionSpec | None = None) -> ResourceTemplate | None
async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None
async def list_prompts(self, prompts: Sequence[Prompt]) -> Sequence[Prompt]
async def list_resource_templates(self, templates: Sequence[ResourceTemplate]) -> Sequence[ResourceTemplate]
async def list_resources(self, resources: Sequence[Resource]) -> Sequence[Resource]
async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]
```

## Implementors (11)

Transitive. Read one before writing your own.

- `fastmcp.experimental.transforms.code_mode.CodeMode`
- `fastmcp.server.transforms.catalog.CatalogTransform`
- `fastmcp.server.transforms.namespace.Namespace`
- `fastmcp.server.transforms.prompts_as_tools.PromptsAsTools`
- `fastmcp.server.transforms.resources_as_tools.ResourcesAsTools`
- `fastmcp.server.transforms.search.base.BaseSearchTransform`
- `fastmcp.server.transforms.search.bm25.BM25SearchTransform`
- `fastmcp.server.transforms.search.regex.RegexSearchTransform`
- `fastmcp.server.transforms.tool_transform.ToolTransform`
- `fastmcp.server.transforms.version_filter.VersionFilter`
- `fastmcp.server.transforms.visibility.Visibility`

## Demonstrated by 1 upstream file(s)

Matched as syntax -- `class X(Base)` -- not by searching for the name.

- [`corpus/tests/server/transforms/test_catalog.py`](../corpus/tests/server/transforms/test_catalog.py)

## Documentation

Prose: [`api/fastmcp.server.transforms.md`](../api/fastmcp.server.transforms.md) · records: [`model/fastmcp.server.transforms.json`](../model/fastmcp.server.transforms.json)
