# Reference: layout, schemas, limits and recipes

Everything below is generated from pinned sources by `build/build.py` and checked by
`build/verify.py`. Counts are from the current build; `content/PROVENANCE.json` is authoritative.

## What is pinned

| Crate set | Version | Crates | Source |
|---|---|---:|---|
| ruff, product line | 0.16.7 | 3 | docs.rs rustdoc JSON |
| ruff, library line | 0.0.13 | 28 | docs.rs rustdoc JSON |
| pyrefly | 1.3.1 | 11 | local rustdoc from tag `1.3.1` |

Both ruff lines were published together on 2026-09-10, and `ruff_linter 0.16.7` depends on all 18
library crates at `^0.0.13`. They are one release wearing two version numbers.

pyrefly publishes nothing usable: `crates.io/crates/pyrefly` is a name squat at `0.0.1`
(2025-03-13, "Coming soon to crates.io"), and every `pyrefly_*` crate is absent. Its rustdoc JSON
is therefore produced locally from commit `3e3177d0`, with `--document-private-items`, because the
clap structures that define the CLI are private and the surface is otherwise invisible.

The producer is `nightly-2026-08-18` (rustc 1.100.0-nightly `8fa1c96cf`), **not** the newer
`nightly-2026-09-13` that `config/toolchains.toml` names as current. Measured 2026-09-15:
`allocative 0.3.6` fails to compile on the newer one with `E0119`, because its `build.rs` detects
nightly by string-matching `rustc --version`, enables `feature(never_type)`, and adds
`impl Allocative for !` which now conflicts with its own `impl for Infallible`. Both producers
emit `format_version` 61, so one parser adapter reads both crate sets.

Tool catalogs come from capsule-pinned installs of `ruff==0.16.7` and `pyrefly==1.3.1`, never from
`PATH` — this workstation carries ruff 0.14.4 and pyrefly 0.51.1, and a capture from PATH would
have described different software than the index describes. The version is asserted before
anything is recorded.

Counts: 3,029 items · 12,520 methods · 1,358 impls · 643 aliases · 586 modules · 970 rules ·
497 CLI flags · 182 config keys · 144 error kinds · 144 conformance results · 28 extension
points · 13 topics · 10 catalogs · 10 ast-grep rules · 224 corpus files · 1,462 files total.

## Layout

```
content/
  PROVENANCE.json        pins, counts, per-file digests
  index/*.tsv            line-oriented projection            -> ripgrep
  model/*.json           one record per module               -> ast-grep (language: json)
  api/*.md               one prose page per module           -> Read
  traits/*.md            the 28 extension points             -> Read
  topics/*.md            13 capability axes + 00-map.md      -> Read
  catalogs/*.md          direct answers to fixed questions   -> Read
  corpus/ruff/           16 upstream doc pages, verbatim     -> ripgrep
  corpus/pyrefly/        47 upstream doc pages, verbatim     -> ripgrep
  corpus/conformance/    161 files: the suite and its results -> ast-grep
queries/
  sgconfig.yml           always passed explicitly with -c
  rules/{model,corpus,project,generated}/
  utils/                 shared JSON matchers
  rule-tests/            fixtures and snapshots
build/
  acquire.py             capsule, clone, nightly rustdoc, tool oracles  (network + cargo + uv)
  oracles.py             what each tool says about itself
  fetch.py model.py render.py emit.py link.py topics.py traits.py catalogs.py queries.py
  build.py               offline: stdlib + ast-grep only
  verify.py probes.json  five checks, 36 probes
  acquired/              the captured documents, git-tracked
```

### Path construction

From a canonical path, take everything before the last `::` and replace `::` with `.`:

| Canonical path | Prose | Records |
|---|---|---|
| `ruff_python_ast::generated::Expr` | `content/api/ruff_python_ast.generated.md` | `content/model/ruff_python_ast.generated.json` |
| `pyrefly_types::types::Type` | `content/api/pyrefly_types.types.md` | `content/model/pyrefly_types.types.json` |

## Index schemas

Tab-separated, sorted, no header row. Tabs and newlines are scrubbed from free text.

