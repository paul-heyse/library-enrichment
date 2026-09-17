# Reference: layout, schemas, rule inventory, known limits

Everything below is generated from pinned sources by `build/build.py` and checked by
`build/verify.py`. Counts are registered in `build/counts.json` and read back from the artifacts
that produce them — check 9 fails the build if a number here drifts from what the index holds.
`content/PROVENANCE.json` is authoritative.

## What is pinned

| Group | Crates | Version | Captured from |
|---|---:|---|---|
| subject | 2 | 55.0.0 | docs.rs **and** a local `--document-private-items` capture |
| wiring | 5 | `tracing` 0.1.44, `tracing-core` 0.1.36, `tracing-subscriber` 0.3.23, `tracing-futures` 0.2.5, `tracing-attributes` 0.1.31 | docs.rs |
| otel | 4 | `tracing-opentelemetry` 0.32.0, `opentelemetry`/`_sdk`/`-otlp` 0.31.0 | docs.rs |

11 crates, 502 canonical items. Corpus: `datafusion-contrib/datafusion-tracing` tag `55.0.0`,
resolved commit `d8f205bf52ef`, recorded in `PROVENANCE.json` — a tag is a pin only while nobody
moves it, and a corpus whose provenance lives solely in the manifest cannot be checked from the
published tree.

Local capture toolchain: `nightly-2026-09-13`, rustc 1.100.0-nightly `809936eac`, rustdoc format
61. Probe toolchain: stable **1.98.1** — a probe is a claim about what a consumer observes, and a
consumer builds on stable.

**docs.rs served this set at five different rustdoc format versions**: 61 for the subjects,
`tracing` and `tracing-core`; 60 for `tracing-futures` and `tracing-attributes`; 59 for
`tracing-subscriber`; and **56 for all four OpenTelemetry crates** — older than any sibling
repository in this family parses. 56 was not accepted because it is close enough:
`acquire.assert_document_shape` checks that every field `model.py` reads is present and spelled
the way it expects, so a format that merely parses far enough to produce confident nonsense is a
stop rather than a warning.

## Layout

```
content/
  PROVENANCE.json        pins, counts, corpus commits, per-file sha256
  index/*.tsv            line-oriented projections          -> ripgrep
  model/<module>.json    structure, signatures, visibility  -> ast-grep
  api/<module>.md        the full doc prose, stored once    -> Read
  seams/<slug>.md        8 seams, each with its refusals    -> Read
  topics/00-map.md       the router, rendered               -> Read
  catalogs/*.md          options, macros, spans, compat...  -> Read / ripgrep
  corpus/                upstream source, tests, traces, examples, docs, verbatim
  probes/00-index.md     every executed probe and its control
queries/
  sgconfig.yml           always passed explicitly with -c
  rules/{model,corpus,project,generated}/
  utils/                 shared and parameterised sub-rules
  rule-tests/            valid/invalid fixtures + snapshots
build/                   the generator; stdlib + ast-grep, plus cargo for acquire and probes
  acquired/              COMMITTED. The private capture cannot be re-served by anyone.
```

### Path construction

From a canonical path, take everything before the last `::` and replace `::` with `.`:
`datafusion_tracing::options::InstrumentationOptions` →
`content/api/datafusion_tracing.options.md` and `content/model/datafusion_tracing.options.json`.
Anchors within a prose page are the lowercased item name.

## Index schemas

Tab-separated, sorted, no header row.

**`symbols.tsv`** — `canonical_path · kind · crate · subject · visibility · reached_via ·
api_page · alias_count · method_count · summary`

Columns 4–6 have no counterpart in the sibling repositories. `subject` is what makes three crate
groups one index. `visibility` and `reached_via` are the point of the whole build:

| Value | Meaning |
|---|---|
| `supported` | reachable by walking the crate root; this is what docs.rs renders |
| `doc-hidden` | in the document, but the source marks it `#[doc(hidden)]` |
| `reachable-undocumented` | rustdoc calls it public, no published artifact carries it, and a supported item hands it to you |
| `internal` | everything else — including types rustdoc calls public that nothing public yields |

**`methods.tsv`** — `owner_path · method · via_trait ("-" if inherent) · visibility ·
signature · summary`. **16 rows carry `reachable-undocumented`.** Derived `Clone`/`Default`/
`Debug` methods are filtered, exactly as the model records filter them.

**`macros.tsv`** — `macro · arm_ordinal · arm · requires_options · accepts_target ·
accepts_fields · accepts_state · level · family`. 49 arms.

**`spans.tsv`** — `span · target · level · field_count · fields · parents · occurrences ·
scenarios · conditional_on · verdict · repaired`. 9 spans.

**`span-fields.tsv`** — `span · field · kind · occurrences · scenarios · basis · gate ·
example · verdict · repaired`. 69 rows. `kind` ∈ `metric`, `otel`, `object-store`,
`instrumentation`, `custom`, `harness`. `basis` ∈ `option`, `node-dependent`, `unconditional`,
`unexplained` — and `gate` is filled only when an option is genuinely the reason.

**`scenarios.tsv`** — `scenario · query · options · trace`. The natural experiment the
conditionality column rests on: upstream's scenarios differ one option at a time.

**`previews.tsv`** — `node · scenario · ordinal · lines · width · style · capture`. 35 renders.

**`compatibility.tsv`** — `version · published · yanked · datafusion · msrv · edition ·
opentelemetry · opentelemetry_sdk · opentelemetry-otlp · tracing-opentelemetry · manifest`.
16 releases.

