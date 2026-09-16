# `deltalake_core::table::config`

Crate `deltalake-core` · 5 public items · structured records in [`model/deltalake_core.table.config.json`](../model/deltalake_core.table.config.json)

## DEFAULT_NUM_INDEX_COLS

`constant` · `deltalake_core::table::config::DEFAULT_NUM_INDEX_COLS`

Also reachable as `deltalake::table::config::DEFAULT_NUM_INDEX_COLS`

```rust
const DEFAULT_NUM_INDEX_COLS: u64 = 32
```

Default num index cols

---

## DEFAULT_TARGET_FILE_SIZE

`constant` · `deltalake_core::table::config::DEFAULT_TARGET_FILE_SIZE`

Also reachable as `deltalake::table::config::DEFAULT_TARGET_FILE_SIZE`

```rust
const DEFAULT_TARGET_FILE_SIZE: std::num::NonZeroU64 = _
```

Default target file size

---

## DeltaConfigError

`enum` · `deltalake_core::table::config::DeltaConfigError`

Also reachable as `deltalake::table::config::DeltaConfigError`

```rust
enum DeltaConfigError
```

**Variants**: `Validation`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Delta configuration error

---

## TableProperty

`enum` · `deltalake_core::table::config::TableProperty`

Also reachable as `deltalake::TableProperty`, `deltalake::table::config::TableProperty`, `deltalake_core::TableProperty`

```rust
enum TableProperty
```

**Variants**: `AppendOnly`, `AutoOptimizeAutoCompact`, `AutoOptimizeOptimizeWrite`, `CheckpointInterval`, `CheckpointWriteStatsAsJson`, `CheckpointWriteStatsAsStruct`, `CheckpointUseRunLengthEncoding`, `ColumnMappingMode`, `DataSkippingNumIndexedCols`, `DataSkippingStatsColumns`, `DeletedFileRetentionDuration`, `EnableChangeDataFeed`, `EnableDeletionVectors`, `IsolationLevel`, `LogRetentionDuration`, `EnableExpiredLogCleanup`, `MinReaderVersion`, `MinWriterVersion`, `RandomizeFilePrefixes`, `RandomPrefixLength`, `SetTransactionRetentionDuration`, `TargetFileSize`, `TuneFileSizesForRewrites`, `CheckpointPolicy`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`

**Derives**: Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Typed property keys that can be defined on a delta table

<https://docs.delta.io/latest/table-properties.html#delta-table-properties-reference>
<https://learn.microsoft.com/en-us/azure/databricks/delta/table-properties>

---

## TablePropertiesExt

`trait` · `deltalake_core::table::config::TablePropertiesExt`

Also reachable as `deltalake::table::config::TablePropertiesExt`

```rust
trait TablePropertiesExt
```

**Implementors** (1)

- `buoyant_kernel::table_properties::TableProperties`

**Methods** (10)

```rust
fn append_only(&self) -> bool
fn checkpoint_interval(&self) -> NonZero<u64>
fn deleted_file_retention_duration(&self) -> Duration
fn enable_change_data_feed(&self) -> bool
fn enable_expired_log_cleanup(&self) -> bool
fn get_constraints(&self) -> Vec<Constraint>
fn isolation_level(&self) -> IsolationLevel
fn log_retention_duration(&self) -> Duration
fn num_indexed_cols(&self) -> DataSkippingNumIndexedCols
fn target_file_size(&self) -> NonZero<u64>
```

Convenience accessors for reading well-known Delta table properties with their defaults
applied, layered on top of the raw [`TableProperties`] parsed from table metadata.

---
