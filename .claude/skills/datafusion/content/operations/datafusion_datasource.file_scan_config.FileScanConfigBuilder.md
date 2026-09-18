# `datafusion_datasource::file_scan_config::FileScanConfigBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_scan_config.FileScanConfigBuilder.json).

<a id="op-fc85b3f3f7ee6a9b542e85c0"></a>
## FileScanConfigBuilder

`struct` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder` · datafusion-datasource 55.1.0

```rust
struct FileScanConfigBuilder
```

Source: `src/file_scan_config/mod.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A builder for [`FileScanConfig`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-d7aa66cd63851cc944f55f57)'s.

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

<a id="op-33624ed9f35eb7e42233a463"></a>
## build

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::build` · datafusion-datasource 55.1.0

```rust
fn build(self) -> FileScanConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:528`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Build the final [`FileScanConfig`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-d7aa66cd63851cc944f55f57) with all the configured settings.

This method takes ownership of the builder and returns the constructed `FileScanConfig`.
Any unset optional fields will use their default values.

# Errors
Returns an error if projection pushdown fails or if schema operations fail.

<a id="op-5a903bdc185ce71282f9c7d7"></a>
## clone

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> FileScanConfigBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 10], "end": [284, 15], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_scan_config/mod.rs:284`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-029952799db60ab5300ff836"></a>
## from

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::from` · datafusion-datasource 55.1.0

```rust
fn from(config: FileScanConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [588, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfig", "path": "FileScanConfig"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file_scan_config/mod.rs:572`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71a160eb98ea33e5450afbcd"></a>
## new

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::new` · datafusion-datasource 55.1.0

```rust
fn new(object_store_url: ObjectStoreUrl, file_source: Arc<dyn FileSource>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new [`FileScanConfigBuilder`](../operations/datafusion_datasource.file_scan_config.FileScanConfigBuilder.md#op-fc85b3f3f7ee6a9b542e85c0) with default settings for scanning files.

# Parameters:
* `object_store_url`: See [`FileScanConfig::object_store_url`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-c454162bee53ac41a79cc197)
* `file_source`: See [`FileScanConfig::file_source`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-47b12dd2c48dbed38d686054). The file source must have
  a schema set via its constructor.

<a id="op-37c90871e705fd7851a2ed56"></a>
## table_schema

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::table_schema` · datafusion-datasource 55.1.0

```rust
fn table_schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the table schema

<a id="op-5be9ffa0e7f21e227626a37a"></a>
## with_batch_size

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_batch_size` · datafusion-datasource 55.1.0

```rust
fn with_batch_size(self, batch_size: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:502`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the batch_size property

<a id="op-6dc84fcd2ccde2147730496e"></a>
## with_constraints

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_constraints` · datafusion-datasource 55.1.0

```rust
fn with_constraints(self, constraints: Constraints) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the table constraints

<a id="op-c9241a0df6624cdca7d0d56d"></a>
## with_expr_adapter

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_expr_adapter` · datafusion-datasource 55.1.0

```rust
fn with_expr_adapter(self, expr_adapter: Option<Arc<dyn PhysicalExprAdapterFactory>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Register an expression adapter used to adapt filters and projections that are pushed down into the scan
from the logical schema to the physical schema of the file.
This can include things like:
- Column ordering changes
- Handling of missing columns
- Rewriting expression to use pre-computed values or file format specific optimizations

<a id="op-1b3cc8f739606c79a65a935d"></a>
## with_file

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_file` · datafusion-datasource 55.1.0

```rust
fn with_file(self, partitioned_file: PartitionedFile) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Add a file as a single group

See [`Self::with_file_groups`](../operations/datafusion_datasource.file_scan_config.FileScanConfigBuilder.md#op-00a5e852c71227a508011ed6) for more information.

<a id="op-53161ea8bed35199238f28bb"></a>
## with_file_compression_type

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_file_compression_type` · datafusion-datasource 55.1.0

```rust
fn with_file_compression_type(self, file_compression_type: FileCompressionType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the file compression type

<a id="op-287f90a1a42542a4f4153194"></a>
## with_file_group

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_file_group` · datafusion-datasource 55.1.0

```rust
fn with_file_group(self, file_group: FileGroup) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Add a new file group

See [`Self::with_file_groups`](../operations/datafusion_datasource.file_scan_config.FileScanConfigBuilder.md#op-00a5e852c71227a508011ed6) for more information

<a id="op-00a5e852c71227a508011ed6"></a>
## with_file_groups

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_file_groups` · datafusion-datasource 55.1.0

```rust
fn with_file_groups(self, file_groups: Vec<FileGroup>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:450`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the list of files to be processed, grouped into partitions.

Each file must have a schema of `file_schema` or a subset. If
a particular file has a subset, the missing columns are
padded with NULLs.

DataFusion may attempt to read each partition of files
concurrently, however files *within* a partition will be read
sequentially, one after the next.

<a id="op-19c179f4eccf022adc287547"></a>
## with_limit

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_limit` · datafusion-datasource 55.1.0

```rust
fn with_limit(self, limit: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the maximum number of records to read from this plan.

If `None`, all records after filtering are returned.

<a id="op-6e5869c78d3360750fb863cd"></a>
## with_output_ordering

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_output_ordering` · datafusion-datasource 55.1.0

```rust
fn with_output_ordering(self, output_ordering: Vec<LexOrdering>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the output ordering of the files

The expressions are in terms of the entire table schema (file schema +
partition columns), before any projection or filtering from the file
scan is applied.

This is used for optimization purposes, e.g. to determine if a file scan
can satisfy an `ORDER BY` without an additional sort.

<a id="op-4c06cb89ce34211968a14ac2"></a>
## with_output_partitioning

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_output_partitioning` · datafusion-datasource 55.1.0

```rust
fn with_output_partitioning(self, output_partitioning: Option<Partitioning>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set declared physical output partitioning for this scan.

<a id="op-03a92cdf53c6c443157aa757"></a>
## with_preserve_order

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_preserve_order` · datafusion-datasource 55.1.0

```rust
fn with_preserve_order(self, order_sensitive: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set whether the limit should be order-sensitive.

When `true`, files must be read in the exact order specified to produce
correct results (e.g., for `ORDER BY ... LIMIT` queries). When `false`,
DataFusion may reorder file processing for optimization without
affecting correctness.

<a id="op-ab6d2aba7ef781c6be1ef5c4"></a>
## with_projection

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_projection` · datafusion-datasource 55.1.0

```rust
fn with_projection(self, indices: Option<Vec<usize>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the columns on which to project the data. Indexes that are higher than the
number of columns of `file_schema` refer to `table_partition_cols`.

# Deprecated
Use [`Self::with_projection_indices`](../operations/datafusion_datasource.file_scan_config.FileScanConfigBuilder.md#op-82b5fb24e3cd88cbf18dccd3) instead. This method will be removed in a future release.

<a id="op-82b5fb24e3cd88cbf18dccd3"></a>
## with_projection_indices

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_projection_indices` · datafusion-datasource 55.1.0

```rust
fn with_projection_indices(self, indices: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the columns on which to project the data using column indices.

This method attempts to push down the projection to the underlying file
source if supported. If the file source does not support projection
pushdown, an error is returned.

Indexes that are higher than the number of columns of `file_schema`
refer to `table_partition_cols`.

<a id="op-add500a5d3c3c56ec799cd4d"></a>
## with_source

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_source` · datafusion-datasource 55.1.0

```rust
fn with_source(self, file_source: Arc<dyn FileSource>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the file source for scanning files.

This method allows you to change the file source implementation (e.g.
ParquetSource, CsvSource, etc.) after the builder has been created.

<a id="op-0543356469cff3f115cd8043"></a>
## with_statistics

`function` · `datafusion_datasource::file_scan_config::FileScanConfigBuilder::with_statistics` · datafusion-datasource 55.1.0

```rust
fn with_statistics(self, statistics: Statistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_scan_config::FileScanConfigBuilder", "path": "FileScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [569, 2], "filename": "src/file_scan_config/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_scan_config/mod.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the statistics of the files, including partition
columns. Defaults to [`Statistics::new_unknown`].

These statistics are for the entire table (file schema + partition
columns) before any projection or filtering is applied. Projections are
applied when statistics are retrieved, and if a filter is present,
[`FileScanConfig::statistics`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-b928e7f935d14db6c2cc7e96) will mark the statistics as inexact
(counts are not adjusted).

Projections and filters may be applied by the file source, either by
[`Self::with_projection_indices`](../operations/datafusion_datasource.file_scan_config.FileScanConfigBuilder.md#op-82b5fb24e3cd88cbf18dccd3) or a preexisting
[`FileSource::projection`](../operations/datafusion_datasource.file.FileSource.md#op-0222a3b6073f25c90ced3f20) or [`FileSource::filter`](../operations/datafusion_datasource.file.FileSource.md#op-09dcb4160e6584441ae99415).

Unresolved upstream links (retained, not inferred): ``Statistics::new_unknown``.
