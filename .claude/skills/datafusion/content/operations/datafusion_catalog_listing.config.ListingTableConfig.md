# `datafusion_catalog_listing::config::ListingTableConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.config.ListingTableConfig.json).

<a id="op-4acbb7e9df4c811e49fabab5"></a>
## ListingTableConfig

`struct` · `datafusion_catalog_listing::config::ListingTableConfig` · datafusion-catalog-listing 55.1.0

```rust
struct ListingTableConfig
```

Source: `src/config.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Configuration for creating a [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

# Schema Evolution Support

This configuration supports schema evolution through the optional
[`PhysicalExprAdapterFactory`](../operations/datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapterFactory.md#op-66df2b6b668622dcee375500). You might want to override the default factory when you need:

- **Type coercion requirements**: When you need custom logic for converting between
  different Arrow data types (e.g., Int32 ↔ Int64, Utf8 ↔ LargeUtf8)
- **Column mapping**: You need to map columns with a legacy name to a new name
- **Custom handling of missing columns**: By default they are filled in with nulls, but you may e.g. want to fill them in with `0` or `""`.

<a id="op-251dbb92d625095795fc6a30"></a>
## clone

`function` · `datafusion_catalog_listing::config::ListingTableConfig::clone` · datafusion-catalog-listing 55.1.0

```rust
fn clone(&self) -> ListingTableConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc47a5e00b38355eaf882c66"></a>
## default

`function` · `datafusion_catalog_listing::config::ListingTableConfig::default` · datafusion-catalog-listing 55.1.0

