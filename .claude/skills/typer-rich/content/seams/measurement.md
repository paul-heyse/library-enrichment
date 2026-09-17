# Telling rich how wide your renderable wants to be

**kind** `duck-protocol` · **supported** `documented` · **subject** `rich`

A duck protocol. No class declares it -- you add the method and rich finds it, so no hierarchy will ever list you.

Upstream documents this.

## Entry points

- `rich.measure.Measurement` · defined at `rich.measure.Measurement` · `api/rich.measure.md`

## Mental model

Separate from the renderable protocol on purpose: omitting it is what produces a table column of the wrong width, and the symptom points nowhere near this page. Rich asks every renderable for a minimum and a maximum before it allocates; without `__rich_measure__` it falls back to measuring your rendered output, which is usually right and occasionally very wrong.

## Required

- `__rich_measure__`

## Implementors (18)

- `rich.__main__.ColorBox`
- `rich.align.Align`
- `rich.align.VerticalCenter`
- `rich.bar.Bar`
- `rich.console.Group`
- `rich.constrain.Constrain`
- `rich.containers.Renderables`
- `rich.padding.Padding`
- `rich.panel.Panel`
- `rich.pretty.Pretty`
- `rich.progress_bar.ProgressBar`
- `rich.rule.Rule`
- `rich.spinner.Spinner`
- `rich.styled.Styled`
- `rich.syntax.Syntax`
- `rich.table.Table`
- `rich.text.Text`
- `rich.tree.Tree`

## Decision rules

- Your renderable has a natural minimum width: implement it and return `Measurement(minimum, maximum)`.
- Your output is fixed-width: return the same number twice.
- You are inside a Table cell and the column is wrong: this is the page, not the Table constructor.

## Anti-patterns

- Returning a Measurement wider than the console: it is a request, and the console still crops.
