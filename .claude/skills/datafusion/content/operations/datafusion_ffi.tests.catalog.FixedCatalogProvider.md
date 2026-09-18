# `datafusion_ffi::tests::catalog::FixedCatalogProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.tests.catalog.FixedCatalogProvider.json).

<a id="op-f054d0ced89e8e1f67f49dea"></a>
## FixedCatalogProvider

`struct` · `datafusion_ffi::tests::catalog::FixedCatalogProvider` · datafusion-ffi 55.1.0

```rust
struct FixedCatalogProvider
```

Source: `src/tests/catalog.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This catalog provider is intended only for unit tests. It prepopulates with one
schema and only allows for schemas named after four types of fruit.

<a id="op-179749e5f3ce570f616bdd92"></a>
## default

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProvider::default` · datafusion-ffi 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProvider", "path": "FixedCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [137, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/tests/catalog.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2482f3001bb0d4751428626"></a>
## deregister_schema

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProvider::deregister_schema` · datafusion-ffi 55.1.0

```rust
fn deregister_schema(&self, name: &str, cascade: bool) -> Result<Option<Arc<dyn SchemaProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProvider", "path": "FixedCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [169, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/tests/catalog.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45ae91df57ce1c7bba41f18c"></a>
## fmt

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProvider", "path": "FixedCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 10], "end": [124, 15], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tests/catalog.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27ff41c5fe36733c5790b7f0"></a>
## register_schema

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProvider::register_schema` · datafusion-ffi 55.1.0

```rust
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> Result<Option<Arc<dyn SchemaProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProvider", "path": "FixedCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [169, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/tests/catalog.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9db6698dd113fac10022633"></a>
## schema

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProvider::schema` · datafusion-ffi 55.1.0

```rust
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProvider", "path": "FixedCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [169, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/tests/catalog.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-476b83899dd83600ffb0b888"></a>
## schema_names

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProvider::schema_names` · datafusion-ffi 55.1.0

```rust
fn schema_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProvider", "path": "FixedCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [169, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/tests/catalog.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
