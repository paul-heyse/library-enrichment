# Packaging and completion

A Typer app is an ordinary console-script entry point. The parts that touch the user's machine -- completion, app directories -- are the parts this index cannot execute, so they are indexed from source and upstream's own docs.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `typer._click.utils.get_app_dir` | function | `typer.get_app_dir` | [prose](../api/typer._click.utils.md) | [records](../model/typer._click.utils.json) |
| `typer.main.launch` | function | `typer.launch` | [prose](../api/typer.main.md) | [records](../model/typer.main.json) |
| `typer.main.Typer` | class | `typer.Typer` | [prose](../api/typer.main.md) | [records](../model/typer.main.json) |

## Upstream guides

- [`corpus/typer/docs/tutorial/package.md`](../corpus/typer/docs/tutorial/package.md)
- [`corpus/typer/docs/tutorial/options-autocompletion.md`](../corpus/typer/docs/tutorial/options-autocompletion.md)

## Decision rules

- Ship a console script entry point pointing at the `Typer` instance.
- Config or state: `typer.get_app_dir(name)`, which is platform-correct.
- Completion is installed by the user, not by your package.

## Anti-patterns

- Depending on `typer-cli`; it is deprecated and now installs `typer`.
- Trusting this page's completion claims as executed behaviour -- shellingham inspects the parent process and is never run here.
