# Facts at the pins

The reason this repository exists. Both libraries carry behaviour a model answers wrong
from memory, and it is invisible at the call site: the code reads correctly, the type
checker passes, and the failure arrives at runtime or -- worse -- not at all, as
silently wrong output.

## Typer 0.27.2

Stated in the present tense. This index describes the pinned release and does not
catalogue what earlier versions did.

| Fact | Why it matters | Rule |
|---|---|---|
| Typer 0.27.2 carries its own adapted copy of Click at `typer/_click/` and does not depend on the `click` distribution. | An `import click` in a Typer project resolves to an unrelated copy installed for some other reason -- a different library from the one Typer runs -- and it type-checks either way. `typer.main.get_command(app)` returns a `typer._click` Command that no Click plugin accepts. | `project-vendored-click-import` |
| At 0.27.2 `typer-slim` is a shallow wrapper around `typer` and requires `rich` and `shellingham` unconditionally. | Depending on `typer-slim` to keep Rich out of the dependency tree keeps nothing out. Depend on `typer`. | `project-typer-slim-install` |

## Changes within a pinned line

`after` is upstream's own replacement text wherever upstream supplied one.

| Version | Date | Subject | What changed | What to do instead | Rule |
|---|---|---|---|---|---|
| `0.27.0` | 2026-07-15 | typer | Metavar printing changed; an argument now renders as `<str>` in the second help column. | Nothing to change. Update any test asserting on the old help text. | — |
| `15.0.0` | 2026-04-12 | rich | Support for Python 3.8 dropped. | Require Python 3.9 or later. | — |
| `14.3.4` | 2026-04-11 | rich | Import time was improved with lazy loading, and `rich.__version__` stopped existing. | `importlib.metadata.version("rich")`. | `project-rich-version-attr` |
| `14.3.0` | 2026-01-24 | rich | `cell_len` gained a `unicode_version` parameter and a `UNICODE_VERSION` environment variable. | Leave both alone. Setting the variable changes the measured width of characters and reshapes every table containing one. | — |
| `14.1.0` | 2025-06-25 | rich | `TTY_INTERACTIVE` was added to force interactive mode off or on. | Nothing to change; know it exists when output differs in CI. | — |
| `14.0.0` | 2025-03-30 | rich | An empty `NO_COLOR` or `FORCE_COLOR` is now considered disabled, and `TTY_COMPATIBLE` was added. | Unset the variable rather than setting it empty; empty no longer means set. | — |

## The two that catch people

**Click is vendored.** `typer/_click/` *is* Click, adapted, and `click` is not a Typer
dependency: `Requires-Dist` names `shellingham`, `rich`, `annotated-doc` and `colorama`
on Windows, and nothing else. So `import click` in a Typer project resolves to whatever
copy is installed for some other reason -- a different library from the one Typer runs,
and it still type-checks. `typer.main.get_command(app)` returns a `Command`; probe
`V003` shows whose.

**`typer-slim` does not avoid Rich.** It is a shallow wrapper around `typer` that
requires `rich` and `shellingham` unconditionally. Installing it to keep the dependency
tree small accomplishes nothing.
