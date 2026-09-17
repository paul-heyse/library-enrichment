# Shaping the help output

Help is rendered by `typer.rich_utils` through a Console it builds itself and never shares. That single fact explains why your Theme is ignored, why `TERMINAL_WIDTH` must be set before import, and why the supported levers are constructor arguments rather than configuration.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `typer.main.Typer` | class | `typer.Typer` | [prose](../api/typer.main.md) | [records](../model/typer.main.json) |
| `typer.params.Option` | function | `typer.Option` | [prose](../api/typer.params.md) | [records](../model/typer.params.json) |
| `typer.params.Argument` | function | `typer.Argument` | [prose](../api/typer.params.md) | [records](../model/typer.params.json) |

## Upstream guides

- [`corpus/typer/docs/tutorial/commands/help.md`](../corpus/typer/docs/tutorial/commands/help.md)
- [`corpus/typer/docs/tutorial/options/help.md`](../corpus/typer/docs/tutorial/options/help.md)

## Decision rules

- Grouping options under headings: `rich_help_panel=`.
- Markdown in docstrings: `rich_markup_mode='markdown'`.
- Restructuring the layout: a custom `TyperGroup` via `Typer(cls=...)`.
- Turning markup off entirely so brackets survive: `rich_markup_mode=None`.

## Anti-patterns

- Rebinding `typer.rich_utils` constants and calling it configuration. It works and it is private -- see `seams/rich-utils-overrides.md`.
- Passing a Theme to your own Console and expecting help to use it. `rich_utils` builds its own.
