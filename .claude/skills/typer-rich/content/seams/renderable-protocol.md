# Making your own object printable

**kind** `runtime-protocol` · **supported** `documented` · **subject** `rich`

A runtime protocol. Implementors below are the checker's own verdicts, not a class tree -- nothing subclasses this, and nothing needs to.

Upstream documents this.

## Entry points

- `rich.console.ConsoleRenderable` · defined at `rich.console.ConsoleRenderable` · `api/rich.console.md`
- `rich.console.RichCast` · defined at `rich.console.RichCast` · `api/rich.console.md`

## Mental model

One decision, not three, which is why this is one page. `__rich_console__` yields segments or renderables and receives the `ConsoleOptions` -- use it when the output depends on the available width. `__rich__` returns something else already renderable and is the one-liner. `rich.abc.RichRenderable` is neither: its own docstring says there is no need to extend it, it exists for `isinstance`, and it has zero subclasses in the whole library.

## Required

- `__rich__`
- `__rich_console__`

## Implementors (49)

- `rich.__main__.ColorBox`
- `rich._inspect.Inspect`
- `rich.align.Align`
- `rich.align.VerticalCenter`
- `rich.bar.Bar`
- `rich.color.Color`
- `rich.columns.Columns`
- `rich.console.Group`
- `rich.console.NewLine`
- `rich.console.ScreenUpdate`
- `rich.constrain.Constrain`
- `rich.containers.Lines`
- `rich.containers.Renderables`
- `rich.control.Control`
- `rich.emoji.Emoji`
- `rich.json.JSON`
- `rich.layout.Layout`
- `rich.layout._Placeholder`
- `rich.live_render.LiveRender`
- `rich.markdown.BlockQuote`
- _…and 29 more_

## Decision rules

- Returning a Text, a Table or a str: write `__rich__`.
- Output depends on width, height or justification: write `__rich_console__` and read the ConsoleOptions.
- Neither, and you only want a nicer repr: write `__rich_repr__` instead; it feeds `Pretty`, not the console.
- Never subclass `rich.abc.RichRenderable` to gain behaviour. It grants none.

## Anti-patterns

- Relying on `__str__`: it is reached only after markup parsing, so any bracket in it is re-interpreted and may be deleted (see probe M001).
- Returning a plain str from `__rich_console__` when the caller expects an iterable -- it is a generator protocol, so yield.
