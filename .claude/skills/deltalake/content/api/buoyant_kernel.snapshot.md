# `buoyant_kernel::snapshot`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.snapshot.json`](../model/buoyant_kernel.snapshot.json)

## CheckpointWriteResult

`enum` · `buoyant_kernel::snapshot::CheckpointWriteResult`

Also reachable as `delta_kernel::snapshot::CheckpointWriteResult`

```rust
enum CheckpointWriteResult
```

**Variants**: `AlreadyExists`, `Written`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Result of attempting to write a checkpoint file.

---

## ChecksumWriteResult

`enum` · `buoyant_kernel::snapshot::ChecksumWriteResult`

Also reachable as `delta_kernel::snapshot::ChecksumWriteResult`

```rust
enum ChecksumWriteResult
```

**Variants**: `AlreadyExists`, `Written`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Result of attempting to write a version checksum (CRC) file.

---

## Snapshot

`struct` · `buoyant_kernel::snapshot::Snapshot`

Also reachable as `buoyant_kernel::Snapshot`, `delta_kernel::snapshot::Snapshot`

```rust
struct Snapshot
```

**Implements**: `core::ops::drop::Drop`, `deltalake_core::kernel::arrow::engine_ext::SnapshotExt`

**Derives**: Debug, Eq, PartialEq

**Methods** (33)

```rust
fn alter_table(Arc<self>) -> AlterTableTransactionBuilder
fn builder_for(table_root: impl AsRef<str>) -> SnapshotBuilder
fn builder_from(existing_snapshot: SnapshotRef) -> SnapshotBuilder
fn checkpoint(&SnapshotRef, engine: &dyn Engine, spec: Option<&CheckpointSpec>) -> DeltaResult<(CheckpointWriteResult, SnapshotRef)>
fn crc(&self) -> Option<&Arc<Crc>>
fn create_checkpoint_writer(Arc<self>, engine: &dyn Engine) -> DeltaResult<CheckpointWriter>
fn estimated_owned_heap_size_bytes(&self) -> usize
fn get_all_domain_metadata(&self, engine: &dyn Engine) -> DeltaResult<Vec<DomainMetadata>>
fn get_app_id_version(&self, application_id: &str, engine: &dyn Engine) -> DeltaResult<Option<i64>>
fn get_domain_metadata(&self, domain: &str, engine: &dyn Engine) -> DeltaResult<Option<String>>
fn get_domain_metadata_internal(&self, domain: &str, engine: &dyn Engine) -> DeltaResult<Option<String>>
fn get_domain_metadatas_internal(&self, engine: &dyn Engine, domains: Option<&HashSet<&str>>) -> DeltaResult<std::collections::HashMap<String, actions::DomainMetadata>>
fn get_file_stats_if_present(&self) -> Option<FileStats>
fn get_in_commit_timestamp(&self, engine: &dyn Engine) -> DeltaResult<Option<i64>>
fn get_logical_clustering_columns(&self, engine: &dyn Engine) -> DeltaResult<Option<Vec<ColumnName>>>
fn get_physical_clustering_columns(&self, engine: &dyn Engine) -> DeltaResult<Option<Vec<ColumnName>>>
fn get_protocol_derived_properties(&self) -> HashMap<String, String>
fn get_timestamp(&self, engine: &dyn Engine) -> DeltaResult<i64>
fn incremental_scan_builder(Arc<self>, base_version: Version) -> IncrementalScanBuilder
fn log_compaction_writer(Arc<self>, start_version: Version, end_version: Version) -> DeltaResult<LogCompactionWriter>
fn log_segment(&self) -> &LogSegment
fn metadata_configuration(&self) -> &HashMap<String, String>
fn new(log_segment: LogSegment, table_configuration: TableConfiguration) -> Self
fn publish(&SnapshotRef, engine: &dyn Engine, committer: &dyn Committer) -> DeltaResult<SnapshotRef>
fn scan_builder(Arc<self>) -> ScanBuilder
fn schema(&self) -> SchemaRef
fn table_configuration(&self) -> &TableConfiguration
fn table_properties(&self) -> &TableProperties
fn table_root(&self) -> &Url
fn transaction(Arc<self>, committer: Box<dyn Committer>, engine: &dyn Engine) -> DeltaResult<Transaction>
fn try_new(log_segment: LogSegment, table_configuration: TableConfiguration) -> DeltaResult<Self>
fn version(&self) -> Version
fn write_checksum(&SnapshotRef, engine: &dyn Engine) -> DeltaResult<(ChecksumWriteResult, SnapshotRef)>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

In-memory representation of a specific snapshot of a Delta table. While a `DeltaTable` exists
throughout time, `Snapshot`s represent a view of a table at a specific point in time; they
have a defined schema (which may change over time for any given table), specific version, and
frozen log segment.

---

## SnapshotRef

`type_alias` · `buoyant_kernel::snapshot::SnapshotRef`

Also reachable as `buoyant_kernel::SnapshotRef`, `delta_kernel::snapshot::SnapshotRef`

```rust
type SnapshotRef = std::sync::Arc<Snapshot>
```

A shared, thread-safe reference to a [`Snapshot`].

---
