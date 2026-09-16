# `fastmcp.server.transforms.namespace`

Distribution: `fastmcp`

## _URI_PATTERN

`fastmcp.server.transforms.namespace._URI_PATTERN`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_URI_PATTERN = re.compile('^([^:]+://)(.*?)$')
```

## Namespace

Import as `fastmcp.server.transforms.Namespace`  ·  defined at `fastmcp.server.transforms.namespace.Namespace`

```python
class Namespace(Transform)
```

**Also exported as** `fastmcp.server.transforms.Namespace`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Transform`

**Declared members (8)**

- `async def get_prompt(self, name: str, call_next: GetPromptNext, version: VersionSpec | None = None) -> Prompt | None`  _async_
  Get prompt by namespaced name.
- `async def get_resource(self, uri: str, call_next: GetResourceNext, version: VersionSpec | None = None) -> Resource | None`  _async_
  Get resource by namespaced URI.
- `async def get_resource_template(self, uri: str, call_next: GetResourceTemplateNext, version: VersionSpec | None = None) -> ResourceTemplate | None`  _async_
  Get resource template by namespaced URI.
- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
  Get tool by namespaced name.
- `async def list_prompts(self, prompts: Sequence[Prompt]) -> Sequence[Prompt]`  _async_
  Prefix prompt names with namespace.
- `async def list_resource_templates(self, templates: Sequence[ResourceTemplate]) -> Sequence[ResourceTemplate]`  _async_
  Add namespace path segment to template URIs.
- `async def list_resources(self, resources: Sequence[Resource]) -> Sequence[Resource]`  _async_
  Add namespace path segment to resource URIs.
- `async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
  Prefix tool names with namespace.

Prefixes component names with a namespace.

- Tools: name → namespace_name
- Prompts: name → namespace_name
- Resources: protocol://path → protocol://namespace/path
- Resource Templates: same as resources

Example:
    ```python
    transform = Namespace("math")
    # Tool "add" becomes "math_add"
    # Resource "file://data.txt" becomes "file://math/data.txt"
    ```


