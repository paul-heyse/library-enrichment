# `datafusion_datasource_parquet::source`

Crate `datafusion-datasource-parquet` · 1 public items · structured records in [`model/datafusion_datasource_parquet.source.json`](../model/datafusion_datasource_parquet.source.json)

## ParquetSource

`struct` · `datafusion_datasource_parquet::source::ParquetSource`

Also reachable as `datafusion::datasource::physical_plan::ParquetSource`

```rust
struct ParquetSource
```

**Implements**: `datafusion_datasource::file::FileSource`

**Derives**: Clone, Debug

**Methods** (17)

```rust
fn max_in_list_size(&self) -> usize
fn max_predicate_cache_size(&self) -> Option<usize>
fn new(table_schema: impl Into<TableSchema>) -> Self
fn parquet_file_reader_factory(&self) -> Option<&Arc<dyn ParquetFileReaderFactory>>
fn predicate(&self) -> Option<&Arc<dyn PhysicalExpr>>
fn table_parquet_options(&self) -> &TableParquetOptions
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> datafusion_common::Result<Arc<dyn datafusion_physical_plan::ExecutionPlan>>
fn with_bloom_filter_on_read(self, bloom_filter_on_read: bool) -> Self
fn with_bloom_filter_on_write(self, enable_bloom_filter_on_write: bool) -> Self
fn with_enable_page_index(self, enable_page_index: bool) -> Self
fn with_encryption_factory(self, encryption_factory: Arc<dyn EncryptionFactory>) -> Self
fn with_metadata_size_hint(self, metadata_size_hint: usize) -> Self
fn with_parquet_file_reader_factory(self, parquet_file_reader_factory: Arc<dyn ParquetFileReaderFactory>) -> Self
fn with_predicate(&self, predicate: Arc<dyn PhysicalExpr>) -> Self
fn with_pushdown_filters(self, pushdown_filters: bool) -> Self
fn with_reorder_filters(self, reorder_filters: bool) -> Self
fn with_table_parquet_options(self, table_parquet_options: TableParquetOptions) -> Self
```

**via `datafusion_datasource::file::FileSource`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> datafusion_common::Result<TreeNodeRecursion>) -> datafusion_common::Result<TreeNodeRecursion>
fn create_file_opener(&self, _object_store: Arc<dyn ObjectStore>, _base_config: &FileScanConfig, _partition: usize) -> datafusion_common::Result<Arc<dyn FileOpener>>
fn create_morselizer(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition: usize) -> datafusion_common::Result<Box<dyn Morselizer>>
fn file_type(&self) -> &str
fn filter(&self) -> Option<Arc<dyn PhysicalExpr>>
fn fmt_extra(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
fn metrics(&self) -> &ExecutionPlanMetricsSet
fn projection(&self) -> Option<&ProjectionExprs>
fn reorder_files(&self, files: Vec<datafusion_datasource::PartitionedFile>) -> Vec<datafusion_datasource::PartitionedFile>
fn table_schema(&self) -> &TableSchema
fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> datafusion_common::Result<FilterPushdownPropagation<Arc<dyn FileSource>>>
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> datafusion_common::Result<Option<Arc<dyn FileSource>>>
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr], eq_properties: &EquivalenceProperties) -> datafusion_common::Result<SortOrderPushdownResult<Arc<dyn FileSource>>>
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> datafusion_common::Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.source.ParquetSource.md).


Execution plan for reading one or more Parquet files.

```text
            ▲
            │
            │  Produce a stream of
            │  RecordBatches
            │
┌───────────────────────┐
│                       │
│     DataSourceExec    │
│                       │
└───────────────────────┘
            ▲
            │  Asynchronously read from one
            │  or more parquet files via
            │  ObjectStore interface
            │
            │
  .───────────────────.
 │                     )
 │`───────────────────'│
 │    ObjectStore      │
 │.───────────────────.│
 │                     )
  `───────────────────'
```

# Example: Create a `DataSourceExec`
```
# use std::sync::Arc;
# use arrow::datatypes::Schema;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource_parquet::source::ParquetSource;
# use datafusion_datasource::PartitionedFile;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_physical_expr::expressions::lit;
# use datafusion_datasource::source::DataSourceExec;
# use datafusion_common::config::TableParquetOptions;

# let file_schema = Arc::new(Schema::empty());
# let object_store_url = ObjectStoreUrl::local_filesystem();
# let predicate = lit(true);
let source = Arc::new(
    ParquetSource::new(Arc::clone(&file_schema))
        .with_predicate(predicate)
);
// Create a DataSourceExec for reading `file1.parquet` with a file size of 100MB
let config = FileScanConfigBuilder::new(object_store_url, source)
   .with_file(PartitionedFile::new("file1.parquet", 100*1024*1024)).build();
let exec = DataSourceExec::from_data_source(config);
```

# Features

Supports the following optimizations:

* Concurrent reads: reads from one or more files in parallel as multiple
  partitions, including concurrently reading multiple row groups from a single
  file.

* Predicate push down: skips row groups, pages, rows based on metadata
  and late materialization. See "Predicate Pushdown" below.

* Projection pushdown: reads and decodes only the columns required.

* Limit pushdown: stop execution early after some number of rows are read.

* Custom readers: customize reading  parquet files, e.g. to cache metadata,
  coalesce I/O operations, etc. See [`ParquetFileReaderFactory`] for more
  details.

* Schema evolution: read parquet files with different schemas into a unified
  table schema. See [`DefaultPhysicalExprAdapterFactory`] for more details.

