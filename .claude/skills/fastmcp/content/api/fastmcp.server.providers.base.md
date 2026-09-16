# `fastmcp.server.providers.base`

Distribution: `fastmcp`

## Provider

Import as `fastmcp.server.providers.Provider`  ·  defined at `fastmcp.server.providers.base.Provider`

```python
class Provider
```

**Also exported as** `fastmcp.server.providers.Provider`

_14 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (17)**

- `def add_transform(self, transform: Transform) -> None`
  Add a transform to this provider.
- `def disable(self, names: set[str] | None = None, keys: set[str] | None = None, version: VersionSpec | None = None, tags: set[str] | None = None, components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None) -> Self`
  Disable components matching all specified criteria.
- `def enable(self, names: set[str] | None = None, keys: set[str] | None = None, version: VersionSpec | None = None, tags: set[str] | None = None, components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None, only: bool = False) -> Self`
  Enable components matching all specified criteria.
- `async def get_app_tool(self, app_name: str, tool_name: str) -> Tool | None`  _async_
  Look up an app-visible tool by original name, bypassing transforms.
- `async def get_prompt(self, name: str, version: VersionSpec | None = None) -> Prompt | None`  _async_
  Get prompt by transformed name with all transforms applied.
- `async def get_resource(self, uri: str, version: VersionSpec | None = None) -> Resource | None`  _async_
  Get resource by transformed URI with all transforms applied.
- `async def get_resource_template(self, uri: str, version: VersionSpec | None = None) -> ResourceTemplate | None`  _async_
  Get resource template by transformed URI with all transforms applied.
- `async def get_tasks(self) -> Sequence[FastMCPComponent]`  _async_
  Return components that should be registered as background tasks.
- `async def get_tool(self, name: str, version: VersionSpec | None = None) -> Tool | None`  _async_
  Get tool by transformed name with all transforms applied.
- `async def get_tool_by_hash(self, tool_hash: str, tool_name: str) -> Tool | None`  _async_
  Look up an app-visible tool by its deterministic hash.
- `async def lifespan(self) -> AsyncIterator[None]`  _async_
  User-overridable lifespan for custom setup and teardown.
- `async def list_prompts(self) -> Sequence[Prompt]`  _async_
  List prompts with all transforms applied.
- `async def list_resource_templates(self) -> Sequence[ResourceTemplate]`  _async_
  List resource templates with all transforms applied.
- `async def list_resources(self) -> Sequence[Resource]`  _async_
  List resources with all transforms applied.
- `async def list_tools(self) -> Sequence[Tool]`  _async_
  List tools with all transforms applied.
- `transforms: list[Transform]`  _property_
  All transforms applied to components from this provider.
- `def wrap_transform(self, transform: Transform) -> Provider`
  Return a new provider with this transform applied (immutable).

Base class for dynamic component providers.

Subclass and override whichever methods you need. Default implementations
return empty lists / None, so you only need to implement what your provider
supports.

Provider semantics:
    - Return `None` from `get_*` methods to indicate "I don't have it" (search continues)
    - Static components (registered via decorators) always take precedence over providers
    - Providers are queried in registration order; first non-None wins
    - Components execute themselves via run()/read()/render() - providers just source them

Error handling:
    - `list_*` methods: Errors are logged and the provider returns empty (graceful degradation).
      This allows other providers to still contribute their components.


