# Design review — connected typed evidence slice

## 1. Decision and scope

**Decision: Revise the connected T1–T5 implementation for S1–S3 below.** The accepted target
contract remains the design authority. This source audit does not certify Plan 10, resource
qualification, producer/client acceptance or Phase 4–6 completion.

Date: 2026-09-14. Standard: [design charter](../design_principles/DATA_MODEL_DESIGN_CHARTER.md),
directive and repository addendum. Related:
[target-contract review](design_review_arrow-target-contract_2026-09-14.md),
[Plan 10](../../plans/10-arrow-datafusion-architecture.md), ADR-0022/0023/0024.

**Method and coverage:** read current store `comparison.rs`, `bundle.rs`, `query.rs`,
`provider.rs`; relevant repository publication/admission/materialization paths; catalog
attempt/export validation; comparison/metadata projections and views; core ingestion and
reference/coverage models; daemon publication, replay and comparison consumers. Anchors identify
the reviewed functions as well as current line numbers because implementation continues in the
shared tree. Evidence strength is **Implemented source inspection**, not **Tested**. No code
edits, builds, tests, gates, containers or data deletion were performed by this review.

Known open work was supplied by the implementer: native assembly/rebase, provider/export leases,
manual cleanup/reset, catalog compaction, T6 product scope and measured performance. Those are
not rediscovered here. Old-format readers and compatibility preservation are outside the target.

## 2. Authority and lifecycle map

| Fact | Actual authority/path | Required interpretation |
|---|---|---|
| Coverage | `CoverageFact` carries producer, subject, outcome and gaps | Indexed presence does not imply complete API coverage. |
| Symbol endpoint | `PublicBinding::id_for` binds package, typed path, kind and qualifier | `path_id` alone identifies a lexical path, not a qualified binding. |
| Producer semantics | Snapshot `producer_runs` relation contains semantic bindings | Catalog attempts must actually match those bindings before they qualify snapshot facts. |
| Attempt association | Catalog `SnapshotAttempt` names snapshot, run and acquisition descriptors | A valid snapshot FK and valid independent run do not prove their association. |
| Portable evidence | Bundle checksums plus repository/catalog admission | Recomputed physical hashes cannot make inconsistent semantic references valid. |

## 3. Invariants and enforcement gaps

| Invariant | Existing mechanism | Gap / oracle |
|---|---|---|
| Partial/complete distinction | Typed `Indexed`, `Partial`, `Missing` outcomes | S1: daemon comparison derives completeness from coarser manifest sets. |
| Qualified edge endpoints | Typed local symbol/definition targets and validated FKs | S2: comparison projects symbol endpoints to lexical paths and loses qualifiers. |
| Actual producing attempt | Run identity checks and attempt→snapshot existence | S3: no selected-snapshot attempt→semantic-binding check. |
| Physical integrity | Exact file hashes, witnesses, schema and blob admission | Necessary controls already present; they do not repair the semantic gaps above. |

## 4. Derivation and execution

The connected route consumes producer transport through Rust normalization, writes typed
relations, validates snapshot components and blobs, commits catalog associations, and binds
native views. Comparison performs native grouped/set queries and bounded hydration. Bundle
verification independently opens the exported catalog/snapshot after inventory checks.

`ExactParquet` declares pushed filters inexact, preserves the residual filter and avoids a
scan limit when filters are present. This inspected mechanism did not reveal a new filter/limit
correctness defect. Its execution-time file lifetime is part of the implementer's known open
lease work and is not certified here.

## 5. Representative failing journeys

1. A producer indexes some public API and records an extraction gap. Its `Partial` coverage
   becomes manifest `indexed=PublicApi, missing=[]`; equal-normalizer comparison calls that
   API complete, even when the environment is fully specified.
2. Two symbol bindings share a path but differ in kind or trait qualifier. Retarget an edge
   from one binding to the other. The stored relationship changes, but the comparison's
   lexical target path remains equal, so the relationship delta disappears.
3. Replace a bundle's producing attempt with an internally valid unrelated run associated
   with the same snapshot, updating catalog/checksum hashes consistently. Current checks
   establish the run's own identity and snapshot existence, not that it produced this
   snapshot's semantic binding. Replay then exposes that unrelated run as snapshot provenance.

These are source-derived counterexamples; no crafted fixture was executed in this audit.

## 6. Acceptance gates

Verdicts are for this inspected implementation slice, not acceptance-registry results.

| Gate | Verdict | Evidence / limit |
|---|---|---|
| G1 — Authority | fail | S3 leaves actual attempt attribution unreconciled with snapshot producer authority. |
| G2 — Semantic fidelity | fail | S1 promotes partial coverage; S2 erases binding distinctions during comparison. |
| G3 — Validity | fail | S3 permits a semantically unrelated attempt association through independent admission. |
| G4 — Hidden behavior | unresolved | No new undeclared effect found in the inspected provider/query route; complete cancellation/resource behavior was not exercised. |
| G5 — Consistency and recovery | unresolved | Known lease/rebase/reset work is still open; no new claim of recovery completeness. |
| G6 — Transformation and reuse | fail | S2 changes the result of a typed relationship comparison through a lossy projection. |
| G7 — Truthful capability claims | fail | S1's `api_complete=true` exceeds the typed coverage evidence. |

## 7. Principle findings

