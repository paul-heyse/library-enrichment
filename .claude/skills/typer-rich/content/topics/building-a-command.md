# Building a command-line interface

Typer turns a function signature into a command line. The annotation is the parser, the default decides Argument or Option, and the docstring is the help. Everything else is a refinement of those three facts -- which is why reaching past the annotation for a custom type is usually the wrong move.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `typer.main.Typer` | class | `typer.Typer` | [prose](../api/typer.main.md) | [records](../model/typer.main.json) |
| `typer.params.Argument` | function | `typer.Argument` | [prose](../api/typer.params.md) | [records](../model/typer.params.json) |
| `typer.params.Option` | function | `typer.Option` | [prose](../api/typer.params.md) | [records](../model/typer.params.json) |
| `typer.models.Context` | class | `typer.Context` | [prose](../api/typer.models.md) | [records](../model/typer.models.json) |

## Upstream guides

- [`corpus/typer/docs/tutorial/first-steps.md`](../corpus/typer/docs/tutorial/first-steps.md)
- [`corpus/typer/docs/tutorial/commands/index.md`](../corpus/typer/docs/tutorial/commands/index.md)

## Decision rules

- Always create an explicit `typer.Typer()` and register commands on it; `typer.run` does not extend to a second command.
- Always prefer the `Annotated` form: `name: Annotated[str, typer.Argument()]`. The default-value form still works and carries less.
- Subcommands: `app.add_typer(sub, name=...)`. The name is not inferred from the callback -- pass it.
- A parameter with a default is an Option; one without is an Argument.

## Anti-patterns

- Installing `typer-slim` to avoid Rich. It is a wrapper around `typer` and pulls Rich anyway.
- Reaching for `click` to do something Typer cannot. Typer carries its own copy; the `click` you import is a different library.
