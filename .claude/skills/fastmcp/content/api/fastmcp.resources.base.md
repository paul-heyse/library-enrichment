# `fastmcp.resources.base`

Distribution: `fastmcp`

## __all__

`fastmcp.resources.base.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['Resource', 'ResourceContent', 'ResourceResult']
```

## InputRequiredResourceResult

Import as `fastmcp.server.providers.proxy.InputRequiredResourceResult`  ·  defined at `fastmcp.resources.base.InputRequiredResourceResult`

```python
class InputRequiredResourceResult(ResourceResult)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ResourceResult`

**Declared members (1)**

- `input_required: mcp_types.InputRequiredResult = pydantic.Field(description='The client-input request this read resolved to (SEP-2322)')`  _class-attribute, instance-attribute_

**Inherited (3)**

- from `fastmcp.resources.base.ResourceResult`: `contents`, `meta`, `to_mcp_result`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The full result of a single multi-round-trip resource read (SEP-2322).

`InputRequiredResult` is a result type, not a `tools/call` feature: any
request may resolve to one. When a resource or resource template returns an
`InputRequiredResult` from its body to ask the client for input, that ask is
the legitimate result of this `resources/read` — so FastMCP wraps it in this
`ResourceResult` subclass, mirroring `InputRequiredToolResult` and
`InputRequiredPromptResult`, and it flows through the middleware chain as an
ordinary return value.

Invariant: the wrapped `InputRequiredResult` is never serialized as resource
contents. `contents` is always empty; the wire handler (`_on_read_resource`)
reads `.input_required` and returns it to the runner.


## Resource

Import as `fastmcp.resources.Resource`  ·  defined at `fastmcp.resources.base.Resource`

```python
class Resource(FastMCPComponent)
```

**Also exported as** `fastmcp.resources.Resource`

_26 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPComponent`

**Declared members (14)**

- `KEY_PREFIX: str = 'resource'`  _class-attribute_
- `annotations: Annotated[Annotations | None, Field(description="Optional annotations about the resource's behavior")] = None`  _class-attribute, instance-attribute_
- `auth: Annotated[SkipJsonSchema[AuthCheck | list[AuthCheck] | None], Field(description='Authorization checks for this resource', exclude=True)] = None`  _class-attribute, instance-attribute_
- `def convert_result(self, raw_value: Any) -> ResourceResult`
  Convert a raw result to ResourceResult.
- `def from_function(cls, fn: Callable[..., Any], uri: str | AnyUrl, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, mime_type: str | None = None, tags: set[str] | None = None, annotations: Annotations | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> FunctionResource`  _classmethod_
- `def get_span_attributes(self) -> dict[str, Any]`
- `key: str`  _property_
  The globally unique lookup key for this resource.
- `mime_type: str = Field(default='text/plain', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `name: str = Field(default='', description='Name of the resource')`  _class-attribute, instance-attribute_
- `async def read(self) -> str | bytes | ResourceResult`  _async_
  Read the resource content.
- `def set_default_mime_type(cls, mime_type: str | None) -> str`  _classmethod_
  Set default MIME type if not provided.
- `def set_default_name(self) -> Self`
  Set default name from URI if not provided.
- `def to_mcp_resource(self, overrides: Any = {}) -> SDKResource`
  Convert the resource to an SDKResource.
- `uri: Annotated[AnyUrl, UrlConstraints(host_required=False)] = Field(default=..., description='URI of the resource')`  _class-attribute, instance-attribute_

**Inherited (11)**

- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Base class for all resources.


## ResourceContent

Import as `fastmcp.resources.ResourceContent`  ·  defined at `fastmcp.resources.base.ResourceContent`

```python
class ResourceContent(pydantic.BaseModel)
```

**Also exported as** `fastmcp.resources.ResourceContent`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `pydantic.BaseModel`

**Declared members (4)**

- `content: str | bytes`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `mime_type: str | None = None`  _class-attribute, instance-attribute_
- `def to_mcp_resource_contents(self, uri: AnyUrl | str) -> mcp_types.TextResourceContents | mcp_types.BlobResourceContents`
  Convert to MCP resource contents type.

Wrapper for resource content with optional MIME type and metadata.

Accepts any value for content - strings and bytes pass through directly,
other types (dict, list, BaseModel, etc.) are automatically JSON-serialized.

Example:
    ```python
    from fastmcp.resources import ResourceContent

    # String content
    ResourceContent("plain text")

    # Binary content
    ResourceContent(b"binary data", mime_type="application/octet-stream")

    # Auto-serialized to JSON
    ResourceContent({"key": "value"})
    ResourceContent(["a", "b", "c"])
    ```


## ResourceResult

Import as `fastmcp.resources.ResourceResult`  ·  defined at `fastmcp.resources.base.ResourceResult`

```python
class ResourceResult(pydantic.BaseModel)
```

**Also exported as** `fastmcp.resources.ResourceResult`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `pydantic.BaseModel`

**Declared members (3)**

- `contents: list[ResourceContent]`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `def to_mcp_result(self, uri: AnyUrl | str) -> mcp_types.ReadResourceResult`
  Convert to MCP ReadResourceResult.

Canonical result type for resource reads.

Provides explicit control over resource responses: multiple content items,
per-item MIME types, and metadata at both the item and result level.

Accepts:
    - str: Wrapped as single ResourceContent (text/plain)
    - bytes: Wrapped as single ResourceContent (application/octet-stream)
    - list[ResourceContent]: Used directly for multiple items or custom MIME types

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.resources import ResourceResult, ResourceContent

    mcp = FastMCP()

    # Simple string content
    @mcp.resource("data://simple")
    def get_simple() -> ResourceResult:
        return ResourceResult("hello world")

    # Multiple items with custom MIME types
    @mcp.resource("data://items")
    def get_items() -> ResourceResult:
        return ResourceResult(
            contents=[
                ResourceContent({"key": "value"}),  # auto-serialized to JSON
                ResourceContent(b"binary data"),
            ],
            meta={"count": 2}
        )
    ```


## _public_content_meta

`fastmcp.resources.base._public_content_meta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _public_content_meta(meta: dict[str, Any] | None) -> dict[str, Any] | None
```

Strip FastMCP's internal bookkeeping out of component meta.

Component `meta` carries private entries under the `fastmcp` namespace
(e.g. `_internal.visibility`) that must never reach the wire. Listings
already filter these via `FastMCPComponent.get_meta()`; content items
served by `resources/read` need the same treatment.

Returns None when nothing public remains, so resources without user
metadata keep an absent `_meta` rather than an empty object.


## convert_raw_to_resource_result

Import as `fastmcp.resources.template.convert_raw_to_resource_result`  ·  defined at `fastmcp.resources.base.convert_raw_to_resource_result`

```python
def convert_raw_to_resource_result(raw_value: Any, mime_type: str | None, meta: dict[str, Any] | None) -> ResourceResult
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Wrap a user function's return value in a ResourceResult.

Shared by `Resource` and `ResourceTemplate` so both honor the MIME type
the component declares in listings. A component that advertises
`text/csv` must not serve `text/plain` on read.

Args:
    raw_value: The value returned by the user's function.
    mime_type: The component's declared MIME type, forwarded to content items.
    meta: Component-level meta (e.g. `ui` metadata for MCP Apps CSP/permissions)
        propagated to each content item.


