# Progress bars and live displays

A live display owns the terminal until it exits. Everything that writes to the same stream must go through its Console, which is why mixing the two libraries' progress surfaces goes wrong. In CI, disable it rather than working around it.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `rich.progress.Progress` | class | `rich.progress.Progress` | [prose](../api/rich.progress.md) | [records](../model/rich.progress.json) |
| `rich.live.Live` | class | `rich.live.Live` | [prose](../api/rich.live.md) | [records](../model/rich.live.json) |
| `rich.status.Status` | class | `rich.status.Status` | [prose](../api/rich.status.md) | [records](../model/rich.status.json) |

## Upstream guides

- [`corpus/rich/docs/progress.rst`](../corpus/rich/docs/progress.rst)
- [`corpus/rich/docs/live.rst`](../corpus/rich/docs/live.rst)

## Decision rules

- Prefer `rich.progress` over `typer.progressbar`: the latter is the vendored Click one and fights a live Console.
- In CI or a non-terminal: `Progress(disable=True)`, which is a clean no-op (probe R003).
- Nested displays are supported since 14.1; two Consoles are not.

## Anti-patterns

- Asserting on animated output in a test. This index refuses to capture it, and so should you -- see `catalogs/refused.md`.
