# `buoyant_kernel::metrics::metered_parquet`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.metrics.metered_parquet.json`](../model/buoyant_kernel.metrics.metered_parquet.json)

## MeteredParquetHandler

`struct` · `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.metered_parquet.MeteredParquetHandler.md)

Also reachable as `buoyant_kernel::metrics::MeteredParquetHandler`, `delta_kernel::metrics::metered_parquet::MeteredParquetHandler`

```rust
struct MeteredParquetHandler
```

**Implements**: `buoyant_kernel::ParquetHandler`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn ParquetHandler>) -> Self
```

**via `buoyant_kernel::ParquetHandler`**

```rust
fn read_parquet_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
fn read_parquet_footer(&self, file: &FileMeta) -> DeltaResult<ParquetFooter>
fn write_parquet_file(&self, location: url::Url, data: DeltaResultIteratorStatic<Box<dyn EngineData>>) -> DeltaResult<()>
```

Decorator over an engine-provided `Arc<dyn ParquetHandler>` that emits a
`ParquetReadCompleted` span on every `read_parquet_files` call.
`read_parquet_footer` and `write_parquet_file` are pass-through and emit nothing.

---
