# Choose a Delta-aware scan or table provider

Use the Delta provider for query composition and reliable name-based projection; scan_table provides a direct batch stream. The retained partition-first fixture exposes a LoadBuilder projection-order defect. Raw Parquet reads a different physical relation.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| table_provider | SQL, projection/filter planning or joins should compose with DataFusion | The provider holds snapshot state; session-derived scan settings do not imply automatic freshness |
| scan_table().with_columns(...) | You need a direct stream of selected logical columns | LoadBuilder is callable by inference but its private canonical module is not an import path |
| Raw Parquet scan | You intentionally inspect physical files independently of Delta semantics | Removed rows/files, deletion vectors, partition reconstruction and physical names need separate treatment |

## Contract

**input/output.** LoadBuilder's IntoFuture::Output contains DeltaTable and SendableRecordBatchStream inside DeltaResult; keep streaming errors distinct from builder errors.
Claim `delta.read.1`; source_observation; evidence: upstream, source.

**logical rows.** The tombstone and deletion-vector fixtures return 0 vs 2 and 8 vs 10 rows respectively when compared with raw Parquet. Column-mapping scans expose logical field names.
Claim `delta.read.2`; runtime_observation; evidence: runtime.

**schema.** The partition-first schema fixture exposed a positional projection defect: scan_table().with_columns(["id"]) returned the label column. The full scan reconstructed id correctly, and projecting id by name through a DataFusion provider returned the requested values. Inspect returned field names and types; do not blindly downcast by position.
Claim `delta.read.3`; runtime_observation; evidence: runtime.

**snapshot.** A retained provider continues reading its old snapshot after another table handle appends; rebuild from a refreshed table when current results are required.
Claim `delta.read.4`; runtime_observation; evidence: runtime.

## Implementation

- Resolve access paths through aliases; canonical paths document identity, not guaranteed public imports.
- Preserve the stream schema and apply Arrow kernels that support the actual array representation.
- Treat file pruning and row filtering as different promises; inspect TableProvider pushdown contracts.
- At this pin prefer provider/DataFrame name-based projection for partitioned schemas; retain the LoadBuilder projection regression control.

## Effects

- read snapshot files
- stream batches

## Errors

- A constructed stream can fail during consumption, including after vacuum removed historical files.

## Limits and unknowns

- Predicate pushdown efficiency, allocations and RSS are not measured by the row-correctness fixtures.

## Exact contracts

- [`deltalake_core::table::DeltaTable::table_provider`](../operations/deltalake_core.table.DeltaTable.md#op-e4d0e5daa6b84019197e3110) — `fn table_provider(&self) -> TableProviderBuilder`
- [`deltalake_core::table::DeltaTable::scan_table`](../operations/deltalake_core.table.DeltaTable.md#op-d8d9f3a456c209a46d18989d) — `fn scan_table(&self) -> LoadBuilder`
- [`deltalake_core::operations::load::LoadBuilder::with_columns`](../operations/deltalake_core.operations.load.LoadBuilder.md#op-3cbadb75441606b267067afa) — `fn with_columns(self, columns: impl IntoIterator<Item = impl Into<String>>) -> Self`
- [`deltalake_core::operations::load::LoadBuilder::Output`](../operations/deltalake_core.operations.load.LoadBuilder.md#op-10d61fc5ea060fbd1511386d) — `type Output = Result<(DeltaTable, Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>), DeltaTableError>`
- [`deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_table_version`](../operations/deltalake_core.delta_datafusion.table_provider.TableProviderBuilder.md#op-871e0b2228633a876325f515) — `fn with_table_version(self, version: impl Into<Option<Version>>) -> Self`
- [`deltalake_core::delta_datafusion::table_provider::TableProviderBuilder::with_session`](../operations/deltalake_core.delta_datafusion.table_provider.TableProviderBuilder.md#op-0369e5839026aab638b06488) — `fn with_session<S>(self, session: Arc<S>) -> Self where S: Session + 'static`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/delta_datafusion/table_provider.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: provider_snapshot_and_unnameable_load_builder, delta_snapshot_excludes_removed_parquet_files, deletion_vectors_filter_rows_and_optimize_keeps_logical_rows, unloaded_handle_explicit_version_and_partition_reconstruction
