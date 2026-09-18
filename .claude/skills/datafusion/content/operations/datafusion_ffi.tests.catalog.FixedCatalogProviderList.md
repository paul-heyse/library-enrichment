# `datafusion_ffi::tests::catalog::FixedCatalogProviderList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.tests.catalog.FixedCatalogProviderList.json).

<a id="op-e57a669118067540021596b2"></a>
## FixedCatalogProviderList

`struct` · `datafusion_ffi::tests::catalog::FixedCatalogProviderList` · datafusion-ffi 55.1.0

```rust
struct FixedCatalogProviderList
```

Source: `src/tests/catalog.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This catalog provider list is intended only for unit tests. It prepopulates with one
catalog and only allows for catalogs named after four colors.

<a id="op-64c39a5fe9edec62472afc99"></a>
## catalog

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProviderList::catalog` · datafusion-ffi 55.1.0

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProviderList", "path": "FixedCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [221, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/tests/catalog.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a0b13fc768aa2f7dfdf8f0d"></a>
## catalog_names

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProviderList::catalog_names` · datafusion-ffi 55.1.0

```rust
fn catalog_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProviderList", "path": "FixedCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [221, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/tests/catalog.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-368a2821d953d7d537af293d"></a>
## default

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProviderList::default` · datafusion-ffi 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProviderList", "path": "FixedCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [196, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/tests/catalog.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d8ff72a3407a90bae5aa596"></a>
## fmt

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProviderList::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProviderList", "path": "FixedCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 10], "end": [180, 15], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tests/catalog.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5af382c947344498be927459"></a>
## register_catalog

`function` · `datafusion_ffi::tests::catalog::FixedCatalogProviderList::register_catalog` · datafusion-ffi 55.1.0

```rust
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::tests::catalog::FixedCatalogProviderList", "path": "FixedCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [221, 2], "filename": "src/tests/catalog.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/tests/catalog.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
