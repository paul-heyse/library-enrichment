# ParquetHandler

`buoyant_kernel::ParquetHandler`

```rust
trait ParquetHandler: AsAny
```

Also reachable as `delta_kernel::ParquetHandler`

Prose: [`api/buoyant_kernel.md`](../api/buoyant_kernel.md#parquethandler) · records: [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## Required

Every implementation must supply these.

```rust
fn read_parquet_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
fn read_parquet_footer(&self, file: &FileMeta) -> DeltaResult<ParquetFooter>
fn write_parquet_file(&self, location: url::Url, data: DeltaResultIteratorStatic<Box<dyn EngineData>>) -> DeltaResult<()>
```

## Implementors (3)

Read one before writing your own.

- `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler`
- `buoyant_kernel_engine::parquet::DefaultParquetHandler`
- `deltalake_core::delta_datafusion::engine::file_formats::DataFusionFileFormatHandler`

## Documentation

Provides Parquet file related functionalities to Delta Kernel.

Connectors can leverage this trait to provide their own custom
implementation of Parquet data file functionalities to Delta Kernel.
