# Cross-references and the call graph

`reference.md` used to say this was unanswerable. It is answerable, from the checker's
batch reports rather than its language server -- one run over 388 files, not a loop over
3,056 symbols.

9,813 call edges · 14,122 reference rows · 876 external targets.

## Who calls this?

```bash
rg -P '^fastmcp\.server\.server\.FastMCP\t' content/index/callers.tsv | cut -f2,3,4
```

Column 1 is the callee, column 2 the caller, then the file and line. A caller reading
`… (module level)` is import-time code, `… (decorator)` a decorator expression, and
`… (class body)` a class-level statement -- all three are real call sites that no
function name would describe.

A constructor call is recorded against the **class**, not `__init__`: the question is
who builds a `FastMCP`, not who calls `object.__init__`.

## Where is this referenced?

```bash
rg -P '^fastmcp\.server\.context\.Context\t' content/index/references.tsv
```

One row per (target, file) with a count. References into typeshed and third-party code
are separated into `external-refs.tsv` by the target's **defining file**, which the
report carries -- not by guessing from the name.

## The most-called items

| Item | Callers | Referencing files | References |
|---|---:|---:|---:|
| `rich.console.Console` | 134 | 48 | 223 |
| `rich.text.Text` | 101 | 32 | 250 |
| `rich.segment.Segment` | 56 | 22 | 201 |
| `rich.style.Style` | 56 | 23 | 400 |
| `rich.console.ConsoleOptions` | 31 | 29 | 116 |
| `rich.table.Table` | 28 | 18 | 53 |
| `rich.measure.Measurement` | 27 | 19 | 88 |
| `typer._click.core.Context` | 27 | 16 | 158 |
| `typer._click.core.Command` | 26 | 11 | 45 |
| `rich.progress.Progress` | 25 | 1 | 8 |
| `typer._click.utils.echo` | 23 | 12 | 48 |
| `rich.text.Span` | 21 | 3 | 34 |
| `rich.cells.cell_len` | 18 | 10 | 36 |
| `rich.color.Color` | 18 | 8 | 49 |
| `rich.control.Control` | 18 | 4 | 30 |
| `rich.panel.Panel` | 18 | 14 | 35 |
| `rich.syntax.Syntax` | 18 | 8 | 23 |
| `rich.highlighter.ReprHighlighter` | 16 | 9 | 19 |
| `typer.core.TyperGroup` | 16 | 8 | 27 |
| `typer._click.formatting.HelpFormatter` | 15 | 3 | 16 |
| `rich._loop.loop_last` | 13 | 9 | 26 |
| `rich.get_console` | 13 | 7 | 20 |
| `typer._click.core.Parameter` | 13 | 14 | 74 |
| `rich.console.Group` | 11 | 10 | 28 |
| `rich.pretty.Pretty` | 11 | 7 | 21 |

Full ranking in `content/index/usage.tsv`, which carries every item the checker saw
referenced or called. An item absent from it was never referenced *within these three
distributions* -- which is not the same as unused, because callers outside them are
outside this index's scope.
