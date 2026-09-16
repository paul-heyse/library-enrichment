# `fastmcp.resources.function_resource`

Distribution: `fastmcp`

## F

`fastmcp.resources.function_resource.F`

```python
F = TypeVar('F', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## DecoratedResource

`fastmcp.resources.function_resource.DecoratedResource`

```python
class DecoratedResource(Protocol)
```

**Bases** `Protocol`

Protocol for functions decorated with @resource.


## FunctionResource

Import as `fastmcp.resources.FunctionResource`  ·  defined at `fastmcp.resources.function_resource.FunctionResource`

```python
class FunctionResource(Resource)
```

**Also exported as** `fastmcp.resources.FunctionResource`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Resource`

**Declared members (3)**

- `fn: SkipJsonSchema[Callable[..., Any]]`  _instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], uri: str | AnyUrl | None = None, metadata: ResourceMeta | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, mime_type: str | None = None, tags: set[str] | None = None, annotations: Annotations | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> FunctionResource`  _classmethod_
  Create a FunctionResource from a function.
- `async def read(self) -> str | bytes | ResourceResult`  _async_
  Read the resource by calling the wrapped function.

**Inherited (23)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `get_span_attributes`, `key`, `mime_type`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that defers data loading by wrapping a function.

The function is only called when the resource is read, allowing for lazy loading
of potentially expensive data. This is particularly useful when listing resources,
as the function won't be called until the resource is actually accessed.

The function can return:
- str for text content (default)
- bytes for binary content
- other types will be converted to JSON


## ResourceMeta

Import as `fastmcp.decorators.ResourceMeta`  ·  defined at `fastmcp.resources.function_resource.ResourceMeta`

```python
class ResourceMeta
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (14)**

- `annotations: Annotations | None = None`  _class-attribute, instance-attribute_
- `auth: AuthCheck | list[AuthCheck] | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `enabled: bool = True`  _class-attribute, instance-attribute_
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `mime_type: str | None = None`  _class-attribute, instance-attribute_
- `name: str | None = None`  _class-attribute, instance-attribute_
- `security: ResourceSecurity | None | InheritSecurity = INHERIT_SECURITY`  _class-attribute, instance-attribute_
- `tags: set[str] | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_
- `type: Literal['resource'] = field(default='resource', init=False)`  _class-attribute, instance-attribute_
- `uri: str`  _instance-attribute_
- `version: str | int | None = None`  _class-attribute, instance-attribute_

Metadata attached to functions by the @resource decorator.


## resource

Import as `fastmcp.resources.resource`  ·  defined at `fastmcp.resources.function_resource.resource`

```python
def resource(uri: str, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, mime_type: str | None = None, tags: set[str] | None = None, annotations: Annotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None, security: ResourceSecurity | None | InheritSecurity = INHERIT_SECURITY) -> Callable[[F], F]
```

**Also exported as** `fastmcp.resources.resource`

Standalone decorator to mark a function as an MCP resource.

Returns the original function with metadata attached. Register with a server
using mcp.add_resource().


