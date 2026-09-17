# Why the output is the width it is

Width resolves in an order, and the order has a surprise in it: an explicit `width=` loses to `TERM=dumb`. Separately, how wide a *character* is depends on a Unicode table that an environment variable can switch. Both produce the same symptom and have different causes.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `rich.console.Console` | class | `rich.console.Console` | [prose](../api/rich.console.md) | [records](../model/rich.console.json) |
| `rich.cells.cell_len` | function | `rich.cells.cell_len` | [prose](../api/rich.cells.md) | [records](../model/rich.cells.json) |
| `rich.measure.Measurement` | class | `rich.measure.Measurement` | [prose](../api/rich.measure.md) | [records](../model/rich.measure.json) |

## Upstream guides

- [`corpus/rich/docs/console.rst`](../corpus/rich/docs/console.rst)

## Decision rules

- Resolution order: an explicit `width=`, then `COLUMNS`, then the terminal, then 80 -- with `TERM=dumb` able to beat the explicit width.
- Cell width is not string length: `cell_len` counts columns, and CJK and emoji are two.
- Deterministic output for a test: pass `width=` and write to a StringIO.

## Anti-patterns

- Using `len()` or `str.ljust` to align text containing emoji or CJK.
- Trusting `CliRunner` to pin the width of Typer's Rich help. It pins the plain formatter only.
