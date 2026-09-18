# `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.dynamic_file.catalog.DynamicFileSchemaProvider.json).

<a id="op-18a0ffa0dca78fd49c6fa6e3"></a>
## DynamicFileSchemaProvider

`struct` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider` · datafusion-catalog 55.1.0

```rust
struct DynamicFileSchemaProvider
```

Source: `src/dynamic_file/catalog.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Implements the [DynamicFileSchemaProvider](../operations/datafusion_catalog.dynamic_file.catalog.DynamicFileSchemaProvider.md#op-18a0ffa0dca78fd49c6fa6e3) that can create tables provider from the file path.

The provider will try to create a table provider from the file path if the table provider
isn't exist in the inner schema provider.

<a id="op-87c2ba506c9adff2d277a2f9"></a>
## deregister_table

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider::deregister_table` · datafusion-catalog 55.1.0

```rust
fn deregister_table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider", "path": "DynamicFileSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [164, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/dynamic_file/catalog.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac96c1a405190d0ea0d4959e"></a>
## fmt

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider", "path": "DynamicFileSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 10], "end": [111, 15], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dynamic_file/catalog.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-578501dd1054c99b14265701"></a>
## new

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider::new` · datafusion-catalog 55.1.0

```rust
fn new(inner: Arc<dyn SchemaProvider>, factory: Arc<dyn UrlTableFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider", "path": "DynamicFileSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [127, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": null, "trait_path": null}`

Source: `src/dynamic_file/catalog.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Create a new [DynamicFileSchemaProvider](../operations/datafusion_catalog.dynamic_file.catalog.DynamicFileSchemaProvider.md#op-18a0ffa0dca78fd49c6fa6e3) with the given inner schema provider.

<a id="op-15578cffdeb04554d82ed6d2"></a>
## register_table

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider::register_table` · datafusion-catalog 55.1.0

```rust
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider", "path": "DynamicFileSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [164, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/dynamic_file/catalog.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7f002d73790d92fa3f0b535"></a>
## table

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider::table` · datafusion-catalog 55.1.0

```rust
async fn table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider", "path": "DynamicFileSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [164, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/dynamic_file/catalog.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55a7c6408fc6fe2ae3c42719"></a>
## table_exist

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider::table_exist` · datafusion-catalog 55.1.0

```rust
fn table_exist(&self, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider", "path": "DynamicFileSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [164, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/dynamic_file/catalog.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d059d29c3fe9abfaaa995db3"></a>
## table_names

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider::table_names` · datafusion-catalog 55.1.0

```rust
fn table_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider", "path": "DynamicFileSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [164, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/dynamic_file/catalog.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
