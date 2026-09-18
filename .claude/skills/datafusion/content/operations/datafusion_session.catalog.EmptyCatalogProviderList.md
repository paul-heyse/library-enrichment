# `datafusion_session::catalog::EmptyCatalogProviderList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.catalog.EmptyCatalogProviderList.json).

<a id="op-5221d4b70432dd91fe091b3b"></a>
## EmptyCatalogProviderList

`struct` · `datafusion_session::catalog::EmptyCatalogProviderList` · datafusion-session 55.1.0

```rust
struct EmptyCatalogProviderList
```

Source: `src/catalog.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A catalog list that contains no catalogs.

[`Session`](crate::Session) implementations that do not provide catalog
access can return this list explicitly.

<a id="op-93966a94672a4be3fb249123"></a>
## catalog

`function` · `datafusion_session::catalog::EmptyCatalogProviderList::catalog` · datafusion-session 55.1.0

```rust
fn catalog(&self, _name: &str) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::catalog::EmptyCatalogProviderList", "path": "EmptyCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [49, 2], "filename": "src/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/catalog.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3f98ce81ad50e881490fd23"></a>
## catalog_names

`function` · `datafusion_session::catalog::EmptyCatalogProviderList::catalog_names` · datafusion-session 55.1.0

```rust
fn catalog_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::catalog::EmptyCatalogProviderList", "path": "EmptyCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [49, 2], "filename": "src/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/catalog.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95e8eaf89397e093a5e779d6"></a>
## default

`function` · `datafusion_session::catalog::EmptyCatalogProviderList::default` · datafusion-session 55.1.0

```rust
fn default() -> EmptyCatalogProviderList
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::catalog::EmptyCatalogProviderList", "path": "EmptyCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 24], "filename": "src/catalog.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/catalog.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5772510493017793b486589e"></a>
## fmt

`function` · `datafusion_session::catalog::EmptyCatalogProviderList::fmt` · datafusion-session 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::catalog::EmptyCatalogProviderList", "path": "EmptyCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/catalog.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/catalog.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e190ebddb000000611587d2"></a>
## register_catalog

`function` · `datafusion_session::catalog::EmptyCatalogProviderList::register_catalog` · datafusion-session 55.1.0

```rust
fn register_catalog(&self, _name: String, _catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::catalog::EmptyCatalogProviderList", "path": "EmptyCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [49, 2], "filename": "src/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/catalog.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
