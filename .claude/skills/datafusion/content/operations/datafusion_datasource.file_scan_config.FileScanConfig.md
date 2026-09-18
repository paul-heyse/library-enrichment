# `datafusion_datasource::file_scan_config::FileScanConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_scan_config.FileScanConfig.json).

<a id="op-d7aa66cd63851cc944f55f57"></a>
## FileScanConfig

`struct` · `datafusion_datasource::file_scan_config::FileScanConfig` · datafusion-datasource 55.1.0

```rust
struct FileScanConfig
```

Source: `src/file_scan_config/mod.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

[`FileScanConfig`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-d7aa66cd63851cc944f55f57) represents scanning data from a group of files

`FileScanConfig` is used to create a [`DataSourceExec`], the physical plan
for scanning files with a particular file format.

The [`FileSource`](../operations/datafusion_datasource.file.FileSource.md#op-8c1b19f80ea0c73466216a63) (e.g. `ParquetSource`, `CsvSource`, etc.) is responsible
for creating the actual execution plan to read the files based on a
`FileScanConfig`. Fields in a `FileScanConfig` such as Statistics represent
information about the files **before** any projection or filtering is
applied in the file source.

Use [`FileScanConfigBuilder`](../operations/datafusion_datasource.file_scan_config.FileScanConfigBuilder.md#op-fc85b3f3f7ee6a9b542e85c0) to construct a `FileScanConfig`.

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

<a id="op-547a06d5d731fd3800fa1f10"></a>
## apply_expressions

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::apply_expressions` · datafusion-datasource 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:1169`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c0028e939b982a1cc5447d1"></a>
## batch_size

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::batch_size` · datafusion-datasource 55.1.0

```rust
batch_size: Option<usize>
```

Source: `src/file_scan_config/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Batch size while creating new batches
Defaults to [`datafusion_common::config::ExecutionOptions`](../operations/datafusion_common.config.ExecutionOptions.md#op-15fab70a83be6ab4e79c51a4) batch_size.

<a id="op-14167a8ff18d24df38e1c7b1"></a>
## clone

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> FileScanConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 10], "end": [151, 15], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_scan_config/mod.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d96f3d28f59761cd5d8fc751"></a>
## constraints

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::constraints` · datafusion-datasource 55.1.0

```rust
constraints: datafusion_common::Constraints
```

Source: `src/file_scan_config/mod.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Table constraints

<a id="op-89580010e266a36e3591b8c9"></a>
## create_sibling_state

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::create_sibling_state` · datafusion-datasource 55.1.0

```rust
fn create_sibling_state(&self, config: &ConfigOptions) -> Option<Arc<dyn Any + Send + Sync>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:1185`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create any shared state that should be passed between sibling streams
during one execution.

This returns `None` when sibling streams must not share work, such as
when file order must be preserved, the file groups define the output
partitioning needed for the rest of the plan, or work stealing is
disabled via
`datafusion.execution.enable_file_stream_work_stealing`.

<a id="op-defb6e1302a18843e1451fc7"></a>
## eq_properties

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::eq_properties` · datafusion-datasource 55.1.0

```rust
fn eq_properties(&self) -> EquivalenceProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:877`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Computes the effective equivalence properties of this file scan, taking
into account the file schema, any projections or filters applied by the
file source, and the output ordering.

<a id="op-331353fdbc468234ad9a5c8b"></a>
## expr_adapter_factory

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::expr_adapter_factory` · datafusion-datasource 55.1.0

```rust
expr_adapter_factory: Option<std::sync::Arc<dyn PhysicalExprAdapterFactory>>
```

Source: `src/file_scan_config/mod.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Expression adapter used to adapt filters and projections that are pushed down into the scan
from the logical schema to the physical schema of the file.

<a id="op-7bc03fdfe8d3c8c2397f7ed6"></a>
## fetch

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::fetch` · datafusion-datasource 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:965`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4219091b677f975e59a7d10"></a>
## file_column_projection_indices

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::file_column_projection_indices` · datafusion-datasource 55.1.0

```rust
fn file_column_projection_indices(&self) -> Option<Vec<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1340`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aafc450af33df3a4bb8ce76d"></a>
## file_compression_type

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::file_compression_type` · datafusion-datasource 55.1.0

```rust
file_compression_type: file_compression_type::FileCompressionType
```

Source: `src/file_scan_config/mod.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

