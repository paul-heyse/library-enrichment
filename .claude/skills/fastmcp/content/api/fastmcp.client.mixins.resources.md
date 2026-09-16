# `fastmcp.client.mixins.resources`

Distribution: `fastmcp`

## AUTO_PAGINATION_MAX_PAGES

`fastmcp.client.mixins.resources.AUTO_PAGINATION_MAX_PAGES`

```python
AUTO_PAGINATION_MAX_PAGES = 250
```

**Inferred type** (`ty`, not declared in the source): `Literal[250]`

## logger

`fastmcp.client.mixins.resources.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ClientResourcesMixin

Import as `fastmcp.client.mixins.ClientResourcesMixin`  ·  defined at `fastmcp.client.mixins.resources.ClientResourcesMixin`

```python
class ClientResourcesMixin
```

**Also exported as** `fastmcp.client.mixins.ClientResourcesMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `async def list_resource_templates(self: Client, max_pages: int = AUTO_PAGINATION_MAX_PAGES) -> list[mcp_types.ResourceTemplate]`  _async_
  Retrieve all resource templates available on the server.
- `async def list_resource_templates_mcp(self: Client, cursor: str | None = None, cache_mode: CacheMode = 'use') -> mcp_types.ListResourceTemplatesResult`  _async_
  Send a resources/listResourceTemplates request and return the complete MCP protocol result.
- `async def list_resources(self: Client, max_pages: int = AUTO_PAGINATION_MAX_PAGES) -> list[mcp_types.Resource]`  _async_
  Retrieve all resources available on the server.
- `async def list_resources_mcp(self: Client, cursor: str | None = None, cache_mode: CacheMode = 'use') -> mcp_types.ListResourcesResult`  _async_
  Send a resources/list request and return the complete MCP protocol result.
- `async def read_resource(self: Client, uri: AnyUrl | str, version: str | None = None, meta: dict[str, Any] | None = None) -> list[mcp_types.TextResourceContents | mcp_types.BlobResourceContents]`  _async_
  Read the contents of a resource or resolved template.
- `async def read_resource_mcp(self: Client, uri: AnyUrl | str, meta: dict[str, Any] | None = None) -> mcp_types.ReadResourceResult`  _async_
  Send a resources/read request and return the complete MCP protocol result.

Mixin providing resource-related methods for Client.


