# `typer._click.globals`

Distribution: `typer`

## _local

`typer._click.globals._local`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_local = local()
```

## get_current_context

Import as `typer.main.get_current_context`  ·  defined at `typer._click.globals.get_current_context`

```python
def get_current_context(silent: bool = False) -> Union[Context, None]
```

**Overloads** (the signature above is the runtime dispatcher):

- `def get_current_context(silent: Literal[False] = False) -> Context`
- `def get_current_context(silent: bool = ...) -> Union[Context, None]`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Returns the current click context.  This can be used as a way to
access the current context object from anywhere.  This is a more implicit
alternative to the `pass_context` decorator.  This function is
primarily useful for helpers such as `echo` which might be
interested in changing its behavior based on the current context.

To push the current context, `Context.scope` can be used.


## pop_context

`typer._click.globals.pop_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def pop_context() -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Removes the top level from the stack.


## push_context

`typer._click.globals.push_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def push_context(ctx: Context) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Pushes a new context to the current stack.


## resolve_color_default

`typer._click.globals.resolve_color_default`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def resolve_color_default(color: bool | None = None) -> bool | None
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Internal helper to get the default value of the color flag.  If a
value is passed it's returned unchanged, otherwise it's looked up from
the current context.


