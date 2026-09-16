# DeltaWriter

`deltalake_core::writer::DeltaWriter`

```rust
trait DeltaWriter<T>
```

Also reachable as `deltalake::writer::DeltaWriter`

Prose: [`api/deltalake_core.writer.md`](../api/deltalake_core.writer.md#deltawriter) · records: [`model/deltalake_core.writer.json`](../model/deltalake_core.writer.json)

## Required

Every implementation must supply these.

```rust
async fn flush(&mut self) -> Result<Vec<Add>, DeltaTableError>
async fn write(&mut self, values: T) -> Result<(), DeltaTableError>
async fn write_with_mode(&mut self, values: T, mode: WriteMode) -> Result<(), DeltaTableError>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
async fn flush_and_commit(&mut self, table: &mut DeltaTable) -> Result<Version, DeltaTableError>
```

## Implementors (2)

Read one before writing your own.

- `deltalake_core::writer::json::JsonWriter`
- `deltalake_core::writer::record_batch::RecordBatchWriter`

## Documentation

Trait for writing data to Delta tables
