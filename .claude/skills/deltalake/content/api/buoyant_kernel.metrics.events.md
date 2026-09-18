# `buoyant_kernel::metrics::events`

Crate `buoyant_kernel` · 25 public items · structured records in [`model/buoyant_kernel.metrics.events.json`](../model/buoyant_kernel.metrics.events.json)

## CommitFailureReason

`enum` · `buoyant_kernel::metrics::events::CommitFailureReason`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.CommitFailureReason.md)

Also reachable as `buoyant_kernel::metrics::CommitFailureReason`, `delta_kernel::metrics::events::CommitFailureReason`

```rust
enum CommitFailureReason
```

**Variants**: `Conflict`, `RetryableIo`, `Error`

**Implements**: `core::convert::AsRef`, `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &str) -> ::core::result::Result<CommitFailureReason, <Self as ::core::convert::TryFrom>::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<CommitFailureReason, <Self as ::core::str::FromStr>::Err>
```

Why a transaction commit did not succeed.

Serializes to its `snake_case` name for the `failure_reason` span field (e.g.
`RetryableIo` -> `"retryable_io"`).

---

## MetricEvent

`enum` · `buoyant_kernel::metrics::events::MetricEvent`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.MetricEvent.md)

Also reachable as `buoyant_kernel::metrics::MetricEvent`, `delta_kernel::metrics::events::MetricEvent`

```rust
enum MetricEvent
```

**Variants**: `LogSegmentLoadSuccess`, `LogSegmentLoadFailure`, `ProtocolMetadataLoadSuccess`, `ProtocolMetadataLoadFailure`, `SnapshotBuildSuccess`, `SnapshotBuildFailure`, `TransactionCommitSuccess`, `TransactionCommitFailure`, `DomainMetadataLoadSuccess`, `DomainMetadataLoadFailure`, `SetTransactionLoadSuccess`, `SetTransactionLoadFailure`, `CrcReadSuccess`, `CrcReadFailure`, `JsonReadCompleted`, `ParquetReadCompleted`, `ScanMetadataCompleted`, `StorageListCompleted`, `StorageReadCompleted`, `StorageCopyCompleted`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Metric events emitted during Delta Kernel operations.

---

## ScanType

`enum` · `buoyant_kernel::metrics::events::ScanType`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.ScanType.md)

Also reachable as `buoyant_kernel::metrics::ScanType`, `delta_kernel::metrics::events::ScanType`

```rust
enum ScanType
```

**Variants**: `SequentialPhase`, `ParallelPhase`, `Full`

**Implements**: `core::convert::AsRef`, `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &str) -> ::core::result::Result<ScanType, <Self as ::core::convert::TryFrom>::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<ScanType, <Self as ::core::str::FromStr>::Err>
```

Identifies which scan execution path produced a scan metadata metrics event.

Serializes to the explicit `serialize` name on each variant for the `scan_type` span field
(e.g. `SequentialPhase` -> `"sequential"`).

---

## TableType

`enum` · `buoyant_kernel::metrics::events::TableType`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.TableType.md)

Also reachable as `buoyant_kernel::metrics::TableType`, `delta_kernel::metrics::events::TableType`

```rust
enum TableType
```

**Variants**: `PathBased`, `CatalogManaged`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn from_catalog_managed(is_catalog_managed: bool) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Whether a table is path-based or catalog-managed.

---

## emit_json_read_completed

`function` · `buoyant_kernel::metrics::events::emit_json_read_completed`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.emit_json_read_completed.md)

Also reachable as `buoyant_kernel::metrics::emit_json_read_completed`, `delta_kernel::metrics::events::emit_json_read_completed`

```rust
fn emit_json_read_completed(num_files: u64, bytes_read: u64)
```

Emit a [`MetricEvent::JsonReadCompleted`] via a tracing span.

Call once per [`crate::JsonHandler::read_json_files`] invocation, at iterator exhaustion or
drop.

---

## emit_parquet_read_completed

`function` · `buoyant_kernel::metrics::events::emit_parquet_read_completed`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.emit_parquet_read_completed.md)

Also reachable as `buoyant_kernel::metrics::emit_parquet_read_completed`, `delta_kernel::metrics::events::emit_parquet_read_completed`

```rust
fn emit_parquet_read_completed(num_files: u64, bytes_read: u64)
```

Emit a [`MetricEvent::ParquetReadCompleted`] via a tracing span.

Call once per [`crate::ParquetHandler::read_parquet_files`] invocation, at iterator exhaustion
or drop.

---

## CrcReadSuccess

`struct` · `buoyant_kernel::metrics::events::CrcReadSuccess`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.CrcReadSuccess.md)

Also reachable as `buoyant_kernel::metrics::CrcReadSuccess`, `delta_kernel::metrics::events::CrcReadSuccess`

```rust
struct CrcReadSuccess
```

**Fields**: `bytes_read`, `duration`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A CRC file was read and parsed successfully. `bytes_read` is the raw byte count from storage.

---

## DomainMetadataLoadSuccess

