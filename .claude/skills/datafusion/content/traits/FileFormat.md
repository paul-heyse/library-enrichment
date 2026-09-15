# FileFormat

`datafusion_datasource::file_format::FileFormat`

```rust
trait FileFormat: Any + Send + Sync + fmt::Debug
```

Also reachable as `datafusion::datasource::file_format::FileFormat`

Prose: [`api/datafusion_datasource.file_format.md`](../api/datafusion_datasource.file_format.md#fileformat) · records: [`model/datafusion_datasource.file_format.json`](../model/datafusion_datasource.file_format.json)

## Required

Every implementation must supply these.

```rust
fn compression_type(&self) -> Option<FileCompressionType>
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
fn get_ext(&self) -> String
fn get_ext_with_compression(&self, _file_compression_type: &FileCompressionType) -> Result<String>
async fn infer_schema(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
async fn infer_stats(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<Statistics>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
async fn create_writer_physical_plan(&self, _input: Arc<dyn ExecutionPlan>, _state: &dyn Session, _conf: FileSinkConfig, _order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
async fn infer_ordering(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, _table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Option<LexOrdering>>
async fn infer_stats_and_ordering(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<FileMeta>
```

## Implementors (5)

Read one before writing your own.

- `datafusion_datasource_arrow::file_format::ArrowFormat`
- `datafusion_datasource_avro::file_format::AvroFormat`
- `datafusion_datasource_csv::file_format::CsvFormat`
- `datafusion_datasource_json::file_format::JsonFormat`
- `datafusion_datasource_parquet::file_format::ParquetFormat`

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/custom_data_source/custom_file_format.rs`](../corpus/examples/custom_data_source/custom_file_format.rs)

## Documentation

This trait abstracts all the file format specific implementations
from the [`TableProvider`]. This helps code re-utilization across
providers that support the same file formats.

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html