**`behaviors.tsv`** — `probe · topic · verdict · question · construction · expect · control ·
control_expect · mode · evidence`. **12 probes: 11 confirmed, 1 recorded.** Verdicts are
`confirmed`, `refuted`, `recorded`, `divergent` (fails the build) and `blocked`. Two of them do
not observe spans at all: `B001` and `B002` interrogate the two rustdoc captures, and they are
the tripwire on this repository's central claim.

**`questions.tsv`** — `question · area · entry_point · rejected · why_rejected · recipe ·
probe`. 18 rows; `rejected` is the load-bearing column.

Also `aliases.tsv`, `impls.tsv`, `features.tsv`, `unreachable.tsv` (`name · crate · why ·
instead`) and `unresolved.tsv`.

## Catalogs

| File | Holds |
|---|---|
| `options.md` | both option types and every builder method, with visibility. The only place the 16 undocumented methods are written down |
| `macros.md` | 49 invocation arms, the two families, and the custom-field coupling |
| `spans.md` | the span contract, the open metrics vocabulary, the caller-target finding, and the malformed upstream snapshot |
| `compatibility.md` | 16 releases × the opentelemetry pairs, and the off-by-one rule |
| `crate-map.md` | 11 crates by group, with the five rustdoc formats docs.rs served |
| `not-reachable.md` | what exists and cannot be used, in three kinds, each with an alternative |

## Rule inventory

11 rules, each with `valid` and `invalid` fixtures under `queries/rule-tests/`. All carry
`severity: hint`: a match is a finding of interest, never a violation.

| Rule | Family | Answers |
|---|---|---|
| `model-undocumented-but-reachable` | model | the 16 methods plus the 2 builders |
| `model-doc-hidden` | model | public only because a macro needs it |
| `model-builder-methods` | model | chainable configuration |
| `corpus-macro-invocation` | corpus | how upstream invokes the macros |
| `corpus-options-chain` | corpus | which builder methods anyone reaches for |
| `corpus-subscriber-layer` | corpus | the layer stacks upstream ships |
| `project-instrument-rule-not-last` | project | a rule registered after the instrumentation rule |
| `project-instrumented-exec-referenced` | project | use of a type that is private by design |
| `project-object-store-not-instrumented` | project | a store registered unwrapped |
| `project-options-without-preview` | project | an options chain that records no preview |
| `project-doc-hidden-api` | generated | qualified use of a `#[doc(hidden)]` item |

`project-doc-hidden-api` is regenerated from the model on every build, so it cannot go stale.

## Known limits

Eight things this repository deliberately does not claim.

1. **`datafusion.metrics.*` is an open vocabulary.** Names are built with `format!()` from
   whatever node ran. Six fields appeared in every metrics-enabled scenario and 35 more in one
   scenario each. A field absent from `span-fields.tsv` is **unobserved, not unavailable**;
   `basis: node-dependent` says so per row rather than pinning it on a correlated option.

2. **Snapshot-derived rows are `recorded`, not `confirmed`.** They are upstream's observations
   under upstream's harness and carry no control. Only a probe promotes a row.

3. **`reachable-undocumented` describes this release.** If upstream re-exports the builders the
   column collapses to nothing. Probe `B001` is the tripwire, not a guarantee.

4. **One upstream snapshot is not valid JSON**, and the repair is named rather than heuristic.
   `06_object_store_all_options_trace.snap` carries `Some\("…"\)` inside a JSON string — two
   invalid escapes and an unescaped quote — because upstream's insta filter's replacement text
   carries the regex escapes `\(` `\)` verbatim and drops the `\\"` its pattern matched. This
   build substitutes that one literal and flags every row that depends on it. Any other
   unparseable snapshot stops the build.

5. **rustdoc emits no macro bodies, at any format version.** Every arm ends `=> { ... }` in the
   document too. `macros.tsv` carries the grammar; the expansion is established by
   `corpus/source/` and by probes, never by the index.

6. **Two facts cannot come from rustdoc at all.** `#[doc(hidden)]` is not in `attrs` at format
   61 — measured: `new_instrument_rule` comes back with `attrs: []`. And `instrument_session_state`
   is `pub use`d by `lib.rs` and appears in **neither** capture, hosted or private. Both are read
   from the source corpus, and the second is recorded in `unreachable.tsv` rather than dropped.

7. **Probe verdicts describe the pinned toolchain on the machine that built this.** They are
   observations, not guarantees. `blocked` means a prerequisite was missing, never that the
   behaviour is absent; `divergent` means the control did not discriminate, and fails the build.

8. **ast-grep resolves no imports and cannot parse TOML** — measured: `--lang toml` is rejected
   by 0.45.3. So `project-doc-hidden-api` matches qualified paths only,
   `project-instrument-rule-not-last` decides only the two cases syntax can decide, and
   manifest-level checks are ripgrep recipes because an ast-grep rule would be the wrong tier.

DataFusion, Arrow and `object_store` are not indexed; their access paths are in
`unresolved.tsv`. That is a boundary, not a gap.

## Rebuilding

```bash
python3 build/acquire.py    # network + cargo + the pinned nightly. Once.
python3 build/build.py      # offline, from build/acquired/
python3 build/verify.py     # nine checks
python3 build/verify.py --execute-probes   # also re-runs the probes; compiles DataFusion
ast-grep test -c queries/sgconfig.yml      # rule fixtures alone
```

The skill directory is self-contained: Python 3.11+ standard library plus the `ast-grep` binary,
with cargo and a toolchain needed only by `acquire.py` and `probes.py`. No imports from any host
repository — `verify.py` check 5 enforces it across the generators, the rules and the prose.
Copy it anywhere and rebuild.
