# Evidence 07 — Typer and Rich: the query interface surface

**Source:** the `typer-rich` capability repository (typer 0.27.2, rich 15.0.0, and Typer's
**vendored** `typer._click` 201 items — not the `click` distribution), read 2026-09-16.
19 executed probes, 16 confirmed.

The CLI is the thin front end over the Rust core (per the settled language split): it renders
decision packets and formats structured output. That makes correctness of *rendering* the whole
job, and this dossier leads with the hazard that matters most for this particular tool.

---

## 1. The severe hazard: Rich silently eats bracketed text

Probe **M002** (recorded), measured verbatim:

```python
{s: render('v ' + s + ' w').strip()
 for s in ['[path]', '[key=value]', '[a b]', '[x]', '[FILE]', '[1-9]', '[--flag]', '[0]']}
```

```text
{'[path]':      'v  w',            # EATEN
 '[key=value]': 'v  w',            # EATEN
 '[a b]':       'v  w',            # EATEN
 '[x]':         'v  w',            # EATEN
 '[FILE]':      'v [FILE] w',
 '[1-9]':       'v [1-9] w',
 '[--flag]':    'v [--flag] w',
 '[0]':         'v [0] w'}
```

The skill's own note: *"lowercase word-ish content is consumed; uppercase, digits and a leading
double dash survive. The rule is 'parses as a style name', not 'contains a square bracket', which
is why the trap is so hard to spot by eye."*

**This tool's output is made of exactly the strings that get eaten.** A capability catalog for
ast-grep, ripgrep and PCRE2 renders, constantly:

| Thing this tool prints | Fate under an unescaped Rich `Console.print` | Source |
|---|---|---|
| character class `[x]`, `[a-z]`, `[a-zA-Z_]` | **all three deleted** | P01 |
| character class `[A-Z]`, `[0-9]`, `[^a-z]` | survive — one character's difference from the row above | P01 |
| POSIX class `[[:alpha:]]`, `[[:^digit:]]` | survive; the doubled bracket does not parse as a style | P01 |
| ast-grep rule fields `[pattern]` `[kind]` `[inside]` `[has]` `[stopBy]` `[regex]` | **all six deleted** — the whole rule vocabulary | P01 |
| option placeholders `[path]`, `[glob]`, `[key=value]` | **deleted** | P01, M002 |
| option placeholders `[PATH]`, `[--flag]`, `[-n]`, `[FILE]` | survive | P01, M002 |

**Design rule, non-negotiable:** every value that came from the catalog, a pattern, a user, or a
probe is printed either through `rich.markup.escape` or with `markup=False`. Round-1 probe P01
measured both remedies against 24 forms drawn from this tool's own output: `escape()` rescued
every altered form and `markup=False` preserved every form, with no failures. Only literal
strings authored inside the CLI may carry markup. The plan states this as a lint, not a
convention — see §5.

Corollary from probe **O001** (confirmed): `typer.echo` is literal — no markup, no wrapping, no
width. For raw pattern echo it is the correct tool and Rich is the wrong one.

Probe **M001** (confirmed): `render('status [ok] done')` → `contains:[ok]` — a reminder that some
bracketed forms do survive, which is precisely what makes eyeballing unreliable.

---

## 2. Typer versus Rich: which surface for which job

Quoted from `content/catalogs/same-job-different-answer.md`; rows with a probe id were measured.

| I want to… | Write | Not | Because | Probe |
|---|---|---|---|---|
| print plain text from a CLI | `typer.echo` | `rich.print`, `Console.print` | Both Rich surfaces parse markup, so any bracketed word may be silently deleted | `O001` |
| print text I did not write | `rich.markup.escape` | `Console.print` | Untrusted text reaching a Console is parsed as markup | `M003` |
| print a table, panel or layout | `Console.print` | `typer.echo` | `typer.echo` has no concept of a renderable or a width | — |
| colour one line | `typer.secho` | `typer.style`, `rich.style.Style` | `typer.style` returns a string you still have to echo; a Rich `Style` object is not accepted by either Typer function | `O002` |
| reuse a colour | `rich.style.Style` | `typer.colors.RED` | `typer.colors.RED` is the plain string `"red"`; it works in markup only because the spellings coincide | `O002` |
| show a progress bar | `rich.progress.Progress` | `typer.progressbar` | `typer.progressbar` is the vendored Click one — no columns, no transient mode, and it writes to a stream Rich does not own, so it fights a live Console | — |
| ask a question | `rich.prompt.Prompt.ask` | `typer.prompt`, `typer.confirm` | Both Typer prompts bypass the Console, so under `Live` or `Progress` the prompt is overwritten | — |
| exit with a chosen code | `typer.Exit` | `typer.Abort` | `Abort` always exits 1 and prints "Aborted."; `Exit` takes the code | — |
| catch any Typer error | `typer.TyperException` | `typer.BadParameter` | `BadParameter` is the only vendored Click exception re-exported at top level | — |
| assert on CLI output in a test | `typer.testing.CliRunner` | `rich.console.Console` | A recording Console sees only what you printed through it, not what vendored Click wrote | — |
| control output width | `rich.console.Console` | `typer.get_terminal_size` | Width is a property of the Console, and even an explicit width loses to `TERM=dumb` | `E003` |
| get the installed version | `importlib.metadata.version` | `rich.__version__` | **Does not exist at rich 15.0.0**, while `typer.__version__` does | `V002` |

