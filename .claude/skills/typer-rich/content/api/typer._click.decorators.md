# `typer._click.decorators`

Distribution: `typer`

## CmdType

`typer._click.decorators.CmdType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
CmdType = TypeVar('CmdType', bound=Command)
```

## GrpType

`typer._click.decorators.GrpType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
GrpType = TypeVar('GrpType', bound=TyperGroup)
```

## P

`typer._click.decorators.P`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
P = ParamSpec('P')
```

## R

`typer._click.decorators.R`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
R = TypeVar('R')
```

## T

`typer._click.decorators.T`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
T = TypeVar('T')
```

## _AnyCallable

`typer._click.decorators._AnyCallable`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_AnyCallable = Callable[..., Any]
```

## help_option

`typer._click.decorators.help_option`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def help_option(param_decls: list[str]) -> Callable[[Command], Command]
```

Help option which prints the help page and exits the program.


## option

`typer._click.decorators.option`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def option(param_decls: list[str], cls: type[TyperOption] | None = None, attrs: Any = {}) -> Callable[[Command], Command]
```

Attaches an option to the command.


