# `datafusion_datasource_csv::source`

Crate `datafusion-datasource-csv` · 3 public items · structured records in [`model/datafusion_datasource_csv.source.json`](../model/datafusion_datasource_csv.source.json)

## plan_to_csv

`function` · `datafusion_datasource_csv::source::plan_to_csv`

Also reachable as `datafusion::datasource::physical_plan::csv::plan_to_csv`

```rust
async fn plan_to_csv(task_ctx: std::sync::Arc<datafusion_execution::TaskContext>, plan: std::sync::Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.source.plan_to_csv.md).


---

## CsvOpener

`struct` · `datafusion_datasource_csv::source::CsvOpener`

Also reachable as `datafusion::datasource::physical_plan::CsvOpener`, `datafusion::datasource::physical_plan::csv::CsvOpener`

```rust
struct CsvOpener
```

**Implements**: `datafusion_datasource::file_stream::FileOpener`

**Methods** (1)

```rust
fn new(config: Arc<CsvSource>, file_compression_type: FileCompressionType, object_store: Arc<dyn ObjectStore>) -> Self
```

**via `datafusion_datasource::file_stream::FileOpener`**

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.source.CsvOpener.md).


A [`FileOpener`] that opens a CSV file and yields a [`FileOpenFuture`]

---

## CsvSource

`struct` · `datafusion_datasource_csv::source::CsvSource`

Also reachable as `datafusion::datasource::physical_plan::CsvSource`, `datafusion::datasource::physical_plan::csv::CsvSource`

```rust
struct CsvSource
```

**Implements**: `datafusion_datasource::file::FileSource`

**Derives**: Clone, Debug

**Methods** (15)

```rust
fn comment(&self) -> Option<u8>
fn delimiter(&self) -> u8
fn escape(&self) -> Option<u8>
fn has_header(&self) -> bool
fn new(table_schema: impl Into<TableSchema>) -> Self
fn newlines_in_values(&self) -> bool
fn quote(&self) -> u8
fn terminator(&self) -> Option<u8>
fn truncate_rows(&self) -> bool
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn with_comment(&self, comment: Option<u8>) -> Self
fn with_csv_options(self, options: CsvOptions) -> Self
fn with_escape(&self, escape: Option<u8>) -> Self
fn with_terminator(&self, terminator: Option<u8>) -> Self
fn with_truncate_rows(&self, truncate_rows: bool) -> Self
```

**via `datafusion_datasource::file::FileSource`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn datafusion_physical_plan::PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition_index: usize) -> Result<Arc<dyn FileOpener>>
fn file_type(&self) -> &str
fn fmt_extra(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn metrics(&self) -> &ExecutionPlanMetricsSet
fn projection(&self) -> Option<&ProjectionExprs>
fn supports_repartitioning(&self) -> bool
fn table_schema(&self) -> &TableSchema
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>>
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.source.CsvSource.md).


A Config for [`CsvOpener`]

# Example: create a `DataSourceExec` for CSV
```
# use std::sync::Arc;
# use arrow::datatypes::Schema;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource_csv::source::CsvSource;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_datasource::source::DataSourceExec;
# use datafusion_common::config::CsvOptions;

# let object_store_url = ObjectStoreUrl::local_filesystem();
# let file_schema = Arc::new(Schema::empty());

let options = CsvOptions {
    has_header: Some(true),
    delimiter: b',',
    quote: b'"',
    newlines_in_values: Some(true), // The file contains newlines in values
    ..Default::default()
};
let source = Arc::new(CsvSource::new(file_schema.clone())
    .with_csv_options(options)
    .with_terminator(Some(b'#'))
);
// Create a DataSourceExec for reading the first 100MB of `file1.csv`
let config = FileScanConfigBuilder::new(object_store_url, source)
    .with_file(PartitionedFile::new("file1.csv", 100*1024*1024))
    .build();
let exec = (DataSourceExec::from_data_source(config));
```

---
