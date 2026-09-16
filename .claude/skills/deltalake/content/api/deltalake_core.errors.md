# `deltalake_core::errors`

Crate `deltalake-core` · 3 public items · structured records in [`model/deltalake_core.errors.json`](../model/deltalake_core.errors.json)

## ColumnMappingOperation

`enum` · `deltalake_core::errors::ColumnMappingOperation`

Also reachable as `deltalake::ColumnMappingOperation`, `deltalake::errors::ColumnMappingOperation`, `deltalake_core::ColumnMappingOperation`

```rust
enum ColumnMappingOperation
```

**Variants**: `Read`, `Write`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Whether an unsupported column-mapping access was a read or a write.

---

## DeltaTableError

`enum` · `deltalake_core::errors::DeltaTableError`

Also reachable as `deltalake::DeltaTableError`, `deltalake::errors::DeltaTableError`, `deltalake_core::DeltaTableError`

```rust
enum DeltaTableError
```

**Variants**: `KernelError`, `ObjectStore`, `Parquet`, `Arrow`, `InvalidJsonLog`, `InvalidStatsJson`, `InvalidVersion`, `VersionDowngrade`, `InvalidDateTimeString`, `InvalidData`, `NotATable`, `NoSchema`, `SchemaMismatch`, `PartitionError`, `InvalidPartitionFilter`, `Io`, `CommitValidation`, `Transaction`, `VersionAlreadyExists`, `VersionMismatch`, `MissingFeature`, `InvalidTableLocation`, `SerializeLogJson`, `Generic`, `GenericError`, `Kernel`, `MetadataError`, `NotInitialized`, `NotInitializedWithFiles`, `ChangeDataNotRecorded`, `ChangeDataNotEnabled`, `ChangeDataInvalidVersionRange`, `ChangeDataTimestampGreaterThanCommit`, `NoStartingVersionOrTimestamp`, `UnsupportedColumnMapping`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (3)

```rust
fn generic(msg: impl ToString) -> Self
fn not_a_table(path: impl AsRef<str>) -> Self
fn unsupported_column_mapping(mode: ColumnMappingOperation, operation: impl ToString) -> Self
```

**via `core::convert::From`**

```rust
fn from(source: ObjectStoreError) -> Self
fn from(source: chrono::ParseError) -> Self
fn from(err: object_store::path::Error) -> Self
fn from(err: CommitBuilderError) -> Self
fn from(source: delta_kernel::error::Error) -> Self
fn from(source: arrow::error::ArrowError) -> Self
fn from(source: kernel::Error) -> Self
fn from(err: TransactionError) -> Self
fn from(source: parquet::errors::ParquetError) -> Self
fn from(source: std::io::Error) -> Self
fn from(value: serde_json::Error) -> Self
fn from(err: DataFusionError) -> Self
fn from(err: VacuumError) -> Self
fn from(err: WriteError) -> Self
fn from(err: DeltaWriterError) -> Self
fn from(err: Error) -> Self
fn from(err: CreateError) -> Self
fn from(err: RestoreError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Delta Table specific error

---

## DeltaResult

`type_alias` · `deltalake_core::errors::DeltaResult`

Also reachable as `deltalake::DeltaResult`, `deltalake::errors::DeltaResult`, `deltalake_core::DeltaResult`

```rust
type DeltaResult<T, E = DeltaTableError> = Result<T, E>
```

A result returned by delta-rs

---
