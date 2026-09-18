# `datafusion_catalog_listing::options::ListingOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.options.ListingOptions.json).

<a id="op-0221fce132b7e0b8e5f08de7"></a>
## ListingOptions

`struct` · `datafusion_catalog_listing::options::ListingOptions` · datafusion-catalog-listing 55.1.0

```rust
struct ListingOptions
```

Source: `src/options.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Options for creating a [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

<a id="op-e00c4debf3b84bda156202ab"></a>
## clone

`function` · `datafusion_catalog_listing::options::ListingOptions::clone` · datafusion-catalog-listing 55.1.0

```rust
fn clone(&self) -> ListingOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/options.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4308c0700455c528ef66df0f"></a>
## file_extension

`struct_field` · `datafusion_catalog_listing::options::ListingOptions::file_extension` · datafusion-catalog-listing 55.1.0

```rust
file_extension: String
```

Source: `src/options.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

A suffix on which files should be filtered (leave empty to
keep all files on the path)

<a id="op-aeed47965969ef6c92614a05"></a>
## file_sort_order

`struct_field` · `datafusion_catalog_listing::options::ListingOptions::file_sort_order` · datafusion-catalog-listing 55.1.0

```rust
file_sort_order: Vec<Vec<datafusion_expr::SortExpr>>
```

Source: `src/options.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Optional pre-known sort order(s). Must be `SortExpr`s.

DataFusion may take advantage of this ordering to omit sorts
or use more efficient algorithms. Currently sortedness must be
provided if it is known by some external mechanism, but may in
the future be automatically determined, for example using
parquet metadata.

See <https://github.com/apache/datafusion/issues/4177>

NOTE: This attribute stores all equivalent orderings (the outer `Vec`)
      where each ordering consists of an individual lexicographic
      ordering (encapsulated by a `Vec<Expr>`). If there aren't
      multiple equivalent orderings, the outer `Vec` will have a
      single element.

<a id="op-60c270ab31830129cdc2f009"></a>
## fmt

`function` · `datafusion_catalog_listing::options::ListingOptions::fmt` · datafusion-catalog-listing 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "src/options.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/options.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d475902a0ace7667ffa0308"></a>
## format

`struct_field` · `datafusion_catalog_listing::options::ListingOptions::format` · datafusion-catalog-listing 55.1.0

```rust
format: std::sync::Arc<dyn FileFormat>
```

Source: `src/options.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

The file format

<a id="op-f907e9a525e016c88657d136"></a>
## infer_partitions

`function` · `datafusion_catalog_listing::options::ListingOptions::infer_partitions` · datafusion-catalog-listing 55.1.0

```rust
async fn infer_partitions(&self, state: &dyn Session, table_path: &ListingTableUrl) -> datafusion_common::Result<Vec<String>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Infer the partitioning at the given path on the provided object store.
For performance reasons, it doesn't read all the files on disk
and therefore may fail to detect invalid partitioning.

<a id="op-54b025590158bc13f43edb10"></a>
## infer_schema

`function` · `datafusion_catalog_listing::options::ListingOptions::infer_schema` · datafusion-catalog-listing 55.1.0

