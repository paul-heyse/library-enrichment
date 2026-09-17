# Typer and Rich capability repository — reference

Read `SKILL.md` first; this is the layer underneath it.

## What is pinned

| | Pin | Note |
|---|---|---|
| `typer` | 0.27.2 | released 2026-08-28, tag `0.27.2` (no `v` prefix) |
| `rich` | 15.0.0 | released 2026-04-12, tag `v15.0.0` (`v` prefix) |
| `typer._click` | vendored Click | ships inside typer 0.27.2; indexed as its own subject |
| Python | 3.14 | the capsule interpreter |
| Pygments | 2.21.0 | load-bearing: `Syntax` output depends on it |
| ast-grep | 0.45.3 | a hard gate in acquisition, not a warning |

**The declared pins are not a pin.** The resolved environment is 7 distributions and three of
them arrive through ranges — `pygments>=2.13.0,<3.0.0`, `markdown-it-py>=2.2.0`,
`annotated-doc>=0.0.2`. `manifests/resolved.json` is asserted against a fresh resolve on every
acquisition and a mismatch aborts naming both sides.

One capsule venv, not two. Typer depends on Rich by range, so resolving them separately would let
this index describe a Rich that Typer never runs against — which is exactly the fact the render
probes exist to pin. A Rich-only re-pin therefore changes the envelope key and invalidates
Typer's captures, and that is correct: bumping Rich changes what Typer's `--help` renders.

## Layout

```
SKILL.md          the router
reference.md      this file
content/
  index/*.tsv     headerless, sorted, tab-separated          -> rg
  model/*.json    one record-set per module, language: json  -> ast-grep
  api/*.md        one page per module, path by rule          -> Read
  catalogs/*.md   direct answers to fixed questions          -> Read
  topics/*.md     13 capability axes + 00-map.md             -> Read
  seams/*.md      11 seams + 00-map.md                       -> Read
  probes/         captures + 00-index.md                     -> Read
  corpus/         855 verbatim upstream files                -> rg
  PROVENANCE.json pins, counts, per-file sha256
queries/          sgconfig.yml, rules/, rule-tests/, fixtures/
build/            acquire.py -> build.py -> verify.py
```

### Path construction

A rule, not a lookup. No table exists because none is needed: take the module part of the
canonical path, verbatim.

| Canonical path | Page |
|---|---|
| `rich.table.Table` | `content/api/rich.table.md` |
| `typer.core.TyperGroup` | `content/api/typer.core.md` |
| `typer._click.utils.echo` | `content/api/typer._click.utils.md` |

## Index schemas

Headerless, sorted, tab-separated; free text has tabs and newlines scrubbed. An empty table is an
empty file, not a blank line — `wc -l` on a file holding one newline reports 1, which reads as
one finding for a table whose whole purpose is to be empty.

**`symbols.tsv`** is the one that differs most from the sibling repositories:

| # | Column | Notes |
|---:|---|---|
| 1 | canonical | where it is defined |
| 2 | preferred | **the spelling to import.** Guaranteed runtime-importable — see limit 1 |
| 3 | kind | class, function, attribute, reexport |
| 4 | **subject** | `typer`, `rich` or `click`. A `click` here is Typer's vendored copy |
| 5 | api_page | the prose page |
| 6 | nameable | whether any writable spelling exists |
| 7 | **reachability** | how you meet it when it is not nameable |
| 8 | **reached_via** | the spelling(s) that deliver it |
| 9 | in_all | declared in the containing module's exports |
| 10 | alias_count | how many paths reach it |
| 11 | summary | first line of the docstring |

Columns 4, 7 and 8 have no fastmcp counterpart. Column 4 is what makes three subjects one index:
`typer._click.Command` and `click.Command` are different classes with the same name, so a row
that cannot say which library it describes is worse than no row. Columns 7 and 8 are what make
201 vendored-Click items legible rather than merely marked unavailable.

`reachability` takes one of seven values:

| Value | Meaning | Count |
|---|---|---:|
| `importable` | a writable spelling exists; column 2 is it | 632 |
| `subclassed-via` | a class you *can* name inherits from it | 5 |
| `subtype-of` | you handle it through an ancestor you can name | 7 |
| `received` | it arrives in a parameter of something you can name | 14 |
| `declared` | what your annotation becomes; no graph edge carries this | 14 |
| `registered-via` | reached only through a registrar function | 5 |
| `internal` | genuinely never met | 325 |