Two of these bind the plan directly:

- **Progress must be Rich's**, because the extraction pipeline is long-running and the CLI will
  want columns and a transient display. Mixing in `typer.progressbar` would fight the Console.
- **Prompts must be Rich's** if anything is displayed live, which the `validate` operation will
  want.

---

## 3. Renderables available for decision packets

All present at rich 15.0.0: `Table`, `Tree`, `Panel`, `Syntax`, `JSON`, `Columns`, `Live`,
`Progress`, `Console`, `Markdown`, `Pretty`, `Group`, `Rule`, `Padding`.

Mapping onto the proposal's §9 decision-packet contents:

| Packet element | Renderable |
|---|---|
| candidate mechanisms, why each matches | `Table` with a `Precision`-coloured column |
| distinguishing alternatives | `Columns` of `Panel`s, one per candidate |
| necessary parameters and effective defaults | `Table`, with defaults dimmed |
| relevant interactions | `Tree` rooted at the mechanism |
| a constructed rule or pattern | `Syntax` (yaml / regex), which does **not** markup-parse its content |
| guarantees and non-guarantees | `Panel` with a border style keyed to `Precision` |
| evidence handles | `Table` of ids, never the evidence itself (proposal §11) |
| machine-readable form | `JSON`, or bypass Rich entirely — see §4 |

`Syntax` is the right answer for rendering patterns because it takes the text as code rather than
as markup, which sidesteps §1 for the highest-risk content.

### Console recording and export

```
rich.console.Console.record / capture
  .export_text  .export_html  .export_svg
  .save_text    .save_html    .save_svg
```

`record=True` plus `export_text()` is how the plan's own documentation examples stay true: a
rendered decision packet can be captured and committed, and re-captured on rebuild to detect
drift. This mirrors the probe-and-control discipline the surrounding skills already use.

---

## 4. Structured output must not go through Rich

The Rust core emits Arrow IPC or JSON (settled language split). The CLI has two output modes and
they must not share a path:

```
human    → Rich renderables, everything escaped per §1
machine  → written to stdout by `typer.echo` or a raw stream write, never through a Console
```

A Console applies width, wrapping and markup; all three corrupt machine-readable output. Probe
`E003` confirms width is a Console property, and the skill records that even an explicit width
loses to `TERM=dumb` — so a piped `--json` through a Console could wrap at 80 columns on one
machine and not on another. The `--json` path therefore bypasses Rich entirely.

---

## 5. Consequences for the plan

1. **A lint rule, not a convention.** Every `Console.print` call site whose argument is not a
   literal must pass through `escape()` or set `markup=False`. This is mechanically checkable and
   belongs in the repository's ast-grep corpus, next to the existing `project-*` rules.
2. **`--json` never touches a Console.**
3. **Rich for progress and prompts, Typer for literal echo and exit codes.**
4. **`typer.testing.CliRunner` for CLI tests**, with the caveat that it pins the plain
   formatter's width but not the Rich one — so Rich output assertions must pin a `Console` width
   explicitly, as every probe in this skill does.
5. **Do not read `rich.__version__`.** Use `importlib.metadata.version`.

## 6. Open questions for round 2

1. ~~Whether `[a-z]` and `[[:alpha:]]` survive markup parsing.~~ **Answered in round 1 by probe
   P01** (`evidence/09`, `evidence/probes/P01-*`): `[a-z]` is **deleted**, `[[:alpha:]]` survives,
   and so is every ast-grep rule field name in brackets — `[pattern]`, `[kind]`, `[inside]`,
   `[has]`, `[stopBy]`, `[regex]`. Both remedies were controlled: `escape()` rescued all 11
   altered forms and `markup=False` preserved all 24. §1 and §5 above are therefore measured,
   not inferred.
2. Whether `Syntax` supports a `regex` lexer, or whether patterns should render as `text`.
3. Whether Typer's `rich_markup_mode` setting can be switched off globally, which would make §1's
   hazard a configuration rather than a per-call-site discipline.
