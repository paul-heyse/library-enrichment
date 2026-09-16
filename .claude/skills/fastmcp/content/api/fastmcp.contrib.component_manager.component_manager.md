# `fastmcp.contrib.component_manager.component_manager`

Distribution: `fastmcp`

## _build_routes

`fastmcp.contrib.component_manager.component_manager._build_routes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_routes(server: FastMCP, base_path: str) -> list[Route]
```

Build all component management routes.


## _make_endpoint

`fastmcp.contrib.component_manager.component_manager._make_endpoint`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_endpoint(server: FastMCP, component_type: str, action: str)
```

Create an endpoint function for enabling/disabling a component type.


## set_up_component_manager

Import as `fastmcp.contrib.component_manager.set_up_component_manager`  ·  defined at `fastmcp.contrib.component_manager.component_manager.set_up_component_manager`

```python
def set_up_component_manager(server: FastMCP, path: str = '/', required_scopes: list[str] | None = None) -> None
```

**Also exported as** `fastmcp.contrib.component_manager.set_up_component_manager`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Set up HTTP routes for enabling/disabling tools, resources, and prompts.

Args:
    server: The FastMCP server instance.
    path: Base path for component management routes.
    required_scopes: Optional list of scopes required for these routes.
        Applies only if authentication is enabled.

Routes created:
    POST /tools/{name}/enable[?version=v1]
    POST /tools/{name}/disable[?version=v1]
    POST /resources/{uri}/enable[?version=v1]
    POST /resources/{uri}/disable[?version=v1]
    POST /prompts/{name}/enable[?version=v1]
    POST /prompts/{name}/disable[?version=v1]