The other tables follow fastmcp's schemas: `members.tsv` (owner · name · kind · declared_on ·
labels · signature · summary), `parameters.tsv` (function · ordinal · position · name · kind ·
required · annotation), `aliases.tsv`, `bases.tsv`, `descendants.tsv`, `locations.tsv`,
`satisfies.tsv`, `mro.tsv`, `overrides.tsv`, `overloads.tsv`, `callers.tsv`, `references.tsv`,
`imports.tsv`, `inferred.tsv`, `unannotated.tsv`, `type-coverage.tsv`, `checker-types.tsv`,
`types-used.tsv`, `external-refs.tsv`, `usage.tsv`, `decorators.tsv`, `suppressions.tsv`,
`conditional.tsv`, `extras.tsv`, `unresolved.tsv`, `not-indexed.tsv` (deliberately empty).

**`behaviors.tsv`** is new here: probe · subject · topic · verdict · question · construction ·
expect · control_expect · normalise · channel · digest · capture path.

## The probe protocol

This repository executes its subjects. `ast-grep-ripgrep/build/probes.py` says the sibling
library repositories cannot — "the only thing a builder can do with them is read their documented
surface" — and that is true of DataFusion and delta-rs. Typer and Rich render, deterministically,
into a `StringIO`, so a claim about their behaviour can be an observation.

**A probe without a control is not evidence.** A rendering that looks right proves only that
something rendered.

**`differs` is the default control mode, not `opposite`.** For a program you can demand an error;
for a renderer "must fail" is almost never an exception. `NO_COLOR=1` does not raise — it quietly
drops the colour and keeps the bold. The finding is the difference, so the difference is asserted.

**Verdicts.** `confirmed` — probe and control both came out as expected and differed. `recorded` —
a shape probe with nothing to falsify; weaker, and labelled. `inconclusive` — demonstrates
nothing, never reported as a pass. `refuted` — the claim was wrong.

**Determinism is built in four layers**, because no single one covers the surface:

1. The process environment is **replaced**, not scrubbed — a literal dict from the manifest,
   inheriting nothing, with `HOME` pointing at a scratch directory so `typer.get_app_dir()` is
   stable. This is the only layer that reaches `UNICODE_VERSION` and `DATABRICKS_RUNTIME_VERSION`,
   which are read from `os.environ` directly, and Typer's `rich_utils` constants, which are frozen
   at first import.
2. `Console(_environ=<literal dict>)` on every Rich probe — Rich's own injection point.
3. Explicit `width`, `color_system`, `force_terminal`, `legacy_windows`, `get_time`,
   `get_datetime`.
4. One subprocess per probe, `PYTHONHASHSEED=0`, because Rich's Unicode loader is cached and
   `rich_utils`'s constants are frozen at import, which would otherwise make probe order
   load-bearing.

Verified: two consecutive runs are byte-identical, and a run with `COLUMNS=40 FORCE_COLOR=1
NO_COLOR=1 UNICODE_VERSION=12.0.0 TERMINAL_WIDTH=100 TERM=dumb GITHUB_ACTIONS=true` set in the
parent shell is byte-identical to a clean one.

**Two capture channels**, because normalising one destroys the other. `plain` is written
byte-exact and is greppable. `ansi` is written as the `repr()` of the string — raw escapes break
`rg`, are re-interpreted by any terminal that cats the file, and would defeat the transferability
scan that reads every shipped file as text.

**Normalisation is never silent.** Where a row names a normaliser (`address`, `path`, `duration`),
the row says so and the page repeats it.

## Query recipes

```
rg -P '\tclick\t' content/index/symbols.tsv | cut -f1,7 | head
rg -P '\tdeclared\t' content/index/symbols.tsv | cut -f1,8
rg -P '^rich\.progress\.Progress\t' content/index/members.tsv | cut -f2,6
rg -P '^typer\.Typer\t|^typer\.main\.Typer\t' content/index/symbols.tsv
rg -P '^typer\.main\.Typer\.__init__\t' content/index/parameters.tsv | cut -f4,7
rg -P '\trich\.console\.RichCast\t' content/index/satisfies.tsv | cut -f1,3
rg -P '\tmarkup-and-escaping\t' content/index/behaviors.tsv | cut -f1,5
rg -c '' content/index/behaviors.tsv
rg -F 'ConsoleRenderable' content/seams/renderable-protocol.md
rg -l 'rich_markup_mode' content/corpus/typer
sort content/index/not-indexed.tsv | wc -l
ast-grep scan -c queries/sgconfig.yml --filter '^project-' queries/fixtures
```

## Rule inventory

All `severity: hint`, none carries a `fix`. They are questions about your code, not proven
defects: ast-grep resolves no imports and no types.

| Rule | Family | Generated from |
|---|---|---|
| `project-vendored-click-import` | project | `catalogs.json` current fact `vendored-click` |
| `project-typer-slim-install` | project | current fact `typer-slim-is-a-wrapper` |
| `project-rich-version-attr` | project | Rich breaking-change row 14.3.4 |

