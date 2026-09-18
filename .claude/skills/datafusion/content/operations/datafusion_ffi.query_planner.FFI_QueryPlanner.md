# `datafusion_ffi::query_planner::FFI_QueryPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.query_planner.FFI_QueryPlanner.json).

<a id="op-3c95c5a8ef88787dcecb3bd4"></a>
## FFI_QueryPlanner

`struct` · `datafusion_ffi::query_planner::FFI_QueryPlanner` · datafusion-ffi 55.1.0

```rust
struct FFI_QueryPlanner
```

Source: `src/query_planner.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

An ABI-stable handle to a [`QueryPlanner`](../operations/datafusion_session.planner.QueryPlanner.md#op-d105e63a68841dd69dcdcc42) owned by another library.

The Rust-facing adapters serialize the input [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) and resulting
[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673); callers do not invoke the byte-oriented function pointer
directly.

<a id="op-614456e26838cf2d70629af4"></a>
## clone

`function` · `datafusion_ffi::query_planner::FFI_QueryPlanner::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::query_planner::FFI_QueryPlanner", "path": "FFI_QueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 1], "end": [217, 2], "filename": "src/query_planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/query_planner.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b99647d6faae9a21248956a5"></a>
## create_physical_plan_with_session_runtime

`function` · `datafusion_ffi::query_planner::FFI_QueryPlanner::create_physical_plan_with_session_runtime` · datafusion-ffi 55.1.0

```rust
async fn create_physical_plan_with_session_runtime(&self, logical_plan: &LogicalPlan, session: &dyn Session, session_runtime: Option<Handle>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::query_planner::FFI_QueryPlanner", "path": "FFI_QueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [320, 2], "filename": "src/query_planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/query_planner.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a physical plan through this planner's FFI interface.

This serializes `logical_plan`, exports `session` as an
`FFI_SessionRef`, invokes the planner's owning library, and
deserializes its physical-plan response. `session_runtime` is attached
to the exported session for callbacks that need its Tokio runtime.

The [`QueryPlanner`](../operations/datafusion_session.planner.QueryPlanner.md#op-d105e63a68841dd69dcdcc42) implementation for [`ForeignQueryPlanner`](../operations/datafusion_ffi.query_planner.ForeignQueryPlanner.md#op-7669e102126cfc063986e52c) cannot
obtain the session owner's runtime from the trait API, so it calls this
method with `None`. Embedders that own the runtime and need session
callbacks to enter it must call this method directly with `Some(handle)`.

<a id="op-d047a1dd8882ef775b5aa21c"></a>
## drop

`function` · `datafusion_ffi::query_planner::FFI_QueryPlanner::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::query_planner::FFI_QueryPlanner", "path": "FFI_QueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 1], "end": [211, 2], "filename": "src/query_planner.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/query_planner.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d156866379dc14865554b0d"></a>
## fmt

`function` · `datafusion_ffi::query_planner::FFI_QueryPlanner::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::query_planner::FFI_QueryPlanner", "path": "FFI_QueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 10], "end": [89, 15], "filename": "src/query_planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/query_planner.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfe8fce1b6af3f0ed64809ed"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::query_planner::FFI_QueryPlanner::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/query_planner.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc).

<a id="op-b24806d7083cb728d5f890f8"></a>
## new

`function` · `datafusion_ffi::query_planner::FFI_QueryPlanner::new` · datafusion-ffi 55.1.0

```rust
fn new(planner: Arc<dyn QueryPlanner + Send + Sync>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Arc<dyn LogicalExtensionCodec>, physical_codec: Arc<dyn PhysicalExtensionCodec>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::query_planner::FFI_QueryPlanner", "path": "FFI_QueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [320, 2], "filename": "src/query_planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/query_planner.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates an [`FFI_QueryPlanner`](../operations/datafusion_ffi.query_planner.FFI_QueryPlanner.md#op-3c95c5a8ef88787dcecb3bd4) with native extension codecs.

Both codecs are required so that the caller states which extension nodes
survive the boundary. Pass
[`DefaultLogicalExtensionCodec`](datafusion_proto::logical_plan::DefaultLogicalExtensionCodec)
and
[`DefaultPhysicalExtensionCodec`](datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec)
when no custom nodes are involved. `runtime` and `task_ctx_provider`
support codec callbacks across the FFI boundary.

<a id="op-64fe6f9e810ecf7f16e7e7d4"></a>
## new_with_ffi_codecs

`function` · `datafusion_ffi::query_planner::FFI_QueryPlanner::new_with_ffi_codecs` · datafusion-ffi 55.1.0

```rust
fn new_with_ffi_codecs(planner: Arc<dyn QueryPlanner + Send + Sync>, logical_codec: FFI_LogicalExtensionCodec, physical_codec: FFI_PhysicalExtensionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::query_planner::FFI_QueryPlanner", "path": "FFI_QueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [320, 2], "filename": "src/query_planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/query_planner.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates an [`FFI_QueryPlanner`](../operations/datafusion_ffi.query_planner.FFI_QueryPlanner.md#op-3c95c5a8ef88787dcecb3bd4) using prebuilt FFI extension codecs.

If `planner` is already foreign, this re-exports its original FFI handle
rather than adding another wrapper layer. The handle still adopts the
codecs supplied here, so they are never silently discarded.

<a id="op-12531584ce9943e82bc6e5a1"></a>
## version

`struct_field` · `datafusion_ffi::query_planner::FFI_QueryPlanner::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/query_planner.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this planner.