* metadata_size_hint: controls the number of bytes read from the end of the
  file in the initial I/O when the default [`ParquetFileReaderFactory`]. If a
  custom reader is used, it supplies the metadata directly and this parameter
  is ignored. [`ParquetSource::with_metadata_size_hint`] for more details.

* User provided  `ParquetAccessPlan`s to skip row groups and/or pages
  based on external information. See "Implementing External Indexes" below

# Predicate Pushdown

`DataSourceExec` uses the provided [`PhysicalExpr`] predicate as a filter to
skip reading unnecessary data and improve query performance using several techniques:

* Row group pruning: skips entire row groups based on min/max statistics
  found in [`ParquetMetaData`] and any Bloom filters that are present.

* Page pruning: skips individual pages within a ColumnChunk using the
  [Parquet PageIndex], if present.

* Row filtering: skips rows within a page using a form of late
  materialization. When possible, predicates are applied by the parquet
  decoder *during* decode (see [`ArrowPredicate`] and [`RowFilter`] for more
  details). This is only enabled if `ParquetScanOptions::pushdown_filters` is set to true.

Note: If the predicate can not be used to accelerate the scan, it is ignored
(no error is raised on predicate evaluation errors).

[`ArrowPredicate`]: parquet::arrow::arrow_reader::ArrowPredicate
[`RowFilter`]: parquet::arrow::arrow_reader::RowFilter
[Parquet PageIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

# Example: rewriting `DataSourceExec`

You can modify a `DataSourceExec` using [`ParquetSource`], for example
to change files or add a predicate.

```no_run
# use std::sync::Arc;
# use arrow::datatypes::Schema;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource::source::DataSourceExec;

# fn parquet_exec() -> DataSourceExec { unimplemented!() }
// Split a single DataSourceExec into multiple DataSourceExecs, one for each file
let exec = parquet_exec();
let data_source = exec.data_source();
let base_config = data_source.downcast_ref::<FileScanConfig>().unwrap();
let existing_file_groups = &base_config.file_groups;
let new_execs = existing_file_groups
  .iter()
  .map(|file_group| {
    // create a new exec by copying the existing exec's source config
    let new_config = FileScanConfigBuilder::from(base_config.clone())
       .with_file_groups(vec![file_group.clone()])
      .build();

    (DataSourceExec::from_data_source(new_config))
  })
  .collect::<Vec<_>>();
```

# Implementing External Indexes

It is possible to restrict the row groups and selections within those row
groups that the DataSourceExec will consider by providing an initial
`ParquetAccessPlan` as `extensions` on `PartitionedFile`. This can be
used to implement external indexes on top of parquet files and select only
portions of the files.

If the external index naturally produces a file-level
[`RowSelection`](parquet::arrow::arrow_reader::RowSelection), wrap it in
[`ParquetRowSelection`](crate::ParquetRowSelection) and provide it as an
extension. DataFusion will use the parquet metadata to split the selection
into row-group-level access.

The `DataSourceExec` will try and reduce any provided `ParquetAccessPlan`
further based on the contents of `ParquetMetadata` and other settings.

## Example of providing a ParquetAccessPlan

```
# use std::sync::Arc;
# use arrow::datatypes::{Schema, SchemaRef};
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource_parquet::ParquetAccessPlan;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource_parquet::source::ParquetSource;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_datasource::source::DataSourceExec;

# fn schema() -> SchemaRef {
#   Arc::new(Schema::empty())
# }
// create an access plan to scan row group 0, 1 and 3 and skip row groups 2 and 4
let mut access_plan = ParquetAccessPlan::new_all(5);
access_plan.skip(2);
access_plan.skip(4);
// provide the plan as extension to the FileScanConfig
let partitioned_file = PartitionedFile::new("my_file.parquet", 1234)
  .with_extension(access_plan);
// create a FileScanConfig to scan this file
let config = FileScanConfigBuilder::new(ObjectStoreUrl::local_filesystem(), Arc::new(ParquetSource::new(schema())))
    .with_file(partitioned_file).build();
// this parquet DataSourceExec will not even try to read row groups 2 and 4. Additional
// pruning based on predicates may also happen
let exec = DataSourceExec::from_data_source(config);
```

For a complete example, see the [`advanced_parquet_index` example]).

[`parquet_index_advanced` example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/data_io/parquet_advanced_index.rs

# Execution Overview

* Step 1: `DataSourceExec::execute` is called, returning a `FileStream`
  configured to morselize parquet files with a `ParquetMorselizer`.

* Step 2: When the stream is polled, the `ParquetMorselizer` is called to
  plan the file.

* Step 3: The `ParquetMorselizer` gets the [`ParquetMetaData`] (file metadata)
  via [`ParquetFileReaderFactory`], creating a `ParquetAccessPlan` by
  applying predicates to metadata. The plan and projections are used to
  determine what pages must be read.

* Step 4: The stream begins reading data, fetching the required parquet
  pages incrementally decoding them, and applying any row filters (see
  [`Self::with_pushdown_filters`]).

* Step 5: As each [`RecordBatch`] is read, it may be adapted by a
  [`DefaultPhysicalExprAdapterFactory`] to match the table schema. By default missing columns are
  filled with nulls, but this can be customized via [`PhysicalExprAdapterFactory`].

[`RecordBatch`]: arrow::record_batch::RecordBatch
[`ParquetMetadata`]: parquet::file::metadata::ParquetMetaData
[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

---
