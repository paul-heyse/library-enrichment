# `datafusion_catalog_listing::config`

Crate `datafusion-catalog-listing` · 2 public items · structured records in [`model/datafusion_catalog_listing.config.json`](../model/datafusion_catalog_listing.config.json)

## SchemaSource

`enum` · `datafusion_catalog_listing::config::SchemaSource`

Also reachable as `datafusion_catalog_listing::SchemaSource`

```rust
enum SchemaSource
```

**Variants**: `Unset`, `Inferred`, `Specified`

**Derives**: Clone, Copy, Debug, Default, PartialEq, StructuralPartialEq

Indicates the source of the schema for a [`crate::ListingTable`]

---

## ListingTableConfig

`struct` · `datafusion_catalog_listing::config::ListingTableConfig`

Also reachable as `datafusion::datasource::listing::ListingTableConfig`, `datafusion_catalog_listing::ListingTableConfig`

```rust
struct ListingTableConfig
```

**Fields**: `table_paths`, `file_schema`, `options`

**Implements**: `datafusion::datasource::listing::table::ListingTableConfigExt`

**Derives**: Clone, Debug, Default

**Methods** (10)

```rust
fn infer_file_extension_and_compression_type(path: &str) -> datafusion_common::Result<(String, Option<String>)>
async fn infer_partitions_from_path(self, state: &dyn Session) -> datafusion_common::Result<Self>
async fn infer_schema(self, state: &dyn Session) -> datafusion_common::Result<Self>
fn new(table_path: ListingTableUrl) -> Self
fn new_with_multi_paths(table_paths: Vec<ListingTableUrl>) -> Self
fn schema_source(&self) -> SchemaSource
fn with_expr_adapter_factory(self, expr_adapter_factory: Arc<dyn PhysicalExprAdapterFactory>) -> Self
fn with_listing_options(self, listing_options: ListingOptions) -> Self
fn with_schema(self, schema: SchemaRef) -> Self
fn with_schema_adapter_factory(self, _schema_adapter_factory: Arc<dyn SchemaAdapterFactory>) -> Self
```

Configuration for creating a [`crate::ListingTable`]

# Schema Evolution Support

This configuration supports schema evolution through the optional
[`PhysicalExprAdapterFactory`]. You might want to override the default factory when you need:

- **Type coercion requirements**: When you need custom logic for converting between
  different Arrow data types (e.g., Int32 ↔ Int64, Utf8 ↔ LargeUtf8)
- **Column mapping**: You need to map columns with a legacy name to a new name
- **Custom handling of missing columns**: By default they are filled in with nulls, but you may e.g. want to fill them in with `0` or `""`.

---
