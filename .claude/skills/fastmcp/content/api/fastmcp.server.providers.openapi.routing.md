# `fastmcp.server.providers.openapi.routing`

Distribution: `fastmcp`

## ComponentFn

Import as `fastmcp.server.providers.openapi.ComponentFn`  ·  defined at `fastmcp.server.providers.openapi.routing.ComponentFn`

```python
ComponentFn = Callable[[HTTPRoute, 'OpenAPITool | OpenAPIResource | OpenAPIResourceTemplate'], None]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( HTTPRoute, OpenAPITool | OpenAPIResource | OpenAPIResourceTemplate, / ) -> None'> ````

**Also exported as** `fastmcp.server.providers.openapi.ComponentFn`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DEFAULT_ROUTE_MAPPINGS

Import as `fastmcp.server.providers.openapi.provider.DEFAULT_ROUTE_MAPPINGS`  ·  defined at `fastmcp.server.providers.openapi.routing.DEFAULT_ROUTE_MAPPINGS`

```python
DEFAULT_ROUTE_MAPPINGS = [RouteMap(mcp_type=MCPType.TOOL)]
```

**Inferred type** (`ty`, not declared in the source): `list[RouteMap]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## RouteMapFn

Import as `fastmcp.server.providers.openapi.RouteMapFn`  ·  defined at `fastmcp.server.providers.openapi.routing.RouteMapFn`

```python
RouteMapFn = Callable[[HTTPRoute, 'MCPType'], 'MCPType | None']
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( HTTPRoute, MCPType, / ) -> MCPType | None'> ````

**Also exported as** `fastmcp.server.providers.openapi.RouteMapFn`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## __all__

`fastmcp.server.providers.openapi.routing.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ComponentFn', 'MCPType', 'RouteMap', 'RouteMapFn']
```

## logger

`fastmcp.server.providers.openapi.routing.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## MCPType

Import as `fastmcp.server.providers.openapi.MCPType`  ·  defined at `fastmcp.server.providers.openapi.routing.MCPType`

```python
class MCPType(enum.Enum)
```

**Also exported as** `fastmcp.server.providers.openapi.MCPType`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `enum.Enum`

**Declared members (4)**

- `EXCLUDE = 'EXCLUDE'`  _class-attribute, instance-attribute_
- `RESOURCE = 'RESOURCE'`  _class-attribute, instance-attribute_
- `RESOURCE_TEMPLATE = 'RESOURCE_TEMPLATE'`  _class-attribute, instance-attribute_
- `TOOL = 'TOOL'`  _class-attribute, instance-attribute_

Type of FastMCP component to create from a route.

Enum values:
    TOOL: Convert the route to a callable Tool
    RESOURCE: Convert the route to a Resource (typically GET endpoints)
    RESOURCE_TEMPLATE: Convert the route to a ResourceTemplate (typically GET with path params)
    EXCLUDE: Exclude the route from being converted to any MCP component


## RouteMap

Import as `fastmcp.server.providers.openapi.RouteMap`  ·  defined at `fastmcp.server.providers.openapi.routing.RouteMap`

```python
class RouteMap
```

**Also exported as** `fastmcp.server.providers.openapi.RouteMap`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `mcp_tags: set[str] = field(default_factory=set, metadata={'description': 'A set of tags to apply to the generated FastMCP component.'})`  _class-attribute, instance-attribute_
- `mcp_type: MCPType = field(metadata={'description': 'The type of FastMCP component to create.'})`  _class-attribute, instance-attribute_
- `methods: list[HttpMethod] | Literal['*'] = field(default='*')`  _class-attribute, instance-attribute_
- `pattern: Pattern[str] | str = field(default='.*')`  _class-attribute, instance-attribute_
- `tags: set[str] = field(default_factory=set, metadata={'description': 'A set of tags to match. All tags must match.'})`  _class-attribute, instance-attribute_

Mapping configuration for HTTP routes to FastMCP component types.


## _determine_route_type

`fastmcp.server.providers.openapi.routing._determine_route_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _determine_route_type(route: HTTPRoute, mappings: list[RouteMap]) -> RouteMap
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Determine the FastMCP component type based on the route and mappings.


