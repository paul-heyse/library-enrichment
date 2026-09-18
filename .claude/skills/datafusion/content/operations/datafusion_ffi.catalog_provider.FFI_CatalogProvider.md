# `datafusion_ffi::catalog_provider::FFI_CatalogProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.catalog_provider.FFI_CatalogProvider.json).

<a id="op-3fbf4c2765896349192d5687"></a>
## FFI_CatalogProvider

`struct` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider` · datafusion-ffi 55.1.0

```rust
struct FFI_CatalogProvider
```

Source: `src/catalog_provider.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad) across FFI boundaries.

<a id="op-9dc51e94a213147ccd25b9c1"></a>
## clone

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/catalog_provider.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the execution plan. This should
only need to be called by the receiver of the plan.

<a id="op-cb478cb7f41585daca96a544"></a>
## clone

`function` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::FFI_CatalogProvider", "path": "FFI_CatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [297, 2], "filename": "src/catalog_provider.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/catalog_provider.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f6b5a814867893e8290ccfc"></a>
## deregister_schema

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::deregister_schema` · datafusion-ffi 55.1.0

```rust
deregister_schema: unsafe fn(&Self, stabby::string::String, bool) -> util::FFI_Result<util::FFI_Option<schema_provider::FFI_SchemaProvider>>
```

Source: `src/catalog_provider.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffe81f1e36670b381b27b835"></a>
## drop

`function` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::FFI_CatalogProvider", "path": "FFI_CatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [226, 2], "filename": "src/catalog_provider.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/catalog_provider.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de0fd88bb8b4d7594554d9a7"></a>
## fmt

`function` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::FFI_CatalogProvider", "path": "FFI_CatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/catalog_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/catalog_provider.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a0c43335a90c045adc3069a"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/catalog_provider.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-bb60bac49076feef6329fce6"></a>
## logical_codec

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::logical_codec` · datafusion-ffi 55.1.0

```rust
logical_codec: proto::logical_extension_codec::FFI_LogicalExtensionCodec
```

Source: `src/catalog_provider.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-689bde66afef5d19939a1001"></a>
## new

`function` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::new` · datafusion-ffi 55.1.0

```rust
fn new(provider: Arc<dyn CatalogProvider>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::FFI_CatalogProvider", "path": "FFI_CatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [271, 2], "filename": "src/catalog_provider.rs"}, "trait": null, "trait_path": null}`

Source: `src/catalog_provider.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_CatalogProvider`](../operations/datafusion_ffi.catalog_provider.FFI_CatalogProvider.md#op-3fbf4c2765896349192d5687).

<a id="op-e805d1b01136a86e859e356e"></a>
## new_with_ffi_codec

`function` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::new_with_ffi_codec` · datafusion-ffi 55.1.0

```rust
fn new_with_ffi_codec(provider: Arc<dyn CatalogProvider>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::catalog_provider::FFI_CatalogProvider", "path": "FFI_CatalogProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [271, 2], "filename": "src/catalog_provider.rs"}, "trait": null, "trait_path": null}`

Source: `src/catalog_provider.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b76957b4a83cd6214af9955"></a>
## private_data

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/catalog_provider.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
A [`ForeignCatalogProvider`](../operations/datafusion_ffi.catalog_provider.ForeignCatalogProvider.md#op-c7b38b09384d19d2ec24cdf2) should never attempt to access this data.

<a id="op-1d2ec1bba96e13bafe4ba1a8"></a>
## register_schema

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::register_schema` · datafusion-ffi 55.1.0

```rust
register_schema: unsafe fn(&Self, stabby::string::String, &schema_provider::FFI_SchemaProvider) -> util::FFI_Result<util::FFI_Option<schema_provider::FFI_SchemaProvider>>
```

Source: `src/catalog_provider.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0236198f644e218b9d8d5642"></a>
## release

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/catalog_provider.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-7443c3ebb4364ada5ada05c6"></a>
## schema

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::schema` · datafusion-ffi 55.1.0

```rust
schema: unsafe fn(&Self, stabby::string::String) -> util::FFI_Option<schema_provider::FFI_SchemaProvider>
```

Source: `src/catalog_provider.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0e6fbed251599e8df1c7448"></a>
## schema_names

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::schema_names` · datafusion-ffi 55.1.0

```rust
schema_names: unsafe fn(&Self) -> stabby::vec::Vec<stabby::string::String>
```

Source: `src/catalog_provider.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9baa7cc2817cf0abd43b288"></a>
## version

`struct_field` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/catalog_provider.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this provider.