| File | Rows | Columns |
|---|---:|---|
| `symbols.tsv` | 3,029 | `canonical · kind · crate · api_page · n_aliases · n_methods · summary` |
| `methods.tsv` | 12,520 | `owner · method · via_trait ("-" if inherent) · signature · summary` |
| `impls.tsv` | 1,358 | `trait · implementor · implementor_crate` |
| `aliases.tsv` | 643 | `access_path · canonical · kind` |
| `features.tsv` | 36 | `crate · feature · enables · "default"` |
| `coverage.tsv` | 58 | per-crate documentation coverage |
| `unnameable.tsv` | 409 | items that exist but cannot be imported |
| `unresolved.tsv` | 5 | access paths leaving the indexed set |
| `foreign-impls.tsv` | 13,302 | `trait · implementor · nameable · crate` |
| `rules.tsv` | 970 | `code · name · linter · fix · status · since · source_file · enum_variant` |
| `config.tsv` | 182 | `tool · key · type · default · deprecated · doc` |
| `cli.tsv` | 497 | `tool · subcommand · group · flag · value · summary` |
| `error-kinds.tsv` | 144 | `tool · PascalCase · kebab-case` — the checker's filtering vocabulary |
| `conformance.tsv` | 144 | `tool · test file · pass\|fail · deviations` |

`rules.tsv` is the join that matters. All 970 rules the tool reports map to a variant of
`ruff_linter::codes::Rule`, which carries exactly 970. The join folds case and separators, not
PascalCase: naive PascalCase loses 33 rules, every one an acronym — `blanket-noqa` is
`BlanketNOQA`, `non-pep585-annotation` is `NonPEP585Annotation`, `io-error` is `IOError`. The
build fails if two variants ever collide under folding.

## Catalogs

| File | What it answers |
|---|---|
| `cross-references.md` | who calls this, where is this referenced — the Glean predicate inventory |
| `import-graph.md` | what `ruff analyze graph` does and the four ways it is narrower than its name |
| `rules.md` | all 970 rules by status and linter, joined to the implementing variant |
| `structured-outputs.md` | every machine-readable emitter of both tools |
| `protocols.md` | LSP and TSP capabilities, from the servers' own `initialize` replies |
| `config-options.md` | ruff's 182 keys with types and defaults |
| `crate-map.md` | which crate owns what, and which version line it is on |
| `cli-surface.md` | 497 flags across 12 command surfaces, parsed from `--help` |
| `error-kinds.md` | the checker's 144 error kinds, in both spellings |
| `conformance.md` | 139 of 144 typing-conformance tests pass; the 5 that do not, named |

## Query recipes

**Who calls this?**
```bash
pyrefly check --report-glean ./glean-out
jq -r '.[] | select(.predicate=="python.CalleeToCaller.4")' ./glean-out/*.json
```

**Was this rule removed, or is it merely preview?**
```bash
rg -P '^UP027\t' content/index/rules.tsv
rg -P '\tPreview\t' content/index/rules.tsv | wc -l
```

**Which Rust item implements a rule?**
```bash
rg -P '^F401\t' content/index/rules.tsv | cut -f7,8
```

**Does this API exist, and in which crate and version line?**
```bash
rg -i 'sourceorder|semanticmodel' content/index/symbols.tsv | cut -f1,2,3
```

**What does this type let me do?**
```bash
rg -P '^pyrefly_types::types::Type\t' content/index/methods.tsv | cut -f2,4
```

**What must I implement to plug in?**
```bash
rg -P '^ruff_python_ast::visitor::Visitor\t' content/index/impls.tsv | cut -f2
```

**What is this setting's default and type?**
```bash
rg -P '^ruff\tlint\.' content/index/config.tsv | cut -f2,3,4
```

**Which flags does a subcommand take, and in which group?**
```bash
rg -P '^ruff\tcheck\t' content/index/cli.tsv | cut -f3,4,5
rg -P '\tConfig Overrides\t' content/index/cli.tsv | cut -f2,4 | sort -u
```

**What error kind do I filter on, and how is it spelled?**
```bash
rg -i 'annotation|return' content/index/error-kinds.tsv | cut -f2,3
```

**Is this typing feature supported?**
```bash
rg -P '\tfail\t' content/index/conformance.tsv | cut -f2,4
rg -l 'assert_type' content/corpus/conformance/third_party | head
```

