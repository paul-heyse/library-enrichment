# `datafusion_ffi::tests::catalog::FixedSchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.tests.catalog.FixedSchemaProvider.json).

<a id="op-732cceea4a39e6d2d1de2ce5"></a>
## FixedSchemaProvider

`struct` · `datafusion_ffi::tests::catalog::FixedSchemaProvider` · datafusion-ffi 55.1.0

```rust
struct FixedSchemaProvider
```

Source: `src/tests/catalog.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This schema provider is intended only for unit tests. It prepopulates with one
table and only allows for tables named sales and purchases.

<a id="op-ebbf840ee682941777d3f71e"></a>
## default

`function` · `datafusion_ffi::tests::catalog::FixedSchemaProvider::default` · datafusion-ffi 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedSchemaProvider", "path": "FixedSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [87, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/tests/catalog.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-605349d6569887ef40c4a3fd"></a>
## deregister_table

`function` · `datafusion_ffi::tests::catalog::FixedSchemaProvider::deregister_table` · datafusion-ffi 55.1.0

```rust
fn deregister_table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedSchemaProvider", "path": "FixedSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [120, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/tests/catalog.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8214635f153d596f5cfc8802"></a>
## fmt

`function` · `datafusion_ffi::tests::catalog::FixedSchemaProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedSchemaProvider", "path": "FixedSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tests/catalog.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a0e11189f40043f0a826ec8"></a>
## register_table

`function` · `datafusion_ffi::tests::catalog::FixedSchemaProvider::register_table` · datafusion-ffi 55.1.0

```rust
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedSchemaProvider", "path": "FixedSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [120, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/tests/catalog.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8defcd8a144279d89ef3876"></a>
## table

`function` · `datafusion_ffi::tests::catalog::FixedSchemaProvider::table` · datafusion-ffi 55.1.0

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedSchemaProvider", "path": "FixedSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [120, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/tests/catalog.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d474f42c0ec662edebd3f00"></a>
## table_exist

`function` · `datafusion_ffi::tests::catalog::FixedSchemaProvider::table_exist` · datafusion-ffi 55.1.0

```rust
fn table_exist(&self, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedSchemaProvider", "path": "FixedSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [120, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/tests/catalog.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6fd2b5ccd4b9fc49e4b7a13"></a>
## table_names

`function` · `datafusion_ffi::tests::catalog::FixedSchemaProvider::table_names` · datafusion-ffi 55.1.0

```rust
fn table_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedSchemaProvider", "path": "FixedSchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [120, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::schema::SchemaProvider", "path": "SchemaProvider"}, "trait_path": "datafusion_session::schema::SchemaProvider"}`

Source: `src/tests/catalog.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