```rust
fn default() -> ListingTableConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 24], "end": [54, 31], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bd022953393c8343e35ce2a"></a>
## file_schema

`struct_field` · `datafusion_catalog_listing::config::ListingTableConfig::file_schema` · datafusion-catalog-listing 55.1.0

```rust
file_schema: Option<arrow::datatypes::SchemaRef>
```

Source: `src/config.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Optional `SchemaRef` for the to be created [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

See details on [`ListingTableConfig::with_schema`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-c6eadcb47e9c0683445f27cd)

<a id="op-aea5816e065239b0ac9ead4c"></a>
## fmt

`function` · `datafusion_catalog_listing::config::ListingTableConfig::fmt` · datafusion-catalog-listing 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1aeaae3631e92d4a179d34c"></a>
## infer_file_extension_and_compression_type

`function` · `datafusion_catalog_listing::config::ListingTableConfig::infer_file_extension_and_compression_type` · datafusion-catalog-listing 55.1.0

```rust
fn infer_file_extension_and_compression_type(path: &str) -> datafusion_common::Result<(String, Option<String>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Returns a tuple of `(file_extension, optional compression_extension)`

For example a path ending with blah.test.csv.gz returns `("csv", Some("gz"))`
For example a path ending with blah.test.csv returns `("csv", None)`

<a id="op-552dc3f4e3a23204e6df8c24"></a>
## infer_partitions_from_path

`function` · `datafusion_catalog_listing::config::ListingTableConfig::infer_partitions_from_path` · datafusion-catalog-listing 55.1.0

```rust
async fn infer_partitions_from_path(self, state: &dyn Session) -> datafusion_common::Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Infer the partition columns from `table_paths`.

# Errors
* if `self.options` is not set. See [`Self::with_listing_options`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-175b2dd2f271f628b7e84f72)

<a id="op-be542a1827e82053c419bab6"></a>
## infer_schema

`function` · `datafusion_catalog_listing::config::ListingTableConfig::infer_schema` · datafusion-catalog-listing 55.1.0

```rust
async fn infer_schema(self, state: &dyn Session) -> datafusion_common::Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Infer the [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a) based on `table_path`s.

This method infers the table schema using the first `table_path`.
See [`ListingOptions::infer_schema`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-54b025590158bc13f43edb10) for more details

# Errors
* if `self.options` is not set. See [`Self::with_listing_options`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-175b2dd2f271f628b7e84f72)

<a id="op-fda45c30d3db5680d697628f"></a>
## new

`function` · `datafusion_catalog_listing::config::ListingTableConfig::new` · datafusion-catalog-listing 55.1.0

```rust
fn new(table_path: ListingTableUrl) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Creates new [`ListingTableConfig`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-4acbb7e9df4c811e49fabab5) for reading the specified URL

<a id="op-b41f5daab032cd4944f9a3bf"></a>
## new_with_multi_paths

`function` · `datafusion_catalog_listing::config::ListingTableConfig::new_with_multi_paths` · datafusion-catalog-listing 55.1.0

```rust
fn new_with_multi_paths(table_paths: Vec<ListingTableUrl>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Creates new [`ListingTableConfig`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-4acbb7e9df4c811e49fabab5) with multiple table paths.

 See `ListingTableConfigExt::infer_options` for details on what happens with multiple paths

<a id="op-c660a2cf0c3842436752fcc6"></a>
## options

`struct_field` · `datafusion_catalog_listing::config::ListingTableConfig::options` · datafusion-catalog-listing 55.1.0

```rust
options: Option<options::ListingOptions>
```

Source: `src/config.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Optional [`ListingOptions`](../operations/datafusion_catalog_listing.options.ListingOptions.md#op-0221fce132b7e0b8e5f08de7) for the to be created [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

See details on [`ListingTableConfig::with_listing_options`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-175b2dd2f271f628b7e84f72)

<a id="op-1c9009e1984987f2ac474afe"></a>
## schema_source

`function` · `datafusion_catalog_listing::config::ListingTableConfig::schema_source` · datafusion-catalog-listing 55.1.0

```rust
fn schema_source(&self) -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Returns the source of the schema for this configuration

<a id="op-07b4fa55894a82aefcadbc2c"></a>
## table_paths

`struct_field` · `datafusion_catalog_listing::config::ListingTableConfig::table_paths` · datafusion-catalog-listing 55.1.0

```rust
table_paths: Vec<datafusion_datasource::ListingTableUrl>
```

Source: `src/config.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Paths on the `ObjectStore` for creating [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).
They should share the same schema and object store.

<a id="op-f80a39d13d477b5a50e15f76"></a>
## with_expr_adapter_factory

`function` · `datafusion_catalog_listing::config::ListingTableConfig::with_expr_adapter_factory` · datafusion-catalog-listing 55.1.0

```rust
fn with_expr_adapter_factory(self, expr_adapter_factory: Arc<dyn PhysicalExprAdapterFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Set the [`PhysicalExprAdapterFactory`](../operations/datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapterFactory.md#op-66df2b6b668622dcee375500) for the [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

The expression adapter factory is used to create physical expression adapters that can
handle schema evolution and type conversions when evaluating expressions
with different schemas than the table schema.

<a id="op-175b2dd2f271f628b7e84f72"></a>
## with_listing_options

`function` · `datafusion_catalog_listing::config::ListingTableConfig::with_listing_options` · datafusion-catalog-listing 55.1.0

```rust
fn with_listing_options(self, listing_options: ListingOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Add `listing_options` to [`ListingTableConfig`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-4acbb7e9df4c811e49fabab5)

If not provided, format and other options are inferred via
`ListingTableConfigExt::infer_options`.

# Example: Configuring Parquet Files with Custom Options
```rust
# use std::sync::Arc;
# use datafusion_catalog_listing::{ListingTableConfig, ListingOptions};
# use datafusion_datasource::ListingTableUrl;
# use datafusion_datasource_parquet::file_format::ParquetFormat;
# let table_paths = ListingTableUrl::parse("file:///path/to/data").unwrap();
let options = ListingOptions::new(Arc::new(ParquetFormat::default()))
    .with_file_extension(".parquet");

let config = ListingTableConfig::new(table_paths).with_listing_options(options);
// Configure file format and options
```

<a id="op-c6eadcb47e9c0683445f27cd"></a>
## with_schema

`function` · `datafusion_catalog_listing::config::ListingTableConfig::with_schema` · datafusion-catalog-listing 55.1.0

```rust
fn with_schema(self, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Set the `schema` for the overall [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

[`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a) will automatically coerce, when possible, the schema
for individual files to match this schema.

If a schema is not provided, it is inferred using
[`Self::infer_schema`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-be542a1827e82053c419bab6).

If the schema is provided, it must contain only the fields in the file
without the table partitioning columns.

# Example: Specifying Table Schema
```rust
# use std::sync::Arc;
# use datafusion_catalog_listing::{ListingTableConfig, ListingOptions};
# use datafusion_datasource::ListingTableUrl;
# use datafusion_datasource_parquet::file_format::ParquetFormat;
# use arrow::datatypes::{Schema, Field, DataType};
# let table_paths = ListingTableUrl::parse("file:///path/to/data").unwrap();
# let listing_options = ListingOptions::new(Arc::new(ParquetFormat::default()));
let schema = Arc::new(Schema::new(vec![
    Field::new("id", DataType::Int64, false),
    Field::new("name", DataType::Utf8, true),
]));

let config = ListingTableConfig::new(table_paths)
    .with_listing_options(listing_options)  // Set options first
    .with_schema(schema);                    // Then set schema
```

<a id="op-5100d19b8570f541a1aa6bf2"></a>
## with_schema_adapter_factory

`function` · `datafusion_catalog_listing::config::ListingTableConfig::with_schema_adapter_factory` · datafusion-catalog-listing 55.1.0

```rust
fn with_schema_adapter_factory(self, _schema_adapter_factory: Arc<dyn SchemaAdapterFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::ListingTableConfig", "path": "ListingTableConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [318, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Deprecated: Set the [`SchemaAdapterFactory`](../operations/datafusion_datasource.schema_adapter.SchemaAdapterFactory.md#op-3a8d8affcaa558dad7e4798c) for the [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

`SchemaAdapterFactory` has been removed. Use [`Self::with_expr_adapter_factory`](../operations/datafusion_catalog_listing.config.ListingTableConfig.md#op-f80a39d13d477b5a50e15f76)
and `PhysicalExprAdapterFactory` instead. See `upgrading.md` for more details.

This method is a no-op and returns `self` unchanged.
