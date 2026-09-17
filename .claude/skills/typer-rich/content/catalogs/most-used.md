# The API you will actually meet

3,056 symbols is a flat list until something says which ones matter. This ranks by
how often the library itself references and calls each item, weighting callers above
bare references because a call is a stronger signal of a contract than a mention.

It is a measurement of *this library's own use of itself*, not of what callers do.
Something rare here can still be exactly what your code needs -- `create_proxy` is
called eight times upstream and is the answer to a common question.

| Item | Import as | Callers | References |
|---|---|---:|---:|
| `rich.console.Console` | `rich.console.Console` | 134 | 223 |
| `rich.text.Text` | `rich.text.Text` | 101 | 250 |
| `rich.style.Style` | `rich.style.Style` | 56 | 400 |
| `rich.segment.Segment` | `rich.segment.Segment` | 56 | 201 |
| `typer._click.core.Context` | *not importable* | 27 | 158 |
| `rich.console.ConsoleOptions` | `rich.console.ConsoleOptions` | 31 | 116 |
| `rich.measure.Measurement` | `rich.measure.Measurement` | 27 | 88 |
| `rich.table.Table` | `rich.table.Table` | 28 | 53 |
| `typer._click.core.Command` | `typer.cli.Command` | 26 | 45 |
| `typer._click.utils.echo` | `typer.echo` | 23 | 48 |
| `typer._click.core.Parameter` | *not importable* | 13 | 74 |
| `rich.color.Color` | `rich.color.Color` | 18 | 49 |
| `rich.text.Span` | `rich.text.Span` | 21 | 34 |
| `rich.console.RenderableType` | `rich.console.RenderableType` | 0 | 112 |
| `rich.cells.cell_len` | `rich.cells.cell_len` | 18 | 36 |
| `rich.progress.Progress` | `rich.progress.Progress` | 25 | 8 |
| `rich.panel.Panel` | `rich.panel.Panel` | 18 | 35 |
| `rich.style.StyleType` | `rich.style.StyleType` | 0 | 105 |
| `rich.control.Control` | `rich.control.Control` | 18 | 30 |
| `rich.syntax.Syntax` | `rich.syntax.Syntax` | 18 | 23 |
| `typer.core.TyperGroup` | `typer.core.TyperGroup` | 16 | 27 |
| `typer.models.Default` | `typer.models.Default` | 5 | 69 |
| `rich.highlighter.ReprHighlighter` | `rich.highlighter.ReprHighlighter` | 16 | 19 |
| `rich._loop.loop_last` | `rich.box.loop_last` | 13 | 26 |
| `typer.core.TyperOption` | `typer.core.TyperOption` | 11 | 33 |
| `typer._click.formatting.HelpFormatter` | *not importable* | 15 | 16 |
| `rich.layout.Layout` | `rich.layout.Layout` | 9 | 37 |
| `typer._click.types.ParamType` | *not importable* | 11 | 29 |
| `rich.console.Group` | `rich.console.Group` | 11 | 28 |
| `rich.console.RenderResult` | `rich.console.RenderResult` | 0 | 72 |
| `rich.get_console` | `rich.get_console` | 13 | 20 |
| `typer._click.shell_completion.CompletionItem` | `typer.core.CompletionItem` | 6 | 44 |
| `rich.segment.ControlType` | `rich.segment.ControlType` | 1 | 62 |
| `rich.pretty.Pretty` | `rich.pretty.Pretty` | 11 | 21 |
| `rich._win32_console.WindowsCoordinates` | *not importable* | 10 | 22 |
| `rich.print` | `rich.print` | 9 | 25 |
| `rich.box.Box` | `rich.box.Box` | 3 | 44 |
| `rich.color_triplet.ColorTriplet` | `rich.color_triplet.ColorTriplet` | 8 | 24 |
| `typer.params.Option` | `typer.Option` | 4 | 40 |
| `rich.align.Align` | `rich.align.Align` | 9 | 18 |
