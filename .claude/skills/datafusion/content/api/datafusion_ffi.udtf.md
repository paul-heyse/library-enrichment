# `datafusion_ffi::udtf`

Crate `datafusion-ffi` · 3 public items · structured records in [`model/datafusion_ffi.udtf.json`](../model/datafusion_ffi.udtf.json)

## FFI_TableFunction

`struct` · `datafusion_ffi::udtf::FFI_TableFunction`

```rust
struct FFI_TableFunction
```

**Fields**: `call`, `logical_codec`, `clone`, `release`, `private_data`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (2)

```rust
fn new(udtf: Arc<dyn TableFunctionImpl>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
fn new_with_ffi_codec(udtf: Arc<dyn TableFunctionImpl>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing a [`TableFunctionImpl`] across FFI boundaries.

---

## ForeignTableFunction

`struct` · `datafusion_ffi::udtf::ForeignTableFunction`

```rust
struct ForeignTableFunction
```

**Implements**: `datafusion_session::table::TableFunctionImpl`

**Derives**: Debug, Send, Sync

**via `datafusion_session::table::TableFunctionImpl`**

```rust
fn call(&self, args: &[datafusion_expr::Expr]) -> Result<Arc<dyn TableProvider>>
fn call_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
```

This struct is used to access an UDTF provided by a foreign
library across a FFI boundary.

The ForeignTableFunction is to be used by the caller of the UDTF, so it has
no knowledge or access to the private data. All interaction with the UDTF
must occur through the functions defined in FFI_TableFunction.

---

## TableFunctionPrivateData

`struct` · `datafusion_ffi::udtf::TableFunctionPrivateData`

```rust
struct TableFunctionPrivateData
```

---
