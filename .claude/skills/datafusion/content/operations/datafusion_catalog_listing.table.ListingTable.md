# `datafusion_catalog_listing::table::ListingTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.table.ListingTable.json).

<a id="op-00feae5c1ac67d3f887e695a"></a>
## ListingTable

`struct` · `datafusion_catalog_listing::table::ListingTable` · datafusion-catalog-listing 55.1.0

```rust
struct ListingTable
```

Source: `src/table.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Built in [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) that reads data from one or more files as a single table.

The files are read using an  [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) instance, for example from
local files or objects from AWS S3.

# Features:
* Reading multiple files as a single table
* Hive style partitioning (e.g., directories named `date=2024-06-01`)
* Merges schemas from files with compatible but not identical schemas (see [`ListingTableConfig::file_schema`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-5bd022953393c8343e35ce2a))
* `limit`, `filter` and `projection` pushdown for formats that support it (e.g.,
  Parquet)
* Statistics collection and pruning based on file metadata
* Pre-existing sort order (see [`ListingOptions::file_sort_order`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-aeed47965969ef6c92614a05))
* Metadata caching to speed up repeated queries (see [`FileMetadataCache`])
* Statistics caching (see [`FileStatisticsCache`](../operations/datafusion_execution.cache.cache_manager.FileStatisticsCache.md#op-5a9ca2374f0f0f017e5506c9))

[`FileMetadataCache`]: datafusion_execution::cache::cache_manager::FileMetadataCache

# Reading Directories and Hive Style Partitioning

For example, given the `table1` directory (or object store prefix)

```text
table1
 ├── file1.parquet
 └── file2.parquet
```

A `ListingTable` would read the files `file1.parquet` and `file2.parquet` as
a single table, merging the schemas if the files have compatible but not
identical schemas.

Given the `table2` directory (or object store prefix)

```text
table2
 ├── date=2024-06-01
 │    ├── file3.parquet
 │    └── file4.parquet
 └── date=2024-06-02
      └── file5.parquet
```

A `ListingTable` would read the files `file3.parquet`, `file4.parquet`, and
`file5.parquet` as a single table, again merging schemas if necessary.

Given the hive style partitioning structure (e.g,. directories named
`date=2024-06-01` and `date=2026-06-02`), `ListingTable` also adds a `date`
column when reading the table:
* The files in `table2/date=2024-06-01` will have the value `2024-06-01`
* The files in `table2/date=2024-06-02` will have the value `2024-06-02`.

If the query has a predicate like `WHERE date = '2024-06-01'`
only the corresponding directory will be read.

# See Also

1. [`ListingTableConfig`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-4acbb7e9df4c811e49fabab5): Configuration options
1. [`DataSourceExec`]: `ExecutionPlan` used by `ListingTable`

[`DataSourceExec`]: datafusion_datasource::source::DataSourceExec

# Caching Metadata

Some formats, such as Parquet, use the `FileMetadataCache` to cache file
metadata that is needed to execute but expensive to read, such as row
groups and statistics. The cache is scoped to the `SessionContext` and can
be configured via the [runtime config options].

[runtime config options]: https://datafusion.apache.org/user-guide/configs.html#runtime-configuration-settings

# Example: Read a directory of parquet files using a [`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

```no_run
# use datafusion_common::Result;
# use std::sync::Arc;
# use datafusion_catalog::TableProvider;
# use datafusion_catalog_listing::{ListingOptions, ListingTable, ListingTableConfig};
# use datafusion_datasource::ListingTableUrl;
# use datafusion_datasource_parquet::file_format::ParquetFormat;/// #
# use datafusion_catalog::Session;
async fn get_listing_table(session: &dyn Session) -> Result<Arc<dyn TableProvider>> {
    let table_path = "/path/to/parquet";

    // Parse the path
    let table_path = ListingTableUrl::parse(table_path)?;

    // Create default parquet options
    let file_format = ParquetFormat::new();
    let listing_options = ListingOptions::new(Arc::new(file_format))
        .with_file_extension(".parquet");

// Resolve the schema
let resolved_schema = listing_options
   .infer_schema(session, &table_path)
   .await?;

let config = ListingTableConfig::new(table_path)
  .with_listing_options(listing_options)
  .with_schema(resolved_schema);

// Create a new TableProvider
let provider = Arc::new(ListingTable::try_new(config)?);

Ok(provider)
}
```

