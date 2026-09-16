# `mcp.server.mcpserver.resources.templates`

Distribution: `mcp`

## DEFAULT_RESOURCE_SECURITY

Import as `mcp.server.mcpserver.DEFAULT_RESOURCE_SECURITY`  ·  defined at `mcp.server.mcpserver.resources.templates.DEFAULT_RESOURCE_SECURITY`

```python
DEFAULT_RESOURCE_SECURITY = ResourceSecurity()
```

**Inferred type** (`ty`, not declared in the source): `ResourceSecurity`

**Also exported as** `mcp.server.mcpserver.DEFAULT_RESOURCE_SECURITY`, `mcp.server.mcpserver.resources.DEFAULT_RESOURCE_SECURITY`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Secure-by-default policy: traversal, absolute paths, and null bytes rejected.


## ResourceSecurity

Import as `mcp.server.mcpserver.ResourceSecurity`  ·  defined at `mcp.server.mcpserver.resources.templates.ResourceSecurity`

```python
class ResourceSecurity
```

**Also exported as** `mcp.server.mcpserver.ResourceSecurity`, `mcp.server.mcpserver.resources.ResourceSecurity`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `exempt_params: Set[str] = field(default_factory=frozenset[str])`  _class-attribute, instance-attribute_
  Parameter names to skip all checks for.
- `reject_absolute_paths: bool = True`  _class-attribute, instance-attribute_
  Reject values that look like absolute filesystem paths.
- `reject_null_bytes: bool = True`  _class-attribute, instance-attribute_
  Reject values containing NUL (``\x00``). Null bytes defeat string comparisons (``"..\x00" != ".."``) and can cause truncation in C extensions or subprocess calls.
- `reject_path_traversal: bool = True`  _class-attribute, instance-attribute_
  Reject values containing ``..`` as a path component.
- `def validate(self, params: Mapping[str, str | list[str]]) -> str | None`
  Check all parameter values against the configured policy.

Security policy applied to extracted resource template parameters.

These checks run after :meth:`~mcp.shared.uri_template.UriTemplate.match`
has extracted and decoded parameter values. They catch path-traversal
and absolute-path injection regardless of how the value was encoded in
the URI (literal, ``%2F``, ``%5C``, ``%2E%2E``).

Example::

    # Opt out for a parameter that legitimately contains ..
    @mcp.resource(
        "git://diff/{+range}",
        security=ResourceSecurity(exempt_params={"range"}),
    )
    def git_diff(range: str) -> str: ...


## ResourceSecurityError

Import as `mcp.server.mcpserver.resources.ResourceSecurityError`  ·  defined at `mcp.server.mcpserver.resources.templates.ResourceSecurityError`

```python
class ResourceSecurityError(ValueError)
```

**Also exported as** `mcp.server.mcpserver.resources.ResourceSecurityError`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ValueError`

**Declared members (2)**

- `param = param`  _instance-attribute_
- `template = template`  _instance-attribute_

Raised when an extracted parameter fails :class:`ResourceSecurity` checks.

Distinct from a simple ``None`` non-match so that template
iteration can stop at the first security rejection rather than
falling through to a later, possibly more permissive, template.


## ResourceTemplate

Import as `mcp.server.mcpserver.resources.ResourceTemplate`  ·  defined at `mcp.server.mcpserver.resources.templates.ResourceTemplate`

```python
class ResourceTemplate(BaseModel)
```

**Also exported as** `mcp.server.mcpserver.resources.ResourceTemplate`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (16)**

- `annotations: Annotations | None = Field(default=None, description='Optional annotations for the resource template')`  _class-attribute, instance-attribute_
- `context_kwarg: str | None = Field(None, description='Name of the kwarg that should receive context')`  _class-attribute, instance-attribute_
- `async def create_resource(self, uri: str, params: dict[str, Any], context: Context[LifespanContextT, RequestT]) -> Resource | InputRequiredResult`  _async_
  Create a resource from the template with the given parameters.
- `description: str | None = Field(description='Description of what the resource does')`  _class-attribute, instance-attribute_
- `fn: Callable[..., Any] = Field(exclude=True)`  _class-attribute, instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], uri_template: str, name: str | None = None, title: str | None = None, description: str | None = None, mime_type: str | None = None, icons: list[Icon] | None = None, annotations: Annotations | None = None, meta: dict[str, Any] | None = None, context_kwarg: str | None = None, security: ResourceSecurity = DEFAULT_RESOURCE_SECURITY) -> ResourceTemplate`  _classmethod_
  Create a template from a function.
- `icons: list[Icon] | None = Field(default=None, description='Optional list of icons for the resource template')`  _class-attribute, instance-attribute_
- `def matches(self, uri: str) -> dict[str, str | list[str]] | None`
  Check if a URI matches this template and extract parameters.
- `meta: dict[str, Any] | None = Field(default=None, description='Optional metadata for this resource template')`  _class-attribute, instance-attribute_
- `mime_type: str = Field(default='text/plain', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `name: str = Field(description='Name of the resource')`  _class-attribute, instance-attribute_
- `parameters: dict[str, Any] = Field(description='JSON schema for function parameters')`  _class-attribute, instance-attribute_
- `parsed_template: UriTemplate = Field(exclude=True, description='Parsed RFC 6570 template')`  _class-attribute, instance-attribute_
- `security: ResourceSecurity = Field(exclude=True, description='Path-safety policy for extracted parameters')`  _class-attribute, instance-attribute_
- `title: str | None = Field(description='Human-readable title of the resource', default=None)`  _class-attribute, instance-attribute_
- `uri_template: str = Field(description='URI template with parameters (e.g. weather://{city}/current)')`  _class-attribute, instance-attribute_

A template for dynamically creating resources.


