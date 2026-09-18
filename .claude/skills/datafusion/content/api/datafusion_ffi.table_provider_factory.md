# `datafusion_ffi::table_provider_factory`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.table_provider_factory.json`](../model/datafusion_ffi.table_provider_factory.json)

## FFI_TableProviderFactory

`struct` · `datafusion_ffi::table_provider_factory::FFI_TableProviderFactory`

```rust
struct FFI_TableProviderFactory
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (2)

```rust
fn new(factory: Arc<dyn TableProviderFactory + Send>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
fn new_with_ffi_codec(factory: Arc<dyn TableProviderFactory + Send>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.table_provider_factory.FFI_TableProviderFactory.md).


A stable struct for sharing [`TableProviderFactory`] across FFI boundaries.

Similar to [`FFI_TableProvider`], this struct uses the FFI-safe pattern where:
- The `FFI_*` struct exposes stable function pointers
- Private data is stored as an opaque pointer
- The `Foreign*` wrapper is used by consumers on the other side of the FFI boundary

[`FFI_TableProvider`]: crate::table_provider::FFI_TableProvider

---

## ForeignTableProviderFactory

`struct` · `datafusion_ffi::table_provider_factory::ForeignTableProviderFactory`

```rust
struct ForeignTableProviderFactory
```

**Implements**: `datafusion_session::table::TableProviderFactory`

**Derives**: Debug, Send, Sync

**via `datafusion_session::table::TableProviderFactory`**

```rust
async fn create(&self, session: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.table_provider_factory.ForeignTableProviderFactory.md).


This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_TableProviderFactory to interact with the foreign table provider factory.

---
