# `fastmcp.utilities.json_schema_type`

Distribution: `fastmcp`

## FORMAT_TYPES

`fastmcp.utilities.json_schema_type.FORMAT_TYPES`

```python
FORMAT_TYPES: dict[str, Any] = {'date-time': datetime, 'email': EmailStr, 'uri': AnyUrl, 'json': Json}
```

## _UnsatisfiableType

`fastmcp.utilities.json_schema_type._UnsatisfiableType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_UnsatisfiableType = Annotated[Any, BeforeValidator(_reject_all)]
```

## __all__

`fastmcp.utilities.json_schema_type.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['JSONSchema', 'json_schema_to_type']
```

## _classes

`fastmcp.utilities.json_schema_type._classes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_classes: dict[tuple[str, Any], type | None] = {}
```

## JSONSchema

`fastmcp.utilities.json_schema_type.JSONSchema`

```python
class JSONSchema(TypedDict)
```

**Bases** `TypedDict`

**Declared members (30)**

- `additionalItems: NotRequired[bool | JSONSchema]`  _instance-attribute_
- `additionalProperties: NotRequired[bool | JSONSchema]`  _instance-attribute_
- `allOf: NotRequired[list[JSONSchema]]`  _instance-attribute_
- `anyOf: NotRequired[list[JSONSchema]]`  _instance-attribute_
- `const: NotRequired[Any]`  _instance-attribute_
- `default: NotRequired[Any]`  _instance-attribute_
- `definitions: NotRequired[dict[str, JSONSchema]]`  _instance-attribute_
- `dependencies: NotRequired[dict[str, JSONSchema | list[str]]]`  _instance-attribute_
- `description: NotRequired[str]`  _instance-attribute_
- `enum: NotRequired[list[Any]]`  _instance-attribute_
- `examples: NotRequired[list[Any]]`  _instance-attribute_
- `exclusiveMaximum: NotRequired[int | float]`  _instance-attribute_
- `exclusiveMinimum: NotRequired[int | float]`  _instance-attribute_
- `format: NotRequired[str]`  _instance-attribute_
- `items: NotRequired[JSONSchema | list[JSONSchema]]`  _instance-attribute_
- `maxItems: NotRequired[int]`  _instance-attribute_
- `maxLength: NotRequired[int]`  _instance-attribute_
- `maximum: NotRequired[int | float]`  _instance-attribute_
- `minItems: NotRequired[int]`  _instance-attribute_
- `minLength: NotRequired[int]`  _instance-attribute_
- `minimum: NotRequired[int | float]`  _instance-attribute_
- `multipleOf: NotRequired[int | float]`  _instance-attribute_
- `not_: NotRequired[JSONSchema]`  _instance-attribute_
- `oneOf: NotRequired[list[JSONSchema]]`  _instance-attribute_
- `pattern: NotRequired[str]`  _instance-attribute_
- `properties: NotRequired[dict[str, JSONSchema]]`  _instance-attribute_
- `required: NotRequired[list[str]]`  _instance-attribute_
- `title: NotRequired[str]`  _instance-attribute_
- `type: NotRequired[str | list[str]]`  _instance-attribute_
- `uniqueItems: NotRequired[bool]`  _instance-attribute_

## _create_array_type

`fastmcp.utilities.json_schema_type._create_array_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_array_type(schema: Mapping[str, Any], schemas: Mapping[str, Any], resolving_refs: frozenset[str]) -> type | Annotated[Any, ...]
```

Create list/set type with optional constraints.


## _create_dataclass

`fastmcp.utilities.json_schema_type._create_dataclass`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_dataclass(schema: Mapping[str, Any], name: str | None = None, schemas: Mapping[str, Any] | None = None, resolving_refs: frozenset[str] = frozenset()) -> type
```

Create dataclass from object schema.


## _create_enum

`fastmcp.utilities.json_schema_type._create_enum`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_enum(name: str, values: list[Any]) -> type
```

Create enum type from list of values.


## _create_field_with_default

`fastmcp.utilities.json_schema_type._create_field_with_default`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_field_with_default(field_type: type, default_value: Any, schema: dict[str, Any]) -> Any
```

Create a field with simplified default handling.


## _create_numeric_type

`fastmcp.utilities.json_schema_type._create_numeric_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_numeric_type(base: type[int | float], schema: Mapping[str, Any]) -> type | Annotated[Any, ...]
```

Create numeric type with optional constraints.


## _create_pydantic_model

`fastmcp.utilities.json_schema_type._create_pydantic_model`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_pydantic_model(schema: Mapping[str, Any], name: str | None = None, schemas: Mapping[str, Any] | None = None, resolving_refs: frozenset[str] = frozenset()) -> type
```

Create Pydantic BaseModel from object schema with additionalProperties.


## _create_string_type

`fastmcp.utilities.json_schema_type._create_string_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_string_type(schema: Mapping[str, Any]) -> type | Annotated[Any, ...]
```

Create string type with optional constraints.


## _get_default_value

