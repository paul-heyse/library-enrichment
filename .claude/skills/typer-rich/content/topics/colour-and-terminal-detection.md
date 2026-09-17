# Why there is no colour

Rich decides three things independently: is this a terminal, how many colours does it have, and has the user asked for none. Environment variables win over constructor arguments more often than you would expect, and an empty value means disabled rather than set.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `rich.console.Console` | class | `rich.console.Console` | [prose](../api/rich.console.md) | [records](../model/rich.console.json) |

## Upstream guides

- [`corpus/rich/docs/console.rst`](../corpus/rich/docs/console.rst)
- [`corpus/rich/questions/ansi_escapes.question.md`](../corpus/rich/questions/ansi_escapes.question.md)

## Decision rules

- `NO_COLOR=1` beats `force_terminal=True` and an explicit `color_system`; the bold survives, the colour does not.
- An empty `NO_COLOR` or `FORCE_COLOR` means disabled, since 14.0.
- `FORCE_COLOR` alone gives the `standard` system; `COLORTERM` is what raises it to truecolor.
- `Console(_environ=...)` isolates most of this. Not `UNICODE_VERSION`.

## Anti-patterns

- Debugging missing colour by adding `force_terminal=True` without checking the environment first.
