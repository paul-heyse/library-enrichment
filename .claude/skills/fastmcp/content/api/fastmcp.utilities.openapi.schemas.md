# `fastmcp.utilities.openapi.schemas`

Distribution: `fastmcp`

## __all__

`fastmcp.utilities.openapi.schemas.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['_combine_schemas', '_combine_schemas_and_map_params', '_make_optional_parameter_nullable', 'clean_schema_for_display', 'extract_output_schema_from_responses']
```

## logger

`fastmcp.utilities.openapi.schemas.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _allof_members

`fastmcp.utilities.openapi.schemas._allof_members`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _allof_members(schema: dict[str, Any], schema_defs: dict[str, Any], resolving: set[str] | None = None) -> list[dict[str, Any]]
```

Expand local schema references while collecting ``allOf`` members.


## _combine_schemas

`fastmcp.utilities.openapi.schemas._combine_schemas`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _combine_schemas(route: HTTPRoute) -> dict[str, Any]
```

**Also exported as** `fastmcp.utilities.openapi._combine_schemas`

Combines parameter and request body schemas into a single schema.
Handles parameter name collisions by adding location suffixes.

This is a backward compatibility wrapper around _combine_schemas_and_map_params.

Args:
    route: HTTPRoute object

Returns:
    Combined schema dictionary


## _combine_schemas_and_map_params

`fastmcp.utilities.openapi.schemas._combine_schemas_and_map_params`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _combine_schemas_and_map_params(route: HTTPRoute, convert_refs: bool = True) -> tuple[dict[str, Any], dict[str, dict[str, str]]]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Combines parameter and request body schemas into a single schema.
Handles parameter name collisions by adding location suffixes.
Also returns parameter mapping for request director.

Args:
    route: HTTPRoute object

Returns:
    Tuple of (combined schema dictionary, parameter mapping)
    Parameter mapping format: {'flat_arg_name': {'location': 'path', 'openapi_name': 'id'}}


## _discriminator_target_name

`fastmcp.utilities.openapi.schemas._discriminator_target_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _discriminator_target_name(target: str) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resolve a ``discriminator.mapping`` value to a local schema name.

Mapping values hold "schema names or references", so a bare ``"Cat"`` means
the ``Cat`` component just as ``"#/components/schemas/Cat"`` does. Anything
else — a remote URL, a pointer outside the component schemas — has no local
definition to flatten.


## _flatten_discriminator_subtypes

`fastmcp.utilities.openapi.schemas._flatten_discriminator_subtypes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _flatten_discriminator_subtypes(schema: dict[str, Any], schema_defs: dict[str, Any]) -> dict[str, Any] | None
```

Flatten the subtypes named by an OpenAPI ``discriminator.mapping``.

A parent schema carrying a discriminator describes its children only
through ``mapping``, so the child fields are unreachable from the parent's
own ``properties``. Rather than emitting a branch per subtype, the fields
are merged in as optional and the variants are spelled out on the
discriminator property's description. Top-level ``oneOf`` is filled in
poorly by LLM tool-calling APIs, and the upstream API remains the real
validator either way: a field from the wrong variant is rejected there
rather than locally.

Only the mapping on *schema* itself is expanded. A subtype carrying its own
discriminator is left alone, which also keeps the parent/child reference
cycle from recursing.

Returns replacement ``properties`` for *schema*, or None when there is no
usable discriminator mapping to flatten.


## _make_optional_parameter_nullable

`fastmcp.utilities.openapi.schemas._make_optional_parameter_nullable`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_optional_parameter_nullable(schema: dict[str, Any]) -> dict[str, Any]
```

**Also exported as** `fastmcp.utilities.openapi._make_optional_parameter_nullable`

Make an optional parameter schema nullable to allow None values.

For optional parameters, we need to allow null values in addition to the
specified type to handle cases where None is passed for optional parameters.


## _replace_ref_with_defs

`fastmcp.utilities.openapi.schemas._replace_ref_with_defs`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _replace_ref_with_defs(info: dict[str, Any], description: str | None = None) -> dict[str, Any]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Replace openapi $ref with jsonschema $defs recursively.

Examples:
- {"type": "object", "properties": {"$ref": "#/components/schemas/..."}}
- {"type": "object", "additionalProperties": {"$ref": "#/components/schemas/..."}, "properties": {...}}
- {"$ref": "#/components/schemas/..."}
- {"items": {"$ref": "#/components/schemas/..."}}
- {"anyOf": [{"$ref": "#/components/schemas/..."}]}
- {"allOf": [{"$ref": "#/components/schemas/..."}]}
- {"oneOf": [{"$ref": "#/components/schemas/..."}]}

Args:
    info: dict[str, Any]
    description: str | None

Returns:
    dict[str, Any]


## clean_schema_for_display

Import as `fastmcp.utilities.openapi.clean_schema_for_display`  ·  defined at `fastmcp.utilities.openapi.schemas.clean_schema_for_display`

```python
def clean_schema_for_display(schema: JsonSchema | None) -> JsonSchema | None
```

**Also exported as** `fastmcp.utilities.openapi.clean_schema_for_display`

Clean up a schema dictionary for display by removing internal/complex fields.


## extract_output_schema_from_responses

Import as `fastmcp.utilities.openapi.extract_output_schema_from_responses`  ·  defined at `fastmcp.utilities.openapi.schemas.extract_output_schema_from_responses`

```python
def extract_output_schema_from_responses(responses: dict[str, ResponseInfo], schema_definitions: dict[str, Any] | None = None, openapi_version: str | None = None) -> dict[str, Any] | None
```

**Also exported as** `fastmcp.utilities.openapi.extract_output_schema_from_responses`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract output schema from OpenAPI responses for use as MCP tool output schema.

This function finds the first successful response (200, 201, 202, 204) with a
JSON-compatible content type and extracts its schema. If the schema is not an
object type, it wraps it to comply with MCP requirements.

Args:
    responses: Dictionary of ResponseInfo objects keyed by status code
    schema_definitions: Optional schema definitions to include in the output schema
    openapi_version: OpenAPI version string, used to optimize nullable field handling

Returns:
    dict: MCP-compliant output schema with potential wrapping, or None if no suitable schema found


