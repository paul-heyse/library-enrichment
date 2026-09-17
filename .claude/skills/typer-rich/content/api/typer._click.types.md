# `typer._click.types`

Distribution: `typer`

## BOOL

`typer._click.types.BOOL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
BOOL = BoolParamType()
```

## FLOAT

`typer._click.types.FLOAT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
FLOAT = FloatParamType()
```

## INT

`typer._click.types.INT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
INT = IntParamType()
```

## ParamTypeValue

`typer._click.types.ParamTypeValue`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ParamTypeValue = TypeVar('ParamTypeValue')
```

## STRING

`typer._click.types.STRING`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
STRING = StringParamType()
```

## UUID

`typer._click.types.UUID`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
UUID = UUIDParameterType()
```

## BoolParamType

`typer._click.types.BoolParamType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class BoolParamType(ParamType)
```

**Bases** `ParamType`

**Declared members (4)**

- `bool_states: dict[str, bool] = {'1': True, '0': False, 'yes': True, 'no': False, 'true': True, 'false': False, 'on': True, 'off': False, 't': True, 'f': False, 'y': True, 'n': False, '': False}`  _class-attribute, instance-attribute_
  A mapping of string values to boolean states.
- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> bool`
- `name = 'boolean'`  _class-attribute, instance-attribute_
- `def str_to_bool(value: str | bool) -> bool | None`  _staticmethod_
  Convert a string to a boolean value.

**Inherited (8)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## CompositeParamType

`typer._click.types.CompositeParamType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class CompositeParamType(ParamType)
```

**Bases** `ParamType`

**Declared members (2)**

- `arity: int`  _property_
- `is_composite = True`  _class-attribute, instance-attribute_

**Inherited (8)**

- from `typer._click.types.ParamType`: `convert`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `name`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## DateTime

`typer._click.types.DateTime`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class DateTime(ParamType)
```

**Bases** `ParamType`

**Declared members (4)**

- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> Any`
- `formats: Sequence[str] = formats or ['%Y-%m-%d', '%Y-%m-%dT%H:%M:%S', '%Y-%m-%d %H:%M:%S']`  _instance-attribute_
- `def get_metavar(self, param: Parameter, ctx: Context) -> str | None`
- `name = 'datetime'`  _class-attribute, instance-attribute_

**Inherited (7)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The DateTime type converts date strings into `datetime` objects.

The format strings which are checked are configurable, but default to some
common (non-timezone aware) ISO 8601 formats.

When specifying *DateTime* formats, you should only pass a list or a tuple.
Other iterables, like generators, may lead to surprising results.

The format strings are processed using ``datetime.strptime``, and this
consequently defines the format strings which are allowed.

Parsing is tried using each format, in order, and the first format which
parses successfully is used.


## File

`typer._click.types.File`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class File(ParamType)
```

**Bases** `ParamType`

**Declared members (10)**

- `atomic = atomic`  _instance-attribute_
- `def convert(self, value: str | os.PathLike[str] | IO[Any], param: Union[Parameter, None], ctx: Union[Context, None]) -> IO[Any]`
- `encoding = encoding`  _instance-attribute_
- `envvar_list_splitter: str = os.path.pathsep`  _class-attribute_
- `errors = errors`  _instance-attribute_
- `lazy = lazy`  _instance-attribute_
- `mode = mode`  _instance-attribute_
- `name = 'filename'`  _class-attribute, instance-attribute_
- `def resolve_lazy_flag(self, value: str | os.PathLike[str]) -> bool`
- `def shell_complete(self, ctx: Context, param: Parameter, incomplete: str) -> list[CompletionItem]`
  Return a special completion marker that tells the completion system to use the shell to provide file path completions.

**Inherited (6)**

- from `typer._click.types.ParamType`: `arity`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Declares a parameter to be a file for reading or writing.  The file
is automatically closed once the context tears down (after the command
finished working).

Files can be opened for reading or writing.  The special value ``-``
indicates stdin or stdout depending on the mode.

By default, the file is opened for reading text data, but it can also be
opened in binary mode or for writing.  The encoding parameter can be used
to force a specific encoding.

The `lazy` flag controls if the file should be opened immediately or upon
first IO. The default is to be non-lazy for standard input and output
streams as well as files opened for reading, `lazy` otherwise. When opening a
file lazily for reading, it is still opened temporarily for validation, but
will not be held open until first IO. lazy is mainly useful when opening
for writing to avoid creating the file until it is needed.

Files can also be opened atomically in which case all writes go into a
separate file in the same folder and upon completion the file will
be moved over to the original location.  This is useful if a file
regularly read by other users is modified.


## FloatParamType

`typer._click.types.FloatParamType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class FloatParamType(_NumberParamTypeBase)
```

**Bases** `_NumberParamTypeBase`

**Declared members (1)**

- `name = 'float'`  _class-attribute, instance-attribute_

**Inherited (9)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`
- from `typer._click.types._NumberParamTypeBase`: `convert`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## FloatRange

