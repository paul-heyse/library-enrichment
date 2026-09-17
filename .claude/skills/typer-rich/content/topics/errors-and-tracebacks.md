# Failing usefully

Typer exits by raising. `Exit` carries a code, `Abort` is always 1, and `TyperException` is the ancestor that catches the vendored Click exceptions -- only one of which is re-exported at top level. Typer also installs its own traceback handler, which is why `rich.traceback.install()` appears to do nothing.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `typer.exceptions.Exit` | class | `typer.Exit` | [prose](../api/typer.exceptions.md) | [records](../model/typer.exceptions.json) |
| `typer.exceptions.Abort` | class | `typer.Abort` | [prose](../api/typer.exceptions.md) | [records](../model/typer.exceptions.json) |
| `typer.exceptions.TyperException` | class | `typer.TyperException` | [prose](../api/typer.exceptions.md) | [records](../model/typer.exceptions.json) |
| `rich.traceback.Traceback` | class | `rich.traceback.Traceback` | [prose](../api/rich.traceback.md) | [records](../model/rich.traceback.json) |

## Upstream guides

- [`corpus/typer/docs/tutorial/terminating.md`](../corpus/typer/docs/tutorial/terminating.md)
- [`corpus/rich/docs/traceback.rst`](../corpus/rich/docs/traceback.rst)

## Decision rules

- Choose the exit code: raise `typer.Exit(code)`. `Abort` is always 1.
- Catch anything Typer raises: `except typer.TyperException`. Only `BadParameter` is re-exported at top level.
- Locals in the traceback: `Typer(pretty_exceptions_show_locals=True)`. The default is False.
- `rich.traceback.install()` does not apply to a Typer app -- Typer installs its own handler.

## Anti-patterns

- Raising `SystemExit` instead of `typer.Exit`: `CliRunner` handles the two differently.