```rust
async fn infer_schema<'a>(&'a self, state: &dyn Session, table_path: &'a ListingTableUrl) -> datafusion_common::Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Infer the schema of the files at the given path on the provided object store.

If the table_path contains one or more files (i.e. it is a directory /
prefix of files) their schema is merged by calling [`FileFormat::infer_schema`](../operations/datafusion_datasource.file_format.FileFormat.md#op-17e4cc8199e4c667431a735a).

Returns a `Plan` error if `table_path` contains no files at all (e.g. an
empty or non-existent directory), since an inferred schema with zero
columns produces confusing "column not found" errors at query time.
Callers that need to support empty locations must declare an explicit
schema instead of relying on inference. Locations that contain files
which all happen to be 0-byte are still accepted — the empty files are
filtered out before format-specific inference runs.

Note: The inferred schema does not include any partitioning columns.

This method is called as part of creating a [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

<a id="op-4d4119a0eaedac54d4274da9"></a>
## new

`function` · `datafusion_catalog_listing::options::ListingOptions::new` · datafusion-catalog-listing 55.1.0

```rust
fn new(format: Arc<dyn FileFormat>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Creates an options instance with the given format
Default values:
- use default file extension filter
- no input partition to discover

<a id="op-df20634d5465d3ff2c7a06da"></a>
## output_partitioning

`struct_field` · `datafusion_catalog_listing::options::ListingOptions::output_partitioning` · datafusion-catalog-listing 55.1.0

```rust
output_partitioning: Option<datafusion_expr::Partitioning>
```

Source: `src/options.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Declared output partitioning for scans from this table.

Expressions are logical expressions over the full table schema. When set,
[`ListingTable`](crate::ListingTable) creates one file group per
declared output partition. When unset, file grouping uses the scan-time
[`SessionConfig::target_partitions`](datafusion_execution::config::SessionConfig::target_partitions).

Files are listed in path order, split into whole-file groups across the
declared partition count, and then padded with trailing empty groups when
needed. DataFusion does not route files by partition values or validate
row placement, so callers must ensure file group `i` contains rows for
partition `i`. Layouts that require explicit file-to-partition assignment
are not supported.

For example, range partitioning on column `a` with split points
`[10, 20, 30]` declares four output partitions. With three path-ordered
files, the trailing partition is preserved as empty:

```text
files in path order: f0, f1, f2

file groups:
  partition 0: [f0]
  partition 1: [f1]
  partition 2: [f2]
  partition 3: []
```

With five path-ordered files, a partition can contain multiple files:

```text
files in path order: f0, f1, f2, f3, f4

file groups:
  partition 0: [f0, f1]
  partition 1: [f2, f3]
  partition 2: [f4]
  partition 3: []
```

Unresolved upstream links (retained, not inferred): `datafusion_execution::config::SessionConfig::target_partitions`.

<a id="op-64a5d642ae521672624413a2"></a>
## table_partition_cols

`struct_field` · `datafusion_catalog_listing::options::ListingOptions::table_partition_cols` · datafusion-catalog-listing 55.1.0

```rust
table_partition_cols: Vec<(String, arrow::datatypes::DataType)>
```

Source: `src/options.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

The expected partition column names in the folder structure.
See [Self::with_table_partition_cols](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-69f0ac0ae3a8c210d1de0fd9) for details

<a id="op-49c4a077529ddf30f0556e5a"></a>
## validate_partitions

`function` · `datafusion_catalog_listing::options::ListingOptions::validate_partitions` · datafusion-catalog-listing 55.1.0

```rust
async fn validate_partitions(&self, state: &dyn Session, table_path: &ListingTableUrl) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Infers the partition columns stored in `LOCATION` and compares
them with the columns provided in `PARTITIONED BY` to help prevent
accidental corrupts of partitioned tables.

Allows specifying partial partitions.

<a id="op-a811ae5605a98e99c47fede3"></a>
## with_file_extension

`function` · `datafusion_catalog_listing::options::ListingOptions::with_file_extension` · datafusion-catalog-listing 55.1.0

```rust
fn with_file_extension(self, file_extension: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Set file extension on [`ListingOptions`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-0221fce132b7e0b8e5f08de7) and returns self.

# Example
```
# use std::sync::Arc;
# use datafusion_catalog_listing::ListingOptions;
# use datafusion_datasource_parquet::file_format::ParquetFormat;

let listing_options = ListingOptions::new(Arc::new(ParquetFormat::default()))
    .with_file_extension(".parquet");

assert_eq!(listing_options.file_extension, ".parquet");
```

<a id="op-c2eea3de9f7613d3656bdb82"></a>
## with_file_extension_opt

`function` · `datafusion_catalog_listing::options::ListingOptions::with_file_extension_opt` · datafusion-catalog-listing 55.1.0

```rust
fn with_file_extension_opt<S>(self, file_extension: Option<S>) -> Self where S: Into<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Optionally set file extension on [`ListingOptions`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-0221fce132b7e0b8e5f08de7) and returns self.

If `file_extension` is `None`, the file extension will not be changed

# Example
```
# use std::sync::Arc;
# use datafusion_catalog_listing::ListingOptions;
# use datafusion_datasource_parquet::file_format::ParquetFormat;

let extension = Some(".parquet");
let listing_options = ListingOptions::new(Arc::new(ParquetFormat::default()))
    .with_file_extension_opt(extension);

assert_eq!(listing_options.file_extension, ".parquet");
```

<a id="op-bf6273dc2a8bfd7d77a1a4ed"></a>
## with_file_sort_order

`function` · `datafusion_catalog_listing::options::ListingOptions::with_file_sort_order` · datafusion-catalog-listing 55.1.0

```rust
fn with_file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Set file sort order on [`ListingOptions`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-0221fce132b7e0b8e5f08de7) and returns self.

```
# use std::sync::Arc;
# use datafusion_expr::col;
# use datafusion_catalog_listing::ListingOptions;
# use datafusion_datasource_parquet::file_format::ParquetFormat;

// Tell datafusion that the files are sorted by column "a"
let file_sort_order = vec![vec![col("a").sort(true, true)]];

let listing_options = ListingOptions::new(Arc::new(ParquetFormat::default()))
    .with_file_sort_order(file_sort_order.clone());

assert_eq!(listing_options.file_sort_order, file_sort_order);
```

<a id="op-9dad98a0b23dc5ce460496ee"></a>
## with_output_partitioning

`function` · `datafusion_catalog_listing::options::ListingOptions::with_output_partitioning` · datafusion-catalog-listing 55.1.0

```rust
fn with_output_partitioning(self, output_partitioning: Option<Partitioning>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Set declared output partitioning.

See [`Self::output_partitioning`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-df20634d5465d3ff2c7a06da) for the contract.

<a id="op-69f0ac0ae3a8c210d1de0fd9"></a>
## with_table_partition_cols

`function` · `datafusion_catalog_listing::options::ListingOptions::with_table_partition_cols` · datafusion-catalog-listing 55.1.0

```rust
fn with_table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::options::ListingOptions", "path": "ListingOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [415, 2], "filename": "src/options.rs"}, "trait": null, "trait_path": null}`

Source: `src/options.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

 Set `table partition columns` on [`ListingOptions`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-0221fce132b7e0b8e5f08de7) and returns self.

 "partition columns," used to support [Hive Partitioning], are
 columns added to the data that is read, based on the folder
 structure where the data resides.

 For example, give the following files in your filesystem:

 ```text
 /mnt/nyctaxi/year=2022/month=01/tripdata.parquet
 /mnt/nyctaxi/year=2021/month=12/tripdata.parquet
 /mnt/nyctaxi/year=2021/month=11/tripdata.parquet
 ```

 A [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a) created at `/mnt/nyctaxi/` with partition
 columns "year" and "month" will include new `year` and `month`
 columns while reading the files. The `year` column would have
 value `2022` and the `month` column would have value `01` for
 the rows read from
 `/mnt/nyctaxi/year=2022/month=01/tripdata.parquet`

