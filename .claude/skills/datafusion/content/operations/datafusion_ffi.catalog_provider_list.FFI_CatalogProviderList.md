# `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.catalog_provider_list.FFI_CatalogProviderList.json).

<a id="op-7d17317c6f785dd811000811"></a>
## FFI_CatalogProviderList

`struct` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList` · datafusion-ffi 55.1.0

```rust
struct FFI_CatalogProviderList
```

Source: `src/catalog_provider_list.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`CatalogProviderList`](../operations/datafusion_session.catalog.CatalogProviderList.md#op-d1c9ece1dd28ba403a6492b6) across FFI boundaries.

<a id="op-5cded7633346f491040cfb89"></a>
## catalog

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::catalog` · datafusion-ffi 55.1.0

```rust
catalog: unsafe fn(&Self, stabby::string::String) -> util::FFI_Option<catalog_provider::FFI_CatalogProvider>
```

Source: `src/catalog_provider_list.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Access a catalog

<a id="op-85698307fa0a00f8f54257f6"></a>
## catalog_names

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::catalog_names` · datafusion-ffi 55.1.0

```rust
catalog_names: unsafe fn(&Self) -> stabby::vec::Vec<stabby::string::String>
```

Source: `src/catalog_provider_list.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

List of existing catalogs

<a id="op-2ff560016be9b4c987200c56"></a>
## clone

`function` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList", "path": "FFI_CatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [260, 2], "filename": "src/catalog_provider_list.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/catalog_provider_list.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aee6a6f20ed1a8e3fe1be509"></a>
## clone

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/catalog_provider_list.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the execution plan. This should
only need to be called by the receiver of the plan.

<a id="op-dd79d2c4edb751c5f3f7db08"></a>
## drop

`function` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList", "path": "FFI_CatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [190, 2], "filename": "src/catalog_provider_list.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/catalog_provider_list.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c632dd61f74b5a26fb1a44a"></a>
## fmt

`function` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList", "path": "FFI_CatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/catalog_provider_list.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/catalog_provider_list.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38690ed790cc8d1f9b83b765"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/catalog_provider_list.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-bb7dfa008e0072b00ceb350d"></a>
## logical_codec

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::logical_codec` · datafusion-ffi 55.1.0

```rust
logical_codec: proto::logical_extension_codec::FFI_LogicalExtensionCodec
```

Source: `src/catalog_provider_list.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2e73e9ddbf85012445736dc"></a>
## new

`function` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::new` · datafusion-ffi 55.1.0

```rust
fn new(provider: Arc<dyn CatalogProviderList>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList", "path": "FFI_CatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [233, 2], "filename": "src/catalog_provider_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/catalog_provider_list.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_CatalogProviderList`](../operations/datafusion_ffi.catalog_provider_list.FFI_CatalogProviderList.md#op-7d17317c6f785dd811000811).

<a id="op-a1bba190108e87754e31fa49"></a>
## new_with_ffi_codec

`function` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::new_with_ffi_codec` · datafusion-ffi 55.1.0

```rust
fn new_with_ffi_codec(provider: Arc<dyn CatalogProviderList>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList", "path": "FFI_CatalogProviderList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [233, 2], "filename": "src/catalog_provider_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/catalog_provider_list.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5094af4baf5175138e67ae88"></a>
## private_data

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/catalog_provider_list.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
A [`ForeignCatalogProviderList`](../operations/datafusion_ffi.catalog_provider_list.ForeignCatalogProviderList.md#op-ba4674698f80d5b1d2a85eb7) should never attempt to access this data.

<a id="op-f03223be6d9262d73b891a2a"></a>
## register_catalog

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::register_catalog` · datafusion-ffi 55.1.0

```rust
register_catalog: unsafe fn(&Self, stabby::string::String, &catalog_provider::FFI_CatalogProvider) -> util::FFI_Option<catalog_provider::FFI_CatalogProvider>
```

Source: `src/catalog_provider_list.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Register a catalog

<a id="op-b46e71d18c13f1f5bbe9cf9d"></a>
## release

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/catalog_provider_list.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-09b8c059bb23881a34eb1071"></a>
## version

`struct_field` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/catalog_provider_list.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this provider.
