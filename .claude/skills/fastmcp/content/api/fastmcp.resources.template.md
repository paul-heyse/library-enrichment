# `fastmcp.resources.template`

Distribution: `fastmcp`

## FunctionResourceTemplate

`fastmcp.resources.template.FunctionResourceTemplate`

```python
class FunctionResourceTemplate(ResourceTemplate)
```

**Bases** `ResourceTemplate`

**Declared members (4)**

- `async def create_resource(self, uri: str, params: dict[str, Any]) -> Resource`  _async_
  Create a resource from the template with the given parameters.
- `fn: SkipJsonSchema[Callable[..., Any]]`  _instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], uri_template: str, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, mime_type: str | None = None, tags: set[str] | None = None, annotations: Annotations | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None, security: ResourceSecurity | None | InheritSecurity = INHERIT_SECURITY) -> FunctionResourceTemplate`  _classmethod_
  Create a template from a function.
- `async def read(self, arguments: dict[str, Any]) -> str | bytes | ResourceResult`  _async_
  Read the resource content.

**Inherited (27)**

- from `fastmcp.resources.template.ResourceTemplate`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_mcp_template`, `get_span_attributes`, `key`, `matches`, `mime_type`, `parameters`, `resolve_security`, `security`, `set_default_mime_type`, `to_mcp_template`, `uri_template`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A template for dynamically creating resources.


## ResourceTemplate

Import as `fastmcp.resources.ResourceTemplate`  ·  defined at `fastmcp.resources.template.ResourceTemplate`

```python
class ResourceTemplate(FastMCPComponent)
```

**Also exported as** `fastmcp.resources.ResourceTemplate`

_21 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPComponent`

**Declared members (18)**

- `KEY_PREFIX: str = 'template'`  _class-attribute_
- `annotations: Annotations | None = Field(default=None, description="Optional annotations about the resource's behavior")`  _class-attribute, instance-attribute_
- `auth: SkipJsonSchema[AuthCheck | list[AuthCheck] | None] = Field(default=None, description='Authorization checks for this resource template', exclude=True)`  _class-attribute, instance-attribute_
- `def convert_result(self, raw_value: Any) -> ResourceResult`
  Convert a raw result to ResourceResult.
- `async def create_resource(self, uri: str, params: dict[str, Any]) -> Resource`  _async_
  Create a resource from the template with the given parameters.
- `def from_function(fn: Callable[..., Any], uri_template: str, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, mime_type: str | None = None, tags: set[str] | None = None, annotations: Annotations | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None, security: ResourceSecurity | None | InheritSecurity = INHERIT_SECURITY) -> FunctionResourceTemplate`  _staticmethod_
- `def from_mcp_template(cls, mcp_template: SDKResourceTemplate) -> ResourceTemplate`  _classmethod_
  Creates a FastMCP ResourceTemplate from a raw MCP ResourceTemplate object.
- `def get_span_attributes(self) -> dict[str, Any]`
- `key: str`  _property_
  The globally unique lookup key for this template.
- `def matches(self, uri: str) -> dict[str, Any] | None`
  Check if URI matches template and extract parameters.
- `mime_type: str = Field(default='text/plain', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `parameters: dict[str, Any] = Field(description='JSON schema for function parameters')`  _class-attribute, instance-attribute_
- `async def read(self, arguments: dict[str, Any]) -> str | bytes | ResourceResult`  _async_
  Read the resource content.
- `def resolve_security(self, server_default: ResourceSecurity | None) -> ResourceSecurity | None`
  Resolve the effective security policy for this template.
- `security: SkipJsonSchema[ResourceSecurity | None | InheritSecurity] = Field(default=INHERIT_SECURITY, description='Path-safety policy for extracted parameters. INHERIT_SECURITY (default) inherits the server-wide default; None disables screening; a ResourceSecurity instance applies that explicit policy.', exclude=True)`  _class-attribute, instance-attribute_
- `def set_default_mime_type(cls, mime_type: str | None) -> str`  _classmethod_
  Set default MIME type if not provided.
- `def to_mcp_template(self, overrides: Any = {}) -> SDKResourceTemplate`
  Convert the resource template to an SDKResourceTemplate.
- `uri_template: str = Field(description='URI template with parameters (e.g. weather://{city}/current)')`  _class-attribute, instance-attribute_

**Inherited (12)**

- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A template for dynamically creating resources.


## build_regex

`fastmcp.resources.template.build_regex`

```python
def build_regex(template: str) -> re.Pattern[str] | None
```

Build regex pattern for URI template, handling RFC 6570 syntax.

Supports:
- `{var}` - simple path parameter
- `{var*}` - wildcard path parameter (captures multiple segments)
- `{?var1,var2}` - query parameters (ignored in path matching)

Hyphens in parameter names are normalized to underscores in regex group
names so that matched groups are valid Python identifiers.

Returns None if the template produces an invalid regex (e.g. parameter
names with leading digits or duplicates from a remote server).


## expand_uri_template

Import as `fastmcp.server.providers.proxy.expand_uri_template`  ·  defined at `fastmcp.resources.template.expand_uri_template`

```python
def expand_uri_template(uri_template: str, params: dict[str, Any]) -> str
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Expand a URI template with parameters — inverse of `match_uri_template`.

Supports the same RFC 6570 subset:
- Path params: `{var}`, `{var*}`
- Query params: `{?var1,var2}`


## extract_query_params

`fastmcp.resources.template.extract_query_params`

```python
def extract_query_params(uri_template: str) -> set[str]
```

Extract query parameter names from RFC 6570 `{?param1,param2}` syntax.


## match_uri_template

`fastmcp.resources.template.match_uri_template`

```python
def match_uri_template(uri: str, uri_template: str) -> dict[str, str] | None
```

Match URI against template and extract both path and query parameters.

Supports RFC 6570 URI templates:
- Path params: `{var}`, `{var*}`
- Query params: `{?var1,var2}`


