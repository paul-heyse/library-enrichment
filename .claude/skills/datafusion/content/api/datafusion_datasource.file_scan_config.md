# `datafusion_datasource::file_scan_config`

Crate `datafusion-datasource` · 5 public items · structured records in [`model/datafusion_datasource.file_scan_config.json`](../model/datafusion_datasource.file_scan_config.json)

## output_partitioning_from_partition_fields

`function` · `datafusion_datasource::file_scan_config::output_partitioning_from_partition_fields`

```rust
fn output_partitioning_from_partition_fields(schema: &arrow::datatypes::Schema, partition_cols: &arrow::datatypes::Fields, partition_count: usize) -> Option<datafusion_physical_expr::Partitioning>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_scan_config.output_partitioning_from_partition_fields.md).


Builds output partitioning over `partition_cols` (resolved to their indices in
`schema`) with `partition_count` partitions. Returns `None` when there are no
partition columns. Callers use this to declare the output partitioning of a scan
whose file groups are organized by partition column values.

---

## wrap_partition_type_in_dict

`function` · `datafusion_datasource::file_scan_config::wrap_partition_type_in_dict`

Also reachable as `datafusion::datasource::physical_plan::wrap_partition_type_in_dict`

```rust
fn wrap_partition_type_in_dict(val_type: arrow::datatypes::DataType) -> arrow::datatypes::DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_scan_config.wrap_partition_type_in_dict.md).


Convert type to a type suitable for use as a `ListingTable`
partition column. Returns `Dictionary(UInt16, val_type)`, which is
a reasonable trade off between a reasonable number of partition
values and space efficiency.

This use this to specify types for partition columns. However
you MAY also choose not to dictionary-encode the data or to use a
different dictionary type.

Use [`wrap_partition_value_in_dict`] to wrap a [`ScalarValue`] in the same say.

---

## wrap_partition_value_in_dict

`function` · `datafusion_datasource::file_scan_config::wrap_partition_value_in_dict`

Also reachable as `datafusion::datasource::physical_plan::wrap_partition_value_in_dict`

