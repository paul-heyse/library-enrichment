# `fastmcp.server.providers.local_provider.decorators.resources`

Distribution: `fastmcp`

## F

`fastmcp.server.providers.local_provider.decorators.resources.F`

```python
F = TypeVar('F', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## ResourceDecoratorMixin

Import as `fastmcp.server.providers.local_provider.decorators.ResourceDecoratorMixin`  ·  defined at `fastmcp.server.providers.local_provider.decorators.resources.ResourceDecoratorMixin`

```python
class ResourceDecoratorMixin
```

**Also exported as** `fastmcp.server.providers.local_provider.decorators.ResourceDecoratorMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `def add_resource(self: LocalProvider, resource: Resource | ResourceTemplate | Callable[..., Any]) -> Resource | ResourceTemplate`
  Add a resource to this provider's storage.
- `def add_template(self: LocalProvider, template: ResourceTemplate) -> ResourceTemplate`
  Add a resource template to this provider's storage.
- `def resource(self: LocalProvider, uri: str, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, mime_type: str | None = None, tags: set[str] | None = None, enabled: bool = True, annotations: Annotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None, security: ResourceSecurity | None | InheritSecurity = INHERIT_SECURITY) -> Callable[[F], F]`
  Decorator to register a function as a resource.

Mixin class providing resource decorator functionality for LocalProvider.

This mixin contains all methods related to:
- Resource registration via add_resource()
- Resource template registration via add_template()
- Resource decorator (@provider.resource)


