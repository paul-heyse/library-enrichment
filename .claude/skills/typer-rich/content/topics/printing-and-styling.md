# Getting text onto the terminal

Two output systems live side by side. `typer.echo` writes the string you gave it. Anything on the Rich side goes through a Console, which parses markup and wraps to a width. Choosing between them is choosing whether your text is data or presentation.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `typer._click.utils.echo` | function | `typer.echo` | [prose](../api/typer._click.utils.md) | [records](../model/typer._click.utils.json) |
| `typer._click.termui.secho` | function | `typer.secho` | [prose](../api/typer._click.termui.md) | [records](../model/typer._click.termui.json) |
| `rich.console.Console` | class | `rich.console.Console` | [prose](../api/rich.console.md) | [records](../model/rich.console.json) |
| `rich.style.Style` | class | `rich.style.Style` | [prose](../api/rich.style.md) | [records](../model/rich.style.json) |

## Upstream guides

- [`corpus/typer/docs/tutorial/printing.md`](../corpus/typer/docs/tutorial/printing.md)
- [`corpus/rich/docs/console.rst`](../corpus/rich/docs/console.rst)

## Decision rules

- Plain text you did not author: `typer.echo`, or `Console.print(..., markup=False)`.
- Anything laid out -- a table, a panel, a tree: a Console.
- One coloured line: `typer.secho`. A colour reused in several places: a `rich.style.Style`.
- Untrusted text through a Console: `rich.markup.escape` first.

## Anti-patterns

- Assuming `rich.print` and `typer.echo` differ only in colour. One parses markup and wraps; the other does neither.
- Passing a `rich.style.Style` object to `typer.secho`, which takes colour strings.
