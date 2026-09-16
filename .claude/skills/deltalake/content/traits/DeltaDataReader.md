# DeltaDataReader

`deltalake_core::datafile::DeltaDataReader`

```rust
trait DeltaDataReader: Send + Sync
```

Also reachable as `deltalake::datafile::DeltaDataReader`

Prose: [`api/deltalake_core.datafile.md`](../api/deltalake_core.datafile.md#deltadatareader) · records: [`model/deltalake_core.datafile.json`](../model/deltalake_core.datafile.json)

## Required

Every implementation must supply these.

```rust
async fn read(&self, options: ReadOptions) -> DeltaResult<BatchStream>
```

## Implementors (3)

Read one before writing your own.

- `deltalake_core::datafile::datafusion_ext::DataFusionDataReader`
- `deltalake_core::datafile::reader::KernelDataReader`
- `deltalake_core::datafile::reader::ParquetTableReader`

## Documentation

Dataset tier: a DataFusion-free reader that composes the file tier
([`DataFileReader`]) across a table's data files, applying deletion
vectors, partition values, and column-mapping transforms.
