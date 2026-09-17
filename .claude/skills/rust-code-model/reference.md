# Reference: layout, schemas, recipes and limits

Everything below is generated from pinned inputs by `build/build.py` and checked by
`build/verify.py`. Counts are from the current build; `content/PROVENANCE.json` is authoritative.

## What is pinned

| Subject | Pin | How |
|---|---|---|
| toolchain | `nightly-2026-09-13` | `rustc +… --version` must contain `1.100.0-nightly` and `809936eac` |
| rustdoc format emitted | 61 | a document is produced locally and its `format_version` read |
| `ra_ap_*` (9 crates) | 0.0.352 | each payload's `crate_version`; the family must be uniform |
| `rustdoc-types` | 0.61.0 | payload `crate_version` |
| `cargo_metadata` | 0.23.1 | payload `crate_version` |
| rustc source | `rust-lang/rust` @ `809936eac` | seven files fetched at the commit |
| rust-analyzer source | tag `2026-09-14` | nine files fetched at the tag |
| rustc-dev-guide | commit `238f1ffe` | no releases exist, so it is pinned by commit |
| ast-grep | 0.45.3 | the installed binary; the build refuses any other |
| rust-analyzer binary | **recorded, never asserted** | it reports a rustc release, not an `ra_ap` version |

## Layout

```
content/
  PROVENANCE.json        pins, counts, version readings, per-file sha256
  index/*.tsv            line-oriented projections              -> ripgrep
  model/<module>.json    library structure and signatures       -> ast-grep
  api/<module>.md        the full rustdoc prose, stored once    -> Read
  topics/<slug>.md       9 use-case pages + 00-map.md           -> Read
  layers/<layer>.md      7 layer pages: what each refuses       -> Read
  catalogs/*.md          4 direct-lookup tables                 -> Read / ripgrep
  corpus/                verbatim upstream: rustc MIR and dataflow source, the
                         rust-analyzer grammar and SyntaxKind, the rustc-dev-guide
                         MIR chapters, the rustdoc-types changelog, the Cargo book
queries/
  sgconfig.yml           never auto-discovered; always passed with -c
  rules/project/         rule-tests/ + __snapshots__/   utils/
build/                   the generator; standard library plus the pinned toolchain
```

## Index schemas

Tab-separated, sorted, one record per line, **no header row**. Tabs and newlines are scrubbed
from free text.

| File | Rows | Columns |
|---|---:|---|
| `questions.tsv` | 28 | `question · layer · entry_point · rejected · why_rejected · recipe · probe` |
| `layers.tsv` | 7 | `layer · subject · obtained_by · entry_point · answers · cannot_answer · needs_build · needs_network · stability · pins` |
| `behaviors.tsv` | 33 | `probe · layer · topic · verdict · exit · question · command · evidence · control · control_exit` |
| `version-oracles.tsv` | 8 | `subject · question · command · answers_for · trap · observed` |
| `unreachable.tsv` | 8 | `capability · exists_in · reachable · why · instead` |
| `compiler-views.tsv` | 15 | `view · flag · shows · loses · probe` |
| `mir-vocabulary.tsv` | 105 | `category · variant · purpose · meaning` |
| `dataflow.tsv` | 27 | `owner · kind · name · required_or_provided` |
| `syntax-kinds.tsv` | 331 | `kind · class · text · in_grammar` |
| `ast-nodes.tsv` | 188 | `node · syntax_kind` |
| `symbols.tsv` | 1,026 | `canonical_path · kind · crate · layer · api_page · n_aliases · n_methods · summary` |
| `methods.tsv` | 4,378 | `owner_path · method · via_trait · signature · summary` |
| `impls.tsv` | 571 | `trait_path · implementor_path · crate` |
| `foreign-impls.tsv` | 4,052 | `trait_path · implementor · nameable · crate` |
| `aliases.tsv` | 542 | `access_path · canonical_path · kind` |
| `features.tsv` | 18 | `crate · feature · enables · default` |
| `unnameable.tsv` | 24 | `path · kind · crate · n_methods` |
| `unresolved.tsv` | 162 | `access_path · target` — re-exports leaving the indexed set |

`questions.tsv`'s `rejected` and `why_rejected` are the load-bearing columns. A row that only
named the right layer would leave a reader's first instinct untouched.

`behaviors.tsv`'s `verdict` is `confirmed` when a probe and its control both came out right,
`recorded` for a shape probe with nothing to falsify, `blocked` when a tool was absent, and
`divergent` when a control failed to discriminate — which `verify.py` treats as a failure.

