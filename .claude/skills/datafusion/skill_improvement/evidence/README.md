# Retained assessment evidence

Captured and inspected 2026-09-18. This directory is part of the requested permanent reference.
It contains the scripts, queries, selected upstream source, inputs, and observed outputs used in
the assessment. It does not depend on the host application's service or data.

## What was executed

| Evidence | Result and limits |
|---|---|
| [Inventory](inventory.json) / [collection script](collect_evidence.py) | Counted the live indexes and recorded baseline hashes. Selected seven Arrow capability terms were present in the symbol index and absent from the topic-page text; this is a navigation gap, not capability absence. |
| [Existing verification log](logs/existing-skill-check.log) | Passed recorded integrity, 13 existing query-rule groups and 35 regex navigation probes. `--skip-rebuild` was used; its “reproduced” wording means digest agreement here, not a fresh rebuild. |
| [Documentation audit](documentation-audit.json) / [script](audit_documentation.py) | Read four cached rustdoc documents and retained 22 selected function fragments plus typed signatures. Module-doc counts are raw-document observations, not a public-coverage census. |
| [Source manifest](source-manifest.json) | Twelve exact-release published package archives from the skill's existing cache; selected files copied verbatim, with archive/file SHA-256, origin URLs, manifests and available license notices. |
| [Query runs](query-runs.json) / [runner](run_queries.py) | New declaration rule: one group, three matching/two non-matching fixtures passed. Nine declarations found in selected sources. Existing `collect` rule matched one non-DataFusion receiver control. |
| [Rust run receipt](rust-probe-run.json) / [log](logs/rust-probes.log) | Seven Arrow 59.3.0 behavior tests passed on Rust 1.98.1, Linux x86_64. Isolated Cargo workspace, offline dependency resolution, retained lockfile. No DataFusion runtime tests or performance measurements. |
| [Probe Clippy receipt](probe-clippy-run.json) | All probe targets passed Clippy with warnings denied. |
| [Python quality receipts](python-quality-runs.json) | Five authored scripts passed Ruff lint/format and ty with their sibling-module search path supplied explicitly. |
| [Bundle validation](bundle-validation.json) | Source digests, local document links, 60-crate coverage and preservation of 2,885 existing skill files checked. |

The raw fragments are in [rustdoc-fragments.json](rustdoc-fragments.json). Their item IDs are
artifact-local locators. They must not be treated as stable identities across rustdoc documents.

The source excerpts are whole selected source files rather than rewritten snippets. Manifests and
`.cargo_vcs_info.json` are retained where the archives contained them. These are sufficient to
inspect the claims made here, not to rebuild each entire upstream crate. Their acquisition was
from the pre-existing skill cache; origin URLs are provenance, not a claim of a fresh download.

## Reproduce

From `skill_improvement/` in the copied DataFusion skill:

```bash
uv run --no-project python evidence/collect_evidence.py
uv run --no-project python evidence/audit_documentation.py
uv run --no-project python evidence/run_queries.py
uv run --no-project python evidence/run_rust_probes.py
uv run --no-project python evidence/validate_bundle.py
```

The first two commands require the parent skill's original `build/.cache` inputs. The documentation
audit uses Python 3.14's standard-library zstd. The retained JSON and selected source files can be
read without either dependency. Query execution requires `rg` and `ast-grep`; versions are recorded
in [logs](logs/). Query results are syntax/lexical evidence, not behavioral verification.

Rust probes use their own [manifest](probes/Cargo.toml), [lockfile](probes/Cargo.lock), and
[tests](probes/tests/contracts.rs). `run_rust_probes.py` explicitly selects Rust 1.98.1 and writes
build products to disposable `.build-target/`. It uses `--offline` and uses `--locked` once the
lockfile exists. On a clean machine, the operator must first make that toolchain and the locked
dependencies available; an offline dependency failure is not a passing probe. No system-wide
installation or toolchain change was performed by this assessment.
The [resolved dependency profile](probe-dependency-profile.json) retains package identities and
the feature/dependency graph, along with the exact metadata command used to obtain it.

To prepare dependencies on another machine when network acquisition is intended:

```bash
cargo +1.98.1 fetch --locked --manifest-path evidence/probes/Cargo.toml
```

The build target is excluded from the reference bundle. Keep the source, lockfiles, JSON receipts,
logs, and query fixtures. Copying only this `evidence/` directory is enough to inspect the retained
evidence and run the independent Rust probes; inventory regeneration and original-rule comparisons
also need the parent DataFusion skill.

