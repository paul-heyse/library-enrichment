# `datafusion_catalog_listing::table`

Crate `datafusion-catalog-listing` · 2 public items · structured records in [`model/datafusion_catalog_listing.table.json`](../model/datafusion_catalog_listing.table.json)

## ListFilesResult

`struct` · `datafusion_catalog_listing::table::ListFilesResult`

Also reachable as `datafusion_catalog_listing::ListFilesResult`

```rust
struct ListFilesResult
```

**Fields**: `file_groups`, `statistics`, `grouped_by_partition`

**Derives**: Debug

Result of a file listing operation from [`ListingTable::list_files_for_scan`].

---

## ListingTable

`struct` · `datafusion_catalog_listing::table::ListingTable`

Also reachable as `datafusion::datasource::listing::ListingTable`, `datafusion_catalog_listing::ListingTable`

```rust
struct ListingTable
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Clone, Debug

**Methods** (12)

```rust
async fn list_files_for_scan<'a>(&'a self, ctx: &'a dyn Session, filters: &'a [Expr], limit: Option<usize>) -> datafusion_common::Result<ListFilesResult>
fn options(&self) -> &ListingOptions
fn schema_adapter_factory(&self) -> Option<Arc<dyn SchemaAdapterFactory>>
fn schema_source(&self) -> SchemaSource
fn table_paths(&self) -> &Vec<ListingTableUrl>
fn try_create_output_ordering(&self, execution_props: &ExecutionProps, file_groups: &[FileGroup]) -> datafusion_common::Result<Vec<LexOrdering>>
fn try_new(config: ListingTableConfig) -> datafusion_common::Result<Self>
fn with_cache(self, cache: Option<Arc<FileStatisticsCache>>) -> Self
fn with_column_defaults(self, column_defaults: HashMap<String, Expr>) -> Self
fn with_constraints(self, constraints: Constraints) -> Self
fn with_definition(self, definition: Option<String>) -> Self
fn with_schema_adapter_factory(self, _schema_adapter_factory: Arc<dyn SchemaAdapterFactory>) -> Self
```

**via `datafusion_session::table::TableProvider`**

```rust
fn constraints(&self) -> Option<&Constraints>
fn get_column_default(&self, column: &str) -> Option<&Expr>
fn get_table_definition(&self) -> Option<&str>
async fn insert_into(&self, state: &dyn Session, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> datafusion_common::Result<ScanResult>
fn schema(&self) -> SchemaRef
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> datafusion_common::Result<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
```

Built in [`TableProvider`] that reads data from one or more files as a single table.

The files are read using an  [`ObjectStore`] instance, for example from
local files or objects from AWS S3.

# Features:
* Reading multiple files as a single table
* Hive style partitioning (e.g., directories named `date=2024-06-01`)
* Merges schemas from files with compatible but not identical schemas (see [`ListingTableConfig::file_schema`])
* `limit`, `filter` and `projection` pushdown for formats that support it (e.g.,
  Parquet)
* Statistics collection and pruning based on file metadata
* Pre-existing sort order (see [`ListingOptions::file_sort_order`])
* Metadata caching to speed up repeated queries (see [`FileMetadataCache`])
* Statistics caching (see [`FileStatisticsCache`])

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

1. [`ListingTableConfig`]: Configuration options
1. [`DataSourceExec`]: `ExecutionPlan` used by `ListingTable`

[`DataSourceExec`]: datafusion_datasource::source::DataSourceExec

# Caching Metadata

Some formats, such as Parquet, use the `FileMetadataCache` to cache file
metadata that is needed to execute but expensive to read, such as row
groups and statistics. The cache is scoped to the `SessionContext` and can
be configured via the [runtime config options].

[runtime config options]: https://datafusion.apache.org/user-guide/configs.html#runtime-configuration-settings

# Example: Read a directory of parquet files using a [`ListingTable`]

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

---
