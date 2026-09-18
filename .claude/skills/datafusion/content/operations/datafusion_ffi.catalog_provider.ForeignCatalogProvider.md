# `datafusion_ffi::catalog_provider::ForeignCatalogProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.catalog_provider.ForeignCatalogProvider.json).

<a id="op-c7b38b09384d19d2ec24cdf2"></a>
## ForeignCatalogProvider

`struct` · `datafusion_ffi::catalog_provider::ForeignCatalogProvider` · datafusion-ffi 55.1.0

```rust
struct ForeignCatalogProvider
```

Source: `src/catalog_provider.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_CatalogProvider to interact with the foreign table provider.

<a id="op-07165995706d9bfd00a14087"></a>
## deregister_schema

`function` · `datafusion_ffi::catalog_provider::ForeignCatalogProvider::deregister_schema` · datafusion-ffi 55.1.0

```rust
fn deregister_schema(&self, name: &str, cascade: bool) -> Result<Option<Arc<dyn SchemaProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::ForeignCatalogProvider", "path": "ForeignCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [357, 2], "filename": "src/catalog_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/catalog_provider.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ded8f4366c695fb4cfe63dda"></a>
## fmt

`function` · `datafusion_ffi::catalog_provider::ForeignCatalogProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::ForeignCatalogProvider", "path": "ForeignCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 10], "end": [277, 15], "filename": "src/catalog_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/catalog_provider.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9d324c6870f072ede6dc55f"></a>
## register_schema

`function` · `datafusion_ffi::catalog_provider::ForeignCatalogProvider::register_schema` · datafusion-ffi 55.1.0

```rust
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> Result<Option<Arc<dyn SchemaProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::ForeignCatalogProvider", "path": "ForeignCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [357, 2], "filename": "src/catalog_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/catalog_provider.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76528413e32c40db6135f436"></a>
## schema

`function` · `datafusion_ffi::catalog_provider::ForeignCatalogProvider::schema` · datafusion-ffi 55.1.0

```rust
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::ForeignCatalogProvider", "path": "ForeignCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [357, 2], "filename": "src/catalog_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/catalog_provider.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99b7a1895a5ce6acabf8df60"></a>
## schema_names

`function` · `datafusion_ffi::catalog_provider::ForeignCatalogProvider::schema_names` · datafusion-ffi 55.1.0

```rust
fn schema_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::ForeignCatalogProvider", "path": "ForeignCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [357, 2], "filename": "src/catalog_provider.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/catalog_provider.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
