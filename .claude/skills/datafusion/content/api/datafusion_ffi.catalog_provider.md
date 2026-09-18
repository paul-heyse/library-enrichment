# `datafusion_ffi::catalog_provider`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.catalog_provider.json`](../model/datafusion_ffi.catalog_provider.json)

## FFI_CatalogProvider

`struct` · `datafusion_ffi::catalog_provider::FFI_CatalogProvider`

```rust
struct FFI_CatalogProvider
```

**Fields**: `schema_names`, `schema`, `register_schema`, `deregister_schema`, `logical_codec`, `clone`, `release`, `version`, `private_data`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (2)

```rust
fn new(provider: Arc<dyn CatalogProvider>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
fn new_with_ffi_codec(provider: Arc<dyn CatalogProvider>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.catalog_provider.FFI_CatalogProvider.md).


A stable struct for sharing [`CatalogProvider`] across FFI boundaries.

---

## ForeignCatalogProvider

`struct` · `datafusion_ffi::catalog_provider::ForeignCatalogProvider`

```rust
struct ForeignCatalogProvider
```

**Implements**: `datafusion_session::catalog::CatalogProvider`

**Derives**: Debug, Send, Sync

**via `datafusion_session::catalog::CatalogProvider`**

```rust
fn deregister_schema(&self, name: &str, cascade: bool) -> Result<Option<Arc<dyn SchemaProvider>>>
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> Result<Option<Arc<dyn SchemaProvider>>>
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
fn schema_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.catalog_provider.ForeignCatalogProvider.md).


This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_CatalogProvider to interact with the foreign table provider.

---