```rust
fn wrap_partition_value_in_dict(val: datafusion_common::ScalarValue) -> datafusion_common::ScalarValue
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_scan_config.wrap_partition_value_in_dict.md).


Convert a [`ScalarValue`] of partition columns to a type, as
described in the documentation of [`wrap_partition_type_in_dict`],
which can wrap the types.

---

## FileScanConfig

`struct` · `datafusion_datasource::file_scan_config::FileScanConfig`

Also reachable as `datafusion::datasource::physical_plan::FileScanConfig`

```rust
struct FileScanConfig
```

**Fields**: `object_store_url`, `file_groups`, `constraints`, `limit`, `preserve_order`, `output_ordering`, `file_compression_type`, `file_source`, `batch_size`, `expr_adapter_factory`, `output_partitioning`

**Implements**: `datafusion_datasource::source::DataSource`, `datafusion_physical_plan::display::DisplayAs`

**Derives**: Clone, Debug

**Methods** (13)

```rust
fn file_column_projection_indices(&self) -> Option<Vec<usize>>
fn file_schema(&self) -> &SchemaRef
fn file_source(&self) -> &Arc<dyn FileSource>
fn newlines_in_values(&self) -> bool
fn parse_table_schema_from_proto(conf: &protobuf::FileScanExecConf) -> Result<TableSchema>
fn projected_constraints(&self) -> Constraints
fn projected_schema(&self) -> Result<Arc<Schema>>
fn split_groups_by_statistics(table_schema: &SchemaRef, file_groups: &[FileGroup], sort_order: &LexOrdering) -> Result<Vec<FileGroup>>
fn split_groups_by_statistics_with_target_partitions(table_schema: &SchemaRef, file_groups: &[FileGroup], sort_order: &LexOrdering, target_partitions: usize) -> Result<Vec<FileGroup>>
fn statistics(&self) -> Statistics
fn table_partition_cols(&self) -> &Fields
fn try_from_proto(conf: &protobuf::FileScanExecConf, ctx: &ExecutionPlanDecodeCtx<'_>, file_source: Arc<dyn FileSource>) -> Result<FileScanConfig>
fn try_to_proto(&self, ctx: &ExecutionPlanEncodeCtx<'_>) -> Result<protobuf::FileScanExecConf>
```

**via `datafusion_datasource::source::DataSource`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn create_sibling_state(&self, config: &ConfigOptions) -> Option<Arc<dyn Any + Send + Sync>>
fn eq_properties(&self) -> EquivalenceProperties
fn fetch(&self) -> Option<usize>
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> FmtResult
fn metrics(&self) -> ExecutionPlanMetricsSet
fn open(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn open_with_args(&self, args: OpenArgs) -> Result<SendableRecordBatchStream>
fn output_partitioning(&self) -> Partitioning
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
fn repartitioned(&self, target_partitions: usize, repartition_file_min_size: usize, output_ordering: Option<LexOrdering>) -> Result<Option<Arc<dyn DataSource>>>
fn scheduling_type(&self) -> SchedulingType
fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn DataSource>>>
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn DataSource>>>
fn try_swapping_with_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn DataSource>>>
fn try_to_proto(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn DataSource>>
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn DataSource>>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> FmtResult
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md).


[`FileScanConfig`] represents scanning data from a group of files

`FileScanConfig` is used to create a [`DataSourceExec`], the physical plan
for scanning files with a particular file format.

The [`FileSource`] (e.g. `ParquetSource`, `CsvSource`, etc.) is responsible
for creating the actual execution plan to read the files based on a
`FileScanConfig`. Fields in a `FileScanConfig` such as Statistics represent
information about the files **before** any projection or filtering is
applied in the file source.

Use [`FileScanConfigBuilder`] to construct a `FileScanConfig`.

Use [`DataSourceExec::from_data_source`] to create a [`DataSourceExec`] from
a `FileScanConfig`.

# Example
```
# use std::sync::Arc;
# use arrow::datatypes::{Field, Fields, DataType, Schema, SchemaRef};
# use object_store::ObjectStore;
# use datafusion_common::Result;
# use datafusion_common::tree_node::TreeNodeRecursion;
# use datafusion_datasource::file::FileSource;
# use datafusion_physical_plan::PhysicalExpr;
# use datafusion_datasource::file_groups::FileGroup;
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource::file_stream::FileOpener;
# use datafusion_datasource::source::DataSourceExec;
# use datafusion_datasource::table_schema::TableSchema;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_physical_expr::projection::ProjectionExprs;
# use datafusion_physical_plan::ExecutionPlan;
# use datafusion_physical_plan::metrics::ExecutionPlanMetricsSet;
# let file_schema = Arc::new(Schema::new(vec![
#  Field::new("c1", DataType::Int32, false),
#  Field::new("c2", DataType::Int32, false),
#  Field::new("c3", DataType::Int32, false),
#  Field::new("c4", DataType::Int32, false),
# ]));
# // Note: crate mock ParquetSource, as ParquetSource is not in the datasource crate
#[derive(Clone)]
# struct ParquetSource {
#    table_schema: TableSchema,
# };
# impl FileSource for ParquetSource {
#  fn create_file_opener(&self, _: Arc<dyn ObjectStore>, _: &FileScanConfig, _: usize) -> Result<Arc<dyn FileOpener>> { unimplemented!() }
#  fn table_schema(&self) -> &TableSchema { &self.table_schema }
#  fn with_batch_size(&self, _: usize) -> Arc<dyn FileSource> { unimplemented!() }
#  fn metrics(&self) -> &ExecutionPlanMetricsSet { unimplemented!() }
#  fn file_type(&self) -> &str { "parquet" }
#  // Note that this implementation drops the projection on the floor, it is not complete!
#  fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>> { Ok(Some(Arc::new(self.clone()) as Arc<dyn FileSource>)) }
#  fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion> { Ok(TreeNodeRecursion::Continue) }
#  }
# impl ParquetSource {
#  fn new(table_schema: impl Into<TableSchema>) -> Self { Self {table_schema: table_schema.into()} }
# }
// create FileScan config for reading parquet files from file://
let object_store_url = ObjectStoreUrl::local_filesystem();
let file_source = Arc::new(ParquetSource::new(file_schema.clone()));
let config = FileScanConfigBuilder::new(object_store_url, file_source)
  .with_limit(Some(1000))            // read only the first 1000 records
  .with_projection_indices(Some(vec![2, 3])) // project columns 2 and 3
  .expect("Failed to push down projection")
   // Read /tmp/file1.parquet with known size of 1234 bytes in a single group
  .with_file(PartitionedFile::new("file1.parquet", 1234))
  // Read /tmp/file2.parquet 56 bytes and /tmp/file3.parquet 78 bytes
  // in a  single row group
  .with_file_group(FileGroup::new(vec![
   PartitionedFile::new("file2.parquet", 56),
   PartitionedFile::new("file3.parquet", 78),
  ])).build();
// create an execution plan from the config
let plan: Arc<dyn ExecutionPlan> = DataSourceExec::from_data_source(config);
```

[`DataSourceExec`]: crate::source::DataSourceExec
[`DataSourceExec::from_data_source`]: crate::source::DataSourceExec::from_data_source

---

## FileScanConfigBuilder

`struct` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder`

Also reachable as `datafusion::datasource::physical_plan::FileScanConfigBuilder`

```rust
struct FileScanConfigBuilder
```

**Implements**: `core::convert::From`

**Derives**: Clone

**Methods** (18)

```rust
fn build(self) -> FileScanConfig
fn new(object_store_url: ObjectStoreUrl, file_source: Arc<dyn FileSource>) -> Self
fn table_schema(&self) -> &SchemaRef
fn with_batch_size(self, batch_size: Option<usize>) -> Self
fn with_constraints(self, constraints: Constraints) -> Self
fn with_expr_adapter(self, expr_adapter: Option<Arc<dyn PhysicalExprAdapterFactory>>) -> Self
fn with_file(self, partitioned_file: PartitionedFile) -> Self
fn with_file_compression_type(self, file_compression_type: FileCompressionType) -> Self
fn with_file_group(self, file_group: FileGroup) -> Self
fn with_file_groups(self, file_groups: Vec<FileGroup>) -> Self
fn with_limit(self, limit: Option<usize>) -> Self
fn with_output_ordering(self, output_ordering: Vec<LexOrdering>) -> Self
fn with_output_partitioning(self, output_partitioning: Option<Partitioning>) -> Self
fn with_preserve_order(self, order_sensitive: bool) -> Self
fn with_projection(self, indices: Option<Vec<usize>>) -> Self
fn with_projection_indices(self, indices: Option<Vec<usize>>) -> Result<Self>
fn with_source(self, file_source: Arc<dyn FileSource>) -> Self
fn with_statistics(self, statistics: Statistics) -> Self
```

**via `core::convert::From`**

```rust
fn from(config: FileScanConfig) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_scan_config.FileScanConfigBuilder.md).


A builder for [`FileScanConfig`]'s.

Example:

```rust
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_datasource::file_scan_config::{FileScanConfigBuilder, FileScanConfig};
# use datafusion_datasource::file_compression_type::FileCompressionType;
# use datafusion_datasource::file_groups::FileGroup;
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource::table_schema::TableSchema;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_common::Statistics;
# use datafusion_datasource::file::FileSource;

# fn main() {
# fn with_source(file_source: Arc<dyn FileSource>) {
    // Create a schema for our Parquet files
    let file_schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("value", DataType::Utf8, false),
    ]));

    // Create partition columns
    let partition_cols = vec![
        Arc::new(Field::new("date", DataType::Utf8, false)),
    ];

    // Create table schema with file schema and partition columns
    let table_schema = TableSchema::builder(file_schema)
        .with_table_partition_cols(partition_cols)
        .build();

    // Create a builder for scanning Parquet files from a local filesystem
    let config = FileScanConfigBuilder::new(
        ObjectStoreUrl::local_filesystem(),
        file_source,
    )
    // Set a limit of 1000 rows
    .with_limit(Some(1000))
    // Project only the first column
    .with_projection_indices(Some(vec![0]))
    .expect("Failed to push down projection")
    // Add a file group with two files
    .with_file_group(FileGroup::new(vec![
        PartitionedFile::new("data/date=2024-01-01/file1.parquet", 1024),
        PartitionedFile::new("data/date=2024-01-01/file2.parquet", 2048),
    ]))
    // Set compression type
    .with_file_compression_type(FileCompressionType::UNCOMPRESSED)
    // Build the final config
    .build();
# }
# }
```

---
