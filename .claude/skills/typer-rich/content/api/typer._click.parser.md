# `typer._click.parser`

Distribution: `typer`

## V

`typer._click.parser.V`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
V = TypeVar('V')
```

## _Argument

`typer._click.parser._Argument`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Argument
```

**Declared members (4)**

- `dest = dest`  _instance-attribute_
- `nargs = nargs`  _instance-attribute_
- `obj = obj`  _instance-attribute_
- `def process(self, value: str | Sequence[str | None] | None, state: _ParsingState) -> None`

## _Option

`typer._click.parser._Option`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Option
```

**Declared members (8)**

- `action = action`  _instance-attribute_
- `const = const`  _instance-attribute_
- `dest = dest`  _instance-attribute_
- `nargs = nargs`  _instance-attribute_
- `obj = obj`  _instance-attribute_
- `prefixes: set[str] = set()`  _instance-attribute_
- `def process(self, value: Any, state: _ParsingState) -> None`
- `takes_value: bool`  _property_

## _OptionParser

`typer._click.parser._OptionParser`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _OptionParser
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `def add_argument(self, obj: CoreArgument, dest: str | None, nargs: int = 1) -> None`
  Adds a positional argument named `dest` to the parser.
- `def add_option(self, obj: CoreOption, opts: Sequence[str], dest: str | None, action: str = 'store', nargs: int = 1, const: Any | None = None) -> None`
  Adds a new option named `dest` to the parser.  The destination is not inferred (unlike with optparse) and needs to be explicitly provided.  Action can be any of ``store``, ``store_const``, ``append``, ``append_const`` or ``count``.
- `allow_interspersed_args: bool = True`  _instance-attribute_
- `ctx = ctx`  _instance-attribute_
- `ignore_unknown_options: bool = False`  _instance-attribute_
- `def parse_args(self, args: list[str]) -> tuple[dict[str, Any], list[str], list[CoreParameter]]`
  Parses positional arguments and returns ``(values, args, order)`` for the parsed options and arguments as well as the leftover arguments if there are any.  The order is a list of objects as they appear on the command line.  If arguments ap…

The option parser is an internal class that is ultimately used to
parse options and arguments.  It's modelled after optparse and brings
a similar but vastly simplified API.  It should generally not be used
directly as the high level Click classes wrap it for you.

It's not nearly as extensible as optparse or argparse as it does not
implement features that are implemented on a higher level (such as
types or defaults).


## _ParsingState

`typer._click.parser._ParsingState`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ParsingState
```

**Declared members (4)**

- `largs: list[str] = []`  _instance-attribute_
- `opts: dict[str, Any] = {}`  _instance-attribute_
- `order: list[CoreParameter] = []`  _instance-attribute_
- `rargs = rargs`  _instance-attribute_

## _normalize_opt

`typer._click.parser._normalize_opt`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_opt(opt: str, ctx: Union[Context, None]) -> str
```

## _split_opt

`typer._click.parser._split_opt`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _split_opt(opt: str) -> tuple[str, str]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _unpack_args

`typer._click.parser._unpack_args`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _unpack_args(args: Sequence[str], nargs_spec: Sequence[int]) -> tuple[Sequence[str | Sequence[str | None] | None], list[str]]
```

Given an iterable of arguments and an iterable of nargs specifications,
it returns a tuple with all the unpacked arguments at the first index
and all remaining arguments as the second.

The nargs specification is the number of arguments that should be consumed
or `-1` to indicate that this position should eat up all the remainders.


