# `fastmcp.server.providers.openapi.components`

Distribution: `fastmcp`

## _DEFAULT_MIME_TYPE

`fastmcp.server.providers.openapi.components._DEFAULT_MIME_TYPE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_MIME_TYPE = 'application/json'
```

## _SAFE_HEADERS

`fastmcp.server.providers.openapi.components._SAFE_HEADERS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SAFE_HEADERS = frozenset({'accept', 'accept-encoding', 'accept-language', 'cache-control', 'connection', 'content-length', 'content-type', 'host', 'user-agent'})
```

## __all__

`fastmcp.server.providers.openapi.components.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['OpenAPIResource', 'OpenAPIResourceTemplate', 'OpenAPITool', '_extract_mime_type_from_route']
```

## logger

`fastmcp.server.providers.openapi.components.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## OpenAPIResource

Import as `fastmcp.server.providers.openapi.OpenAPIResource`  ·  defined at `fastmcp.server.providers.openapi.components.OpenAPIResource`

```python
class OpenAPIResource(Resource)
```

**Also exported as** `fastmcp.server.providers.openapi.OpenAPIResource`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Resource`

**Declared members (2)**

- `async def read(self) -> ResourceResult`  _async_
  Fetch the resource data by making an HTTP request.
- `task_config: TaskConfig = TaskConfig(mode='forbidden')`  _class-attribute, instance-attribute_

**Inherited (23)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `get_span_attributes`, `key`, `mime_type`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Resource implementation for OpenAPI endpoints.


## OpenAPIResourceTemplate

Import as `fastmcp.server.providers.openapi.OpenAPIResourceTemplate`  ·  defined at `fastmcp.server.providers.openapi.components.OpenAPIResourceTemplate`

```python
class OpenAPIResourceTemplate(ResourceTemplate)
```

**Also exported as** `fastmcp.server.providers.openapi.OpenAPIResourceTemplate`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ResourceTemplate`

**Declared members (2)**

- `async def create_resource(self, uri: str, params: dict[str, Any], context: Context | None = None) -> Resource`  _async_
  Create a resource with the given parameters.
- `task_config: TaskConfig = TaskConfig(mode='forbidden')`  _class-attribute, instance-attribute_

**Inherited (28)**

- from `fastmcp.resources.template.ResourceTemplate`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `from_mcp_template`, `get_span_attributes`, `key`, `matches`, `mime_type`, `parameters`, `read`, `resolve_security`, `security`, `set_default_mime_type`, `to_mcp_template`, `uri_template`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `name`, `tags`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Resource template implementation for OpenAPI endpoints.


## OpenAPITool

Import as `fastmcp.server.providers.openapi.OpenAPITool`  ·  defined at `fastmcp.server.providers.openapi.components.OpenAPITool`

```python
class OpenAPITool(Tool)
```

**Also exported as** `fastmcp.server.providers.openapi.OpenAPITool`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Tool`

**Declared members (2)**

- `async def run(self, arguments: dict[str, Any]) -> ToolResult`  _async_
  Execute the HTTP request using RequestDirector.
- `task_config: TaskConfig = TaskConfig(mode='forbidden')`  _class-attribute, instance-attribute_

**Inherited (25)**

- from `fastmcp.tools.base.Tool`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `execution`, `from_function`, `from_tool`, `get_span_attributes`, `output_schema`, `parameters`, `return_type`, `timeout`, `to_mcp_tool`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Tool implementation for OpenAPI endpoints.


## _extract_mime_type_from_route

`fastmcp.server.providers.openapi.components._extract_mime_type_from_route`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _extract_mime_type_from_route(route: HTTPRoute) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract the primary MIME type from an HTTPRoute's response definitions.

Looks for the first successful response (2xx) and returns its content type.
Prefers JSON-compatible types when multiple are available.
Falls back to "application/json" when no response content type is declared.


## _path_argument_name

`fastmcp.server.providers.openapi.components._path_argument_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _path_argument_name(route: HTTPRoute, parameter_name: str) -> str
```

## _raise_for_status

`fastmcp.server.providers.openapi.components._raise_for_status`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _raise_for_status(response: httpx2.Response) -> None
```

Raise an OpenAPI-formatted error without relying on client exception types.


## _redact_headers

`fastmcp.server.providers.openapi.components._redact_headers`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _redact_headers(headers: httpx2.Headers) -> dict[str, str]
```

## _send_request

`fastmcp.server.providers.openapi.components._send_request`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _send_request(client: httpx2.AsyncClient, request: httpx2.Request) -> httpx2.Response
```

Send a request while preserving transitional legacy-client errors.


## _slugify

`fastmcp.server.providers.openapi.components._slugify`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _slugify(text: str) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Convert text to a URL-friendly slug format.

Only contains lowercase letters, uppercase letters, numbers, and underscores.


