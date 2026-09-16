# DeltaDataWriterExt

`deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt`

```rust
trait DeltaDataWriterExt
```

Also reachable as `deltalake::datafile::datafusion_ext::DeltaDataWriterExt`

Prose: [`api/deltalake_core.datafile.datafusion_ext.md`](../api/deltalake_core.datafile.datafusion_ext.md#deltadatawriterext) · records: [`model/deltalake_core.datafile.datafusion_ext.json`](../model/deltalake_core.datafile.datafusion_ext.json)

## Required

Every implementation must supply these.

```rust
async fn write_plan(Box<self>, session: &dyn Session, plan: Arc<dyn ExecutionPlan>) -> DeltaResult<Vec<Add>>
```

## Implementors (1)

Read one before writing your own.

- `deltalake_core::datafile::writer::DeltaWriter`

## Documentation

DataFusion extension to [`DeltaDataWriter`]: write the output of an execution plan.
