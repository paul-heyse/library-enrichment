# `mcp.server.mcpserver.utilities.func_metadata`

Distribution: `mcp`

## _CONTENT_SEQUENCE_ORIGINS

`mcp.server.mcpserver.utilities.func_metadata._CONTENT_SEQUENCE_ORIGINS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CONTENT_SEQUENCE_ORIGINS = (list, tuple, Sequence)
```

## _CONTENT_TYPES

`mcp.server.mcpserver.utilities.func_metadata._CONTENT_TYPES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CONTENT_TYPES = (*get_args(ContentBlock), Image, Audio)
```

## _LOCAL_DEFS_PREFIX

`mcp.server.mcpserver.utilities.func_metadata._LOCAL_DEFS_PREFIX`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_LOCAL_DEFS_PREFIX = '#/$defs/'
```

## _no_default

`mcp.server.mcpserver.utilities.func_metadata._no_default`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_no_default = object()
```

## logger

`mcp.server.mcpserver.utilities.func_metadata.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ArgModelBase

`mcp.server.mcpserver.utilities.func_metadata.ArgModelBase`

```python
class ArgModelBase(BaseModel)
```

**Bases** `BaseModel`

**Declared members (1)**

- `def model_dump_one_level(self) -> dict[str, Any]`
  Return a dict of the model's fields, one level deep.

A model representing the arguments to a function.


## FuncMetadata

Import as `mcp.server.mcpserver.tools.base.FuncMetadata`  ·  defined at `mcp.server.mcpserver.utilities.func_metadata.FuncMetadata`

```python
class FuncMetadata(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (9)**

- `arg_model: Annotated[type[ArgModelBase], WithJsonSchema(None)]`  _instance-attribute_
- `async def call_fn(self, fn: Callable[..., Any | Awaitable[Any]], fn_is_async: bool, arguments: dict[str, Any], arguments_to_pass_directly: dict[str, Any] | None = None) -> Any`  _async_
  Call the function with already-validated `arguments` plus `arguments_to_pass_directly`.
- `async def call_fn_with_arg_validation(self, fn: Callable[..., Any | Awaitable[Any]], fn_is_async: bool, arguments_to_validate: dict[str, Any], arguments_to_pass_directly: dict[str, Any] | None, pre_validated: dict[str, Any] | None = None) -> Any`  _async_
  Validate `arguments_to_validate` (unless `pre_validated` is given) and call the function.
- `def convert_result(self, result: Any) -> CallToolResult | InputRequiredResult`
  Convert a function call result into a `CallToolResult`.
- `output_model: Annotated[Any, WithJsonSchema(None)] = None`  _class-attribute, instance-attribute_
- `output_schema: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `def pre_parse_json(self, data: dict[str, Any]) -> dict[str, Any]`
  Pre-parse data from JSON.
- `def validate_arguments(self, arguments_to_validate: dict[str, Any]) -> dict[str, Any]`
  Validate raw arguments into a one-level kwargs dict (no function call).
- `wrap_output: bool = False`  _class-attribute, instance-attribute_

A tool function's argument model plus, for structured output, the published `output_schema` and the
`output_model` type annotation results are validated against. Constructing one with an `output_model` and no
schema derives the schema (and raises if pydantic can't); the fields are read live, so reassigning them later
takes effect on the next call.


## StrictJsonSchema

`mcp.server.mcpserver.utilities.func_metadata.StrictJsonSchema`

```python
class StrictJsonSchema(GenerateJsonSchema)
```

**Bases** `GenerateJsonSchema`

**Declared members (1)**

- `def emit_warning(self, kind: JsonSchemaWarningKind, detail: str) -> None`

A JSON schema generator that raises exceptions instead of emitting warnings.

This is used to detect non-serializable types during schema generation.


## _as_typing_extensions_typeddict

`mcp.server.mcpserver.utilities.func_metadata._as_typing_extensions_typeddict`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _as_typing_extensions_typeddict(td_type: type[Any]) -> type[Any]
```

## _convert_to_content

`mcp.server.mcpserver.utilities.func_metadata._convert_to_content`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _convert_to_content(result: Any) -> list[ContentBlock]
```

Convert a result to a sequence of content objects.

Note: This conversion logic comes from previous versions of MCPServer and is being
retained for purposes of backwards compatibility. It produces different unstructured
output than the lowlevel server tool call handler, which just serializes structured
content verbatim. `_returns_content` is the annotation-level mirror of these branches.


## _create_model_from_class

`mcp.server.mcpserver.utilities.func_metadata._create_model_from_class`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_model_from_class(cls: type[Any], type_hints: dict[str, Any]) -> type[BaseModel]
```

Create a Pydantic model from an ordinary class.

The created model will:
- Have the same name as the class
- Have fields with the same names and types as the class's fields
- Include all fields whose type does not include None in the set of required fields

Precondition: cls must have type hints (i.e., `type_hints` is non-empty)


## _create_output_model

`mcp.server.mcpserver.utilities.func_metadata._create_output_model`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_output_model(original_annotation: Any, type_expr: Any, func_name: str) -> tuple[Any, bool]
```

Pick the type structured output is validated against for the given return annotation.

Args:
    original_annotation: The original return annotation (may be wrapped in `Annotated`).
    type_expr: The underlying type expression derived from the return annotation
        (`Annotated` and type qualifiers were stripped).
    func_name: The name of the function.

Returns:
    tuple of (model or None, wrap_output)
    Model is None if the type cannot carry structured output.
    wrap_output is True if the result needs to be wrapped in {"result": ...}


## _create_wrapped_model

`mcp.server.mcpserver.utilities.func_metadata._create_wrapped_model`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_wrapped_model(func_name: str, annotation: Any) -> type[BaseModel]
```

Create a model that wraps a type in a 'result' field.

This is used for primitive types, generic types like list/dict, etc.


## _inline_root_ref

`mcp.server.mcpserver.utilities.func_metadata._inline_root_ref`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _inline_root_ref(schema: dict[str, Any]) -> dict[str, Any]
```

Give a schema whose root is a bare `$ref` into `$defs` an inline root.

pydantic emits a self-referential model as `{"$defs": {...}, "$ref": "#/$defs/Model"}`, with no
`type` at the root; `Tool.outputSchema` needs an object root (required on the wire through
2025-11-25). The referenced definition is copied onto the root and `$defs` is kept, since nested
references still point into it. Root siblings of the `$ref` win over the definition's keys.


## _is_input_required_type

`mcp.server.mcpserver.utilities.func_metadata._is_input_required_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_input_required_type(obj: Any) -> bool
```

## _pydantic_readable_typeddict

`mcp.server.mcpserver.utilities.func_metadata._pydantic_readable_typeddict`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _pydantic_readable_typeddict(output_model: type[Any]) -> type[Any]
```

pydantic refuses `typing.TypedDict` below Python 3.12 (it needs `__orig_bases__`); rebuild such a return
type as an equivalent `typing_extensions.TypedDict` so tool authors don't have to know. Only the class itself
(its keys, docstring and own config) is rebuilt: stdlib TypedDicts nested inside it, or config inherited from
one, still need `typing_extensions` there. Delete with 3.11 support.


## _returns_content

`mcp.server.mcpserver.utilities.func_metadata._returns_content`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _returns_content(annotation: Any) -> bool
```

Whether a return annotation declares content blocks or the `Image`/`Audio` helpers, bare or as
the items of a list/tuple or the arms of a union: the values `_convert_to_content` renders as blocks
rather than dumping as data. Keep the two in sync.


## func_metadata

Import as `mcp.server.mcpserver.tools.base.func_metadata`  ·  defined at `mcp.server.mcpserver.utilities.func_metadata.func_metadata`

```python
def func_metadata(func: Callable[..., Any], skip_names: Sequence[str] = (), structured_output: bool | None = None) -> FuncMetadata
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Given a function, return metadata including a Pydantic model representing its signature.

The use case for this is
```
meta = func_metadata(func)
validated_args = meta.arg_model.model_validate(some_raw_data_dict)
return func(**validated_args.model_dump_one_level())
```

**critically** it also provides a pre-parse helper to attempt to parse things from
JSON.

Args:
    func: The function to convert to a Pydantic model
    skip_names: A list of parameter names to skip. These will not be included in
        the model.
    structured_output: Controls whether the tool's output is structured or unstructured
        - If None, auto-detects based on the function's return type annotation
        - If True, creates a structured tool (return type annotation permitting)
        - If False, unconditionally creates an unstructured tool

        If structured, creates a Pydantic model for the function's result based on its annotation.
        Supports various return types:
        - BaseModel subclasses (used directly)
        - Primitive types (str, int, float, bool, bytes, None) - wrapped in a
            model with a 'result' field
        - TypedDict - used directly
        - Dataclasses and other annotated classes - converted to Pydantic models
        - Generic types (list, dict, Union, etc.) - wrapped in a model with a 'result' field
        - Content blocks (TextContent, EmbeddedResource, ...), Image and Audio, bare or inside a
            list, tuple or union - unstructured when auto-detecting; structured_output=True bypasses
            this rule (a content block then publishes its own schema; Image/Audio have none and raise)

Returns:
    A FuncMetadata object containing:
    - arg_model: A Pydantic model representing the function's arguments
    - output_schema: The published JSON schema for structured output, or None if the output is unstructured
    - output_model: The type structured output is validated against: the declared BaseModel or TypedDict,
        or a synthesized model for wrapped, `dict[str, T]` and annotated-class returns
    - wrap_output: Whether the function result needs to be wrapped in `{"result": ...}` for structured output.


