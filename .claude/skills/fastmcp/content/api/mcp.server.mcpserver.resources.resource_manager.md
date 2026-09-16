# `mcp.server.mcpserver.resources.resource_manager`

Distribution: `mcp`

## logger

`mcp.server.mcpserver.resources.resource_manager.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ResourceManager

Import as `mcp.server.mcpserver.resources.ResourceManager`  ·  defined at `mcp.server.mcpserver.resources.resource_manager.ResourceManager`

```python
class ResourceManager
```

**Also exported as** `mcp.server.mcpserver.resources.ResourceManager`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `def add_resource(self, resource: Resource) -> Resource`
  Add a resource to the manager.
- `def add_template(self, fn: Callable[..., Any], uri_template: str, name: str | None = None, title: str | None = None, description: str | None = None, mime_type: str | None = None, icons: list[Icon] | None = None, annotations: Annotations | None = None, meta: dict[str, Any] | None = None, security: ResourceSecurity = DEFAULT_RESOURCE_SECURITY) -> ResourceTemplate`
  Add a template from a function.
- `async def get_resource(self, uri: AnyUrl | str, context: Context[LifespanContextT, RequestT]) -> Resource | InputRequiredResult`  _async_
  Get resource by URI, checking concrete resources first, then templates.
- `def list_resources(self) -> list[Resource]`
  List all registered resources.
- `def list_templates(self) -> list[ResourceTemplate]`
  List all registered templates.
- `warn_on_duplicate_resources = warn_on_duplicate_resources`  _instance-attribute_

Manages MCPServer resources.


