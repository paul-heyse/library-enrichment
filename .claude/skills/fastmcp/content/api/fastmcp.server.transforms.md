# `fastmcp.server.transforms`

Distribution: `fastmcp`

## __all__

`fastmcp.server.transforms.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['Namespace', 'PromptsAsTools', 'ResourcesAsTools', 'ToolTransform', 'Transform', 'VersionFilter', 'VersionSpec', 'Visibility', 'is_enabled']
```

## GetPromptNext

Import as `fastmcp.server.providers.base.GetPromptNext`  ·  defined at `fastmcp.server.transforms.GetPromptNext`

```python
class GetPromptNext(Protocol)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

Protocol for get_prompt call_next functions.


## GetResourceNext

Import as `fastmcp.server.providers.base.GetResourceNext`  ·  defined at `fastmcp.server.transforms.GetResourceNext`

```python
class GetResourceNext(Protocol)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

Protocol for get_resource call_next functions.


## GetResourceTemplateNext

Import as `fastmcp.server.providers.base.GetResourceTemplateNext`  ·  defined at `fastmcp.server.transforms.GetResourceTemplateNext`

```python
class GetResourceTemplateNext(Protocol)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

Protocol for get_resource_template call_next functions.


## GetToolNext

Import as `fastmcp.server.providers.base.GetToolNext`  ·  defined at `fastmcp.server.transforms.GetToolNext`

```python
class GetToolNext(Protocol)
```

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

Protocol for get_tool call_next functions.


## Transform

Import as `fastmcp.server.server.Transform`  ·  defined at `fastmcp.server.transforms.Transform`

```python
class Transform
```

_10 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (8)**

- `async def get_prompt(self, name: str, call_next: GetPromptNext, version: VersionSpec | None = None) -> Prompt | None`  _async_
  Get a prompt by name.
- `async def get_resource(self, uri: str, call_next: GetResourceNext, version: VersionSpec | None = None) -> Resource | None`  _async_
  Get a resource by URI.
- `async def get_resource_template(self, uri: str, call_next: GetResourceTemplateNext, version: VersionSpec | None = None) -> ResourceTemplate | None`  _async_
  Get a resource template by URI.
- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
  Get a tool by name.
- `async def list_prompts(self, prompts: Sequence[Prompt]) -> Sequence[Prompt]`  _async_
  List prompts with transformation applied.
- `async def list_resource_templates(self, templates: Sequence[ResourceTemplate]) -> Sequence[ResourceTemplate]`  _async_
  List resource templates with transformation applied.
- `async def list_resources(self, resources: Sequence[Resource]) -> Sequence[Resource]`  _async_
  List resources with transformation applied.
- `async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
  List tools with transformation applied.

Base class for component transformations.

List operations use a pure function pattern: transforms receive sequences
and return transformed sequences. Get operations use a middleware pattern
with `call_next` to chain lookups.

Example:
    ```python
    class MyTransform(Transform):
        async def list_tools(self, tools):
            return [transform(t) for t in tools]  # Transform sequence

        async def get_tool(self, name, call_next, *, version=None):
            original = self.reverse_name(name)  # Map to original name
            tool = await call_next(original, version=version)  # Get from downstream
            return transform(tool) if tool else None
    ```


