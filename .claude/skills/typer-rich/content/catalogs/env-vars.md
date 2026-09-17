# Environment variables that change the output

Why the colour is gone, why the width is 80, why the table looks different in CI. Most
of these are documented only in a changelog entry, and two of them beat settings you
passed explicitly to the constructor.

Ordered by the layer they act on, not alphabetically.

| Variable | Read by | Effect | Empty value | Since | Probe |
|---|---|---|---|---|---|
| `NO_COLOR` | rich Console | Drops colour but keeps other attributes, even with force_terminal and an explicit color_system. | **ignored** since 14.0 | rich 14.0 | `E001` |
| `FORCE_COLOR` | rich Console, typer rich_utils | Treats the stream as a terminal. Yields the `standard` colour system on its own, not truecolor. | **ignored** since 14.0 | rich 14.0 | — |
| `TTY_COMPATIBLE` | rich Console | Overrides TTY auto-detection. Beats FORCE_COLOR. | ignored | rich 14.0 | — |
| `TTY_INTERACTIVE` | rich Console | Forces interactive mode off or on, which governs Live and Progress. | ignored | rich 14.1 | — |
| `TERM` | rich Console | `dumb` overrides a width passed explicitly to the constructor and reports 80. | treated as absent | rich (long-standing) | `E003` |
| `COLORTERM` | rich Console | `truecolor` or `24bit` raises the colour system above `standard`. | ignored | rich (long-standing) | — |
| `COLUMNS` | rich Console | Sets the width when none was passed. Does not override an explicit width. | ignored | rich (long-standing) | `E004` |
| `LINES` | rich Console | Sets the height when none was passed. | ignored | rich (long-standing) | — |
| `UNICODE_VERSION` | rich._unicode_data, via os.environ **directly** | Changes how many cells a character measures, reshaping every table containing one. Not covered by `Console(_environ=...)`, and cached, so it is order-dependent too. | ignored | rich 14.3 | `E005` |
| `JUPYTER_COLUMNS` | rich.jupyter | Width inside a notebook. Never exercised here -- see known limit 6. | ignored | rich (long-standing) | — |
| `JUPYTER_LINES` | rich.jupyter | Height inside a notebook. Never exercised here. | ignored | rich (long-standing) | — |
| `TERMINAL_WIDTH` | typer.rich_utils, **at first import** | Fixes the width of Typer's Rich help output. Read once into `MAX_WIDTH`, so setting it after the module is imported does nothing. | treated as absent | typer (long-standing) | — |
| `_TYPER_FORCE_DISABLE_TERMINAL` | typer.rich_utils | Forces Typer's help Console to render as a non-terminal. Private, and named as such. | treated as absent | typer (long-standing) | — |
| `PY_COLORS` | typer.rich_utils | Collapsed into FORCE_TERMINAL at import, alongside FORCE_COLOR. | treated as absent | typer (long-standing) | — |
| `GITHUB_ACTIONS` | typer.rich_utils | Presence forces terminal mode, so help output is coloured in CI logs. | treated as present | typer (long-standing) | — |
| `DATABRICKS_RUNTIME_VERSION` | rich.console, via os.getenv **directly** | Participates in Jupyter detection. Not covered by `Console(_environ=...)`. | treated as present | rich (long-standing) | — |

## Two that are not intuitive

`NO_COLOR=1` strips the colour from a Console built with `force_terminal=True` and
`color_system="truecolor"` -- both explicit, and neither wins. The bold survives; only
the colour goes (probe `E001`). An **empty** `NO_COLOR` is ignored, which is the
opposite of the usual presence-is-enough convention (probe `E002`).

`Console(_environ=...)` is Rich's own injection point and isolates most of this. It does
**not** cover `UNICODE_VERSION`, which `rich/_unicode_data` reads from `os.environ`
directly and then caches -- so that one is order-dependent as well (probe `E005`).
