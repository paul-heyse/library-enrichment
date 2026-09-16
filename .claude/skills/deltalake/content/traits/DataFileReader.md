# DataFileReader

`deltalake_core::datafile::DataFileReader`

```rust
trait DataFileReader: Send + Sync
```

Also reachable as `deltalake::datafile::DataFileReader`

Prose: [`api/deltalake_core.datafile.md`](../api/deltalake_core.datafile.md#datafilereader) · records: [`model/deltalake_core.datafile.json`](../model/deltalake_core.datafile.json)

## Required

Every implementation must supply these.

```rust
async fn read_file(&self, path: object_store::path::Path) -> DeltaResult<BatchStream>
```

## Implementors (2)

Read one before writing your own.

- `deltalake_core::datafile::reader::KernelDataFileReader`
- `deltalake_core::datafile::reader::ParquetFileReader`

## Documentation

File tier: reads a single parquet data file (the per-file decryption seam,
mirroring [`DataFileWriter`]). Impl: [`reader::ParquetFileReader`].
