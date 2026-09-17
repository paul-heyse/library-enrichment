# Asking the user something

Both libraries can ask a question. The difference only matters when something else owns the screen: the Typer prompts write past the Console, so a live display overwrites them.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `typer._click.termui.prompt` | function | `typer.prompt` | [prose](../api/typer._click.termui.md) | [records](../model/typer._click.termui.json) |
| `typer._click.termui.confirm` | function | `typer.confirm` | [prose](../api/typer._click.termui.md) | [records](../model/typer._click.termui.json) |
| `rich.prompt.Prompt` | class | `rich.prompt.Prompt` | [prose](../api/rich.prompt.md) | [records](../model/rich.prompt.json) |

## Upstream guides

- [`corpus/typer/docs/tutorial/prompt.md`](../corpus/typer/docs/tutorial/prompt.md)
- [`corpus/rich/docs/prompt.rst`](../corpus/rich/docs/prompt.rst)

## Decision rules

- Standalone prompt: either works.
- Inside a `Live` or `Progress`: use the Rich prompt, because the Typer ones bypass the Console and get overwritten.
- A prompt driven by a CLI parameter: `typer.Option(prompt=True)`, which handles the plumbing.

## Anti-patterns

- Mixing `typer.prompt` with an active `Progress` on the same stream.
