# `fastmcp.server.providers.openapi.provider`

Distribution: `fastmcp`

## DEFAULT_TIMEOUT

`fastmcp.server.providers.openapi.provider.DEFAULT_TIMEOUT`

```python
DEFAULT_TIMEOUT: float = 30.0
```

## __all__

`fastmcp.server.providers.openapi.provider.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['OpenAPIProvider']
```

## logger

`fastmcp.server.providers.openapi.provider.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## OpenAPIProvider

Import as `fastmcp.server.providers.OpenAPIProvider`  ·  defined at `fastmcp.server.providers.openapi.provider.OpenAPIProvider`

```python
class OpenAPIProvider(Provider)
```

**Also exported as** `fastmcp.server.providers.OpenAPIProvider`, `fastmcp.server.providers.openapi.OpenAPIProvider`

**Bases** `Provider`

**Declared members (2)**

- `async def get_tasks(self) -> Sequence[FastMCPComponent]`  _async_
  Return empty list - OpenAPI components don't support tasks.
- `async def lifespan(self) -> AsyncIterator[None]`  _async_
  Manage the lifecycle of the auto-created httpx client.

**Inherited (15)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `get_tool_by_hash`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider that creates MCP components from an OpenAPI specification.

Components are created eagerly during initialization by parsing the OpenAPI
spec. Each component makes HTTP calls to the described API endpoints.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.providers.openapi import OpenAPIProvider
    import httpx2

    client = httpx2.AsyncClient(base_url="https://api.example.com")
    provider = OpenAPIProvider(openapi_spec=spec, client=client)

    mcp = FastMCP("API Server")
    mcp.add_provider(provider)
    ```


## _is_legacy_httpx_client

`fastmcp.server.providers.openapi.provider._is_legacy_httpx_client`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_legacy_httpx_client(client: object) -> bool
```

Detect a legacy httpx client without importing the legacy package.


