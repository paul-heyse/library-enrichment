# Running code around a command, and sharing state

**kind** `parameter` · **supported** `documented` · **subject** `typer`

A value you pass in. There is nothing to implement.

Upstream documents this.

## Entry points

- `typer.Context` · defined at `typer.models.Context` · `api/typer.models.md`
- `typer.CallbackParam` · defined at `typer.models.CallbackParam` · `api/typer.models.md`

## Mental model

`typer.Context` is Typer's subclass of the vendored `typer._click.core.Context`, so most of what you can call on it is declared in a module you cannot import -- which is why the index records reachability beside nameability. `ctx.obj` is the shared-state slot; a `@app.callback` runs before any subcommand.

## Members

- `_parse_decls`
- `add_to_parser`
- `get_help_record`
- `value_is_missing`

## Decision rules

- State shared by every subcommand: set `ctx.obj` in the group callback.
- A flag that should short-circuit, like `--version`: `is_eager=True` and raise `typer.Exit`.
- A parameter that should not reach your function: `expose_value=False`.

## Anti-patterns

- Annotating a parameter as the vendored `Context` rather than `typer.Context`.
- Doing work in a callback that the `--help` path also triggers: it runs first.