## Probe assertions and controls

| Test | Positive assertion | Discriminating control |
|---|---|---|
| `filter_null_is_not_selected_and_metadata_is_retained` | Null mask excludes its row; tested metadata survives; reusable mask agrees | Replacing null with true increases the selected count |
| `take_preserves_requested_order_duplicates_and_null_indices` | Requested order, duplicates and null output are preserved | Checked out-of-range index returns error |
| `cast_safe_true_nulls_failed_parses_but_false_errors` | Default safe cast nulls invalid numeric text | Strict option rejects it; valid-only strict cast succeeds |
| `row_order_changes_with_null_placement` | Nulls-first ordering is observed | Nulls-last changes the permutation |
| `row_decode_hydrates_dictionary_and_preserves_values` | Dictionary values decode as Utf8; equal values compare equal | Different values compare unequal |
| `empty_schema_exposes_different_batch_selection_contracts` | Empty-schema filter retains correct row count; take returns error | Ordinary batch take succeeds |
| `batch_construction_checks_top_level_nullability` | Non-nullable field rejects observed null | Nullable field accepts the same array |

These assertions are limited to the selected inputs/configuration. They do not establish all
supported Arrow datatypes, metadata behavior throughout DataFusion, bounded RSS, universal
performance, or the quality of the proposed skill for independent agents.

## Retained searches and structural queries

[commands.json](commands.json) records the exact lexical search and verification argument arrays,
working directories relative to the skill, exits and logs. This includes the multi-term Arrow
discovery query, source-contract candidate query, and runtime-default query.

[queries/rules/contract-entrypoints.yml](queries/rules/contract-entrypoints.yml) matches function
definitions and signatures by node kind and name field, including multi-line declarations.
[Its fixtures](queries/tests/contract-entrypoints-test.yml) exclude calls and unrelated names.
The rule locates places to inspect; it does not infer input/output semantics.

[non_datafusion_collect.rs](queries/non_datafusion_collect.rs) is a parsing fixture for the existing
DataFusion project's capability hint, not a compiled runtime test. A match demonstrates why an
awaited method name alone cannot establish DataFusion receiver identity.

## Discovery sources and unresolved retrieval

Context7 library resolution selected `/apache/datafusion` and `/apache/arrow-rs`. Queries were
scoped to provider pushdown, execution/memory, and row conversion:

- [Provider result](context7-0.json): discovery lead from the current DataFusion branch. Its
  example signature differs from the retained 55.1.0 `scan` signature; do not copy it as exact-pin proof.
- [Execution result](context7-1.json): discovery lead for collection/streaming and configuration.
- [Arrow result](context7-2.json): no matching documentation returned; this does not establish absence.
- [Arrow fallback](context7-arrow-fallback.json): `/websites/rs_arrow_arrow` returned sorting and
  `SortOptions` references rather than the requested converter contract.

The web tool could not open the exact docs.rs URLs for DataFusion 55.1.0, arrow-row 59.3.0, or
arrow-select 59.3.0. A primary-site search found the
[Arrow row documentation](https://arrow.apache.org/rust/arrow_row/index.html) and
[DataFusion configuration documentation](https://datafusion.apache.org/user-guide/configs.html),
but those moving references were not used as exact-release behavioral proof. The assessment's
exact claims use the retained package source, cached rustdoc, and local probes instead.

## Initial failures retained

The first source-copy run requested Parquet's former `selection.rs` path and stopped with a
`KeyError`; archive listing showed `selection/mod.rs`, and the collection recipe was corrected.
No capability conclusion was drawn from that missing path.

The first Rust compile required an explicit `ArrayRef` coercion in the generic
`RecordBatch::try_from_iter` fixture. Its
[diagnostic](logs/rust-probes-initial-compile-error.log) and
[receipt](rust-probe-initial-run.json) are retained. The corrected tests then passed.

The first new ast-grep rule test had no authored snapshots. Its
[failure log](logs/contract-query-initial-missing-snapshots.log) is retained. Snapshots were generated
once with `--update-all`; the recorded passing run is the subsequent ordinary test invocation.

An initial ty invocation inherited the host project's source roots and could not resolve the
scripts' local `collect_evidence` import. Supplying the evidence directory through
`--extra-search-path` resolved the configuration issue; no suppression or source workaround was added.
