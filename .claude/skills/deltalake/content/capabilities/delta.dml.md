# Update or delete through built-in Delta operations

Use table.update or table.delete for target-only changes. Their builders and metrics differ from merge clause helpers, and an operation with no affected rows need not create a new version.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| UpdateBuilder | New values derive from target expressions | Set a predicate for a subset and choose cast/session policy |
| DeleteBuilder | Rows should be removed from the logical table | Physical file retention is handled separately |
| MergeBuilder | Actions depend on another relation | Requires source-target matching and clause rules |

## Contract

**result.** Update and delete return a new table value and metrics. DeleteMetrics::num_deleted_rows is optional; absence must not be interpreted as zero.
Claim `delta.dml.1`; source_observation; evidence: upstream, source.

**no-op.** Deleting a predicate that matched no rows preserved the version in the local probe. Updating id 1 and deleting a null row reported the expected counts and rows.
Claim `delta.dml.2`; runtime_observation; evidence: runtime.

**physical/logical.** A logical delete need not delete the underlying Parquet file immediately. Delta-aware scans follow log actions/deletion vectors.
Claim `delta.dml.3`; runtime_observation; evidence: runtime.

## Implementation

- Keep standalone builder types separate from merge's closure types.
- Read the result table after execution and handle optional metrics as optional evidence.

## Effects

- logical row update/delete
- possible log commit

## Errors

- A stale update can conflict with an intervening rewrite even if append retries are allowed.

## Limits and unknowns

- Metrics availability and rewrite/DV strategy depend on the path and table features.

## Exact contracts

- [`deltalake_core::table::DeltaTable::update`](../operations/deltalake_core.table.DeltaTable.md#op-4da0a0df2c2c4c5389f518ca) — `fn update(self) -> UpdateBuilder`
- [`deltalake_core::table::DeltaTable::delete`](../operations/deltalake_core.table.DeltaTable.md#op-cc21a8977a893e16e9c6f12c) — `fn delete(self) -> DeleteBuilder`
- [`deltalake_core::operations::update::UpdateBuilder::with_update`](../operations/deltalake_core.operations.update.UpdateBuilder.md#op-4ce68114cef28f80c1ce94a7) — `fn with_update<S: Into<DeltaColumn>, E: Into<Expression>>(self, column: S, expression: E) -> Self`
- [`deltalake_core::operations::update::UpdateBuilder::Output`](../operations/deltalake_core.operations.update.UpdateBuilder.md#op-92fb88dd0cb8c29f60b9d8d9) — `type Output = Result<(DeltaTable, UpdateMetrics), DeltaTableError>`
- [`deltalake_core::operations::delete::DeleteBuilder::Output`](../operations/deltalake_core.operations.delete.DeleteBuilder.md#op-e74c7e65dd2b30f69a94a0bb) — `type Output = Result<(DeltaTable, DeleteMetrics), DeltaTableError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/delete.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: dml_noop_and_update_delete_metrics, delta_snapshot_excludes_removed_parquet_files
