# `datafusion_ffi::catalog_provider_list`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.catalog_provider_list.json`](../model/datafusion_ffi.catalog_provider_list.json)

## FFI_CatalogProviderList

`struct` · `datafusion_ffi::catalog_provider_list::FFI_CatalogProviderList`

```rust
struct FFI_CatalogProviderList
```

**Fields**: `register_catalog`, `catalog_names`, `catalog`, `logical_codec`, `clone`, `release`, `version`, `private_data`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (2)

```rust
fn new(provider: Arc<dyn CatalogProviderList>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
fn new_with_ffi_codec(provider: Arc<dyn CatalogProviderList>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing [`CatalogProviderList`] across FFI boundaries.

---

## ForeignCatalogProviderList

`struct` · `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList`

```rust
struct ForeignCatalogProviderList
```

**Implements**: `datafusion_session::catalog::CatalogProviderList`

**Derives**: Debug, Send, Sync

**via `datafusion_session::catalog::CatalogProviderList`**

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_CatalogProviderList to interact with the foreign catalog provider list.

---
