# `datafusion_ffi::query_planner`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.query_planner.json`](../model/datafusion_ffi.query_planner.json)

## FFI_QueryPlanner

`struct` · `datafusion_ffi::query_planner::FFI_QueryPlanner`

```rust
struct FFI_QueryPlanner
```

**Fields**: `version`, `library_marker_id`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**Methods** (3)

```rust
async fn create_physical_plan_with_session_runtime(&self, logical_plan: &LogicalPlan, session: &dyn Session, session_runtime: Option<Handle>) -> Result<Arc<dyn ExecutionPlan>>
fn new(planner: Arc<dyn QueryPlanner + Send + Sync>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Arc<dyn LogicalExtensionCodec>, physical_codec: Arc<dyn PhysicalExtensionCodec>) -> Self
fn new_with_ffi_codecs(planner: Arc<dyn QueryPlanner + Send + Sync>, logical_codec: FFI_LogicalExtensionCodec, physical_codec: FFI_PhysicalExtensionCodec) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

An ABI-stable handle to a [`QueryPlanner`] owned by another library.

The Rust-facing adapters serialize the input [`LogicalPlan`] and resulting
[`ExecutionPlan`]; callers do not invoke the byte-oriented function pointer
directly.

---

## ForeignQueryPlanner

`struct` · `datafusion_ffi::query_planner::ForeignQueryPlanner`

```rust
struct ForeignQueryPlanner
```

**Implements**: `datafusion_session::planner::QueryPlanner`

**Derives**: Debug, Send, Sync

**via `datafusion_session::planner::QueryPlanner`**

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

Consumer-side [`QueryPlanner`] adapter for an [`FFI_QueryPlanner`].

Calls serialize the logical plan, invoke the producing library, and
deserialize its physical-plan response.

---
