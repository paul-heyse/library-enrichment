# `fastmcp.utilities.openapi.json_schema_converter`

Distribution: `fastmcp`

## OPENAPI_SPECIFIC_FIELDS

`fastmcp.utilities.openapi.json_schema_converter.OPENAPI_SPECIFIC_FIELDS`

```python
OPENAPI_SPECIFIC_FIELDS = {'nullable', 'discriminator', 'readOnly', 'writeOnly', 'xml', 'externalDocs', 'deprecated'}
```

**Inferred type** (`ty`, not declared in the source): `set[str]`

## RECURSIVE_FIELDS

`fastmcp.utilities.openapi.json_schema_converter.RECURSIVE_FIELDS`

```python
RECURSIVE_FIELDS = {'properties': dict, '$defs': dict, '$definitions': dict, 'items': dict, 'additionalProperties': dict, 'allOf': list, 'anyOf': list, 'oneOf': list, 'not': dict}
```

**Inferred type** (`ty`, not declared in the source): `dict[str, <class 'dict'> | <class 'list'>]`

## logger

`fastmcp.utilities.openapi.json_schema_converter.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _convert_nullable_field

`fastmcp.utilities.openapi.json_schema_converter._convert_nullable_field`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _convert_nullable_field(schema: dict[str, Any]) -> dict[str, Any]
```

Convert OpenAPI nullable field to JSON Schema type array.


## _filter_properties_by_access

`fastmcp.utilities.openapi.json_schema_converter._filter_properties_by_access`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _filter_properties_by_access(schema: dict[str, Any], remove_read_only: bool, remove_write_only: bool) -> dict[str, Any]
```

Remove readOnly and/or writeOnly properties from schema.


## _has_read_only_properties

`fastmcp.utilities.openapi.json_schema_converter._has_read_only_properties`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_read_only_properties(schema: dict[str, Any]) -> bool
```

Quick check if schema has any readOnly properties.


## _has_write_only_properties

`fastmcp.utilities.openapi.json_schema_converter._has_write_only_properties`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_write_only_properties(schema: dict[str, Any]) -> bool
```

Quick check if schema has any writeOnly properties.


## _needs_recursive_processing

`fastmcp.utilities.openapi.json_schema_converter._needs_recursive_processing`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _needs_recursive_processing(schema: dict[str, Any], openapi_version: str | None, remove_read_only: bool, remove_write_only: bool, convert_one_of_to_any_of: bool) -> bool
```

Check if the schema needs recursive processing (smarter than just checking for recursive fields).


## convert_openapi_schema_to_json_schema

Import as `fastmcp.utilities.openapi.convert_openapi_schema_to_json_schema`  ·  defined at `fastmcp.utilities.openapi.json_schema_converter.convert_openapi_schema_to_json_schema`

```python
def convert_openapi_schema_to_json_schema(schema: dict[str, Any], openapi_version: str | None = None, remove_read_only: bool = False, remove_write_only: bool = False, convert_one_of_to_any_of: bool = True) -> dict[str, Any]
```

**Also exported as** `fastmcp.utilities.openapi.convert_openapi_schema_to_json_schema`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Convert an OpenAPI schema to JSON Schema format.

This is a clean, systematic approach that:
1. Removes OpenAPI-specific fields
2. Converts nullable fields to type arrays (for OpenAPI 3.0 only)
3. Converts oneOf to anyOf for overlapping union handling
4. Recursively processes nested schemas
5. Optionally removes readOnly/writeOnly properties

Args:
    schema: OpenAPI schema dictionary
    openapi_version: OpenAPI version for optimization
    remove_read_only: Whether to remove readOnly properties
    remove_write_only: Whether to remove writeOnly properties
    convert_one_of_to_any_of: Whether to convert oneOf to anyOf

Returns:
    JSON Schema-compatible dictionary


## convert_schema_definitions

Import as `fastmcp.utilities.openapi.convert_schema_definitions`  ·  defined at `fastmcp.utilities.openapi.json_schema_converter.convert_schema_definitions`

```python
def convert_schema_definitions(schema_definitions: dict[str, Any] | None, openapi_version: str | None = None, kwargs = {}) -> dict[str, Any]
```

**Also exported as** `fastmcp.utilities.openapi.convert_schema_definitions`

Convert a dictionary of OpenAPI schema definitions to JSON Schema.

Args:
    schema_definitions: Dictionary of schema definitions
    openapi_version: OpenAPI version for optimization
    **kwargs: Additional arguments passed to convert_openapi_schema_to_json_schema

Returns:
    Dictionary of converted schema definitions