| Finding | Principles / severity | Concrete evidence and consequence | Bounded correction | Required oracle |
|---|---|---|---|---|
| **S1 — Partial API is reported complete** | DM-08/24/59; high | `repository.rs::coverage` at 834–846 treats every non-Missing outcome as indexed and removes missing whenever that kind is indexed. `validate_semantics` at 575–594 enforces the same coarse summary. `ops/compare.rs` at 177–182 uses only those sets and normalizer equality for `api_complete`. `CoverageOutcome::Partial` has mandatory gaps (`relational.rs:641–677`) yet satisfies that predicate. | Compute API completeness from typed coverage for the selected subject/configuration, with an explicit rule for alternative producer coverage. Preserve unresolved gaps unless a declared complete observation covers that same scope; do not infer completeness from presence. Use the same coverage projection for comparison confounders and partial status. | Two snapshots with matching known environments and normalizers but Partial PublicApi must return `api_complete=false` and a scope-qualified limitation. Add disjoint subscopes and mixed complete/partial producer cases to prevent blanket kind-level masking. |
| **S2 — Relationship comparison loses same-path endpoint identity** | DM-06/09/24; high | `comparison.rs::axes`, relationship axis at 105–114, uses source `s.path_id` as key and target `t.path_id` as value, omitting the symbol IDs and their kind/trait qualifiers. `PublicBinding::id_for` (`relational.rs:302–325`) correctly distinguishes these bindings, while `RelationshipObservation` preserves their IDs. A source move or target change between same-path bindings produces equal projected sets. | Include stable qualified source and target binding identity in the comparison key/value, or an equivalent typed tuple containing every identity dimension. Keep lexical paths as display labels. Definition/external/unresolved target tags retain their existing distinct domains. | Independently test target retargeting and source reassignment between same-path bindings of different kind and trait qualifier. Each must emit a stable relationship delta; row order and acquisition-only changes must not. |
| **S3 — Catalog attempts are not admitted against snapshot producer bindings** | DM-07/14/46/48; high | `catalog_generation.rs::validate` at 706 checks only attempt→snapshot existence. `projection/catalog.rs::attempts_from_batch` at 467–498 checks association ID and acquisition descriptors against that run's inputs. `repository.rs::open_snapshot` at 439–484 then admits snapshot tables/blobs without cross-checking selected catalog attempts against its semantic `producer_runs`. `query.rs::producer_runs` at 426–445 returns those attempts as snapshot provenance; `bundle.rs::verify` at 295–316 delegates to the same incomplete admission. | At selected-snapshot admission/publication, reconcile actual attempts with snapshot semantic producer bindings, including input/configuration/outcome identity. Require actual attribution for bindings that qualify retained facts; verify referenced input/acquisition closure. Cache this check under both immutable snapshot and pinned catalog identity, since new attempts can be appended without changing snapshot bytes. | In an exported bundle, replace/add an unrelated internally valid run and recompute all physical hashes: verification must reject. Remove the only matching producing attempt: reject. Append another valid equivalent attempt: accept without changing snapshot semantic identity. |

**Applicability:** authority, semantic types, identity/consistency, transformations, provenance,
boundary validation and truthful evidence are directly implicated (groups 1–3, 5, 9–12).
Execution/resource and performance principles were considered only at the inspected boundaries;
the known open work and actual runtime/client qualification remain outside the claim.

## 8. Alternatives and proportionality

All three fixes fit the target architecture. A typed coverage projection, qualified endpoint
keys, and a catalog/snapshot attribution join are sufficient. No storage-version rollback,
legacy compatibility layer, new database, generic workflow engine or custom optimizer is needed.

Retain the existing component/file identity split and native grouped comparison rather than
returning to whole-snapshot DTO assembly. Correct the semantic projection feeding those plans.

## 9. Verification plan

| Claim to establish | Targeted future evidence | This audit |
|---|---|---|
| Complete API means complete in its selected scope | S1 typed fixture plus daemon comparison envelope assertions | Not executed |
| Typed relationship identity survives comparison | S2 source/target same-path variants with independent expected changes | Not executed |
| Actual attempts support selected snapshot facts | S3 bundle semantic tampering and valid repeat-acquisition cases | Not executed |
| Corrections remain integrated | Affected store/core/daemon tests and source-bound MCP fixture receipts | Not executed |

Correctness receipts should name the assertions, not merely that a compare/export operation
returned successfully. Broader gates remain the implementation owner's responsibility after
the focused corrections and known open work are complete.

## 10. Exceptions and remaining uncertainty

No new exception to the accepted target is proposed. The three findings describe source paths,
not an observed failed command. Implementation may change while this review is being written;
resolve a finding with current code and its independent oracle before upgrading its evidence.

The disclosed cold/offline performance gap is not duplicated as a new finding, and no speedup
or whole-slice acceptance is inferred from native plans or bounded output buffers.

## 11. Decision and required changes

| Priority | Change | Completion evidence |
|---|---|---|
| High | S1 typed completeness; S2 qualified comparison endpoints | Independent semantic fixtures and real tool-envelope assertions |
| High | S3 attempt/snapshot producer reconciliation | Forged/missing/equivalent attempt bundle admission tests |
| Scope | **Revise this implementation slice; target contract remains accepted** | Fixes and focused receipts, followed separately by outstanding Plan 10 work |