All three are generated by `build/queries.py` rather than typed, so a re-pin rewrites them and
a hand-edit would be lost silently. The two Typer rules come from present-tense facts about
0.27.2 rather than from version history: this index describes the pinned release and does not
catalogue what earlier Typer versions did. Every rule has a `rule-tests/` entry
with both `valid:` and `invalid:` fixtures — a rule with no valid fixture has not been shown to
be precise.

## Known limits — what this index does **not** claim

1. **A preferred spelling is a promise that the import runs, and nothing more.** It is not a
   promise that upstream documents it. The guarantee exists because Griffe records
   `if TYPE_CHECKING:` imports as ordinary aliases, and trusting that made `rich.Console` the
   recommended spelling for `rich.console.Console` — a line both `ty` and `pyrefly` accept and
   Python rejects. Guarded spellings are excluded from column 2 and kept as their own alias kind.

2. **Animated and time-dependent rendering is not captured.** `Progress`, `Live`, `Status`,
   `Spinner` and `track` depend on elapsed time and refresh rate. No construction produces stable
   bytes, and a normalised capture would be a fiction dressed as evidence. Probe R003 covers the
   deterministic `disable=True` path, which is what belongs in CI; for the rest, upstream's own
   `content/corpus/rich/tests/test_progress.py` is the better answer.

3. **Terminal capability negotiation on a real TTY is not captured.** Every probe writes to a
   `StringIO`, which is not a terminal, and pins `force_terminal` and `color_system` explicitly.
   `catalogs/env-vars.md` records precedence as observed through that path. What a real TTY
   negotiates — `TERM` parsing, Windows VT mode, the 8/256/truecolor ladder — is indexed from
   source and never asserted.

4. **`Syntax` output depends on the installed Pygments, not on Rich.** Lexers and token
   boundaries change between Pygments releases; a captured highlight would be a claim about
   Pygments wearing Rich's name. Pygments is pinned at 2.21.0 in `resolved.json`.

5. **`Traceback` is indexed for structure, not for bytes.** A real traceback carries absolute
   paths, an interpreter version and frame counts that differ per machine — and the paths are a
   *transferability* leak into `content/`, not merely a determinism one. Probe R002 raises inside
   a synthetic filename so no real path can reach the capture, and the normalisation is recorded
   on the row.

6. **Windows-only code paths are never executed.** `rich/_win32_console.py`,
   `rich/_windows_renderer.py`, `legacy_windows=True`, the box substitution tables and Typer's
   conditional `colorama` dependency are indexed from source and carry no probe. Every probe
   passes `legacy_windows=False` explicitly, which is part of why they are deterministic. A box
   glyph shown here may be substituted on a legacy Windows console.

7. **Jupyter rendering is not verified.** `JupyterMixin`, `_repr_mimebundle_`, `rich.jupyter`,
   `JUPYTER_COLUMNS` and `force_jupyter` are indexed; no probe imports IPython and `is_jupyter()`
   is False in all of them. `seams/jupyter.md` is marked `supported: observed-only` for this
   reason.

8. **`rich._unicode_data` (23 generated width tables) and `rich._emoji_codes` are excluded from
   the API index.** They are data, not API, and indexing them would be roughly 40% of the symbol
   count in rows nobody queries. They are excluded from the coverage expectation as well as from
   the walk — excluding them from only one would make the coverage assertion pass vacuously.
   Their behaviour is covered: `UNICODE_VERSION` is in `env-vars.tsv` and probed through
   `cell_len` (E005).

9. **Shell completion is indexed, not run.** `shellingham` inspects the parent process and
   `--install-completion` writes to the user's home directory. Neither is safe or stable to
   execute in a build.

10. **The 304 `docs_src/` tutorial files are indexed, never executed.** They are runnable, and
    executing them would mean importing 304 third-party modules inside the build — a materially
    different safety posture from Griffe's `allow_inspection=False`, which exists precisely to
    avoid that. `corpus.tsv`'s `kind` column does not imply otherwise.

11. **There is no call-graph question this index answers well.** `callers.tsv` and
    `references.tsv` exist because the checker produced them cheaply, but nobody asks who calls
    `Segment.split_cells`. The useful cross-reference here is upstream usage, answered by `rg`
    over `content/corpus`.

12. **`typer._click` rows are facts about Typer's copy at this pin.** Upstream Click has moved on
    independently and will diverge further. Never cite one as a fact about the `click` package.

## Rebuilding

See `build/README.md`. In short: `python3 acquire.py` once per pin (network, `uv`, Griffe, `ty`,
pyrefly, and the probe execution), then `python3 build.py` (offline: standard library plus the
`ast-grep` binary), then `python3 verify.py`.
