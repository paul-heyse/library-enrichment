# `datafusion_catalog::listing_schema::ListingSchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.listing_schema.ListingSchemaProvider.json).

<a id="op-2ea5d688cb1b249139f89be4"></a>
## ListingSchemaProvider

`struct` · `datafusion_catalog::listing_schema::ListingSchemaProvider` · datafusion-catalog 55.1.0

```rust
struct ListingSchemaProvider
```

Source: `src/listing_schema.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

A [`SchemaProvider`](../operations/datafusion_session.schema.SchemaProvider.md#op-009a61a5d6d9122859b6d788) that scans an [`ObjectStore`] to automatically discover tables

A subfolder relationship is assumed, i.e. given:
- authority = `s3://host.example.com:3000`
- path = `/data/tpch`
- factory = `DeltaTableFactory`

A table called "customer" will be registered for the folder:
`s3://host.example.com:3000/data/tpch/customer`

assuming it contains valid deltalake data, i.e:
- `s3://host.example.com:3000/data/tpch/customer/part-00000-xxxx.snappy.parquet`
- `s3://host.example.com:3000/data/tpch/customer/_delta_log/`

[`ObjectStore`]: object_store::ObjectStore

<a id="op-2476180e7321156a9e65ba67"></a>
## deregister_table

`function` · `datafusion_catalog::listing_schema::ListingSchemaProvider::deregister_table` · datafusion-catalog 55.1.0

```rust
fn deregister_table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::listing_schema::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [194, 2], "filename": "src/listing_schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/listing_schema.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe15754e19152f193c78ac44"></a>
## fmt

`function` · `datafusion_catalog::listing_schema::ListingSchemaProvider::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::listing_schema::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "src/listing_schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/listing_schema.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99508ed1dc739bb1b20a2dc5"></a>
## new

`function` · `datafusion_catalog::listing_schema::ListingSchemaProvider::new` · datafusion-catalog 55.1.0

```rust
fn new(authority: String, path: object_store::path::Path, factory: Arc<dyn TableProviderFactory>, store: Arc<dyn ObjectStore>, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::listing_schema::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/listing_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/listing_schema.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Create a new `ListingSchemaProvider`

Arguments:
`authority`: The scheme (i.e. s3://) + host (i.e. example.com:3000)
`path`: The root path that contains subfolders which represent tables
`factory`: The `TableProviderFactory` to use to instantiate tables for each subfolder
`store`: The `ObjectStore` containing the table data
`format`: The `FileFormat` of the tables
`has_header`: Indicates whether the created external table has the has_header flag enabled

<a id="op-cbbf890f698c5b4fdbb64f9b"></a>
## refresh

`function` · `datafusion_catalog::listing_schema::ListingSchemaProvider::refresh` · datafusion-catalog 55.1.0

```rust
async fn refresh(&self, state: &dyn Session) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::listing_schema::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [144, 2], "filename": "src/listing_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/listing_schema.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Reload table information from ObjectStore

<a id="op-45d4ed8f44afcb1dd83390ae"></a>
## register_table

`function` · `datafusion_catalog::listing_schema::ListingSchemaProvider::register_table` · datafusion-catalog 55.1.0

```rust
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::listing_schema::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [194, 2], "filename": "src/listing_schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/listing_schema.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc0262289e0bb0dafe252c01"></a>
## table

`function` · `datafusion_catalog::listing_schema::ListingSchemaProvider::table` · datafusion-catalog 55.1.0

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::listing_schema::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [194, 2], "filename": "src/listing_schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/listing_schema.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4a0f783e60314e3e7d7faf5"></a>
## table_exist

`function` · `datafusion_catalog::listing_schema::ListingSchemaProvider::table_exist` · datafusion-catalog 55.1.0

```rust
fn table_exist(&self, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::listing_schema::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [194, 2], "filename": "src/listing_schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/listing_schema.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b45c435bd486d50d64126db1"></a>
## table_names

`function` · `datafusion_catalog::listing_schema::ListingSchemaProvider::table_names` · datafusion-catalog 55.1.0

```rust
fn table_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::listing_schema::ListingSchemaProvider", "path": "ListingSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [194, 2], "filename": "src/listing_schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/listing_schema.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
