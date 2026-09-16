# `fastmcp.tools.function_parsing`

Distribution: `fastmcp`

## T

`fastmcp.tools.function_parsing.T`

```python
T = TypeVarExt('T', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _UNCONSTRAINED_SEQUENCE_ORIGINS

`fastmcp.tools.function_parsing._UNCONSTRAINED_SEQUENCE_ORIGINS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_UNCONSTRAINED_SEQUENCE_ORIGINS = (list, tuple, Sequence)
```

## logger

`fastmcp.tools.function_parsing.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ParsedFunction

Import as `fastmcp.tools.function_tool.ParsedFunction`  ·  defined at `fastmcp.tools.function_parsing.ParsedFunction`

```python
class ParsedFunction
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (7)**

- `description: str | None`  _instance-attribute_
- `fn: Callable[..., Any]`  _instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], validate: bool = True, wrap_non_object_output_schema: bool = True) -> ParsedFunction`  _classmethod_
- `input_schema: dict[str, Any]`  _instance-attribute_
- `name: str`  _instance-attribute_
- `output_schema: dict[str, Any] | None`  _instance-attribute_
- `return_type: Any = None`  _class-attribute, instance-attribute_

## _ToolOutputSchemaGenerator

`fastmcp.tools.function_parsing._ToolOutputSchemaGenerator`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ToolOutputSchemaGenerator(GenerateJsonSchema)
```

**Bases** `GenerateJsonSchema`

**Declared members (2)**

- `def dataclass_schema(self, schema: core_schema.DataclassSchema) -> JsonSchemaValue`
- `def model_schema(self, schema: core_schema.ModelSchema) -> JsonSchemaValue`

Generate each model's schema with its configured serialization aliases.

Pydantic's serializer consults ``serialize_by_alias`` per model, while its
JSON Schema API otherwise applies one ``by_alias`` value to the whole tree.


## _UnserializableType

`fastmcp.tools.function_parsing._UnserializableType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _UnserializableType
```

## _WrappedResult

`fastmcp.tools.function_parsing._WrappedResult`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _WrappedResult(Generic[T])
```

**Bases** `Generic[T]`

**Declared members (1)**

- `result: T`  _instance-attribute_

## _contains_bytes_type

`fastmcp.tools.function_parsing._contains_bytes_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _contains_bytes_type(tp: Any) -> bool
```

Check if *tp* is or contains bytes, recursing through unions and Annotated.


## _contains_input_required

`fastmcp.tools.function_parsing._contains_input_required`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _contains_input_required(tp: Any) -> bool
```

True when `InputRequiredResult` appears anywhere in *tp*.

Recurses through `TypeAliasType`, `Annotated`, and unions so a guard arm is
found even when factored through a composed alias (``str | Value`` where
``Value = int | InputRequiredResult``).


## _contains_prefab_type

`fastmcp.tools.function_parsing._contains_prefab_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _contains_prefab_type(tp: Any) -> bool
```

Check if *tp* is or contains a prefab type, recursing through unions and Annotated.


## _is_input_required_type

`fastmcp.tools.function_parsing._is_input_required_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_input_required_type(tp: Any) -> bool
```

True when *tp* is the `InputRequiredResult` type (SEP-2322).

Resolves a `TypeAliasType` and peels an `Annotated` wrapper first, so an
aliased arm or a metadata-carrying arm such as
``Annotated[InputRequiredResult, Field(...)]`` is recognized as a guard
signal, not just the bare class.


## _is_object_schema

`fastmcp.tools.function_parsing._is_object_schema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_object_schema(schema: dict[str, Any], _root_schema: dict[str, Any] | None = None, _seen_refs: set[str] | None = None) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if a JSON schema represents an object type.


## _is_unconstrained_sequence

`fastmcp.tools.function_parsing._is_unconstrained_sequence`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_unconstrained_sequence(tp: Any) -> bool
```

A sequence annotation that says nothing about its items.

Bare `list`, `tuple`, `Sequence` (and their `typing` spellings), plus
`list[Any]`, `Sequence[Any]` and the variadic `tuple[Any, ...]`. The schema
these produce -- ``{"result": {"items": {}, "type": "array"}}`` -- constrains
nothing, which is why `Any` itself is already excluded from inference. A
fixed-length `tuple[Any, Any]` still pins its length, so it keeps its schema.


## _residual_union_arms

`fastmcp.tools.function_parsing._residual_union_arms`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _residual_union_arms(tp: Any) -> list[Any]
```

Flatten a (possibly aliased/nested) union into its non-guard arms.

Every `InputRequiredResult` arm is dropped at any depth, and aliased union
arms are flattened inline so the residual is a flat union of data arms.


## _strip_input_required

`fastmcp.tools.function_parsing._strip_input_required`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _strip_input_required(tp: Any) -> Any
```

Remove `InputRequiredResult` arms from a union return annotation.

A guard tool typically annotates its return as ``X | InputRequiredResult``;
the ``InputRequiredResult`` arm is a suspend signal, not output data, so it
is dropped before schema derivation. Stripping recurses through
`TypeAliasType` and nested unions, so a guard arm factored through an alias
(even ``str | Value`` where ``Value = int | InputRequiredResult``) is still
removed. A non-union annotation, or one with no such arm, is returned
unchanged. A bare ``InputRequiredResult`` annotation (no other arm) is left
intact and suppressed downstream like other non-serializable return types.


## _unwrap_type_alias

`fastmcp.tools.function_parsing._unwrap_type_alias`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _unwrap_type_alias(tp: Any) -> Any
```

Resolve a PEP 695 ``type X = ...`` alias to its underlying value.

``get_origin()`` returns ``None`` for a ``TypeAliasType``, so an alias that
factors out a guard union (``type Result = str | InputRequiredResult``) — or
a lone aliased arm — would otherwise slip past union detection. Resolving to
``__value__`` (repeatedly, for chained aliases) restores the concrete type.


