# `datafusion_catalog_listing::options`

Crate `datafusion-catalog-listing` · 1 public items · structured records in [`model/datafusion_catalog_listing.options.json`](../model/datafusion_catalog_listing.options.json)

## ListingOptions

`struct` · `datafusion_catalog_listing::options::ListingOptions`

Also reachable as `datafusion::datasource::listing::ListingOptions`, `datafusion_catalog_listing::ListingOptions`

```rust
struct ListingOptions
```

**Fields**: `file_extension`, `format`, `table_partition_cols`, `file_sort_order`, `output_partitioning`

**Derives**: Clone, Debug

**Methods** (9)

```rust
async fn infer_partitions(&self, state: &dyn Session, table_path: &ListingTableUrl) -> datafusion_common::Result<Vec<String>>
async fn infer_schema<'a>(&'a self, state: &dyn Session, table_path: &'a ListingTableUrl) -> datafusion_common::Result<SchemaRef>
fn new(format: Arc<dyn FileFormat>) -> Self
async fn validate_partitions(&self, state: &dyn Session, table_path: &ListingTableUrl) -> datafusion_common::Result<()>
fn with_file_extension(self, file_extension: impl Into<String>) -> Self
fn with_file_extension_opt<S>(self, file_extension: Option<S>) -> Self where S: Into<String>
fn with_file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self
fn with_output_partitioning(self, output_partitioning: Option<Partitioning>) -> Self
fn with_table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

Options for creating a [`crate::ListingTable`]

---
