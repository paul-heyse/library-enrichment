# `fastmcp.server.transforms.version_filter`

Distribution: `fastmcp`

## VersionFilter

Import as `fastmcp.server.transforms.VersionFilter`  ·  defined at `fastmcp.server.transforms.version_filter.VersionFilter`

```python
class VersionFilter(Transform)
```

**Also exported as** `fastmcp.server.transforms.VersionFilter`

**Bases** `Transform`

**Declared members (11)**

- `async def get_prompt(self, name: str, call_next: GetPromptNext, version: VersionSpec | None = None) -> Prompt | None`  _async_
- `async def get_resource(self, uri: str, call_next: GetResourceNext, version: VersionSpec | None = None) -> Resource | None`  _async_
- `async def get_resource_template(self, uri: str, call_next: GetResourceTemplateNext, version: VersionSpec | None = None) -> ResourceTemplate | None`  _async_
- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
- `include_unversioned = include_unversioned`  _instance-attribute_
- `async def list_prompts(self, prompts: Sequence[Prompt]) -> Sequence[Prompt]`  _async_
- `async def list_resource_templates(self, templates: Sequence[ResourceTemplate]) -> Sequence[ResourceTemplate]`  _async_
- `async def list_resources(self, resources: Sequence[Resource]) -> Sequence[Resource]`  _async_
- `async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
- `version_gte = version_gte`  _instance-attribute_
- `version_lt = version_lt`  _instance-attribute_

Filters components by version range.

When applied to a provider or server, components within the version range
are visible, and unversioned components are included by default. Within
that filtered set, the highest version of each component is exposed to
clients (standard deduplication behavior). Set
``include_unversioned=False`` to exclude unversioned components.

Parameters mirror comparison operators for clarity:

    # Versions < 3.0 (v1 and v2)
    server.add_transform(VersionFilter(version_lt="3.0"))

    # Versions >= 2.0 and < 3.0 (only v2.x)
    server.add_transform(VersionFilter(version_gte="2.0", version_lt="3.0"))

Works with any version string - PEP 440 (1.0, 2.0) or dates (2025-01-01).

Args:
    version_gte: Versions >= this value pass through.
    version_lt: Versions < this value pass through.
    include_unversioned: Whether unversioned components (``version=None``)
        should pass through the filter. Defaults to True.


