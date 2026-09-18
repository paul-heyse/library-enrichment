# `deltalake_core::writer`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.writer.json`](../model/deltalake_core.writer.json)

## WriteMode

`enum` · `deltalake_core::writer::WriteMode`
[Full member contracts, output types and access classification](../operations/deltalake_core.writer.WriteMode.md)

Also reachable as `deltalake::writer::WriteMode`

```rust
enum WriteMode
```

**Variants**: `Default`, `MergeSchema`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

Write mode for the [DeltaWriter]

---

## DeltaWriter

`trait` · `deltalake_core::writer::DeltaWriter`
[Full member contracts, output types and access classification](../operations/deltalake_core.writer.DeltaWriter.md)

Also reachable as `deltalake::writer::DeltaWriter`

```rust
trait DeltaWriter<T>
```

**Implementors** (2)

- `deltalake_core::writer::json::JsonWriter`
- `deltalake_core::writer::record_batch::RecordBatchWriter`

**Methods** (4)

```rust
async fn flush(&mut self) -> Result<Vec<Add>, DeltaTableError>
async fn flush_and_commit(&mut self, table: &mut DeltaTable) -> Result<Version, DeltaTableError>
async fn write(&mut self, values: T) -> Result<(), DeltaTableError>
async fn write_with_mode(&mut self, values: T, mode: WriteMode) -> Result<(), DeltaTableError>
```

Trait for writing data to Delta tables

---
