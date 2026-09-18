# Merge with explicit key, null, clause and duplicate semantics

Merge composes a source DataFrame, target snapshot and ordered conditional actions. Decide null-key and duplicate-source policy before treating it as an upsert.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| MergeBuilder | Different matched/unmatched cases need different actions | Aliases and ordered predicates define the semantics |
| WriteBuilder append/replace_where | Source data is already a complete replacement slice or pure append | Simpler input contract; no source-target action matrix |
| Pre-aggregate/deduplicate source | Multiple source rows can match one target with conflicting updates | Requires an application-defined tie-break rule; do not silently drop duplicates |

## Contract

**clauses.** The first applicable matched update won in the probe; source and target null keys did not match under ordinary equality. Unmatched insert and by-source delete contributed independently.
Claim `delta.merge.1`; runtime_observation; evidence: runtime.

**duplicates.** Two matching source rows attempting to update one target produced an error and left the observed version unchanged in the tested case.
Claim `delta.merge.2`; runtime_observation; evidence: runtime.

**construction.** when_* takes a closure configuring a merge clause builder. Its UpdateBuilder/DeleteBuilder types are not constructors for standalone table update/delete operations.
Claim `delta.merge.3`; source_observation; evidence: upstream, source.

**result.** Await returns updated table plus MergeMetrics; interpret inserted/updated/deleted counts in the context of source multiplicity and chosen clauses.
Claim `delta.merge.4`; source_observation; evidence: upstream, source.

## Implementation

- Use explicit source/target aliases and qualified columns.
- Choose equality versus a deliberate null-safe condition and independently specify duplicate-source handling.
- Assert rows, versions and per-action metrics; a completed merge is not evidence of an application-level unique key.

## Effects

- rewrite or add/remove data
- publish transaction
- return metrics

## Errors

- Invalid expressions, ambiguous/multiple matches, protocol/schema restrictions and optimistic conflicts require distinct recovery.

## Limits and unknowns

- Every schema-evolution/duplicate-delete combination and concurrency schedule is not certified by the selected fixture.

## Exact contracts

- [`deltalake_core::table::DeltaTable::merge`](../operations/deltalake_core.table.DeltaTable.md#op-6520d30123b98065a7c363a4) — `fn merge<E: Into<Expression>>(self, source: datafusion::prelude::DataFrame, predicate: E) -> MergeBuilder`
- [`deltalake_core::operations::merge::MergeBuilder::when_matched_update`](../operations/deltalake_core.operations.merge.MergeBuilder.md#op-3b956eb522c0b07669f78099) — `fn when_matched_update<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(UpdateBuilder) -> UpdateBuilder`
- [`deltalake_core::operations::merge::MergeBuilder::when_not_matched_insert`](../operations/deltalake_core.operations.merge.MergeBuilder.md#op-5df71d61188e1549d37abb11) — `fn when_not_matched_insert<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(InsertBuilder) -> InsertBuilder`
- [`deltalake_core::operations::merge::MergeBuilder::when_not_matched_by_source_delete`](../operations/deltalake_core.operations.merge.MergeBuilder.md#op-c47b2f8a582c539f8251a343) — `fn when_not_matched_by_source_delete<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(DeleteBuilder) -> DeleteBuilder`
- [`deltalake_core::operations::merge::MergeBuilder::Output`](../operations/deltalake_core.operations.merge.MergeBuilder.md#op-246dbe6c66042a837a13c116) — `type Output = Result<(DeltaTable, MergeMetrics), DeltaTableError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/merge/mod.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: merge_clause_order_null_keys_and_duplicate_updates