## Query recipes

All paths are relative to the skill directory.

```
rg -P '\tsyntax\t' content/index/questions.tsv
rg -P '^mir\t' content/index/layers.tsv
rg -P '^TerminatorKind\t' content/index/mir-vocabulary.tsv
rg -P '^Analysis\tmethod\t' content/index/dataflow.tsv
rg -P '\tnode\t' content/index/syntax-kinds.tsv
rg -P '^ra_ap_hir::semantics::Semantics\t' content/index/methods.tsv
rg -P '\tconfirmed\t' content/index/behaviors.tsv
rg 'rust-analyzer' content/index/version-oracles.tsv
ast-grep scan -c queries/sgconfig.yml --filter '^project-' content/corpus
```

To find out what a compiler layer actually contains, read the pinned source rather than
searching for documentation that does not exist: `content/corpus/rustc/mir-syntax.rs` defines
every statement, terminator and rvalue, and `content/corpus/rustc/dataflow-framework.rs`
defines the `Analysis` trait.

## Rule inventory

| Rule | Runs against | A hit means |
|---|---|---|
| `project-rustdoc-format-unasserted` | your repository | rustdoc JSON deserialized without checking `format_version` |
| `project-rustdoc-format-pinned-url` | your repository | a docs.rs `/json/<n>` URL, which is exact-match and 404s otherwise |
| `project-cargo-metadata-no-format-version` | your repository | `cargo metadata` without `--format-version` |
| `project-floating-nightly` | your repository | an undated toolchain override, which pins nothing |

All carry `severity: hint` and no `fix`, and all four have valid/invalid fixtures with recorded
snapshots.

## Known limits

1. **`ra_ap_hir` is 12.84% documented upstream**, against 95.93% for `rustdoc-types`. The index
   inherits that: many rows are a signature and nothing else. That is the upstream state, not a
   failure of extraction. `content/corpus/rust-analyzer/hir-lib.rs` is vendored for this reason.

2. **162 access paths re-export out of the indexed set** — mostly into `ra_ap_hir_ty`,
   `ra_ap_hir_expand`, `ra_ap_hir_def` and `ra_ap_ide_db`, which are deliberately not indexed.
   `unresolved.tsv` is a boundary, not a gap: use the owning crate's own documentation there.

3. **No rustdoc JSON exists for any rustc-internal crate.** The `rust-docs-json` rustup
   component ships `std`, `core`, `alloc`, `proc_macro`, `test` and `std_detect` only. The MIR
   and dataflow layers are indexed from pinned source plus executed probes. For the full API
   surface, the `rustc-docs` rustup component is the escape hatch — about 36 MB of HTML, not
   vendored here.

4. **Textual MIR is a view, not an interface**, and says so in its own first line. Anything that
   parses `-Zunpretty=mir` output is reading something upstream has reserved the right to
   change silently.

5. **Probe verdicts describe the pinned toolchain on the machine that built this.** They are
   observations, not guarantees. A `blocked` verdict means a tool was absent, never that the
   behaviour is absent.

6. **`ra_ap_*` is pinned to one weekly release.** Anything built a week later is a different
   library, and the tag-to-version correspondence is matched **by publication date** — an
   inference, recorded as one, because upstream publishes no mapping.

7. **`cargo_metadata` 0.23.1 was published 2025-11-12** after a roughly bimonthly cadence.
   Whether the gap since reflects stability or reduced maintenance is **unverified** and
   recorded as unverified. It may lag fields Cargo already emits.

8. **The `SyntaxKind` classification is derived, not authoritative.** A kind has text exactly
   when `SyntaxKind::text` returns one; kinds with neither text nor a grammar entry are written
   as `token-no-fixed-text`, which describes the evidence rather than guessing intent.

## Rebuilding

```
python3 build/acquire.py        # network; writes build/acquired and build/.cache
python3 build/build.py          # offline; regenerates content/ from acquired/
python3 build/verify.py         # all eight checks; exit 1 on any failure
```

`acquire.py` is never imported by `build.py`. That separation is what makes the determinism
check meaningful: if acquisition could run inside the build, a rebuild could pick up different
bytes from the network and still look reproducible.

`verify.py --skip-rebuild` checks the recorded digests without regenerating, which is the fast
path while editing. The full run rebuilds and compares byte-for-byte, re-runs all 33 behaviour
probes with their controls, re-executes every recipe in `questions.tsv`, and runs every command
printed in a fenced block in `SKILL.md`, this file, and the topic and layer pages.
