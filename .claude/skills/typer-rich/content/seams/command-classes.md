# Changing how a command or group behaves

**kind** `subclass` · **supported** `documented` · **subject** `typer`

A concrete class you are meant to extend.

Upstream documents this.

## Entry points

- `typer.core.TyperCommand` · defined at `typer.core.TyperCommand` · `api/typer.core.md`
- `typer.core.TyperGroup` · defined at `typer.core.TyperGroup` · `api/typer.core.md`

## Mental model

`Typer(cls=...)` takes a Group subclass, and `@app.command(cls=...)` a Command subclass. This is the supported way to change help ordering, add a shared option, or intercept resolution -- and it is the alternative the `rich-utils-overrides` page points at.

## Implementors (1)

- `typer.cli.TyperCLIGroup`

## Decision rules

- Reordering or filtering commands in `--help`: override `list_commands` on a `TyperGroup`.
- Every command needs the same option: add it in a group callback, not by patching.
- You were about to rebind a module global in `rich_utils`: do this instead.

## Anti-patterns

- Subclassing the vendored `typer._click.core.Group` directly rather than `typer.core.TyperGroup`.
