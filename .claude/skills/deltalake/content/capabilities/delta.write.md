# Write batches or a logical plan; distinguish staging from publication

Prefer WriteBuilder for a complete append/replacement operation. RecordBatchWriter provides staged file production with an explicit commit boundary. At this commit, Ignore on an existing WriteBuilder path appended rows in the retained probe.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| DeltaTable::write(batches) | An operation should write data and publish a Delta commit | Builder consumes the table value and returns an updated DeltaTable |
| with_input_plan(LogicalPlan) | DataFusion produces the source relation | The supplied operation session must support Delta's custom planning nodes |
| RecordBatchWriter | You need incremental batch ingestion and explicit flush/publication control | flush creates actions/files; flush_and_commit publishes them |

## Contract

**types.** with_input_plan accepts LogicalPlan. The deprecated with_input_execution_plan spelling accepts Arc<LogicalPlan>, despite its name; it is not an ExecutionPlan adapter.
Claim `delta.write.1`; source_observation; evidence: upstream, source.

**visibility.** Rows buffered by RecordBatchWriter were invisible to a reloaded observer until flush_and_commit completed.
Claim `delta.write.2`; runtime_observation; evidence: runtime.

**save mode.** Append is the default. ErrorIfExists rejected an existing table; SaveMode::Ignore on this existing-table write path committed additional rows. Do not use Ignore as a skip or dedup guarantee at this pin.
Claim `delta.write.3`; runtime_observation; evidence: runtime.

**resources.** Target file size and write batch size are tuning inputs, not exact output size or process-memory caps. A plan/stream input alone proves no bounded-memory property.
Claim `delta.write.4`; source_observation; evidence: upstream, source.

**staging boundary.** write/write_with_mode can perform object I/O and finalize files before explicit flush when target size or schema rotation is reached. flush returns staged Add actions; Delta visibility requires log publication. reset can leave finalized unreferenced files for cleanup. The target-size probe waits for the background upload to finish before explicit flush and confirms Delta rows remain unchanged until commit.
Claim `delta.write.5`; source_observation; evidence: writer_source, runtime.

## Implementation

- Select save mode from the tested operation path; CreateBuilder's behavior must not be assumed to match WriteBuilder.
- Use file actions and version/metrics to observe publication; coordinate low-level writer retries with the commit contract.
- For a logical plan, provide a Delta-configured SessionState that contains required source/UDF context.

## Effects

- write data files
- publish log version
- may leave unreferenced files after failure

## Errors

- Validation or planning can fail before log publication. A later hook failure can occur after a visible commit; see delta.commit.

## Limits and unknowns

- Exact file-size targets, throughput and memory/spill behavior require workload-specific measurement.

## Exact contracts

- [`deltalake_core::table::DeltaTable::write`](../operations/deltalake_core.table.DeltaTable.md#op-8c868b08ecd74ae0be1525f8) — `fn write(self, batches: impl IntoIterator<Item = RecordBatch>) -> WriteBuilder`
- [`deltalake_core::operations::write::WriteBuilder::with_input_plan`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-96e3a149764f7356c388c2fc) — `fn with_input_plan(self, plan: LogicalPlan) -> Self`
- [`deltalake_core::operations::write::WriteBuilder::with_save_mode`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-a2d377816dc436cf1c93443b) — `fn with_save_mode(self, save_mode: SaveMode) -> Self`
- [`deltalake_core::operations::write::WriteBuilder::Output`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-3b3adb069c90d8711cc194f5) — `type Output = Result<DeltaTable, DeltaTableError>`
- [`deltalake_core::writer::record_batch::RecordBatchWriter::for_table`](../operations/deltalake_core.writer.record_batch.RecordBatchWriter.md#op-ed1304f90217d8fc14f245c9) — `fn for_table(table: &DeltaTable) -> Result<Self, DeltaTableError>`
- [`deltalake_core::writer::DeltaWriter::flush_and_commit`](../operations/deltalake_core.writer.DeltaWriter.md#op-85f06c0b7b6353f28c49881a) — `async fn flush_and_commit(&mut self, table: &mut DeltaTable) -> Result<Version, DeltaTableError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/write/mod.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: logical_plan_write_and_staged_writer_visibility, save_modes_replace_where_and_validation_before_commit
- [writer_source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/writer/record_batch.rs): write_with_mode, reset and target-size rolling at pinned commit
  Tests: 
