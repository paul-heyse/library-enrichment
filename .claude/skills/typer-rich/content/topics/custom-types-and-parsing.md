# Turning command-line strings into your types

Your annotation becomes a `ParamType` -- in Typer's vendored copy of Click, which you cannot import. That path is closed deliberately, so the supported answers are a parser function or a callback, not a subclass.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `typer.params.Option` | function | `typer.Option` | [prose](../api/typer.params.md) | [records](../model/typer.params.json) |
| `typer.params.Argument` | function | `typer.Argument` | [prose](../api/typer.params.md) | [records](../model/typer.params.json) |
| `typer._click.types.ParamType` | class | *not importable* | [prose](../api/typer._click.types.md) | [records](../model/typer._click.types.json) |

## Upstream guides

- [`corpus/typer/docs/tutorial/parameter-types/index.md`](../corpus/typer/docs/tutorial/parameter-types/index.md)
- [`corpus/typer/docs/tutorial/click.md`](../corpus/typer/docs/tutorial/click.md)

## Decision rules

- A standard type: just annotate it. That is Typer's whole proposition.
- Something custom: `typer.Option(parser=...)` or a callback, not a `ParamType` subclass.
- Constraints: `typer.Option(min=, max=)` produces an `IntRange` or `FloatRange` for you.
- To see what an annotation becomes, read the `declared` rows in `catalogs/vendored-click.md`.

## Anti-patterns

- `from typer._click.types import ParamType`. Private, and upstream closes this off explicitly.
- Passing a `click.ParamType` from the installed click package.