`fastmcp.utilities.json_schema_type._get_default_value`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_default_value(schema: dict[str, Any], prop_name: str, parent_default: dict[str, Any] | None = None) -> Any
```

Get default value with proper priority ordering.
1. Value from parent's default if it exists
2. Property's own default if it exists
3. None


## _get_from_type_handler

`fastmcp.utilities.json_schema_type._get_from_type_handler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_from_type_handler(schema: Mapping[str, Any], schemas: Mapping[str, Any], resolving_refs: frozenset[str]) -> Callable[..., Any]
```

Get the appropriate type handler for the schema.


## _hash_schema

`fastmcp.utilities.json_schema_type._hash_schema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _hash_schema(schema: Mapping[str, Any]) -> str
```

Generate a deterministic hash for schema caching.

Handles non-JSON-native types (datetime, date, bool keys) that can
appear in schemas loaded from YAML, which auto-parses date strings.
Uses ``default=str`` for unserializable values and drops ``sort_keys``
to avoid ``TypeError`` when dicts mix ``bool`` and ``str`` keys.


## _merge_defaults

`fastmcp.utilities.json_schema_type._merge_defaults`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _merge_defaults(data: Mapping[str, Any], schema: Mapping[str, Any], parent_default: Mapping[str, Any] | None = None) -> dict[str, Any]
```

Merge defaults with provided data at all levels.


## _normalize_yaml_types

`fastmcp.utilities.json_schema_type._normalize_yaml_types`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_yaml_types(obj: Any) -> Any
```

Convert YAML-parsed types back to JSON-native types.

``yaml.safe_load`` converts ISO date-time strings to ``datetime``/``date``
objects.  These crash ``json.dumps`` and produce wrong default values in
dataclass fields.  This function recursively normalises them to strings.


## _object_schema_to_type

`fastmcp.utilities.json_schema_type._object_schema_to_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _object_schema_to_type(schema: Mapping[str, Any], schemas: Mapping[str, Any], name: str | None = None, resolving_refs: frozenset[str] = frozenset()) -> type
```

Convert an object schema to the appropriate Python type.

Single source of truth for the four object-schema cases, used by both the
top-level ``json_schema_to_type`` entry point and the recursive
``_schema_to_type`` path:

1. No ``properties`` with ``additionalProperties`` truthy — ``dict[str, T]``
   (``T = Any`` when ``additionalProperties is True``, else the value schema's type)
2. No ``properties`` and no ``additionalProperties`` — ``dict[str, Any]``
3. Has ``properties`` and ``additionalProperties is True`` — Pydantic model
   (so ``extra="allow"`` can preserve unknown keys)
4. Has ``properties`` otherwise — dataclass

``name`` is used as the generated class name for cases 3 and 4; it falls
back to the schema's ``title`` when not provided.


## _reject_all

`fastmcp.utilities.json_schema_type._reject_all`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _reject_all(v: Any) -> Any
```

Validator that rejects every value, implementing JSON Schema `false`.


## _resolve_ref

`fastmcp.utilities.json_schema_type._resolve_ref`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolve_ref(ref: str, schemas: Mapping[str, Any]) -> Mapping[str, Any]
```

Resolve JSON Schema reference to target schema.


## _return_Any

`fastmcp.utilities.json_schema_type._return_Any`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _return_Any() -> Any
```

## _sanitize_name

`fastmcp.utilities.json_schema_type._sanitize_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _sanitize_name(name: str) -> str
```

Convert string to valid Python identifier.


## _schema_to_type

`fastmcp.utilities.json_schema_type._schema_to_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _schema_to_type(schema: Mapping[str, Any] | bool, schemas: Mapping[str, Any], resolving_refs: frozenset[str] = frozenset()) -> type | ForwardRef
```

Convert schema to appropriate Python type.


## json_schema_to_type

Import as `fastmcp.client.elicitation.json_schema_to_type`  ·  defined at `fastmcp.utilities.json_schema_type.json_schema_to_type`

```python
def json_schema_to_type(schema: Mapping[str, Any] | bool, name: str | None = None) -> type
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Convert JSON schema to appropriate Python type with validation.

Args:
    schema: A JSON Schema dictionary defining the type structure and validation rules.
        Boolean schemas are also accepted (``True`` = any type, ``False`` = unsatisfiable).
    name: Optional name for object schemas. Only allowed when schema type is "object".
        If not provided for objects, name will be inferred from schema's "title"
        property or default to "Root".

Returns:
    A Python type (typically a dataclass for objects) with Pydantic validation

Raises:
    ValueError: If a name is provided for a non-object schema

Examples:
    Create a dataclass from an object schema:
    ```python
    schema = {
        "type": "object",
        "title": "Person",
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "age": {"type": "integer", "minimum": 0},
            "email": {"type": "string", "format": "email"}
        },
        "required": ["name", "age"]
    }

    Person = json_schema_to_type(schema)
    # Creates a dataclass with name, age, and optional email fields:
    # @dataclass
    # class Person:
    #     name: str
    #     age: int
    #     email: str | None = None
    ```
    Person(name="John", age=30)

    Create a scalar type with constraints:
    ```python
    schema = {
        "type": "string",
        "minLength": 3,
        "pattern": "^[A-Z][a-z]+$"
    }

    NameType = json_schema_to_type(schema)
    # Creates Annotated[str, StringConstraints(min_length=3, pattern="^[A-Z][a-z]+$")]

    @dataclass
    class Name:
        name: NameType
    ```


