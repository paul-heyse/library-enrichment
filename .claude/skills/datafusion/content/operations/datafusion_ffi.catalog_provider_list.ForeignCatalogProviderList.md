# `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.catalog_provider_list.ForeignCatalogProviderList.json).

<a id="op-ba4674698f80d5b1d2a85eb7"></a>
## ForeignCatalogProviderList

`struct` · `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList` · datafusion-ffi 55.1.0

```rust
struct ForeignCatalogProviderList
```

Source: `src/catalog_provider_list.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_CatalogProviderList to interact with the foreign catalog provider list.

<a id="op-bd3f4f1613907e9c647d8570"></a>
## catalog

`function` · `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList::catalog` · datafusion-ffi 55.1.0

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList", "path": "ForeignCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [302, 2], "filename": "src/catalog_provider_list.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/catalog_provider_list.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce3152ca9f2421fc7301a1b7"></a>
## catalog_names

`function` · `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList::catalog_names` · datafusion-ffi 55.1.0

```rust
fn catalog_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList", "path": "ForeignCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [302, 2], "filename": "src/catalog_provider_list.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/catalog_provider_list.rs:284`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed5c3a9cd21747a3146c3672"></a>
## fmt

`function` · `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList", "path": "ForeignCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 10], "end": [239, 15], "filename": "src/catalog_provider_list.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/catalog_provider_list.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebeb2fd1b09a27c533980ba4"></a>
## register_catalog

`function` · `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList::register_catalog` · datafusion-ffi 55.1.0

```rust
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList", "path": "ForeignCatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [302, 2], "filename": "src/catalog_provider_list.rs"}, "trait": {"args": null, "id": "datafusion_session::catalog::CatalogProviderList", "path": "CatalogProviderList"}, "trait_path": "datafusion_session::catalog::CatalogProviderList"}`

Source: `src/catalog_provider_list.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
