# `datafusion_catalog::memory::catalog::MemoryCatalogProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.memory.catalog.MemoryCatalogProvider.json).

<a id="op-faa1b922b564ab8766a89660"></a>
## MemoryCatalogProvider

`struct` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider` · datafusion-catalog 55.1.0

```rust
struct MemoryCatalogProvider
```

Source: `src/memory/catalog.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Simple in-memory implementation of a catalog.

<a id="op-1b18644d7ef64fd8739a2f52"></a>
## default

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider::default` · datafusion-catalog 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProvider", "path": "MemoryCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [85, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/memory/catalog.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3254f0176dfcde13cbe2f88"></a>
## deregister_schema

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider::deregister_schema` · datafusion-catalog 55.1.0

```rust
fn deregister_schema(&self, name: &str, cascade: bool) -> datafusion_common::Result<Option<Arc<dyn SchemaProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProvider", "path": "MemoryCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [126, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/memory/catalog.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47845ab7afb44f4518af728e"></a>
## fmt

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProvider", "path": "MemoryCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory/catalog.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a4de01b0be80c18bb9ef842"></a>
## new

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider::new` · datafusion-catalog 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProvider", "path": "MemoryCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [79, 2], "filename": "src/memory/catalog.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory/catalog.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Instantiates a new MemoryCatalogProvider with an empty collection of schemas.

<a id="op-f0c8fe109a32cdb5d2921770"></a>
## register_schema

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider::register_schema` · datafusion-catalog 55.1.0

```rust
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> datafusion_common::Result<Option<Arc<dyn SchemaProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProvider", "path": "MemoryCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [126, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/memory/catalog.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63ee61bdc8a51c0ba45c9d22"></a>
## schema

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider::schema` · datafusion-catalog 55.1.0

```rust
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProvider", "path": "MemoryCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [126, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/memory/catalog.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac0f2d7926576c79c80b15ac"></a>
## schema_names

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProvider::schema_names` · datafusion-catalog 55.1.0

```rust
fn schema_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProvider", "path": "MemoryCatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [126, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProvider", "path": "CatalogProvider"}, "trait_path": "datafusion_session::catalog::CatalogProvider"}`

Source: `src/memory/catalog.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
