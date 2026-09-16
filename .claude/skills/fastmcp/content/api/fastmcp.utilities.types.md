# `fastmcp.utilities.types`

Distribution: `fastmcp`

## AnyFunction

Import as `fastmcp.apps.app.AnyFunction`  ·  defined at `fastmcp.utilities.types.AnyFunction`

```python
AnyFunction: TypeAlias = Callable[..., Any]
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## NotSet

Import as `fastmcp.tools.base.NotSet`  ·  defined at `fastmcp.utilities.types.NotSet`

```python
NotSet = ...
```

**Inferred type** (`ty`, not declared in the source): `EllipsisType`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## NotSetT

Import as `fastmcp.tools.base.NotSetT`  ·  defined at `fastmcp.utilities.types.NotSetT`

```python
NotSetT: TypeAlias = EllipsisType
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## T

`fastmcp.utilities.types.T`

```python
T = TypeVar('T', default=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## Audio

Import as `fastmcp.tools.base.Audio`  ·  defined at `fastmcp.utilities.types.Audio`

```python
class Audio
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `annotations = annotations`  _instance-attribute_
- `data = data`  _instance-attribute_
- `path = Path(os.path.expandvars(str(path))).expanduser() if path else None`  _instance-attribute_
- `def to_audio_content(self, mime_type: str | None = None, annotations: Annotations | None = None) -> mcp_types.AudioContent`

Helper class for returning audio from tools.


## ContextSamplingFallbackProtocol

`fastmcp.utilities.types.ContextSamplingFallbackProtocol`

```python
class ContextSamplingFallbackProtocol(Protocol)
```

**Bases** `Protocol`

## FastMCPBaseModel

Import as `fastmcp.prompts.base.FastMCPBaseModel`  ·  defined at `fastmcp.utilities.types.FastMCPBaseModel`

```python
class FastMCPBaseModel(BaseModel)
```

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

Base model for FastMCP models.


## File

Import as `fastmcp.tools.base.File`  ·  defined at `fastmcp.utilities.types.File`

```python
class File
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `annotations = annotations`  _instance-attribute_
- `data = data`  _instance-attribute_
- `path = Path(os.path.expandvars(str(path))).expanduser() if path else None`  _instance-attribute_
- `def to_resource_content(self, mime_type: str | None = None, annotations: Annotations | None = None) -> mcp_types.EmbeddedResource`

Helper class for returning file data from tools.


## Image

Import as `fastmcp.tools.base.Image`  ·  defined at `fastmcp.utilities.types.Image`

```python
class Image
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `annotations = annotations`  _instance-attribute_
- `data = data`  _instance-attribute_
- `path = self._get_expanded_path(path)`  _instance-attribute_
- `def to_data_uri(self, mime_type: str | None = None) -> str`
  Get image as a data URI.
- `def to_image_content(self, mime_type: str | None = None, annotations: Annotations | None = None) -> mcp_types.ImageContent`
  Convert to MCP ImageContent.

Helper class for returning images from tools.


## create_function_without_params

`fastmcp.utilities.types.create_function_without_params`

```python
def create_function_without_params(fn: Callable[..., Any], exclude_params: list[str]) -> Callable[..., Any]
```

Create a new function with the same code but without the specified parameters in annotations.

This is used to exclude parameters from type adapter processing when they can't be serialized.
The excluded parameters are removed from the function's __annotations__ dictionary.


## find_kwarg_by_type

Import as `fastmcp.server.dependencies.find_kwarg_by_type`  ·  defined at `fastmcp.utilities.types.find_kwarg_by_type`

```python
def find_kwarg_by_type(fn: Callable, kwarg_type: type) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Find the name of the kwarg that is of type kwarg_type.

Includes union types that contain the kwarg_type, as well as Annotated types.


## get_cached_typeadapter

Import as `fastmcp.tools.base.get_cached_typeadapter`  ·  defined at `fastmcp.utilities.types.get_cached_typeadapter`

```python
def get_cached_typeadapter(cls: T) -> TypeAdapter[T]
```

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

TypeAdapters are heavy objects, and in an application context we'd typically
create them once in a global scope and reuse them as often as possible.
However, this isn't feasible for user-generated functions. Instead, we use a
cache to minimize the cost of creating them as much as possible.


## get_fn_name

Import as `fastmcp.contrib.mcp_mixin.mcp_mixin.get_fn_name`  ·  defined at `fastmcp.utilities.types.get_fn_name`

```python
def get_fn_name(fn: Callable[..., Any]) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## is_class_member_of_type

Import as `fastmcp.server.dependencies.is_class_member_of_type`  ·  defined at `fastmcp.utilities.types.is_class_member_of_type`

```python
def is_class_member_of_type(cls: Any, base: type) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if cls is a member of base, even if cls is a type variable.

Base can be a type, a UnionType, or an Annotated type. Generic types are not
considered members (e.g. T is not a member of list[T]).


## issubclass_safe

Import as `fastmcp.tools.tool_transform.issubclass_safe`  ·  defined at `fastmcp.utilities.types.issubclass_safe`

```python
def issubclass_safe(cls: type, base: type) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if cls is a subclass of base, even if cls is a type variable.


## replace_type

Import as `fastmcp.tools.function_parsing.replace_type`  ·  defined at `fastmcp.utilities.types.replace_type`

```python
def replace_type(type_, type_map: dict[type, type])
```

**Inferred type** (`ty`, not declared in the source): `def replace_type( type_, type_map: dict[type, type] ) -> Unknown`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Given a (possibly generic, nested, or otherwise complex) type, replaces all
instances of keys in type_map with their corresponding values.

This is useful for transforming types when creating tools.

Args:
    type_: The type to transform.
    type_map: A mapping of types to replace (keys are replaced by values).

Examples:
```python
>>> replace_type(list[int | bool], {int: str})
list[str | bool]

>>> replace_type(list[list[int]], {int: str})
list[list[str]]
```


