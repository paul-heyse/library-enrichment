# `typer._typing`

Distribution: `typer`

## NONE_TYPES

`typer._typing.NONE_TYPES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
NONE_TYPES: tuple[Any, Any, Any] = (None, NoneType, Literal[None])
```

## NoneType

`typer._typing.NoneType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
NoneType = None.__class__
```

## __all__

`typer._typing.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ('NoneType', 'is_none_type', 'is_callable_type', 'is_literal_type', 'all_literal_values', 'is_union', 'Annotated', 'Literal', 'get_args', 'get_origin', 'get_type_hints')
```

## all_literal_values

`typer._typing.all_literal_values`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def all_literal_values(type_: type[Any]) -> tuple[Any, ...]
```

This method is used to retrieve all Literal values as
Literal can be used recursively (see https://www.python.org/dev/peps/pep-0586)
e.g. `Literal[Literal[Literal[1, 2, 3], "foo"], 5, None]`


## is_callable_type

`typer._typing.is_callable_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def is_callable_type(type_: type[Any]) -> bool
```

## is_literal_type

Import as `typer.main.is_literal_type`  ·  defined at `typer._typing.is_literal_type`

```python
def is_literal_type(type_: type[Any]) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## is_none_type

`typer._typing.is_none_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def is_none_type(type_: Any) -> bool
```

## is_union

Import as `typer.main.is_union`  ·  defined at `typer._typing.is_union`

```python
def is_union(tp: type[Any] | None) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## literal_values

Import as `typer.main.literal_values`  ·  defined at `typer._typing.literal_values`

```python
def literal_values(type_: type[Any]) -> tuple[Any, ...]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## Annotated

`typer._typing.Annotated`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
Annotated  # re-exported from typing.Annotated
```

## Literal

`typer._typing.Literal`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
Literal  # re-exported from typing.Literal
```

## get_args

`typer._typing.get_args`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
get_args  # re-exported from typing.get_args
```

## get_origin

`typer._typing.get_origin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
get_origin  # re-exported from typing.get_origin
```

## get_type_hints

`typer._typing.get_type_hints`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
get_type_hints  # re-exported from typing.get_type_hints
```