`typer._click.types.FloatRange`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class FloatRange(_NumberRangeBase, FloatParamType)
```

**Bases** `_NumberRangeBase`, `FloatParamType`

**Declared members (1)**

- `name = 'float range'`  _class-attribute, instance-attribute_

**Inherited (14)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`
- from `typer._click.types._NumberRangeBase`: `clamp`, `convert`, `max`, `max_open`, `min`, `min_open`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Restrict a `FLOAT` value to a range of accepted
values. See `ranges`.

If ``min`` or ``max`` are not passed, any value is accepted in that
direction. If ``min_open`` or ``max_open`` are enabled, the
corresponding boundary is not included in the range.

If ``clamp`` is enabled, a value outside the range is clamped to the
boundary instead of failing. This is not supported if either
boundary is marked ``open``.


## FuncParamType

`typer._click.types.FuncParamType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class FuncParamType(ParamType)
```

**Bases** `ParamType`

**Declared members (3)**

- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> Any`
- `func = func`  _instance-attribute_
- `name: str = getattr(func, '__name__', 'function')`  _instance-attribute_

**Inherited (8)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## IntParamType

`typer._click.types.IntParamType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class IntParamType(_NumberParamTypeBase)
```

**Bases** `_NumberParamTypeBase`

**Declared members (1)**

- `name = 'int'`  _class-attribute, instance-attribute_

**Inherited (9)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`
- from `typer._click.types._NumberParamTypeBase`: `convert`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## IntRange

`typer._click.types.IntRange`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class IntRange(_NumberRangeBase, IntParamType)
```

**Bases** `_NumberRangeBase`, `IntParamType`

**Declared members (1)**

- `name = 'int range'`  _class-attribute, instance-attribute_

**Inherited (14)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`
- from `typer._click.types._NumberRangeBase`: `clamp`, `convert`, `max`, `max_open`, `min`, `min_open`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Restrict an `INT` value to a range of accepted values. See

If ``min`` or ``max`` are not passed, any value is accepted in that
direction. If ``min_open`` or ``max_open`` are enabled, the
corresponding boundary is not included in the range.

If ``clamp`` is enabled, a value outside the range is clamped to the
boundary instead of failing.


## OptionHelpExtra

`typer._click.types.OptionHelpExtra`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class OptionHelpExtra(TypedDict)
```

**Bases** `TypedDict`

**Declared members (4)**

- `default: str`  _instance-attribute_
- `envvars: tuple[str, ...]`  _instance-attribute_
- `range: str`  _instance-attribute_
- `required: str`  _instance-attribute_

## ParamType

`typer._click.types.ParamType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ParamType
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (10)**

