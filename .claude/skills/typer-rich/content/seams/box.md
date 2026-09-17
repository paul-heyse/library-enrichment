# Choosing the characters a border is drawn with

**kind** `parameter` · **supported** `documented` · **subject** `rich`

A value you pass in. There is nothing to implement.

Upstream documents this.

## Entry points

- `rich.box.Box` · defined at `rich.box.Box` · `api/rich.box.md`

## Mental model

A Box is a block of border glyphs, passed as `box=` to a Table or Panel. The choice is a rendering decision with a captured answer rather than an API question -- see probe R001 and `catalogs/boxes.md`.

## Decision rules

- Output may reach a terminal without box-drawing characters: use an ASCII box.
- Output is going into a file someone will diff: use an ASCII box, and pin the width.
- Legacy Windows console: rich substitutes automatically, and this index never executes that path.

## Anti-patterns

- Assuming the default box renders the same everywhere: probe R001 pins one width and one box, and its control shows the bytes move when the box changes.

## Observed

Probe `R001` — see `content/probes/00-index.md`.
