# `fastmcp.utilities.openapi.director`

Distribution: `fastmcp`

## __all__

`fastmcp.utilities.openapi.director.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['RequestDirector']
```

## logger

`fastmcp.utilities.openapi.director.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## RequestDirector

Import as `fastmcp.server.providers.openapi.provider.RequestDirector`  ·  defined at `fastmcp.utilities.openapi.director.RequestDirector`

```python
class RequestDirector
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `def build(self, route: HTTPRoute, flat_args: dict[str, Any], base_url: str = 'http://localhost') -> httpx2.Request`
  Constructs a final httpx2.Request object, handling all OpenAPI serialization.

Builds httpx2.Request objects from HTTPRoute and arguments using openapi-core.


## _query_scalar_to_str

`fastmcp.utilities.openapi.director._query_scalar_to_str`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _query_scalar_to_str(value: Any) -> str
```

Convert a scalar to its query-string representation.

Booleans are lowercased to match JSON/OpenAPI conventions (true/false)
rather than Python's str(True) → "True".


