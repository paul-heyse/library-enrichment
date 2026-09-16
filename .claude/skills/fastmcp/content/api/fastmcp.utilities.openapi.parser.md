# `fastmcp.utilities.openapi.parser`

Distribution: `fastmcp`

## TOpenAPI

`fastmcp.utilities.openapi.parser.TOpenAPI`

```python
TOpenAPI = TypeVar('TOpenAPI', OpenAPI, OpenAPI_30)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TOperation

`fastmcp.utilities.openapi.parser.TOperation`

```python
TOperation = TypeVar('TOperation', Operation, Operation_30)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TParameter

`fastmcp.utilities.openapi.parser.TParameter`

```python
TParameter = TypeVar('TParameter', Parameter, Parameter_30)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TPathItem

`fastmcp.utilities.openapi.parser.TPathItem`

```python
TPathItem = TypeVar('TPathItem', PathItem, PathItem_30)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TReference

`fastmcp.utilities.openapi.parser.TReference`

```python
TReference = TypeVar('TReference', Reference, Reference_30)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TRequestBody

`fastmcp.utilities.openapi.parser.TRequestBody`

```python
TRequestBody = TypeVar('TRequestBody', RequestBody, RequestBody_30)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TResponse

`fastmcp.utilities.openapi.parser.TResponse`

```python
TResponse = TypeVar('TResponse', Response, Response_30)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## TSchema

`fastmcp.utilities.openapi.parser.TSchema`

```python
TSchema = TypeVar('TSchema', Schema, Schema_30)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## __all__

`fastmcp.utilities.openapi.parser.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['OpenAPIParser', 'parse_openapi_to_http_routes']
```

## logger

`fastmcp.utilities.openapi.parser.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## OpenAPIParser

`fastmcp.utilities.openapi.parser.OpenAPIParser`

```python
class OpenAPIParser(Generic[TOpenAPI, TReference, TSchema, TParameter, TRequestBody, TResponse, TOperation, TPathItem])
```

**Bases** `Generic[TOpenAPI, TReference, TSchema, TParameter, TRequestBody, TResponse, TOperation, TPathItem]`

**Declared members (10)**

- `openapi = openapi`  _instance-attribute_
- `openapi_version = openapi_version`  _instance-attribute_
- `operation_cls = operation_cls`  _instance-attribute_
- `parameter_cls = parameter_cls`  _instance-attribute_
- `def parse(self) -> list[HTTPRoute]`
  Parse the OpenAPI schema into HTTP routes.
- `path_item_cls = path_item_cls`  _instance-attribute_
- `reference_cls = reference_cls`  _instance-attribute_
- `request_body_cls = request_body_cls`  _instance-attribute_
- `response_cls = response_cls`  _instance-attribute_
- `schema_cls = schema_cls`  _instance-attribute_

Unified parser for OpenAPI schemas with generic type parameters to handle both 3.0 and 3.1.


## parse_openapi_to_http_routes

Import as `fastmcp.utilities.openapi.parse_openapi_to_http_routes`  ·  defined at `fastmcp.utilities.openapi.parser.parse_openapi_to_http_routes`

```python
def parse_openapi_to_http_routes(openapi_dict: dict[str, Any]) -> list[HTTPRoute]
```

**Also exported as** `fastmcp.utilities.openapi.parse_openapi_to_http_routes`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Parses an OpenAPI schema dictionary into a list of HTTPRoute objects
using the openapi-pydantic library.

Supports both OpenAPI 3.0.x and 3.1.x versions.


