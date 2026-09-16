# DataFileWriter

`deltalake_core::datafile::DataFileWriter`

```rust
trait DataFileWriter: Send
```

Also reachable as `deltalake::datafile::DataFileWriter`

Prose: [`api/deltalake_core.datafile.md`](../api/deltalake_core.datafile.md#datafilewriter) · records: [`model/deltalake_core.datafile.json`](../model/deltalake_core.datafile.json)

## Required

Every implementation must supply these.

```rust
async fn abort(Box<self>) -> DeltaResult<()>
async fn close(Box<self>) -> DeltaResult<Vec<Add>>
async fn write(&mut self, batch: &RecordBatch) -> DeltaResult<()>
```

## Implementors (1)

Read one before writing your own.

- `deltalake_core::datafile::writer::PartitionWriter`

## Documentation

File tier: writes a single Delta data file (or size-split set for one
partition). The per-file seam where parquet `WriterProperties`/encryption
attach. Impl: [`writer::PartitionWriter`].