File compression type

<a id="op-d9b958594bf2f17701bb745b"></a>
## file_groups

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::file_groups` · datafusion-datasource 55.1.0

```rust
file_groups: Vec<file_groups::FileGroup>
```

Source: `src/file_scan_config/mod.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

List of files to be processed, grouped into partitions

Each file must have a schema of `file_schema` or a subset. If
a particular file has a subset, the missing columns are
padded with NULLs.

DataFusion may attempt to read each partition of files
concurrently, however files *within* a partition will be read
sequentially, one after the next.

Note that when `datafusion.execution.enable_file_stream_work_stealing`
is enabled (the default), files may be reassigned to a different
partition at runtime unless `preserve_order` or
`partitioned_by_file_group` is set, so a file is not guaranteed to be
read by the partition it is grouped under here.

<a id="op-b3afa7cec18b70f284aa8a23"></a>
## file_schema

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::file_schema` · datafusion-datasource 55.1.0

```rust
fn file_schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1253`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get the file schema (schema of the files without partition columns)

<a id="op-47b12dd2c48dbed38d686054"></a>
## file_source

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::file_source` · datafusion-datasource 55.1.0

```rust
fn file_source(&self) -> &Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1515`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the file_source

<a id="op-ff73321f81b0477e3696b477"></a>
## file_source

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::file_source` · datafusion-datasource 55.1.0

```rust
file_source: std::sync::Arc<dyn FileSource>
```

Source: `src/file_scan_config/mod.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

File source such as `ParquetSource`, `CsvSource`, `JsonSource`, etc.

