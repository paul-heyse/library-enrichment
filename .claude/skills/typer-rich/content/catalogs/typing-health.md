# Typing health

From the checker's own coverage report, over public symbols only -- names without a
leading underscore, plus anything an `__all__` exports, followed through re-export
chains. That last part matters here: this library re-exports heavily, and a report over
raw modules would describe a different surface from the one a caller sees.

**85.0% covered**, 83.1% strictly, over 3217 typable positions in 94 modules.

2674 typed · 60 `Any` · 483 untyped · 42 suppressions.

`Any` and untyped are different failures. An `Any` is a type the checker resolved to
the top type -- it will not catch a mistake there. An untyped position has no
annotation at all, and `index/inferred.tsv` may still carry an inferred answer for it.

## Where the types are weakest

| Symbol | Kind | Any | Untyped |
|---|---|---:|---:|
| `typer.main.Typer.__call__` | function | 3 | 0 |
| `rich.jupyter.print` | function | 2 | 0 |
| `rich.reconfigure` | function | 2 | 0 |
| `typer.completion.install_callback` | function | 2 | 0 |
| `typer.completion.show_callback` | function | 2 | 0 |
| `typer.core.TyperCommand.main` | function | 2 | 0 |
| `typer.core.TyperGroup.main` | function | 2 | 0 |
| `typer.models.ParamMeta.__init__` | function | 2 | 0 |
| `rich.align.Align.align` | attr | 0 | 1 |
| `rich.align.Align.height` | attr | 0 | 1 |
| `rich.align.Align.pad` | attr | 0 | 1 |
| `rich.align.Align.renderable` | attr | 0 | 1 |
| `rich.align.Align.style` | attr | 0 | 1 |
| `rich.align.Align.vertical` | attr | 0 | 1 |
| `rich.align.Align.width` | attr | 0 | 1 |
| `rich.align.VerticalCenter.renderable` | attr | 0 | 1 |
| `rich.align.VerticalCenter.style` | attr | 0 | 1 |
| `rich.ansi.AnsiDecoder.style` | attr | 0 | 1 |
| `rich.ansi.console` | attr | 0 | 1 |
| `rich.ansi.decoder` | attr | 0 | 1 |

## Suppressions

42 in the indexed distributions. Each one is a place upstream decided
the checker was wrong or the cost was too high, which makes them the best available map
of where this library's typing is genuinely hard.

```bash
cut -f4 content/index/suppressions.tsv | tr ',' '\n' | sort | uniq -c | sort -rn
```