`struct` · `buoyant_kernel::metrics::events::DomainMetadataLoadSuccess`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.DomainMetadataLoadSuccess.md)

Also reachable as `buoyant_kernel::metrics::DomainMetadataLoadSuccess`, `delta_kernel::metrics::events::DomainMetadataLoadSuccess`

```rust
struct DomainMetadataLoadSuccess
```

**Fields**: `from_cache`, `num_domains_returned`, `duration`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Emitted once per domain metadata load, whether served from the CRC cache (`from_cache`) or
from a log replay. Covers connector-issued loads of user domains and kernel-internal loads of
system (`delta.*`) domains such as clustering or row tracking.

---

## JsonReadCompleted

`struct` · `buoyant_kernel::metrics::events::JsonReadCompleted`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.JsonReadCompleted.md)

Also reachable as `buoyant_kernel::metrics::JsonReadCompleted`, `delta_kernel::metrics::events::JsonReadCompleted`

```rust
struct JsonReadCompleted
```

**Fields**: `num_files`, `bytes_read`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Emitted once per `JsonHandler::read_json_files` call when the returned iterator is fully
consumed or dropped. `bytes_read` is the sum of on-disk `FileMeta::size`, not the
deserialized payload size.

---

## LogSegmentLoadFailure

`struct` · `buoyant_kernel::metrics::events::LogSegmentLoadFailure`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.LogSegmentLoadFailure.md)

Also reachable as `buoyant_kernel::metrics::LogSegmentLoadFailure`, `delta_kernel::metrics::events::LogSegmentLoadFailure`

```rust
struct LogSegmentLoadFailure
```

**Fields**: `operation_id`, `correlation_id`, `table_type`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Listing the log segment for a snapshot failed.

---

## LogSegmentLoadSuccess

`struct` · `buoyant_kernel::metrics::events::LogSegmentLoadSuccess`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.LogSegmentLoadSuccess.md)

Also reachable as `buoyant_kernel::metrics::LogSegmentLoadSuccess`, `delta_kernel::metrics::events::LogSegmentLoadSuccess`

```rust
struct LogSegmentLoadSuccess
```

**Fields**: `operation_id`, `correlation_id`, `table_type`, `num_commit_files`, `num_checkpoint_files`, `num_compaction_files`, `has_latest_crc_file`, `duration`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A log segment was listed and assembled for a snapshot.

---

## MetricId

`struct` · `buoyant_kernel::metrics::events::MetricId`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.MetricId.md)

Also reachable as `buoyant_kernel::metrics::MetricId`, `delta_kernel::metrics::events::MetricId`

