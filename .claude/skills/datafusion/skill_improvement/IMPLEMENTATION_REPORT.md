# Implementation results

Implemented and verified 2026-09-18 inside the portable DataFusion skill. Start at
[SKILL.md](../SKILL.md), [task routes](../content/routes/tasks.md), or
[maintenance.md](../maintenance.md). The host application and the supporting ast-grep/rust-code-model
skills were not modified.

| Plan stage | Result | Evidence |
|---|---|---|
| Recover upstream contracts | 52,330 records, including 1,088 module records, retain full docs, raw types, fields/variants, source spans and artifact-scoped links. Original compact indexes remain. | [Preservation and reader checks](evidence/implementation/contract-checks.json) |
| Introduce routes | Task, representation and crate views cover all 60 pinned packages; canonical/facade symbol lookup remains available. | [Crate routes](../content/routes/crates.md), [coverage](../content/capabilities/coverage.json) |
| Establish contract quality | 16 reviewed decision briefs reference 73 operation paths, with conditional alternatives, claims, implementation considerations and unknowns. | [Capability catalog](../content/capabilities/catalog.json) |
| Automate evidence | Exact archive/source adapter, full rustdoc preservation, runtime registry/config capture, 15 executed assertion tests, local filter timing and Parquet read-request observations. | [Receipt](evidence/implementation/probe-results.json), [profile](evidence/implementation/probe-profile.json), [registry](evidence/implementation/runtime-registry.json) |
| Bounded retrieval | `find`, `show`, and `compare` use task vocabulary, aliases and explicit facets. Byte-limited output has lossless continuations. | [Reader checks](evidence/implementation/contract-checks.json), [CLI](../scripts/reference.py) |
| Evaluate and expand | Independent agents completed the 24-task paired pilot; independent judgment found 13 improvements, 11 matches and no overall regressions. Corrections were incorporated. | [Evaluation report](evidence/implementation/evaluation/REPORT.md), [judgment](evidence/implementation/evaluation/judgment.json) |
| Package and maintain | Reader/research packaging, digest manifests, license provenance, dependency/candidate-surface invalidation and copied-reader qualification are implemented. | [Transfer receipt](evidence/implementation/transfer-qualification.json), [packaging instructions](../maintenance.md) |

## Changes that affect implementation decisions

- Full `TableProvider::scan`, `DataFrame::execute_stream`, `CastOptions::safe` and module contracts
  are addressable without relying on truncated member summaries.
- Selection compares filter, take/take_arrays, slice and projection, including checked indices,
  nullable output fields and a compiled zero-column gather composition.
- Cast failure policy, dictionary hydration and non-durable row encoding are explicit.
- Streaming, output retention, operator state and pool-accounted memory have separate contracts.
- Provider exactness and filter → limit → projection order have differential controls and observed
  scan arguments. The contradictory upstream `Exact` sentence is preserved with a separate,
  evidence-linked annotation.
- Expression construction, coercion, simplification and physical evaluation are separate phases.
  In this release, `coalesce` must be simplified to CASE; direct invocation is not its execution path.
- Function discovery includes 314 observed registry entries and 147 configuration entries from one
  explicit feature profile. The broader SQL documentation catalog and feature inventory remain
  separate. Config setter-name matches are labeled heuristic.
- Joins/set multiplicity, window peers/frames, schema metadata, aggregate state, source/view/cache
  reuse, storage and plan/data/ABI interchange have distinct decision routes.

## Validation and scope

[Build verification](evidence/implementation/build-verification.log) records deterministic
regeneration, legacy cross-reference integrity, all 13 structural rule groups and 35 navigation
checks. The new preservation check compares every retained record with its raw artifact; reader
checks include 8,255 resolved documentation links, alias equivalence and byte-limited continuation.
[Identity controls](evidence/implementation/identity-checks.log) test rustdoc ID renumbering.
[Ruff](evidence/implementation/lint.log), [typing](evidence/implementation/type-check.log),
[Clippy](evidence/implementation/clippy.log) and
[skill validation](evidence/implementation/skill-validation.log) passed.

The [15-test suite](evidence/implementation/probe-results.json) covers the named Arrow, DataFusion,
Parquet and IPC assertions. [Integration mapping](evidence/implementation/evaluation/integration-map.json)
connects these shared controls to the paired task distinctions; evaluator-produced sketches were
not individually compiled. [Filter timings](evidence/implementation/filter-cost.json) are an
unoptimized exploratory workload with fixed method order, not a throughput recommendation.
Parquet byte counts describe the local in-memory ChunkReader, not network traffic or decoder CPU.

The pilot used one independent agent/run per arm. Both arms chose supported core approaches; most
gains replace uncertainty with precise contracts. The candidate took more recorded commands/time,
so no efficiency improvement is claimed. Context consumption, variance and broad generalization
remain unmeasured. The frozen candidate had 15 briefs/13 tests; later coalesce/IPC controls and
review corrections are separately identified in the evaluation report.

There are still 639 explicit link/display diagnostics and 37 unresolved access paths outside the
pinned set. Not every package has a reviewed semantic brief. Cloud behavior, process-RSS bounds,
cancellation latency, arbitrary cross-version interoperability and exhaustive datatype/function
semantics are not certified. These remain visible limits, not inferred negative capability claims.

## Transfer and future changes

Use [reader.tar.gz](evidence/implementation/bundles/reader.tar.gz) for lookup in other repositories;
use [research.tar.gz](evidence/implementation/bundles/research.tar.gz) for maintained authoring,
cached upstream inputs, probe sources and evaluation provenance. The bundles exclude compiler
targets and extracted qualification/evaluation workspaces. Their adjacent JSON files record hashes
and sizes. The reader qualification runs from `/` using the resolved base Python with site hooks
disabled and a minimal environment, tracing original-project file access and Internet sockets.

The exact-release sources are authoritative for this reference. Context7 was only a discovery
lead; its older upgrade guide was not used as DataFusion 55 contract evidence. Consumers should
still match their own Cargo versions and features. Follow the maintenance workflow to refresh
claims and alternatives when those inputs change.
