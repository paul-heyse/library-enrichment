# RowVisitor

`buoyant_kernel::engine_data::RowVisitor`

```rust
trait RowVisitor
```

Also reachable as `buoyant_kernel::RowVisitor`, `delta_kernel::engine_data::RowVisitor`

Prose: [`api/buoyant_kernel.engine_data.md`](../api/buoyant_kernel.engine_data.md#rowvisitor) · records: [`model/buoyant_kernel.engine_data.json`](../model/buoyant_kernel.engine_data.json)

## Required

Every implementation must supply these.

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn visit_rows_of(&mut self, data: &dyn EngineData) -> DeltaResult<()> where Self: Sized
```

## Implementors (22)

Read one before writing your own.

- `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationVisitor`
- `buoyant_kernel::actions::visitors::AddVisitor`
- `buoyant_kernel::actions::visitors::CdcVisitor`
- `buoyant_kernel::actions::visitors::DomainMetadataVisitor`
- `buoyant_kernel::actions::visitors::InCommitTimestampVisitor`
- `buoyant_kernel::actions::visitors::MetadataVisitor`
- `buoyant_kernel::actions::visitors::ProtocolVisitor`
- `buoyant_kernel::actions::visitors::RemoveVisitor`
- `buoyant_kernel::actions::visitors::SelectionVectorVisitor`
- `buoyant_kernel::actions::visitors::SetTransactionVisitor`
- `buoyant_kernel::actions::visitors::SidecarVisitor`
- `buoyant_kernel::crc::file_stats::FileStatsVisitor`
- `buoyant_kernel::engine_data::FilteredVisitorBridge`
- `buoyant_kernel::incremental_scan::IncrementalDedupVisitor`
- `buoyant_kernel::log_segment::crc_replay::CrcReplayVisitor`
- `buoyant_kernel::row_tracking::RowTrackingVisitor`
- `buoyant_kernel::scan::log_replay::AddRemoveDedupVisitor`
- `buoyant_kernel::table_changes::log_replay::FileActionSelectionVisitor`
- `buoyant_kernel::table_changes::log_replay::PreparePhaseVisitor`
- `buoyant_kernel::table_changes::scan_file::CdfScanFileVisitor`
- `buoyant_kernel::transaction::stats_verifier::ColumnStatsValidator`
- `buoyant_kernel::transaction::stats_verifier::NumRecordsValidator`

## Documentation

A `RowVisitor` can be called back to visit extracted data. Aside from calling
[`RowVisitor::visit`] on the visitor passed to [`EngineData::visit_rows`], engines do
not need to worry about this trait.
