# `datafusion_ffi::execution::task_ctx_provider`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.execution.task_ctx_provider.json`](../model/datafusion_ffi.execution.task_ctx_provider.json)

## FFI_TaskContextProvider

`struct` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider`

Also reachable as `datafusion_ffi::execution::FFI_TaskContextProvider`

```rust
struct FFI_TaskContextProvider
```

**Fields**: `task_ctx`, `clone`, `release`, `private_data`, `library_marker_id`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**via `core::convert::From`**

```rust
fn from(ctx: &Arc<dyn TaskContextProvider>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Struct for accessing the [`TaskContext`]. This method contains a weak
reference, so there are no guarantees that the [`TaskContext`] remains
valid. This is used primarily for protobuf encoding and decoding of
data passed across the FFI boundary. See the crate README for
additional information.

---
