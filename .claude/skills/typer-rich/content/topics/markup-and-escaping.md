# Square brackets, and where text goes missing

Square brackets are syntax on the Rich side. A bracketed word that parses as a style name is consumed and dropped with no error; an unbalanced close raises instead. Typer turns this on by default for every help string, which is where it bites hardest.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `rich.markup.escape` | function | `rich.markup.escape` | [prose](../api/rich.markup.md) | [records](../model/rich.markup.json) |
| `rich.errors.MarkupError` | class | `rich.errors.MarkupError` | [prose](../api/rich.errors.md) | [records](../model/rich.errors.json) |

## Upstream guides

- [`corpus/rich/docs/markup.rst`](../corpus/rich/docs/markup.rst)
- [`corpus/rich/questions/square_brackets.question.md`](../corpus/rich/questions/square_brackets.question.md)

## Decision rules

- A bracketed word that parses as a style name is consumed and dropped, with no error.
- An unbalanced closing tag raises `MarkupError` instead, so neither 'always raises' nor 'never raises' is safe.
- `rich.markup.escape` for text you did not author; `markup=False` for a whole call.
- In Typer help strings this is on by default: `Typer()` sets `rich_markup_mode='rich'`.

## Anti-patterns

- Writing `help='pass a [path]'` and expecting the word to appear. Probe M001.
- Concluding from one working example that brackets are safe. `[FILE]` survives and `[path]` does not.
