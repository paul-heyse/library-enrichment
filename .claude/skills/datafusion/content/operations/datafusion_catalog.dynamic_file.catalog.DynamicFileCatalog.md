# `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.dynamic_file.catalog.DynamicFileCatalog.json).

<a id="op-5c92b657507bb03943531628"></a>
## DynamicFileCatalog

`struct` · `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog` · datafusion-catalog 55.1.0

```rust
struct DynamicFileCatalog
```

Source: `src/dynamic_file/catalog.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Wrap another catalog provider list

<a id="op-5750c53540e789f7601264aa"></a>
## catalog

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog::catalog` · datafusion-catalog 55.1.0

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog", "path": "DynamicFileCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [64, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/dynamic_file/catalog.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0446a6184ed51079a88c488"></a>
## catalog_names

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog::catalog_names` · datafusion-catalog 55.1.0

```rust
fn catalog_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog", "path": "DynamicFileCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [64, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/dynamic_file/catalog.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62fbb299e4788ff51336de08"></a>
## fmt

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog", "path": "DynamicFileCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dynamic_file/catalog.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d045ceb8833c7bf714dfc59e"></a>
## new

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog::new` · datafusion-catalog 55.1.0

```rust
fn new(inner: Arc<dyn CatalogProviderList>, factory: Arc<dyn UrlTableFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog", "path": "DynamicFileCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [41, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": null, "trait_path": null}`

Source: `src/dynamic_file/catalog.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccde7209647da324e4b229dc"></a>
## register_catalog

`function` · `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog::register_catalog` · datafusion-catalog 55.1.0

```rust
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog", "path": "DynamicFileCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [64, 2], "filename": "src/dynamic_file/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/dynamic_file/catalog.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
