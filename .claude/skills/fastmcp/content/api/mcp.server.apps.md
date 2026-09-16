# `mcp.server.apps`

Distribution: `mcp`

## APP_MIME_TYPE

`mcp.server.apps.APP_MIME_TYPE`

```python
APP_MIME_TYPE = 'text/html;profile=mcp-app'
```

**Inferred type** (`ty`, not declared in the source): `Literal["text/html;profile=mcp-app"]`

MIME type for a `ui://` app resource.


## EXTENSION_ID

`mcp.server.apps.EXTENSION_ID`

```python
EXTENSION_ID = 'io.modelcontextprotocol/ui'
```

**Inferred type** (`ty`, not declared in the source): `Literal["io.modelcontextprotocol/ui"]`

The MCP Apps extension identifier (the shipped TS/C# constant).


## Visibility

`mcp.server.apps.Visibility`

```python
Visibility = Literal['model', 'app']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["model", "app"]'> ``` --- Where a UI-bound tool is surfaced (`_meta.ui.visibility`).`

Where a UI-bound tool is surfaced (`_meta.ui.visibility`).


## _CallableT

`mcp.server.apps._CallableT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CallableT = TypeVar('_CallableT', bound=Callable[..., Any])
```

## Apps

`mcp.server.apps.Apps`

```python
class Apps(Extension)
```

**Bases** `Extension`

**Declared members (6)**

- `def add_html_resource(self, uri: str, html: str, name: str | None = None, title: str | None = None, description: str | None = None, csp: ResourceCsp | None = None, permissions: ResourcePermissions | None = None, domain: str | None = None, prefers_border: bool | None = None) -> None`
  Register a `ui://` HTML resource served as `text/html;profile=mcp-app`.
- `def add_resource(self, resource: Resource) -> None`
  Register a pre-built `ui://` resource.
- `identifier = EXTENSION_ID`  _class-attribute, instance-attribute_
- `def resources(self) -> Sequence[ResourceBinding]`
- `def tool(self, resource_uri: str, visibility: Sequence[Visibility] | None = None, meta: dict[str, Any] | None = None, tool_kwargs: Any = {}) -> Callable[[_CallableT], _CallableT]`
  Decorator registering a tool bound to a `ui://` resource.
- `def tools(self) -> Sequence[ToolBinding]`
  The bound tools.

**Inherited (3)**

- from `mcp.server.extension.Extension`: `intercept_tool_call`, `methods`, `settings`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The MCP Apps extension: bind tools to `ui://` UI resources.

Register UI-bound tools with `@apps.tool(resource_uri=...)` and their HTML
with `add_html_resource(...)`, then pass the instance to
`MCPServer(extensions=[apps])`.


## ResourceCsp

`mcp.server.apps.ResourceCsp`

```python
class ResourceCsp(BaseModel)
```

**Bases** `BaseModel`

**Declared members (4)**

- `base_uri_domains: list[str] | None = None`  _class-attribute, instance-attribute_
- `connect_domains: list[str] | None = None`  _class-attribute, instance-attribute_
- `frame_domains: list[str] | None = None`  _class-attribute, instance-attribute_
- `resource_domains: list[str] | None = None`  _class-attribute, instance-attribute_

Content-Security-Policy domains for a `ui://` resource (`_meta.ui.csp`).


## ResourcePermissions

`mcp.server.apps.ResourcePermissions`

```python
class ResourcePermissions(BaseModel)
```

**Bases** `BaseModel`

**Declared members (4)**

- `camera: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `clipboard_write: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `geolocation: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `microphone: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

Iframe permissions a `ui://` resource requests (`_meta.ui.permissions`).


## _client_capabilities

`mcp.server.apps._client_capabilities`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _client_capabilities(ctx: Context[Any] | ServerRequestContext[Any, Any]) -> Any
```

## _require_ui_scheme

`mcp.server.apps._require_ui_scheme`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _require_ui_scheme(uri: str) -> None
```

## client_supports_apps

`mcp.server.apps.client_supports_apps`

```python
def client_supports_apps(ctx: Context[Any] | ServerRequestContext[Any, Any]) -> bool
```

Whether the connected client negotiated MCP Apps support.

Returns `True` only when the client advertised the extension AND listed the
`text/html;profile=mcp-app` MIME type in its settings, so a UI-enabled tool
can fall back to text-only output otherwise.


