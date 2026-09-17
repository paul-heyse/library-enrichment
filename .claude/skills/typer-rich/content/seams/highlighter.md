# Automatic styling of text by pattern

**kind** `abstract-base` · **supported** `documented` · **subject** `rich`

An abstract base. The required members are the obligations; everything else is provided, and provided is where the capability hides.

Upstream documents this.

## Entry points

- `rich.highlighter.Highlighter` · defined at `rich.highlighter.Highlighter` · `api/rich.highlighter.md`
- `rich.highlighter.RegexHighlighter` · defined at `rich.highlighter.RegexHighlighter` · `api/rich.highlighter.md`

## Mental model

`Highlighter.highlight` is the single obligation. `RegexHighlighter` implements it for you and asks only for a `highlights` list of regular expressions with named groups -- the group name is the style name. This is what turns numbers, paths and URLs a different colour without anyone writing markup.

## Required

- `highlight`

## Implementors (9)

- `rich.highlighter.ISO8601Highlighter`
- `rich.highlighter.JSONHighlighter`
- `rich.highlighter.NullHighlighter`
- `rich.highlighter.RegexHighlighter`
- `rich.highlighter.ReprHighlighter`
- `rich.traceback.PathHighlighter`
- `typer.rich_utils.NegativeOptionHighlighter`
- `typer.rich_utils.OptionHighlighter`
- `typer.rich_utils.TypesHighlighter`

## Decision rules

- Pattern-based: subclass `RegexHighlighter` and set `highlights`.
- Anything else: subclass `Highlighter` and implement `highlight(text)`, mutating the Text in place.
- Want it off for one call: `Console.print(..., highlight=False)`, not a custom Highlighter.

## Anti-patterns

- Returning a new Text from `highlight`: it mutates in place and the return value is ignored.
