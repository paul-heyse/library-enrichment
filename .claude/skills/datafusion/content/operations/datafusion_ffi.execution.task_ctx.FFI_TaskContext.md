# `datafusion_ffi::execution::task_ctx::FFI_TaskContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.execution.task_ctx.FFI_TaskContext.json).

<a id="op-e8a461d199067548927531f9"></a>
## FFI_TaskContext

`struct` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext` · datafusion-ffi 55.1.0

```rust
struct FFI_TaskContext
```

Source: `src/execution/task_ctx.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) across FFI boundaries.

<a id="op-abe93c942806d6b8c4a67ed4"></a>
## aggregate_functions

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::aggregate_functions` · datafusion-ffi 55.1.0

```rust
aggregate_functions: unsafe fn(&Self) -> stabby::vec::Vec<(stabby::string::String, udaf::FFI_AggregateUDF)>
```

Source: `src/execution/task_ctx.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Returns a vec of name-function pairs for aggregate functions.

<a id="op-d9f93a8f2ade814107c2283b"></a>
## drop

`function` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution::task_ctx::FFI_TaskContext", "path": "FFI_TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [164, 2], "filename": "src/execution/task_ctx.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/execution/task_ctx.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b594573d1b96c5991644012"></a>
## fmt

`function` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution::task_ctx::FFI_TaskContext", "path": "FFI_TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 10], "end": [40, 15], "filename": "src/execution/task_ctx.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution/task_ctx.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce932412ca870ab74b12829d"></a>
## from

`function` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::from` · datafusion-ffi 55.1.0

```rust
fn from(ctx: Arc<TaskContext>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution::task_ctx::FFI_TaskContext", "path": "FFI_TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [182, 2], "filename": "src/execution/task_ctx.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/execution/task_ctx.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ee6acc4f3246d818e08ec32"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/execution/task_ctx.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-76ab2d1b2fff9b5ac1982b2b"></a>
## private_data

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/execution/task_ctx.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
The foreign library should never attempt to access this data.

<a id="op-0e4cec341e300317fbd35b1c"></a>
## release

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/execution/task_ctx.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-b09722f89f4777c30d62ec28"></a>
## scalar_functions

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::scalar_functions` · datafusion-ffi 55.1.0

```rust
scalar_functions: unsafe fn(&Self) -> stabby::vec::Vec<(stabby::string::String, udf::FFI_ScalarUDF)>
```

Source: `src/execution/task_ctx.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Returns a vec of name-function pairs for scalar functions.

<a id="op-29eb896830a5123fd13062a2"></a>
## session_config

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::session_config` · datafusion-ffi 55.1.0

```rust
session_config: unsafe fn(&Self) -> session::config::FFI_SessionConfig
```

Source: `src/execution/task_ctx.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the session configuration.

<a id="op-a47e3395deaaf438c3cc386a"></a>
## session_id

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::session_id` · datafusion-ffi 55.1.0

```rust
session_id: unsafe fn(&Self) -> stabby::string::String
```

Source: `src/execution/task_ctx.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the session ID.

<a id="op-900250248945a33b32cb28d2"></a>
## task_id

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::task_id` · datafusion-ffi 55.1.0

```rust
task_id: unsafe fn(&Self) -> util::FFI_Option<stabby::string::String>
```

Source: `src/execution/task_ctx.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the task ID.

<a id="op-c8bb4ffea3ac8aecf1a99053"></a>
## window_functions

`struct_field` · `datafusion_ffi::execution::task_ctx::FFI_TaskContext::window_functions` · datafusion-ffi 55.1.0

```rust
window_functions: unsafe fn(&Self) -> stabby::vec::Vec<(stabby::string::String, udwf::FFI_WindowUDF)>
```

Source: `src/execution/task_ctx.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Returns a vec of name-function pairs for window functions.
