# `fastmcp.utilities.openapi.models`

Distribution: `fastmcp`

## HttpMethod

Import as `fastmcp.utilities.openapi.HttpMethod`  ·  defined at `fastmcp.utilities.openapi.models.HttpMethod`

```python
HttpMethod = Literal['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'OPTIONS', 'HEAD', 'TRACE']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["GET", "POST", "PUT", "DELETE", "PATCH", ... omitted 3 literals]'> ````

**Also exported as** `fastmcp.utilities.openapi.HttpMethod`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## JsonSchema

Import as `fastmcp.utilities.openapi.JsonSchema`  ·  defined at `fastmcp.utilities.openapi.models.JsonSchema`

```python
JsonSchema = dict[str, Any]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'dict[str, Any]'> ````

**Also exported as** `fastmcp.utilities.openapi.JsonSchema`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ParameterLocation

Import as `fastmcp.utilities.openapi.ParameterLocation`  ·  defined at `fastmcp.utilities.openapi.models.ParameterLocation`

```python
ParameterLocation = Literal['path', 'query', 'header', 'cookie']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["path", "query", "header", "cookie"]'> ````

**Also exported as** `fastmcp.utilities.openapi.ParameterLocation`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## __all__

`fastmcp.utilities.openapi.models.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['HTTPRoute', 'HttpMethod', 'JsonSchema', 'ParameterInfo', 'ParameterLocation', 'RequestBodyInfo', 'ResponseInfo']
```

## HTTPRoute

Import as `fastmcp.utilities.openapi.HTTPRoute`  ·  defined at `fastmcp.utilities.openapi.models.HTTPRoute`

```python
class HTTPRoute(FastMCPBaseModel)
```

**Also exported as** `fastmcp.utilities.openapi.HTTPRoute`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPBaseModel`

**Declared members (15)**

- `description: str | None = None`  _class-attribute, instance-attribute_
- `extensions: dict[str, Any] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `flat_param_schema: JsonSchema = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `method: HttpMethod`  _instance-attribute_
- `openapi_version: str | None = None`  _class-attribute, instance-attribute_
- `operation_id: str | None = None`  _class-attribute, instance-attribute_
- `parameter_map: dict[str, dict[str, str]] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `parameters: list[ParameterInfo] = Field(default_factory=list)`  _class-attribute, instance-attribute_
- `path: str`  _instance-attribute_
- `request_body: RequestBodyInfo | None = None`  _class-attribute, instance-attribute_
- `request_schemas: dict[str, JsonSchema] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `response_schemas: dict[str, JsonSchema] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `responses: dict[str, ResponseInfo] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `summary: str | None = None`  _class-attribute, instance-attribute_
- `tags: list[str] = Field(default_factory=list)`  _class-attribute, instance-attribute_

Intermediate Representation for a single OpenAPI operation.


## ParameterInfo

Import as `fastmcp.utilities.openapi.ParameterInfo`  ·  defined at `fastmcp.utilities.openapi.models.ParameterInfo`

```python
class ParameterInfo(FastMCPBaseModel)
```

**Also exported as** `fastmcp.utilities.openapi.ParameterInfo`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPBaseModel`

**Declared members (7)**

- `description: str | None = None`  _class-attribute, instance-attribute_
- `explode: bool | None = None`  _class-attribute, instance-attribute_
- `location: ParameterLocation`  _instance-attribute_
- `name: str`  _instance-attribute_
- `required: bool = False`  _class-attribute, instance-attribute_
- `schema_: JsonSchema = Field(..., alias='schema')`  _class-attribute, instance-attribute_
- `style: str | None = None`  _class-attribute, instance-attribute_

Represents a single parameter for an HTTP operation in our IR.


## RequestBodyInfo

Import as `fastmcp.utilities.openapi.RequestBodyInfo`  ·  defined at `fastmcp.utilities.openapi.models.RequestBodyInfo`

```python
class RequestBodyInfo(FastMCPBaseModel)
```

**Also exported as** `fastmcp.utilities.openapi.RequestBodyInfo`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPBaseModel`

**Declared members (3)**

- `content_schema: dict[str, JsonSchema] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `required: bool = False`  _class-attribute, instance-attribute_

Represents the request body for an HTTP operation in our IR.


## ResponseInfo

Import as `fastmcp.utilities.openapi.ResponseInfo`  ·  defined at `fastmcp.utilities.openapi.models.ResponseInfo`

```python
class ResponseInfo(FastMCPBaseModel)
```

**Also exported as** `fastmcp.utilities.openapi.ResponseInfo`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPBaseModel`

**Declared members (2)**

- `content_schema: dict[str, JsonSchema] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_

Represents response information in our IR.


