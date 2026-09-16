# `buoyant_kernel::error`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.error.json`](../model/buoyant_kernel.error.json)

## Error

`enum` · `buoyant_kernel::error::Error`

Also reachable as `buoyant_kernel::Error`, `delta_kernel::error::Error`

```rust
enum Error
```

**Variants**: `Backtraced`, `Arrow`, `CheckpointWrite`, `EngineDataType`, `Extract`, `Generic`, `GenericError`, `IOError`, `InternalError`, `Parquet`, `ObjectStore`, `ObjectStorePath`, `Reqwest`, `FileNotFound`, `MissingColumn`, `InvalidPartitionValues`, `UnexpectedColumnType`, `MissingData`, `MissingVersion`, `DeletionVector`, `InvalidSelectionVector`, `InvalidTransactionState`, `InvalidUrl`, `MalformedJson`, `MissingMetadata`, `MissingProtocol`, `InvalidProtocol`, `MissingMetadataAndProtocol`, `ParseError`, `JoinFailure`, `Utf8Error`, `ParseIntError`, `InvalidColumnMappingMode`, `InvalidTableLocation`, `InvalidDecimal`, `InvalidStructData`, `InvalidExpressionEvaluation`, `InvalidLogPath`, `FileAlreadyExists`, `Unsupported`, `ChecksumWriteUnsupported`, `ParseIntervalError`, `ChangeDataFeedUnsupported`, `ChangeDataFeedIncompatibleSchema`, `InvalidCheckpoint`, `LiteralExpressionTransformError`, `Schema`, `StatsValidation`, `LogHistory`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (24)

```rust
fn change_data_feed_unsupported(version: impl Into<Version>) -> Self
fn deletion_vector(msg: impl ToString) -> Self
fn engine_data_type(msg: impl ToString) -> Self
fn file_not_found(path: impl ToString) -> Self
fn generic(msg: impl ToString) -> Self
fn generic_err(source: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self
fn internal_error(msg: impl ToString) -> Self
fn invalid_checkpoint(msg: impl ToString) -> Self
fn invalid_column_mapping_mode(mode: impl ToString) -> Self
fn invalid_decimal(msg: impl ToString) -> Self
fn invalid_expression(msg: impl ToString) -> Self
fn invalid_partition_values(msg: impl ToString) -> Self
fn invalid_protocol(msg: impl ToString) -> Self
fn invalid_struct_data(msg: impl ToString) -> Self
fn invalid_table_location(location: impl ToString) -> Self
fn invalid_transaction_state(msg: impl ToString) -> Self
fn join_failure(msg: impl ToString) -> Self
fn missing_column(name: impl ToString) -> Self
fn missing_data(name: impl ToString) -> Self
fn schema(msg: impl ToString) -> Self
fn stats_validation(msg: impl ToString) -> Self
fn unexpected_column_type(name: impl ToString) -> Self
fn unsupported(msg: impl ToString) -> Self
fn with_backtrace(self) -> Self
```

**via `core::convert::From`**

```rust
fn from(source: ParseIntError) -> Self
fn from(value: ArrowError) -> Self
fn from(source: object_store::path::Error) -> Self
fn from(source: Utf8Error) -> Self
fn from(source: Box<history_manager::error::LogHistoryError>) -> Self
fn from(value: std::io::Error) -> Self
fn from(e: LogHistoryError) -> Self
fn from(source: parquet::errors::ParquetError) -> Self
fn from(source: url::ParseError) -> Self
fn from(source: ParseIntervalError) -> Self
fn from(value: serde_json::Error) -> Self
fn from(value: object_store::Error) -> Self
fn from(value: Infallible) -> Self
fn from(source: reqwest::Error) -> Self
fn from(source: expressions::literal_expression_transform::Error) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

All the types of errors that the kernel can run into

---

## DeltaResult

`type_alias` · `buoyant_kernel::error::DeltaResult`

Also reachable as `buoyant_kernel::DeltaResult`, `delta_kernel::error::DeltaResult`

```rust
type DeltaResult<T, E = Error> = std::result::Result<T, E>
```

**Implements**: `buoyant_kernel::engine::arrow_data::EngineDataArrowExt`

**via `buoyant_kernel::engine::arrow_data::EngineDataArrowExt`**

```rust
fn try_into_record_batch(self) -> DeltaResult<RecordBatch>
```

A [`std::result::Result`] that has the kernel [`Error`] as the error variant

---

## DeltaResultIterator

`type_alias` · `buoyant_kernel::error::DeltaResultIterator`

Also reachable as `buoyant_kernel::DeltaResultIterator`, `delta_kernel::error::DeltaResultIterator`

```rust
type DeltaResultIterator<'a, T> = Box<dyn Iterator<Item = DeltaResult<T>> + Send + 'a>
```

A boxed, `Send` iterator of [`DeltaResult<T>`] items.

Convenience alias for the common pattern of returning a streaming, fallible iterator from
kernel APIs.

---

## DeltaResultIteratorStatic

`type_alias` · `buoyant_kernel::error::DeltaResultIteratorStatic`

Also reachable as `buoyant_kernel::DeltaResultIteratorStatic`, `delta_kernel::error::DeltaResultIteratorStatic`

```rust
type DeltaResultIteratorStatic<T> = DeltaResultIterator<'static, T>
```

`'static` counterpart to [`DeltaResultIterator`] for cases where the iterator does not
reference borrowed data.

---
