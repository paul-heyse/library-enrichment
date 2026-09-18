# `datafusion_datasource_parquet::source::ParquetSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.source.ParquetSource.json).

<a id="op-cb3b0e2705ba6dce197aab0f"></a>
## ParquetSource

`struct` · `datafusion_datasource_parquet::source::ParquetSource` · datafusion-datasource-parquet 55.1.0

```rust
struct ParquetSource
```

Source: `src/source.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

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
  coalesce I/O operations, etc. See [`ParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.ParquetFileReaderFactory.md#op-864a9023955965d6e936e916) for more
  details.

* Schema evolution: read parquet files with different schemas into a unified
  table schema. See [`DefaultPhysicalExprAdapterFactory`](../operations/datafusion_physical_expr_adapter.schema_rewriter.DefaultPhysicalExprAdapterFactory.md#op-a4e5de845da9e33105dd28ae) for more details.

* metadata_size_hint: controls the number of bytes read from the end of the
  file in the initial I/O when the default [`ParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.ParquetFileReaderFactory.md#op-864a9023955965d6e936e916). If a
  custom reader is used, it supplies the metadata directly and this parameter
  is ignored. [`ParquetSource::with_metadata_size_hint`](../operations/datafusion_datasource_parquet.source.ParquetSource.md#op-99bf9fba04a1db8ef6551cac) for more details.

* User provided  `ParquetAccessPlan`s to skip row groups and/or pages
  based on external information. See "Implementing External Indexes" below

# Predicate Pushdown

`DataSourceExec` uses the provided [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) predicate as a filter to
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

You can modify a `DataSourceExec` using [`ParquetSource`](../operations/datafusion_datasource_parquet.source.ParquetSource.md#op-cb3b0e2705ba6dce197aab0f), for example
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
  via [`ParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.ParquetFileReaderFactory.md#op-864a9023955965d6e936e916), creating a `ParquetAccessPlan` by
  applying predicates to metadata. The plan and projections are used to
  determine what pages must be read.

* Step 4: The stream begins reading data, fetching the required parquet
  pages incrementally decoding them, and applying any row filters (see
  [`Self::with_pushdown_filters`](../operations/datafusion_datasource_parquet.source.ParquetSource.md#op-a9698d2934b7a9fab8ce4634)).

* Step 5: As each [`RecordBatch`] is read, it may be adapted by a
  [`DefaultPhysicalExprAdapterFactory`](../operations/datafusion_physical_expr_adapter.schema_rewriter.DefaultPhysicalExprAdapterFactory.md#op-a4e5de845da9e33105dd28ae) to match the table schema. By default missing columns are
  filled with nulls, but this can be customized via [`PhysicalExprAdapterFactory`].

[`RecordBatch`]: arrow::record_batch::RecordBatch
[`ParquetMetadata`]: parquet::file::metadata::ParquetMetaData
[`PhysicalExprAdapterFactory`]: datafusion_physical_expr_adapter::PhysicalExprAdapterFactory

<a id="op-89517c10b3c2ba11f90326f4"></a>
## apply_expressions

`function` · `datafusion_datasource_parquet::source::ParquetSource::apply_expressions` · datafusion-datasource-parquet 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> datafusion_common::Result<TreeNodeRecursion>) -> datafusion_common::Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:1062`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d04e3de9f821913a95a8b575"></a>
## clone

`function` · `datafusion_datasource_parquet::source::ParquetSource::clone` · datafusion-datasource-parquet 55.1.0

```rust
fn clone(&self) -> ParquetSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 10], "end": [288, 15], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/source.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-525a6cf8aeced9fe99cb06b7"></a>
## create_file_opener

`function` · `datafusion_datasource_parquet::source::ParquetSource::create_file_opener` · datafusion-datasource-parquet 55.1.0

```rust
fn create_file_opener(&self, _object_store: Arc<dyn ObjectStore>, _base_config: &FileScanConfig, _partition: usize) -> datafusion_common::Result<Arc<dyn FileOpener>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:560`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-480350bbe6b8607ec14b908f"></a>
## create_morselizer

`function` · `datafusion_datasource_parquet::source::ParquetSource::create_morselizer` · datafusion-datasource-parquet 55.1.0

```rust
fn create_morselizer(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition: usize) -> datafusion_common::Result<Box<dyn Morselizer>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:571`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-453790fe0fb9c876fe8dcf27"></a>
## file_type

`function` · `datafusion_datasource_parquet::source::ParquetSource::file_type` · datafusion-datasource-parquet 55.1.0

```rust
fn file_type(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:730`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b3ede38c380d03bf3651007"></a>
## filter

`function` · `datafusion_datasource_parquet::source::ParquetSource::filter` · datafusion-datasource-parquet 55.1.0

```rust
fn filter(&self) -> Option<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:682`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d58eaf54ab81ec8ad6d887d1"></a>
## fmt

`function` · `datafusion_datasource_parquet::source::ParquetSource::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 17], "end": [288, 22], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/source.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01cfb6219bb71495ecc1cd76"></a>
## fmt_extra

`function` · `datafusion_datasource_parquet::source::ParquetSource::fmt_extra` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt_extra(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:734`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ec38b1cd6dad203e894e693"></a>
## max_in_list_size

`function` · `datafusion_datasource_parquet::source::ParquetSource::max_in_list_size` · datafusion-datasource-parquet 55.1.0

```rust
fn max_in_list_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return the maximum size of an `IN (...)` list that the pruning
predicate will rewrite into per-value statistics checks. Lists
longer than this skip container-level pruning. Reads from
`datafusion.execution.parquet.max_in_list_size`.

<a id="op-e8ffc8a8bf22407dde56bed2"></a>
## max_predicate_cache_size

`function` · `datafusion_datasource_parquet::source::ParquetSource::max_predicate_cache_size` · datafusion-datasource-parquet 55.1.0

```rust
fn max_predicate_cache_size(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return the maximum predicate cache size, in bytes, used when
`pushdown_filters`

<a id="op-3069ecbd35ca62ced6fdd035"></a>
## metrics

`function` · `datafusion_datasource_parquet::source::ParquetSource::metrics` · datafusion-datasource-parquet 55.1.0

```rust
fn metrics(&self) -> &ExecutionPlanMetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:726`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c3953a4190084ac9fd3142f"></a>
## new

`function` · `datafusion_datasource_parquet::source::ParquetSource::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(table_schema: impl Into<TableSchema>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new ParquetSource to read the data specified in the file scan
configuration with the provided schema.

Uses default `TableParquetOptions`.
To set custom options, use [ParquetSource::with_table_parquet_options`](../operations/datafusion_datasource_parquet.source.ParquetSource.md#op-3c481c9badaace7f7d559c06).

<a id="op-c0ac1acd5b92c74d98d396a5"></a>
## parquet_file_reader_factory

`function` · `datafusion_datasource_parquet::source::ParquetSource::parquet_file_reader_factory` · datafusion-datasource-parquet 55.1.0

```rust
fn parquet_file_reader_factory(&self) -> Option<&Arc<dyn ParquetFileReaderFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

return the optional file reader factory

<a id="op-9c58a97a2aa639115e033812"></a>
## predicate

`function` · `datafusion_datasource_parquet::source::ParquetSource::predicate` · datafusion-datasource-parquet 55.1.0

```rust
fn predicate(&self) -> Option<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Optional predicate.

<a id="op-ad2e2d8cd432866b7faa6c8d"></a>
## projection

`function` · `datafusion_datasource_parquet::source::ParquetSource::projection` · datafusion-datasource-parquet 55.1.0

```rust
fn projection(&self) -> Option<&ProjectionExprs>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:722`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-badbae01ed6ece8fa8971302"></a>
## reorder_files

`function` · `datafusion_datasource_parquet::source::ParquetSource::reorder_files` · datafusion-datasource-parquet 55.1.0

```rust
fn reorder_files(&self, files: Vec<datafusion_datasource::PartitionedFile>) -> Vec<datafusion_datasource::PartitionedFile>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87ce35ee187aa1a1c5d577d1"></a>
## table_parquet_options

`function` · `datafusion_datasource_parquet::source::ParquetSource::table_parquet_options` · datafusion-datasource-parquet 55.1.0

```rust
fn table_parquet_options(&self) -> &TableParquetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Options passed to the parquet reader for this scan

<a id="op-172231f345694497f07e1028"></a>
## table_schema

`function` · `datafusion_datasource_parquet::source::ParquetSource::table_schema` · datafusion-datasource-parquet 55.1.0

```rust
fn table_schema(&self) -> &TableSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:678`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41820a05b5c25517e01d68b1"></a>
## try_from_proto

`function` · `datafusion_datasource_parquet::source::ParquetSource::try_from_proto` · datafusion-datasource-parquet 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> datafusion_common::Result<Arc<dyn datafusion_physical_plan::ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1204, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:1110`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Reconstructs a `DataSourceExec` from a protobuf `ParquetScan`.

Rebuilds the reader factory from the decode context because it is not serialized.

<a id="op-3f8da94fc6af96f1c1d2d784"></a>
## try_pushdown_filters

`function` · `datafusion_datasource_parquet::source::ParquetSource::try_pushdown_filters` · datafusion-datasource-parquet 55.1.0

```rust
fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> datafusion_common::Result<FilterPushdownPropagation<Arc<dyn FileSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:821`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa3ba6b288106ad4b24531da"></a>
## try_pushdown_projection

`function` · `datafusion_datasource_parquet::source::ParquetSource::try_pushdown_projection` · datafusion-datasource-parquet 55.1.0

```rust
fn try_pushdown_projection(&self, projection: &ProjectionExprs) -> datafusion_common::Result<Option<Arc<dyn FileSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-013f1614b82377abbd796366"></a>
## try_pushdown_sort

`function` · `datafusion_datasource_parquet::source::ParquetSource::try_pushdown_sort` · datafusion-datasource-parquet 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr], eq_properties: &EquivalenceProperties) -> datafusion_common::Result<SortOrderPushdownResult<Arc<dyn FileSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:931`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Try to optimize the scan to produce data in the requested sort order.

Inputs:
1. The query's required ordering (`order` parameter)
2. The source's equivalence properties (`eq_properties`)

# Returns
- `Exact`: the source's natural ordering already satisfies the
  request. The surrounding `SortExec` can be eliminated provided
  files within each group are non-overlapping (verified by
  `FileScanConfig`).
- `Inexact`: the source can approximate the request via two
  composable runtime steps — stats-based row-group reorder
  (skipped when the leading sort key isn't a plain `Column`
  in the file schema) and row-group iteration reverse. A
  `SortExec` is still required for full correctness, but limit
  pushdown and `TopK` benefit immediately.
- `Unsupported`: no approximation is available.

# How the Inexact result is communicated

The result is carried through two fields on `ParquetSource`:

- `sort_order_for_reorder`: set to the request's `LexOrdering`
  whenever the pushdown fires, regardless of whether the
  leading expression is a plain `Column`. The opener invokes
  `PreparedAccessPlan::reorder_by_statistics`, which skips
  when the expression can't be looked up in parquet metadata.
  Exposing the field unconditionally keeps `EXPLAIN` honest
  about what the source was asked to approximate.
- `reverse_row_groups`: drives the opener's iteration flip.
  When stats reorder applies (column-in-schema), this is just
  the request's direction — the reorder produces ASC-by-min,
  so reverse iff the query asks for DESC. When stats reorder
  doesn't apply but the reversed source ordering satisfies
  the request (function-wrapped case), this is always `true`
  because we're flipping the file's natural order.

<a id="op-7c1f6d2e55a5b37643e60540"></a>
## try_to_proto

`function` · `datafusion_datasource_parquet::source::ParquetSource::try_to_proto` · datafusion-datasource-parquet 55.1.0

```rust
fn try_to_proto(&self, base: &FileScanConfig, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> datafusion_common::Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:1079`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Emit a `ParquetScan` node wrapping the shared base config plus the
Parquet-specific pushdown predicate and `TableParquetOptions`.

<a id="op-5213ad0e9462e45217fd8901"></a>
## with_batch_size

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_batch_size` · datafusion-datasource-parquet 55.1.0

```rust
fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [559, 1], "end": [1103, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file::FileSource", "path": "FileSource"}, "trait_path": "datafusion_datasource::file::FileSource"}`

Source: `src/source.rs:686`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d49248ce90b26b376b64b131"></a>
## with_bloom_filter_on_read

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_bloom_filter_on_read` · datafusion-datasource-parquet 55.1.0

```rust
fn with_bloom_filter_on_read(self, bloom_filter_on_read: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:463`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

If enabled, the reader will read by the bloom filter

<a id="op-5ebe781b823267ad7b1ca142"></a>
## with_bloom_filter_on_write

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_bloom_filter_on_write` · datafusion-datasource-parquet 55.1.0

```rust
fn with_bloom_filter_on_write(self, enable_bloom_filter_on_write: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:469`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

If enabled, the writer will write by the bloom filter

<a id="op-0b055491d9033f18b6b26b08"></a>
## with_enable_page_index

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_enable_page_index` · datafusion-datasource-parquet 55.1.0

```rust
fn with_enable_page_index(self, enable_page_index: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

If enabled, the reader will read the page index
This is used to optimize filter pushdown
via `RowSelector` and `RowFilter` by
eliminating unnecessary IO and decoding

<a id="op-d56fee10df8ba471d02db9b9"></a>
## with_encryption_factory

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_encryption_factory` · datafusion-datasource-parquet 55.1.0

```rust
fn with_encryption_factory(self, encryption_factory: Arc<dyn EncryptionFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the encryption factory to use to generate file decryption properties

<a id="op-99bf9fba04a1db8ef6551cac"></a>
## with_metadata_size_hint

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_metadata_size_hint` · datafusion-datasource-parquet 55.1.0

```rust
fn with_metadata_size_hint(self, metadata_size_hint: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the metadata size hint

This value determines how many bytes at the end of the file the default
[`ParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.ParquetFileReaderFactory.md#op-864a9023955965d6e936e916) will request in the initial IO. If this is
too small, the ParquetSource will need to make additional IO requests to
read the footer.

<a id="op-12d9c88f8bd2e26e89737647"></a>
## with_parquet_file_reader_factory

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_parquet_file_reader_factory` · datafusion-datasource-parquet 55.1.0

```rust
fn with_parquet_file_reader_factory(self, parquet_file_reader_factory: Arc<dyn ParquetFileReaderFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Optional user defined parquet file reader factory.

<a id="op-e9960cbad86e133ce01e19f9"></a>
## with_predicate

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_predicate` · datafusion-datasource-parquet 55.1.0

```rust
fn with_predicate(&self, predicate: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set predicate information.

Predicates referencing virtual columns must go through
[`Self::try_pushdown_filters`](../operations/datafusion_datasource_parquet.source.ParquetSource.md#op-3f8da94fc6af96f1c1d2d784). Passing them here with pushdown
enabled trips a debug assert in the opener.

<a id="op-a9698d2934b7a9fab8ce4634"></a>
## with_pushdown_filters

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_pushdown_filters` · datafusion-datasource-parquet 55.1.0

```rust
fn with_pushdown_filters(self, pushdown_filters: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

If true, the predicate will be used during the parquet scan.
Defaults to false.

<a id="op-afd3f69bf3abcb495408f12d"></a>
## with_reorder_filters

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_reorder_filters` · datafusion-datasource-parquet 55.1.0

```rust
fn with_reorder_filters(self, reorder_filters: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

If true, the `RowFilter` made by `pushdown_filters` may try to
minimize the cost of filter evaluation by reordering the
predicate [`Expr`]s. If false, the predicates are applied in
the same order as specified in the query. Defaults to false.

[`Expr`]: datafusion_expr::Expr

<a id="op-3c481c9badaace7f7d559c06"></a>
## with_table_parquet_options

`function` · `datafusion_datasource_parquet::source::ParquetSource::with_table_parquet_options` · datafusion-datasource-parquet 55.1.0

```rust
fn with_table_parquet_options(self, table_parquet_options: TableParquetOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::source::ParquetSource", "path": "ParquetSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [519, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the `TableParquetOptions` for this ParquetSource.