# Notes

 - If only one level (e.g. `year` in the example above) is
   specified, the other levels are ignored but the files are
   still read.

 - Files that don't follow this partitioning scheme will be
   ignored.

 - Since the columns have the same value for all rows read from
   each individual file (such as dates), they are typically
   dictionary encoded for efficiency. You may use
   [`wrap_partition_type_in_dict`] to request a
   dictionary-encoded type.

 - The partition columns are solely extracted from the file path. Especially they are NOT part of the parquet files itself.

 # Example

 ```
 # use std::sync::Arc;
 # use arrow::datatypes::DataType;
 # use datafusion_expr::col;
 # use datafusion_catalog_listing::ListingOptions;
 # use datafusion_datasource_parquet::file_format::ParquetFormat;

 // listing options for files with paths such as  `/mnt/data/col_a=x/col_b=y/data.parquet`
 // `col_a` and `col_b` will be included in the data read from those files
 let listing_options = ListingOptions::new(Arc::new(
     ParquetFormat::default()
   ))
   .with_table_partition_cols(vec![("col_a".to_string(), DataType::Utf8),
       ("col_b".to_string(), DataType::Utf8)]);

 assert_eq!(listing_options.table_partition_cols, vec![("col_a".to_string(), DataType::Utf8),
     ("col_b".to_string(), DataType::Utf8)]);
 ```

 [Hive Partitioning]: https://docs.cloudera.com/HDPDocuments/HDP2/HDP-2.1.3/bk_system-admin-guide/content/hive_partitioned_tables.html
 [`wrap_partition_type_in_dict`]: datafusion_datasource::file_scan_config::wrap_partition_type_in_dict