```rust
struct MetricId
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn as_bytes(&self) -> [u8; 16]
fn new() -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Unique identifier for a metrics operation.

Each operation (Snapshot, Transaction, Scan) gets a unique `MetricId` that correlates all
events emitted from that operation.

---

## ParquetReadCompleted

`struct` · `buoyant_kernel::metrics::events::ParquetReadCompleted`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.ParquetReadCompleted.md)

Also reachable as `buoyant_kernel::metrics::ParquetReadCompleted`, `delta_kernel::metrics::events::ParquetReadCompleted`

```rust
struct ParquetReadCompleted
```

**Fields**: `num_files`, `bytes_read`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Emitted once per `ParquetHandler::read_parquet_files` call when the returned iterator is
fully consumed or dropped. `bytes_read` is the sum of on-disk `FileMeta::size`, not the
deserialized payload size.

---

## ProtocolMetadataLoadFailure

`struct` · `buoyant_kernel::metrics::events::ProtocolMetadataLoadFailure`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.ProtocolMetadataLoadFailure.md)

Also reachable as `buoyant_kernel::metrics::ProtocolMetadataLoadFailure`, `delta_kernel::metrics::events::ProtocolMetadataLoadFailure`

```rust
struct ProtocolMetadataLoadFailure
```

**Fields**: `operation_id`, `correlation_id`, `table_type`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Reading protocol and metadata from the log failed.

---

## ProtocolMetadataLoadSuccess

`struct` · `buoyant_kernel::metrics::events::ProtocolMetadataLoadSuccess`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.ProtocolMetadataLoadSuccess.md)

Also reachable as `buoyant_kernel::metrics::ProtocolMetadataLoadSuccess`, `delta_kernel::metrics::events::ProtocolMetadataLoadSuccess`

```rust
struct ProtocolMetadataLoadSuccess
```

**Fields**: `operation_id`, `correlation_id`, `table_type`, `duration`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Protocol and metadata actions were read from the log.

---

## ScanMetadataCompleted

`struct` · `buoyant_kernel::metrics::events::ScanMetadataCompleted`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.ScanMetadataCompleted.md)

Also reachable as `buoyant_kernel::metrics::ScanMetadataCompleted`, `delta_kernel::metrics::events::ScanMetadataCompleted`

```rust
struct ScanMetadataCompleted
```

**Fields**: `operation_id`, `correlation_id`, `table_type`, `scan_type`, `duration`, `num_add_files_seen`, `num_active_add_files`, `active_add_files_bytes`, `num_remove_files_seen`, `num_non_file_actions`, `num_predicate_filtered`, `peak_hash_set_size`, `dedup_visitor_time`, `predicate_eval_time`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A `parallel_scan_metadata` scan emits **two** events (one per phase) sharing the same
`operation_id`; `scan_metadata` emits one event with [`ScanType::Full`].

---

## SetTransactionLoadSuccess

`struct` · `buoyant_kernel::metrics::events::SetTransactionLoadSuccess`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.SetTransactionLoadSuccess.md)

Also reachable as `buoyant_kernel::metrics::SetTransactionLoadSuccess`, `delta_kernel::metrics::events::SetTransactionLoadSuccess`

```rust
struct SetTransactionLoadSuccess
```

**Fields**: `from_cache`, `found`, `duration`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Emitted once per `SetTransaction` (app id) load, whether served from the CRC cache
(`from_cache`) or from a log replay. `found` is true when the app id has a committed
transaction version, false when none exists or the existing one is expired.

---

## SnapshotBuildFailure

`struct` · `buoyant_kernel::metrics::events::SnapshotBuildFailure`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.SnapshotBuildFailure.md)

Also reachable as `buoyant_kernel::metrics::SnapshotBuildFailure`, `delta_kernel::metrics::events::SnapshotBuildFailure`

```rust
struct SnapshotBuildFailure
```

**Fields**: `operation_id`, `correlation_id`, `table_type`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Building a snapshot failed.

---

## SnapshotBuildSuccess

`struct` · `buoyant_kernel::metrics::events::SnapshotBuildSuccess`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.SnapshotBuildSuccess.md)

Also reachable as `buoyant_kernel::metrics::SnapshotBuildSuccess`, `delta_kernel::metrics::events::SnapshotBuildSuccess`

```rust
struct SnapshotBuildSuccess
```

**Fields**: `operation_id`, `correlation_id`, `table_type`, `version`, `duration`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A snapshot was built successfully.

---

## SnapshotLoadMetricContext

`struct` · `buoyant_kernel::metrics::events::SnapshotLoadMetricContext`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.SnapshotLoadMetricContext.md)

Also reachable as `buoyant_kernel::metrics::SnapshotLoadMetricContext`, `delta_kernel::metrics::events::SnapshotLoadMetricContext`

```rust
struct SnapshotLoadMetricContext
```

**Derives**: Clone, Debug, Default

Operation-scoped values threaded through the snapshot-load chain to label its metric events.

---

## StorageCopyCompleted

`struct` · `buoyant_kernel::metrics::events::StorageCopyCompleted`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.StorageCopyCompleted.md)

Also reachable as `buoyant_kernel::metrics::StorageCopyCompleted`, `delta_kernel::metrics::events::StorageCopyCompleted`

```rust
struct StorageCopyCompleted
```

**Fields**: `duration`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A storage copy or rename operation completed.

---

## StorageListCompleted

`struct` · `buoyant_kernel::metrics::events::StorageListCompleted`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.StorageListCompleted.md)

Also reachable as `buoyant_kernel::metrics::StorageListCompleted`, `delta_kernel::metrics::events::StorageListCompleted`

```rust
struct StorageListCompleted
```

**Fields**: `duration`, `num_files`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A storage list operation completed.

---

## StorageReadCompleted

`struct` · `buoyant_kernel::metrics::events::StorageReadCompleted`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.StorageReadCompleted.md)

Also reachable as `buoyant_kernel::metrics::StorageReadCompleted`, `delta_kernel::metrics::events::StorageReadCompleted`

```rust
struct StorageReadCompleted
```

**Fields**: `duration`, `num_files`, `bytes_read`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A storage read operation completed.

---

## TransactionCommitFailure

`struct` · `buoyant_kernel::metrics::events::TransactionCommitFailure`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.TransactionCommitFailure.md)

Also reachable as `buoyant_kernel::metrics::TransactionCommitFailure`, `delta_kernel::metrics::events::TransactionCommitFailure`

```rust
struct TransactionCommitFailure
```

**Fields**: `operation_id`, `correlation_id`, `table_type`, `reason`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A transaction commit did not succeed; `reason` distinguishes conflict, retryable IO, and
terminal errors.

---

## TransactionCommitSuccess

`struct` · `buoyant_kernel::metrics::events::TransactionCommitSuccess`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.events.TransactionCommitSuccess.md)

Also reachable as `buoyant_kernel::metrics::TransactionCommitSuccess`, `delta_kernel::metrics::events::TransactionCommitSuccess`

```rust
struct TransactionCommitSuccess
```

**Fields**: `operation_id`, `correlation_id`, `table_type`, `commit_version`, `num_add_files`, `num_remove_files`, `num_dv_updates`, `add_files_bytes`, `remove_files_bytes`, `is_blind_append`, `data_change`, `operation`, `prepare_duration`, `committer_duration`, `total_duration`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A transaction was committed successfully.

---