- `arity: int = 1`  _class-attribute_
- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> Any`
- `envvar_list_splitter: str | None = None`  _class-attribute_
- `def fail(self, message: str, param: Union[Parameter, None] = None, ctx: Union[Context, None] = None) -> NoReturn`
  Helper method to fail with an invalid value message.
- `def get_metavar(self, param: Parameter, ctx: Context) -> str | None`
  Returns the metavar default for this param if it provides one.
- `def get_missing_message(self, param: Parameter, ctx: Union[Context, None]) -> str | None`
  Optionally might return extra information about a missing parameter.
- `is_composite: bool = False`  _class-attribute_
- `name: str`  _instance-attribute_
- `def shell_complete(self, ctx: Context, param: Parameter, incomplete: str) -> list[CompletionItem]`
  Return a list of `CompletionItem` objects for the incomplete value. Most types do not provide completions, but some do, and this allows custom types to provide custom completions as well.
- `def split_envvar_value(self, rv: str) -> Sequence[str]`
  Given a value from an environment variable this splits it up into small chunks depending on the defined envvar list splitter.

Represents the type of a parameter. Validates and converts values
from the command line or Python into the correct type.

To implement a custom type, subclass and implement at least the
following:

-   The `name` class attribute must be set.
-   Calling an instance of the type with ``None`` must return
    ``None``. This is already implemented by default.
-   `convert` must convert string values to the correct type.
-   `convert` must accept values that are already the correct
    type.
-   It must be able to convert a value if the ``ctx`` and ``param``
    arguments are ``None``. This can occur when converting prompt
    input.


## StringParamType

`typer._click.types.StringParamType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class StringParamType(ParamType)
```

**Bases** `ParamType`

**Declared members (2)**

- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> Any`
- `name = 'str'`  _class-attribute, instance-attribute_

**Inherited (8)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## Tuple

`typer._click.types.Tuple`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Tuple(CompositeParamType)
```

**Bases** `CompositeParamType`

**Declared members (4)**

- `arity: int`  _property_
- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> Any`
- `name: str`  _property_
- `types: Sequence[ParamType] = [convert_type(ty) for ty in types]`  _instance-attribute_

**Inherited (7)**

- from `typer._click.types.CompositeParamType`: `is_composite`
- from `typer._click.types.ParamType`: `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The default behavior of Click is to apply a type on a value directly.
This works well in most cases, except for when `nargs` is set to a fixed
count and different types should be used for different items.  In this
case the `Tuple` type can be used.  This type can only be used
if `nargs` is set to a fixed number.

For more information see `tuple-type`.

This can be selected by using a Python tuple literal as a type.


## UUIDParameterType

`typer._click.types.UUIDParameterType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class UUIDParameterType(ParamType)
```

**Bases** `ParamType`

**Declared members (2)**

- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> Any`
- `name = 'uuid'`  _class-attribute, instance-attribute_

**Inherited (8)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## _NumberParamTypeBase

`typer._click.types._NumberParamTypeBase`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _NumberParamTypeBase(ParamType)
```

**Bases** `ParamType`

**Declared members (1)**

- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> Any`

**Inherited (9)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `name`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## _NumberRangeBase

`typer._click.types._NumberRangeBase`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _NumberRangeBase(_NumberParamTypeBase)
```

**Bases** `_NumberParamTypeBase`

**Declared members (6)**

- `clamp = clamp`  _instance-attribute_
- `def convert(self, value: Any, param: Union[Parameter, None], ctx: Union[Context, None]) -> Any`
- `max = max`  _instance-attribute_
- `max_open = max_open`  _instance-attribute_
- `min = min`  _instance-attribute_
- `min_open = min_open`  _instance-attribute_

**Inherited (9)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `name`, `shell_complete`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## _is_file_like

`typer._click.types._is_file_like`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_file_like(value: Any) -> TypeGuard[IO[Any]]
```

## convert_type

`typer._click.types.convert_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def convert_type(ty: Any | None, default: Any | None = None) -> ParamType
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Find the most appropriate `ParamType` for the given Python
type. If the type isn't provided, it can be inferred from a default
value.