**Structured questions over the model:**
```bash
ast-grep scan -c queries/sgconfig.yml --filter '^model-' content/model
ast-grep test -c queries/sgconfig.yml
```

## Rule inventory

All `severity: hint`, none carries a `fix`, every one has fixtures.

| Rule | Family | Finds |
|---|---|---|
| `model-undocumented` | model | public items with no documentation (1,664) |
| `model-builder-methods` | model | chainable `with_*` builders (104) |
| `model-extension-points` | model | traits with implementors (58) |
| `model-ext-traits` | model | extension traits, invisible unless imported (4) |
| `model-async-methods` | model | async surface (2) |
| `model-deprecated` | model | items marked deprecated |
| `project-removed-rule-code` | generated | a ruff code removed at this release, in your Python |
| `project-ruff-check-without-output-format` | project | a `ruff check` whose output must be parsed as text |
| `project-analyze-graph-trusts-exit-status` | project | `check=True` on `analyze graph`, where exit 0 proves nothing |
| `corpus-assert-type` | corpus | the 576 conformance assertions stating a required type |

## Known limits

Eight things this repository deliberately does not claim.

1. **No symbol-to-example edges.** Neither project ships Rust example programs; their corpora are
   documentation and a Python conformance suite. The linking stage says so and records it rather
   than emitting pages that imply nothing demonstrates anything.
   1b. **`conformance.exp` is excluded from the corpus** — 470 KB of raw expected-error dumps that
   `conformance.result` already summarises per file. The summary and the per-file deviations are
   both indexed; the raw dump is not.
2. **`project-removed-rule-code` cannot see a `pyproject.toml`.** ast-grep 0.45 has no TOML
   grammar. For a config file, `rg` over `rules.tsv` is the answer.
3. **The CLI table is parsed from `--help`, not from a schema.** clap's layout is regular enough
   to parse and is not a contract; the flag and its value are separated on the two shapes clap
   actually emits, and a future layout change would show up as a row-count shift rather than an
   error. The captured text itself is in `build/acquired/oracles/`.
4. **The Glean payload is shape evidence, not scale evidence.** Its predicate counts were measured
   against a three-module fixture, so they establish what the report contains, not how big it gets.
5. **`--report-pysa` is recorded as unusable, not decoded.** It writes Cap'n Proto binary plus a
   copy of the bundled typeshed — 7,099 files for a three-module fixture — and reading it needs the
   `.capnp` schema from the source tree.
6. **Feature gating comes from `features.tsv` and nowhere else.** rustdoc JSON records no
   per-item `cfg(feature = ...)` at these format versions, so a signature never tells you.
7. **`pyrefly_derive` contributes no items.** It is a proc-macro crate; its rustdoc JSON is
   captured and validated, but the model admits no macro items at this schema.
8. **Three workspace members are excluded on purpose** — `pyrefly_bench_harness`,
   `pyrefly_lsp_test` and `pyrefly_wasm`. The first two are test scaffolding; the third is version
   `0.0.0`, and indexing it under the crate set's `1.3.1` would assert a version it does not carry.

## Rebuilding

```bash
python3 build/acquire.py                       # once; network + git + cargo + nightly + uv
python3 build/acquire.py --stage oracles       # just the tool catalogs; seconds
python3 build/acquire.py --check               # report whether the pinned tag has moved
python3 build/build.py                         # offline: stdlib + ast-grep only
python3 build/verify.py                        # all five checks
```

`acquire.py` is deliberately unreachable from `build.py`: `verify.py` re-runs the build to prove
determinism, and if acquisition were reachable that check would start compiling Rust.

**What "offline" means here, precisely.** Verified 2026-09-15: `build.py` completes with only
`python3` and `ast-grep` on `PATH` — no cargo, rustup, uv, git, zstd, ruff or pyrefly. The
pyrefly rustdoc and both tool catalogs come from `build/acquired/`, which is committed. The 31
ruff documents come from `build/.cache/`, which is **not** committed, because docs.rs can re-serve
them — so a fresh clone downloads roughly 37 MB on its first `build.py` and is fully offline after
that. Nothing needs a compiler or either binary at any point.
