# `datafusion_catalog::memory::catalog::MemoryCatalogProviderList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.memory.catalog.MemoryCatalogProviderList.json).

<a id="op-937d01547a7438b1ed33a7d0"></a>
## MemoryCatalogProviderList

`struct` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList` · datafusion-catalog 55.1.0

```rust
struct MemoryCatalogProviderList
```

Source: `src/memory/catalog.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Simple in-memory list of catalogs

<a id="op-5f024bfc6aa2878fd962eea7"></a>
## catalog

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList::catalog` · datafusion-catalog 55.1.0

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProviderList", "path": "MemoryCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [64, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/memory/catalog.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9edbd71a09615ef0b79bfcc"></a>
## catalog_names

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList::catalog_names` · datafusion-catalog 55.1.0

```rust
fn catalog_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProviderList", "path": "MemoryCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [64, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/memory/catalog.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8358744548bb16661e5a7691"></a>
## catalogs

`struct_field` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList::catalogs` · datafusion-catalog 55.1.0

```rust
catalogs: dashmap::DashMap<String, std::sync::Arc<dyn CatalogProvider>>
```

Source: `src/memory/catalog.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Collection of catalogs containing schemas and ultimately TableProviders

<a id="op-21489ff63b05fa6600e14204"></a>
## default

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList::default` · datafusion-catalog 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProviderList", "path": "MemoryCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [46, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/memory/catalog.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7eba17e133c11ae9573980ed"></a>
## fmt

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProviderList", "path": "MemoryCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory/catalog.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af74f7db9dd4830f9a5c4f87"></a>
## new

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList::new` · datafusion-catalog 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProviderList", "path": "MemoryCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [40, 2], "filename": "src/memory/catalog.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory/catalog.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Instantiates a new `MemoryCatalogProviderList` with an empty collection of catalogs

<a id="op-a4cdec49f0bf70c2203f17a6"></a>
## register_catalog

`function` · `datafusion_catalog::memory::catalog::MemoryCatalogProviderList::register_catalog` · datafusion-catalog 55.1.0

```rust
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::memory::catalog::MemoryCatalogProviderList", "path": "MemoryCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [64, 2], "filename": "src/memory/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/memory/catalog.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
