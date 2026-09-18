# `buoyant_kernel_engine::parquet`

Crate `buoyant_kernel_engine` · 2 public items · structured records in [`model/buoyant_kernel_engine.parquet.json`](../model/buoyant_kernel_engine.parquet.json)

## DataFileMetadata

`struct` · `buoyant_kernel_engine::parquet::DataFileMetadata`
[Full member contracts, output types and access classification](../operations/buoyant_kernel_engine.parquet.DataFileMetadata.md)

Also reachable as `delta_kernel_default_engine::parquet::DataFileMetadata`

```rust
struct DataFileMetadata
```

**Derives**: Debug

**Methods** (2)

```rust
fn location(&self) -> &url::Url
fn new(file_meta: FileMeta, stats: StructArray) -> Self
```

Metadata of a data file (typically a parquet file).

---

## DefaultParquetHandler

`struct` · `buoyant_kernel_engine::parquet::DefaultParquetHandler`
[Full member contracts, output types and access classification](../operations/buoyant_kernel_engine.parquet.DefaultParquetHandler.md)

Also reachable as `delta_kernel_default_engine::parquet::DefaultParquetHandler`

```rust
struct DefaultParquetHandler<E: TaskExecutor>
```

**Implements**: `buoyant_kernel::ParquetHandler`

**Derives**: Debug

**Methods** (4)

```rust
fn new(store: Arc<DynObjectStore>, task_executor: Arc<E>) -> Self
fn with_batch_size(self, batch_size: NonZero<usize>) -> Self
fn with_buffer_size(self, buffer_size: NonZero<usize>) -> Self
async fn write_parquet_file(&self, data: Box<dyn EngineData>, write_context: &WriteContext) -> DeltaResult<Box<dyn EngineData>>
```

**via `buoyant_kernel::ParquetHandler`**

```rust
fn read_parquet_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
fn read_parquet_footer(&self, file: &FileMeta) -> DeltaResult<ParquetFooter>
fn write_parquet_file(&self, location: url::Url, data: DeltaResultIteratorStatic<Box<dyn EngineData>>) -> DeltaResult<()>
```

---