<a id="op-fb58a954313272a71a4537d6"></a>
## clone

`function` · `datafusion_catalog_listing::table::ListingTable::clone` · datafusion-catalog-listing 55.1.0

```rust
fn clone(&self) -> ListingTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 17], "end": [179, 22], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b47825c03688ba09f844c4bd"></a>
## constraints

`function` · `datafusion_catalog_listing::table::ListingTable::constraints` · datafusion-catalog-listing 55.1.0

```rust
fn constraints(&self) -> Option<&Constraints>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9835ce64b672e31f4a837417"></a>
## fmt

`function` · `datafusion_catalog_listing::table::ListingTable::fmt` · datafusion-catalog-listing 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 10], "end": [179, 15], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01862f2174c1fb5762878378"></a>
## get_column_default

`function` · `datafusion_catalog_listing::table::ListingTable::get_column_default` · datafusion-catalog-listing 55.1.0

```rust
fn get_column_default(&self, column: &str) -> Option<&Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:770`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10a54019dfa8876cd79d969e"></a>
## get_table_definition

`function` · `datafusion_catalog_listing::table::ListingTable::get_table_definition` · datafusion-catalog-listing 55.1.0

```rust
fn get_table_definition(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b92986ced8d83211cbbb73e"></a>
## insert_into

`function` · `datafusion_catalog_listing::table::ListingTable::insert_into` · datafusion-catalog-listing 55.1.0

```rust
async fn insert_into(&self, state: &dyn Session, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:701`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad3eafe20add1ea7c9515eda"></a>
## list_files_for_scan

`function` · `datafusion_catalog_listing::table::ListingTable::list_files_for_scan` · datafusion-catalog-listing 55.1.0

```rust
async fn list_files_for_scan<'a>(&'a self, ctx: &'a dyn Session, filters: &'a [Expr], limit: Option<usize>) -> datafusion_common::Result<ListFilesResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [775, 1], "end": [1051, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:782`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Get the list of files for a scan as well as the file level statistics.
The list is grouped to let the execution plan know how the files should
be distributed to different threads / executors.

If [`ListingOptions::output_partitioning`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-df20634d5465d3ff2c7a06da) is set, returns one file
group per declared partition, including empty trailing groups.

<a id="op-6221a6da9ea5a8799de693ad"></a>
## options

`function` · `datafusion_catalog_listing::table::ListingTable::options` · datafusion-catalog-listing 55.1.0

```rust
fn options(&self) -> &ListingOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Get options ref

<a id="op-74dfc785055e8d748da6e392"></a>
## scan

`function` · `datafusion_catalog_listing::table::ListingTable::scan` · datafusion-catalog-listing 55.1.0

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdeddef1a06bcb056f833d59"></a>
## scan_with_args

`function` · `datafusion_catalog_listing::table::ListingTable::scan_with_args` · datafusion-catalog-listing 55.1.0

```rust
async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> datafusion_common::Result<ScanResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d308660258262a33aa41f42b"></a>
## schema

`function` · `datafusion_catalog_listing::table::ListingTable::schema` · datafusion-catalog-listing 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:480`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adeb958106d942bf5f829225"></a>
## schema_adapter_factory

`function` · `datafusion_catalog_listing::table::ListingTable::schema_adapter_factory` · datafusion-catalog-listing 55.1.0

```rust
fn schema_adapter_factory(&self) -> Option<Arc<dyn SchemaAdapterFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Deprecated: Returns the [`SchemaAdapterFactory`](../operations/datafusion_datasource.schema_adapter.SchemaAdapterFactory.md#op-3a8d8affcaa558dad7e4798c) used by this [`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

`SchemaAdapterFactory` has been removed. Use `PhysicalExprAdapterFactory` instead.
See `upgrading.md` for more details.

Always returns `None`.

<a id="op-889274aa48551e3db0491657"></a>
## schema_source

`function` · `datafusion_catalog_listing::table::ListingTable::schema_source` · datafusion-catalog-listing 55.1.0

```rust
fn schema_source(&self) -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Get the schema source

<a id="op-d653684b22393ebfad13a61a"></a>
## supports_filters_pushdown

`function` · `datafusion_catalog_listing::table::ListingTable::supports_filters_pushdown` · datafusion-catalog-listing 55.1.0

```rust
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> datafusion_common::Result<Vec<TableProviderFilterPushDown>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:673`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec92e8ccd1d25eb5ad35670c"></a>
## table_paths

`function` · `datafusion_catalog_listing::table::ListingTable::table_paths` · datafusion-catalog-listing 55.1.0

```rust
fn table_paths(&self) -> &Vec<ListingTableUrl>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Get paths ref

<a id="op-15289e7e2874f82e5f41bfb6"></a>
## table_type

`function` · `datafusion_catalog_listing::table::ListingTable::table_type` · datafusion-catalog-listing 55.1.0

```rust
fn table_type(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [773, 2], "filename": "src/table.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `src/table.rs:488`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4ca438d9f11f2accc7961cf"></a>
## try_create_output_ordering

`function` · `datafusion_catalog_listing::table::ListingTable::try_create_output_ordering` · datafusion-catalog-listing 55.1.0

```rust
fn try_create_output_ordering(&self, execution_props: &ExecutionProps, file_groups: &[FileGroup]) -> datafusion_common::Result<Vec<LexOrdering>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Creates output ordering from user-specified file_sort_order or derives
from file orderings when user doesn't specify.

If user specified `file_sort_order`, that takes precedence.
Otherwise, attempts to derive common ordering from file orderings in
the provided file groups.

<a id="op-d18fbff9422877af9cc1b171"></a>
## try_new

`function` · `datafusion_catalog_listing::table::ListingTable::try_new` · datafusion-catalog-listing 55.1.0

```rust
fn try_new(config: ListingTableConfig) -> datafusion_common::Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Create new [`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

See documentation and example on [`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a) and [`ListingTableConfig`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-4acbb7e9df4c811e49fabab5)

<a id="op-dc20323167e6f9927865361e"></a>
## with_cache

`function` · `datafusion_catalog_listing::table::ListingTable::with_cache` · datafusion-catalog-listing 55.1.0

```rust
fn with_cache(self, cache: Option<Arc<FileStatisticsCache>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Set the [`FileStatisticsCache`](../operations/datafusion_execution.cache.cache_manager.FileStatisticsCache.md#op-5a9ca2374f0f0f017e5506c9) used to cache parquet file statistics.

Setting a statistics cache on the `SessionContext` can avoid refetching statistics
multiple times in the same session.


<a id="op-9a32922f3d0ed53b16f0c2c6"></a>
## with_column_defaults

`function` · `datafusion_catalog_listing::table::ListingTable::with_column_defaults` · datafusion-catalog-listing 55.1.0

```rust
fn with_column_defaults(self, column_defaults: HashMap<String, Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Assign column defaults

<a id="op-c2401c8815f6ffdfee6ada52"></a>
## with_constraints

`function` · `datafusion_catalog_listing::table::ListingTable::with_constraints` · datafusion-catalog-listing 55.1.0

```rust
fn with_constraints(self, constraints: Constraints) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Assign constraints

<a id="op-277e87ba53f6811e309471df"></a>
## with_definition

`function` · `datafusion_catalog_listing::table::ListingTable::with_definition` · datafusion-catalog-listing 55.1.0

```rust
fn with_definition(self, definition: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Specify the SQL definition for this table, if any

<a id="op-5b1f2e3a30fb80b053a0239d"></a>
## with_schema_adapter_factory

`function` · `datafusion_catalog_listing::table::ListingTable::with_schema_adapter_factory` · datafusion-catalog-listing 55.1.0

```rust
fn with_schema_adapter_factory(self, _schema_adapter_factory: Arc<dyn SchemaAdapterFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListingTable", "path": "ListingTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [378, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Deprecated: Set the [`SchemaAdapterFactory`](../operations/datafusion_datasource.schema_adapter.SchemaAdapterFactory.md#op-3a8d8affcaa558dad7e4798c) for this [`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

`SchemaAdapterFactory` has been removed. Use [`ListingTableConfig::with_expr_adapter_factory`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-f80a39d13d477b5a50e15f76)
and `PhysicalExprAdapterFactory` instead. See `upgrading.md` for more details.

This method is a no-op and returns `self` unchanged.
