# `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.execution.task_ctx_provider.FFI_TaskContextProvider.json).

<a id="op-b9c68e1bf49ed415cc4af800"></a>
## FFI_TaskContextProvider

`struct` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider` · datafusion-ffi 55.1.0

```rust
struct FFI_TaskContextProvider
```

Source: `src/execution/task_ctx_provider.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Struct for accessing the [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0). This method contains a weak
reference, so there are no guarantees that the [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) remains
valid. This is used primarily for protobuf encoding and decoding of
data passed across the FFI boundary. See the crate README for
additional information.

<a id="op-60ea539e75db8bb5a7872755"></a>
## clone

`function` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider", "path": "FFI_TaskContextProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [127, 2], "filename": "src/execution/task_ctx_provider.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution/task_ctx_provider.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7badad91c177be5aa491889"></a>
## clone

`struct_field` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/execution/task_ctx_provider.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the task context accessor. This should
only need to be called by the receiver of the plan.

<a id="op-cd23d15d1d6e7a771f8538b9"></a>
## drop

`function` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider", "path": "FFI_TaskContextProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [121, 2], "filename": "src/execution/task_ctx_provider.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/execution/task_ctx_provider.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a01dd420a6265f2b24f6753"></a>
## fmt

`function` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider", "path": "FFI_TaskContextProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/execution/task_ctx_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution/task_ctx_provider.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1422edfb6a0dfaf468a66098"></a>
## from

`function` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::from` · datafusion-ffi 55.1.0

```rust
fn from(ctx: &Arc<dyn TaskContextProvider>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider", "path": "FFI_TaskContextProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [142, 2], "filename": "src/execution/task_ctx_provider.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_execution::task::TaskContextProvider", "path": "TaskContextProvider"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/execution/task_ctx_provider.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a35a287d7623462d01c4701a"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/execution/task_ctx_provider.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-21927c24349d3db9b1ed63da"></a>
## private_data

`struct_field` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/execution/task_ctx_provider.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
The foreign library should never attempt to access this data.

<a id="op-51a9e3bfa911c0ef05aeb683"></a>
## release

`struct_field` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/execution/task_ctx_provider.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-fd8fd9544cf62e3d15b81b70"></a>
## task_ctx

`struct_field` · `datafusion_ffi::execution::task_ctx_provider::FFI_TaskContextProvider::task_ctx` · datafusion-ffi 55.1.0

```rust
task_ctx: unsafe fn(&Self) -> util::FFI_Result<execution::task_ctx::FFI_TaskContext>
```

Source: `src/execution/task_ctx_provider.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Retrieve the current [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) provided the provider has not
gone out of scope. This function will return an error if the weakly
held reference to the underlying [`TaskContextProvider`](../operations/datafusion_execution.task.TaskContextProvider.md#op-97bb1649c1167f947b80a8ae) is no longer
available.
