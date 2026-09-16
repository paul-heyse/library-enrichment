# DeltaDataReaderExt

`deltalake_core::datafile::datafusion_ext::DeltaDataReaderExt`

```rust
trait DeltaDataReaderExt: DeltaDataReader
```

Also reachable as `deltalake::datafile::datafusion_ext::DeltaDataReaderExt`

Prose: [`api/deltalake_core.datafile.datafusion_ext.md`](../api/deltalake_core.datafile.datafusion_ext.md#deltadatareaderext) · records: [`model/deltalake_core.datafile.datafusion_ext.json`](../model/deltalake_core.datafile.datafusion_ext.json)

## Required

Every implementation must supply these.

```rust
async fn scan(&self, session: &dyn Session, options: ScanOptions) -> DeltaResult<SendableRecordBatchStream>
```

## Implementors (1)

Read one before writing your own.

- `deltalake_core::datafile::datafusion_ext::DataFusionDataReader`

## Documentation

DataFusion extension to [`DeltaDataReader`]: a full scan with pushdown.
