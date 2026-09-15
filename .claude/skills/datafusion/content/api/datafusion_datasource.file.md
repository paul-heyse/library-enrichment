# `datafusion_datasource::file`

Crate `datafusion-datasource` · 2 public items · structured records in [`model/datafusion_datasource.file.json`](../model/datafusion_datasource.file.json)

## as_file_source

`function` · `datafusion_datasource::file::as_file_source`

Also reachable as `datafusion_datasource::as_file_source`

```rust
fn as_file_source<T: FileSource + 'static>(source: T) -> std::sync::Arc<dyn FileSource>
```

Helper function to convert any type implementing [`FileSource`] to `Arc<dyn FileSource>`

---

## FileSource

`trait` · `datafusion_datasource::file::FileSource`

Also reachable as `datafusion::datasource::physical_plan::FileSource`

```rust
trait FileSource: Any + Send + Sync
```

**Implementors** (5)

- `datafusion_datasource_arrow::source::ArrowSource`
- `datafusion_datasource_avro::source::AvroSource`
- `datafusion_datasource_csv::source::CsvSource`
- `datafusion_datasource_json::source::JsonSource`
- `datafusion_datasource_parquet::source::ParquetSource`

**Methods** (20)

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition: usize) -> Result<Arc<dyn FileOpener>>
fn create_morselizer(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition: usize) -> Result<Box<dyn Morselizer>>
fn file_type(&self) -> &str
fn filter(&self) -> Option<Arc<dyn PhysicalExpr>>
fn fmt_extra(&self, _t: DisplayFormatType, _f: &mut Formatter<'_>) -> fmt::Result
fn metrics(&self) -> &ExecutionPlanMetricsSet
fn projection(&self) -> Option<&ProjectionExprs>
fn reorder_files(&self, files: Vec<PartitionedFile>) -> Vec<PartitionedFile>
fn repartitioned(&self, target_partitions: usize, repartition_file_min_size: usize, output_ordering: Option<LexOrdering>, config: &FileScanConfig) -> Result<Option<FileScanConfig>>
fn schema_adapter_factory(&self) -> Option<Arc<dyn SchemaAdapterFactory>>
fn supports_repartitioning(&self) -> bool
fn table_schema(&self) -> &table_schema::TableSchema
fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn FileSource>>>
fn try_pushdown_projection(&self, _projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr], eq_properties: &EquivalenceProperties) -> Result<SortOrderPushdownResult<Arc<dyn FileSource>>>
fn try_reverse_output(&self, _order: &[PhysicalSortExpr], _eq_properties: &EquivalenceProperties) -> Result<SortOrderPushdownResult<Arc<dyn FileSource>>>
fn try_to_proto(&self, _base: &FileScanConfig, _ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
fn with_schema_adapter_factory(&self, _factory: Arc<dyn SchemaAdapterFactory>) -> Result<Arc<dyn FileSource>>
```

File format specific behaviors for [`DataSource`]

# Schema information
There are two important schemas for a [`FileSource`]:
1. [`Self::table_schema`] -- the schema for the overall table
   (file data plus partition columns)
2. The logical output schema, comprised of [`Self::table_schema`] with
   [`Self::projection`] applied

See more details on specific implementations:
* [`ArrowSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.ArrowSource.html)
* [`AvroSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.AvroSource.html)
* [`CsvSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.CsvSource.html)
* [`JsonSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.JsonSource.html)
* [`ParquetSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.ParquetSource.html)

[`DataSource`]: crate::source::DataSource

---
