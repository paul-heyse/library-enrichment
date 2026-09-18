# `datafusion_ffi::schema_provider`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.schema_provider.json`](../model/datafusion_ffi.schema_provider.json)

## FFI_SchemaProvider

`struct` · `datafusion_ffi::schema_provider::FFI_SchemaProvider`

```rust
struct FFI_SchemaProvider
```

**Fields**: `owner_name`, `table_names`, `table`, `register_table`, `deregister_table`, `table_exist`, `logical_codec`, `clone`, `release`, `version`, `private_data`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (2)

```rust
fn new(provider: Arc<dyn SchemaProvider>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
fn new_with_ffi_codec(provider: Arc<dyn SchemaProvider>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.schema_provider.FFI_SchemaProvider.md).


A stable struct for sharing [`SchemaProvider`] across FFI boundaries.

---

## ForeignSchemaProvider

`struct` · `datafusion_ffi::schema_provider::ForeignSchemaProvider`

```rust
struct ForeignSchemaProvider
```

**Implements**: `datafusion_session::schema::SchemaProvider`

**Derives**: Debug, Send, Sync

**via `datafusion_session::schema::SchemaProvider`**

```rust
fn deregister_table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
fn owner_name(&self) -> Option<&str>
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.schema_provider.ForeignSchemaProvider.md).


This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_SchemaProvider to interact with the foreign table provider.

---