<a id="op-ed879ba5f3858c4ee505f3aa"></a>
## fmt

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1524, 1], "end": [1534, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_scan_config/mod.rs:1525`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8800397b4f443269515cec5c"></a>
## fmt_as

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::fmt_as` · datafusion-datasource 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> FmtResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:732`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8ead6cf3c0444fa5bc30894"></a>
## fmt_as

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::fmt_as` · datafusion-datasource 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> FmtResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1536, 1], "end": [1560, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/file_scan_config/mod.rs:1537`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf3456e9621630db7e805df0"></a>
## limit

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::limit` · datafusion-datasource 55.1.0

```rust
limit: Option<usize>
```

Source: `src/file_scan_config/mod.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The maximum number of records to read from this plan. If `None`,
all records after filtering are returned.

<a id="op-131a818cd86ea1cdebd7ae62"></a>
## metrics

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::metrics` · datafusion-datasource 55.1.0

```rust
fn metrics(&self) -> ExecutionPlanMetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:969`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb6a0400d1ef85dec9509f94"></a>
## newlines_in_values

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::newlines_in_values` · datafusion-datasource 55.1.0

```rust
fn newlines_in_values(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1323`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns whether newlines in values are supported.

This method always returns `false`. The actual newlines_in_values setting
has been moved to [`CsvSource`] and should be accessed via
[`CsvSource::csv_options()`] instead.

[`CsvSource`]: https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.CsvSource.html
[`CsvSource::csv_options()`]: https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.CsvSource.html#method.csv_options

<a id="op-c454162bee53ac41a79cc197"></a>
## object_store_url

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::object_store_url` · datafusion-datasource 55.1.0

```rust
object_store_url: datafusion_execution::object_store::ObjectStoreUrl
```

Source: `src/file_scan_config/mod.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Object store URL, used to get an [`ObjectStore`] instance from
[`RuntimeEnv::object_store`]

This `ObjectStoreUrl` should be the prefix of the absolute url for files
as `file://` or `s3://my_bucket`. It should not include the path to the
file itself. The relevant URL prefix must be registered via
[`RuntimeEnv::register_object_store`]

[`ObjectStore`]: object_store::ObjectStore
[`RuntimeEnv::register_object_store`]: datafusion_execution::runtime_env::RuntimeEnv::register_object_store
[`RuntimeEnv::object_store`]: datafusion_execution::runtime_env::RuntimeEnv::object_store

Unresolved upstream links (retained, not inferred): `datafusion_execution::runtime_env::RuntimeEnv::object_store`, `datafusion_execution::runtime_env::RuntimeEnv::register_object_store`.

<a id="op-599541aae6b753edc61951b0"></a>
## open

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::open` · datafusion-datasource 55.1.0

```rust
fn open(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27cdb28b94ee585e907c04d9"></a>
## open_with_args

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::open_with_args` · datafusion-datasource 55.1.0

```rust
fn open_with_args(&self, args: OpenArgs) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:700`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3de65a1c2a07346bbe714c4c"></a>
## output_ordering

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::output_ordering` · datafusion-datasource 55.1.0

```rust
output_ordering: Vec<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

Source: `src/file_scan_config/mod.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

All equivalent lexicographical output orderings of this file scan, in terms of
[`FileSource::table_schema`](../operations/datafusion_datasource.file.FileSource.md#op-5d12d1eeea4656c1e1a3bf3e). See [`FileScanConfigBuilder::with_output_ordering`](../operations/datafusion_datasource.file_scan_config.FileScanConfigBuilder.md#op-6e5869c78d3360750fb863cd) for more
details.

[`Self::eq_properties`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-defb6e1302a18843e1451fc7) uses this information along with projection
and filtering information to compute the effective
[`EquivalenceProperties`](../operations/datafusion_physical_expr.equivalence.properties.EquivalenceProperties.md#op-eeda1c3d472a48e6007359b5)

<a id="op-5f11b7ae6424d6bd3137ff0f"></a>
## output_partitioning

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::output_partitioning` · datafusion-datasource 55.1.0

```rust
output_partitioning: Option<datafusion_physical_expr::Partitioning>
```

Source: `src/file_scan_config/mod.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Declared physical output partitioning for this scan.

Expressions are against the full table schema, before scan projection or
filtering. `ListingTable` validates partition count before building the
scan, and direct builders with mismatched counts fall back to
`UnknownPartitioning`.

<a id="op-a8e4f31c3eea80bb4f0f6724"></a>
## output_partitioning

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::output_partitioning` · datafusion-datasource 55.1.0

```rust
fn output_partitioning(&self) -> Partitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:840`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the output partitioning for this file scan.

When `output_partitioning` is set, this returns the declared partitioning
after applying scan projection, allowing the optimizer to skip hash
repartitioning for aggregates and joins on the partitioning columns.

If projection or partition count validation fails, this returns
`UnknownPartitioning`.

Tradeoffs
- Benefit: Eliminates `RepartitionExec` and `SortExec` for queries whose
  required distribution is satisfied by the scan's output partitioning.
- Cost: Files are grouped by partition values rather than split by byte
  ranges, which may reduce I/O parallelism when partition sizes are uneven.
  For simple aggregations without `ORDER BY`, this cost may outweigh the benefit.

Follow-up Work
- Idea: Could allow byte-range splitting within partition-aware groups,
  preserving I/O parallelism while maintaining partition semantics.

<a id="op-4b00fae6be9647b2ce7cc821"></a>
## parse_table_schema_from_proto

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::parse_table_schema_from_proto` · datafusion-datasource 55.1.0

```rust
fn parse_table_schema_from_proto(conf: &protobuf::FileScanExecConf) -> Result<TableSchema>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "crate::file_scan_config::FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [271, 2], "filename": "src/file_scan_config/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/proto.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Parse a [`TableSchema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-edcf381c15c6591908c8608e) (file schema + partition columns) from a
[`protobuf::FileScanExecConf`](../operations/datafusion_proto_models.generated.datafusion.FileScanExecConf.md#op-04f424c5f288cef6460ccd58). File sources use this to rebuild their
concrete source before calling [`FileScanConfig::try_from_proto`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-4aa3497c101fb07b3e711bac).

Byte-compatible with the former `parse_table_schema_from_proto`.

<a id="op-25013b575cd5ba79e7c090e4"></a>
## partition_statistics

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::partition_statistics` · datafusion-datasource 55.1.0

```rust
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:922`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28db7466f4e96cc4c9b4f959"></a>
## preserve_order

`struct_field` · `datafusion_datasource::file_scan_config::FileScanConfig::preserve_order` · datafusion-datasource 55.1.0

```rust
preserve_order: bool
```

Source: `src/file_scan_config/mod.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Whether the scan's limit is order sensitive
When `true`, files must be read in the exact order specified to produce
correct results (e.g., for `ORDER BY ... LIMIT` queries). When `false`,
DataFusion may reorder file processing for optimization without affecting correctness.

<a id="op-9324832053bd873f2d8b6f91"></a>
## projected_constraints

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::projected_constraints` · datafusion-datasource 55.1.0

```rust
fn projected_constraints(&self) -> Constraints
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1331`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e807e3c57296f2f496c04e69"></a>
## projected_schema

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::projected_schema` · datafusion-datasource 55.1.0

```rust
fn projected_schema(&self) -> Result<Arc<Schema>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1277`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fba43402d1f18e22690d5b9"></a>
## repartitioned

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::repartitioned` · datafusion-datasource 55.1.0

```rust
fn repartitioned(&self, target_partitions: usize, repartition_file_min_size: usize, output_ordering: Option<LexOrdering>) -> Result<Option<Arc<dyn DataSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:799`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

If supported by the underlying [`FileSource`](../operations/datafusion_datasource.file.FileSource.md#op-8c1b19f80ea0c73466216a63), redistribute files across partitions according to their size.

<a id="op-cb8ac006eb9187b3851499c2"></a>
## scheduling_type

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::scheduling_type` · datafusion-datasource 55.1.0

```rust
fn scheduling_type(&self) -> SchedulingType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:918`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4367ef6d14808fb3612d9b47"></a>
## split_groups_by_statistics

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::split_groups_by_statistics` · datafusion-datasource 55.1.0

```rust
fn split_groups_by_statistics(table_schema: &SchemaRef, file_groups: &[FileGroup], sort_order: &LexOrdering) -> Result<Vec<FileGroup>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1443`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Attempts to do a bin-packing on files into file groups, such that any two files
in a file group are ordered and non-overlapping with respect to their statistics.
It will produce the smallest number of file groups possible.

<a id="op-7dd77f54317d7ddab6107a5e"></a>
## split_groups_by_statistics_with_target_partitions

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::split_groups_by_statistics_with_target_partitions` · datafusion-datasource 55.1.0

```rust
fn split_groups_by_statistics_with_target_partitions(table_schema: &SchemaRef, file_groups: &[FileGroup], sort_order: &LexOrdering, target_partitions: usize) -> Result<Vec<FileGroup>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1371`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Splits file groups into new groups based on statistics to enable efficient parallel processing.

The method distributes files across a target number of partitions while ensuring
files within each partition maintain sort order based on their min/max statistics.

The algorithm works by:
1. Takes files sorted by minimum values
2. For each file:
  - Finds eligible groups (empty or where file's min > group's last max)
  - Selects the smallest eligible group
  - Creates a new group if needed

# Parameters
* `table_schema`: Schema containing information about the columns
* `file_groups`: The original file groups to split
* `sort_order`: The lexicographical ordering to maintain within each group
* `target_partitions`: The desired number of output partitions

# Returns
A new set of file groups, where files within each group are non-overlapping with respect to
their min/max statistics and maintain the specified sort order.

<a id="op-b928e7f935d14db6c2cc7e96"></a>
## statistics

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::statistics` · datafusion-datasource 55.1.0

```rust
fn statistics(&self) -> Statistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1267`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the unprojected table statistics, marking them as inexact if filters are present.

When filters are pushed down (including pruning predicates and bloom filters),
we can't guarantee the statistics are exact because we don't know how many
rows will be filtered out.

<a id="op-9d2841c2a126490fee866abb"></a>
## table_partition_cols

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::table_partition_cols` · datafusion-datasource 55.1.0

```rust
fn table_partition_cols(&self) -> &Fields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1522, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:1258`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get the table partition columns

<a id="op-4aa3497c101fb07b3e711bac"></a>
## try_from_proto

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::try_from_proto` · datafusion-datasource 55.1.0

```rust
fn try_from_proto(conf: &protobuf::FileScanExecConf, ctx: &ExecutionPlanDecodeCtx<'_>, file_source: Arc<dyn FileSource>) -> Result<FileScanConfig>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "crate::file_scan_config::FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [271, 2], "filename": "src/file_scan_config/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/proto.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Reconstruct a [`FileScanConfig`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-d7aa66cd63851cc944f55f57) from a [`protobuf::FileScanExecConf`](../operations/datafusion_proto_models.generated.datafusion.FileScanExecConf.md#op-04f424c5f288cef6460ccd58)
and a `file_source` the caller has already rebuilt (typically from the
table schema via [`FileScanConfig::parse_table_schema_from_proto`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-4b00fae6be9647b2ce7cc821)).

Byte-compatible with the former `parse_protobuf_file_scan_config`.

<a id="op-ced58b271fa2251824007ca5"></a>
## try_pushdown_filters

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::try_pushdown_filters` · datafusion-datasource 55.1.0

```rust
fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn DataSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:998`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3fb033cbe6de3ed56df2831"></a>
## try_pushdown_sort

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::try_pushdown_sort` · datafusion-datasource 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn DataSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:1088`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Push sort requirements into file-based data sources.

# Sort Pushdown Architecture

When a partition (file group) contains multiple files in wrong order,
`validated_output_ordering()` strips the ordering and `EnforceSorting`
inserts a `SortExec`. This optimizer fixes the file order by sorting
files within each group by min/max statistics, enabling sort elimination.

This applies to both single-partition and multi-partition plans — any
file group with multiple files in wrong order benefits.

```text
PushdownSort optimizer finds SortExec
  │
  ▼
FileScanConfig::try_pushdown_sort()
  │
  ├─► FileSource returns Exact
  │     (natural ordering satisfies request)
  │     → rebuild_with_source: sort files by stats, verify non-overlapping
  │     → SortExec removed, fetch (LIMIT) pushed to DataSourceExec
  │
  ├─► FileSource returns Inexact
  │     (e.g. column_in_file_schema: opener will reorder RGs at runtime)
  │     → rebuild_with_source: sort files by stats; if the post-sort
  │       file groups are non-overlapping AND the request now validates
  │       AND no NULLs sit in the sort columns of non-last files,
  │       upgrade back to Exact (SortExec removed). Otherwise stays
  │       Inexact and SortExec is kept while the scan is still
  │       optimised via `sort_order_for_reorder` / `reverse_row_groups`.
  │
  └─► FileSource returns Unsupported
        (e.g. expression sort key or partition column)
        → try_sort_file_groups_by_statistics():
          1. Sort files within each group by min/max statistics
          2. Re-check: non-overlapping + ordering valid + no NULLs?
             YES → Exact → SortExec removed
             NO  → Inexact (files reordered, Sort stays)
```

<a id="op-e58ed4403485b646184d4c3d"></a>
## try_swapping_with_projection

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::try_swapping_with_projection` · datafusion-datasource 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn DataSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:973`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-711c3b3405b35ca8e430df8a"></a>
## try_to_proto

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::try_to_proto` · datafusion-datasource 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:1204`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Serialize this file scan by delegating to the concrete
[`FileSource`](../operations/datafusion_datasource.file.FileSource.md#op-8c1b19f80ea0c73466216a63)'s
[`try_to_proto`](crate::file::FileSource::try_to_proto) hook, passing
`self` as the shared spine it needs to emit the base config.

<a id="op-947d8c6473bca5401d05e193"></a>
## try_to_proto

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::try_to_proto` · datafusion-datasource 55.1.0

```rust
fn try_to_proto(&self, ctx: &ExecutionPlanEncodeCtx<'_>) -> Result<protobuf::FileScanExecConf>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "crate::file_scan_config::FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [271, 2], "filename": "src/file_scan_config/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/proto.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Serialize the shared, format-agnostic part of a file scan into a
[`protobuf::FileScanExecConf`](../operations/datafusion_proto_models.generated.datafusion.FileScanExecConf.md#op-04f424c5f288cef6460ccd58).

Each concrete [`FileSource::try_to_proto`](../operations/datafusion_datasource.file.FileSource.md#op-83152ed75bb6a4e441648c13)
wraps the returned value in its own `*ScanExecNode`. Byte-compatible with
the former `serialize_file_scan_config` in `datafusion-proto`.

<a id="op-2891d75abab4faf3b5fed926"></a>
## with_fetch

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::with_fetch` · datafusion-datasource 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn DataSource>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:958`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a199e1095e5842705935e7d5"></a>
## with_preserve_order

`function` · `datafusion_datasource::file_scan_config::FileScanConfig::with_preserve_order` · datafusion-datasource 55.1.0

```rust
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn DataSource>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [691, 1], "end": [1210, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/file_scan_config/mod.rs:1157`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
