# DeltaDataWriter

`deltalake_core::datafile::DeltaDataWriter`

```rust
trait DeltaDataWriter: Send
```

Also reachable as `deltalake::datafile::DeltaDataWriter`

Prose: [`api/deltalake_core.datafile.md`](../api/deltalake_core.datafile.md#deltadatawriter) · records: [`model/deltalake_core.datafile.json`](../model/deltalake_core.datafile.json)

## Required

Every implementation must supply these.

```rust
async fn write_all(Box<self>, batches: BatchStream) -> DeltaResult<Vec<Add>>
```

## Implementors (1)

Read one before writing your own.

- `deltalake_core::datafile::writer::DeltaWriter`

## Documentation

Dataset tier: a DataFusion-free writer that drains a batch stream into a
table's data files (partitioning and composing a [`DataFileWriter`] per
partition). Batches must already conform to the table schema and constraints
(callers on the basic path validate themselves).
