# Asserting on what your CLI printed

Capturing output is easy; capturing it *deterministically* is the actual problem. Width, colour and terminal detection all vary by machine, and `CliRunner` pins only one of the two help formatters.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `typer.testing.CliRunner` | class | `typer.testing.CliRunner` | [prose](../api/typer.testing.md) | [records](../model/typer.testing.json) |
| `rich.console.Console` | class | `rich.console.Console` | [prose](../api/rich.console.md) | [records](../model/rich.console.json) |

## Upstream guides

- [`corpus/typer/docs/tutorial/testing.md`](../corpus/typer/docs/tutorial/testing.md)

## Decision rules

- Testing a CLI: `CliRunner().invoke(app, args)`, and read `.output`, `.stdout` or `.stderr`.
- Testing a library that owns its Console: `Console(record=True)` then `export_text()`.
- Either way, pin the width. `CliRunner` pins the plain formatter's width only, not the Rich one.
- Exports are deterministic: the same construction produces the same bytes across processes.

## Anti-patterns

- Using `capsys` alone: it catches the bytes but not the width, so the assertion becomes machine-dependent.
- Expecting a recording Console to see what the vendored Click wrote.
