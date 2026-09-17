# Naming your own styles

**kind** `parameter` · **supported** `documented` · **subject** `rich`

A value you pass in. There is nothing to implement.

Upstream documents this.

## Entry points

- `rich.theme.Theme` · defined at `rich.theme.Theme` · `api/rich.theme.md`
- `rich.style.Style` · defined at `rich.style.Style` · `api/rich.style.md`

## Mental model

A Style is a value; a Theme is a mapping from names to Styles that makes those names legal inside markup. The three vocabularies in play -- rich's 235 ANSI colour names, its default style names, and `typer.colors` -- overlap only in their strings, and that overlap is a coincidence of spelling rather than a shared type.

## Decision rules

- Reusing a look in several places: build a `Style` and pass it, or name it in a `Theme`.
- Want `[mystyle]` to work in markup: it must be in the Console's Theme.
- Passing colour to Typer: `typer.secho(fg=...)` takes a string, not a rich `Style` object.

## Anti-patterns

- Passing a `rich.style.Style` to `typer.style` or `typer.secho`: they take strings and colour constants.
- Assuming `typer.colors.RED` is a rich type: it is the plain string "red", which is legal in rich markup only because the spellings happen to match.

## Observed

Probe `O002` — see `content/probes/00-index.md`.
