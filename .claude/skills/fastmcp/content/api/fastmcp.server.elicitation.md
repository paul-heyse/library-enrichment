# `fastmcp.server.elicitation`

Distribution: `fastmcp`

## T

`fastmcp.server.elicitation.T`

```python
T = TypeVar('T', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _NONE_RESPONSE_TYPE_ERROR

`fastmcp.server.elicitation._NONE_RESPONSE_TYPE_ERROR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_NONE_RESPONSE_TYPE_ERROR = 'ctx.elicit() requires a response_type. The empty-schema form-mode request produced by response_type=None was ambiguous under the MCP spec and caused some clients to render an empty, non-functional form. Pass a type describing the data you expect back — use `bool` for a confirmation.'
```

## __all__

`fastmcp.server.elicitation.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['AcceptedElicitation', 'CancelledElicitation', 'DeclinedElicitation', 'ElicitConfig', 'ScalarElicitationType', 'get_elicitation_schema', 'handle_elicit_accept', 'parse_elicit_response_type']
```

## logger

`fastmcp.server.elicitation.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AcceptedElicitation

Import as `fastmcp.server.context.AcceptedElicitation`  ·  defined at `fastmcp.server.elicitation.AcceptedElicitation`

```python
class AcceptedElicitation(BaseModel, Generic[T])
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`, `Generic[T]`

**Declared members (2)**

- `action: Literal['accept'] = 'accept'`  _class-attribute, instance-attribute_
- `data: T`  _instance-attribute_

Result when user accepts the elicitation.


## ElicitConfig

`fastmcp.server.elicitation.ElicitConfig`

```python
class ElicitConfig
```

**Declared members (2)**

- `is_raw: bool`  _instance-attribute_
- `response_type: type | None`  _instance-attribute_

Configuration for an elicitation request.

Attributes:
    schema: The JSON schema to send to the client
    response_type: The type to validate responses with (None for raw schemas)
    is_raw: True if schema was built directly (extract "value" from response)


## ElicitationJsonSchema

`fastmcp.server.elicitation.ElicitationJsonSchema`

```python
class ElicitationJsonSchema(GenerateJsonSchema)
```

**Bases** `GenerateJsonSchema`

**Declared members (3)**

- `def enum_schema(self, schema: core_schema.EnumSchema) -> JsonSchemaValue`
  Generate inline enum schema.
- `def generate_inner(self, schema: core_schema.CoreSchema) -> JsonSchemaValue`
  Override to prevent ref generation for enums and handle list schemas.
- `def list_schema(self, schema: core_schema.ListSchema) -> JsonSchemaValue`
  Generate schema for list types, detecting enum items for multi-select.

Custom JSON schema generator for MCP elicitation that always inlines enums.

MCP elicitation requires inline enum schemas without $ref/$defs references.
This generator ensures enums are always generated inline for compatibility.
Optionally adds enumNames for better UI display when available.


## ScalarElicitationType

`fastmcp.server.elicitation.ScalarElicitationType`

```python
class ScalarElicitationType(Generic[T])
```

**Bases** `Generic[T]`

**Declared members (1)**

- `value: T`  _instance-attribute_

## _apply_value_metadata

`fastmcp.server.elicitation._apply_value_metadata`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _apply_value_metadata(schema: dict[str, Any], title: str | None, description: str | None) -> None
```

Override title/description on the wrapped ``value`` property in-place.


## _dict_to_enum_schema

`fastmcp.server.elicitation._dict_to_enum_schema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _dict_to_enum_schema(enum_dict: dict[str, dict[str, str]], multi_select: bool = False) -> dict[str, Any]
```

Convert dict enum to SEP-1330 compliant schema pattern.

Args:
    enum_dict: {"low": {"title": "Low Priority"}, "medium": {"title": "Medium Priority"}}
    multi_select: If True, use anyOf pattern; if False, use oneOf pattern

Returns:
    {"type": "string", "oneOf": [...]} for single-select
    {"anyOf": [...]} for multi-select (used as array items)


## _is_scalar_type

`fastmcp.server.elicitation._is_scalar_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_scalar_type(response_type: Any) -> bool
```

Check if response_type is a scalar type that needs wrapping.


## _parse_dict_syntax

`fastmcp.server.elicitation._parse_dict_syntax`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_dict_syntax(d: dict[str, Any]) -> ElicitConfig
```

Parse dict syntax: {"low": {"title": "..."}} -> single-select titled.


## _parse_generic_list

`fastmcp.server.elicitation._parse_generic_list`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_generic_list(response_type: Any) -> ElicitConfig
```

Parse list[X] type annotation -> multi-select.


## _parse_list_syntax

`fastmcp.server.elicitation._parse_list_syntax`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_list_syntax(lst: list[Any]) -> ElicitConfig
```

Parse list patterns: [[...]], [{...}], or [...].


## _parse_scalar_type

`fastmcp.server.elicitation._parse_scalar_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_scalar_type(response_type: Any) -> ElicitConfig
```

Parse scalar types (bool, int, float, str, Literal, Enum).


## get_elicitation_schema

`fastmcp.server.elicitation.get_elicitation_schema`

```python
def get_elicitation_schema(response_type: type[T]) -> dict[str, Any]
```

Get the schema for an elicitation response.

Args:
    response_type: The type of the response


## handle_elicit_accept

Import as `fastmcp.server.context.handle_elicit_accept`  ·  defined at `fastmcp.server.elicitation.handle_elicit_accept`

```python
def handle_elicit_accept(config: ElicitConfig, content: Any) -> AcceptedElicitation[Any]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Handle an accepted elicitation response.

Args:
    config: The elicitation configuration from parse_elicit_response_type
    content: The response content from the client

Returns:
    AcceptedElicitation with the extracted/validated data


## parse_elicit_response_type

Import as `fastmcp.server.context.parse_elicit_response_type`  ·  defined at `fastmcp.server.elicitation.parse_elicit_response_type`

```python
def parse_elicit_response_type(response_type: Any, response_title: str | None = None, response_description: str | None = None) -> ElicitConfig
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Parse response_type into schema and handling configuration.

A response type is required; ``None`` raises ``TypeError``. Supports
multiple syntaxes:
- dict: `{"low": {"title": "..."}}` -> single-select titled enum
- list patterns:
    - `[["a", "b"]]` -> multi-select untitled
    - `[{"low": {...}}]` -> multi-select titled
    - `["a", "b"]` -> single-select untitled
- `list[X]` type annotation: multi-select with type
- Scalar types (bool, int, float, str, Literal, Enum): single value
- Other types (dataclass, BaseModel): use directly

The ``response_title`` and ``response_description`` arguments customize the
label and description of the wrapped ``value`` property for the scalar/dict/list
shorthand forms. They are only valid when FastMCP is wrapping the response
type; passing them with a full BaseModel/dataclass raises ``TypeError``,
because in those cases the user already controls field metadata via
``Field(title=..., description=...)``.


## validate_elicitation_json_schema

`fastmcp.server.elicitation.validate_elicitation_json_schema`

```python
def validate_elicitation_json_schema(schema: dict[str, Any]) -> None
```

Validate that a JSON schema follows MCP elicitation requirements.

This ensures the schema is compatible with MCP elicitation requirements:
- Must be an object schema
- Must only contain primitive field types (string, number, integer, boolean)
- Must be flat (no nested objects or arrays of objects)
- Allows const fields (for Literal types) and enum fields (for Enum types)
- Only primitive types and their nullable variants are allowed

Args:
    schema: The JSON schema to validate

Raises:
    TypeError: If the schema doesn't meet MCP elicitation requirements


