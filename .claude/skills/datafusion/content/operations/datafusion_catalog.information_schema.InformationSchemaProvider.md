# `datafusion_catalog::information_schema::InformationSchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.information_schema.InformationSchemaProvider.json).

<a id="op-b050598850e7b6d7d17f84a5"></a>
## InformationSchemaProvider

`struct` · `datafusion_catalog::information_schema::InformationSchemaProvider` · datafusion-catalog 55.1.0

```rust
struct InformationSchemaProvider
```

Source: `src/information_schema.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Implements the `information_schema` virtual schema and tables

The underlying tables in the `information_schema` are created on
demand. This means that if more tables are added to the underlying
providers, they will appear the next time the `information_schema`
table is queried.

<a id="op-de856ec2ea57f1a87f86fe02"></a>
## fmt

`function` · `datafusion_catalog::information_schema::InformationSchemaProvider::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemaProvider", "path": "InformationSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 15], "filename": "src/information_schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/information_schema.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1d804929a38e8713ceeef13"></a>
## new

`function` · `datafusion_catalog::information_schema::InformationSchemaProvider::new` · datafusion-catalog 55.1.0

```rust
fn new(catalog_list: Arc<dyn CatalogProviderList>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemaProvider", "path": "InformationSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [101, 2], "filename": "src/information_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/information_schema.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Creates a new [`InformationSchemaProvider`](../operations/datafusion_catalog.information_schema.InformationSchemaProvider.md#op-b050598850e7b6d7d17f84a5) for the provided `catalog_list`

<a id="op-69fb55504bf00719f2206550"></a>
## table

`function` · `datafusion_catalog::information_schema::InformationSchemaProvider::table` · datafusion-catalog 55.1.0

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemaProvider", "path": "InformationSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [609, 2], "filename": "src/information_schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/information_schema.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7dcd45dd512b80ea3e75f56"></a>
## table_exist

`function` · `datafusion_catalog::information_schema::InformationSchemaProvider::table_exist` · datafusion-catalog 55.1.0

```rust
fn table_exist(&self, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemaProvider", "path": "InformationSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [609, 2], "filename": "src/information_schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/information_schema.rs:606`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd8f91e7f7d6ae34d1c7fdb2"></a>
## table_names

`function` · `datafusion_catalog::information_schema::InformationSchemaProvider::table_names` · datafusion-catalog 55.1.0

```rust
fn table_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemaProvider", "path": "InformationSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [609, 2], "filename": "src/information_schema.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/information_schema.rs:578`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e28025eaae70141e00d5812"></a>
## with_table_functions

`function` · `datafusion_catalog::information_schema::InformationSchemaProvider::with_table_functions` · datafusion-catalog 55.1.0

```rust
fn with_table_functions(self, table_functions: HashMap<String, Arc<TableFunction>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::information_schema::InformationSchemaProvider", "path": "InformationSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [101, 2], "filename": "src/information_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/information_schema.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Attach the session's table (UDTF) functions so that they appear in
`information_schema.routines` / `SHOW FUNCTIONS`.
