# `datafusion_ffi::execution::task_ctx`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.execution.task_ctx.json`](../model/datafusion_ffi.execution.task_ctx.json)

## FFI_TaskContext

`struct` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext`

Also reachable as `datafusion_ffi::execution::FFI_TaskContext`

```rust
struct FFI_TaskContext
```

**Fields**: `session_id`, `task_id`, `session_config`, `scalar_functions`, `aggregate_functions`, `window_functions`, `release`, `private_data`, `library_marker_id`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(ctx: Arc<TaskContext>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.execution.task_ctx.FFI_TaskContext.md).


A stable struct for sharing [`TaskContext`] across FFI boundaries.

---
