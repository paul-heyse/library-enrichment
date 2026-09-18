# `datafusion_ffi::query_planner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.query_planner.json).

<a id="op-3caae2f40567c01aae388544"></a>
## query_planner

`module` · `datafusion_ffi::query_planner` · datafusion-ffi 55.1.0

```rust
mod query_planner
```

Source: `src/query_planner.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI support for [`QueryPlanner`](../operations/datafusion_session.planner.QueryPlanner.md#op-d105e63a68841dd69dcdcc42).

A typical deployment has three libraries. Library A (for example,
`datafusion-python`) owns the [`Session`](../operations/datafusion_session.session.Session.md#op-75302dfa669885e17a5c9093) and codec registry. Library B owns
a custom table provider and its extension nodes. Library C (for example,
Ballista or `datafusion-distributed`) owns the query planner. A serializes a
logical plan and invokes C, while `FFI_SessionRef` lets C call session
services in A. C deserializes the logical plan, creates a physical plan,
serializes that result, and returns it for A to deserialize. The logical and
physical extension codecs preserve nodes supplied by B.

The physical result is serialized instead of returned as an
[`crate::execution_plan::FFI_ExecutionPlan`](../operations/datafusion_ffi.execution_plan.FFI_ExecutionPlan.md#op-9600304f9dbd2887afcc0a23). An FFI execution-plan handle is
a foreign trait-object proxy, so even a built-in plan created in C cannot be
downcast to its concrete
type in A. Serialization reconstructs known plan nodes with A's local Rust
type identities, allowing A's optimizers and other consumers to downcast
them. Extension codecs control how custom nodes are reconstructed.

A node returned by B while C is planning is still foreign to C unless a
codec boundary reconstructs it in C. The query-planner boundary guarantees
that C-local serializable nodes, and extension nodes understood by the
configured codecs, are reconstructed for A when the completed plan returns.

# Delegating back to library A

C commonly wants A's built-in planning as a starting point, then rewrites the
result. A must export its planner *before* installing C's planner on the
session, and C must retain that handle: after the swap,
[`Session::query_planner`](../operations/datafusion_session.session.Session.md#op-d9a5770e1d1190279fe94203) reports C's own planner, and
[`Session::create_physical_plan`](../operations/datafusion_session.session.Session.md#op-f8a00c1a238fc9102cc1e8b7) dispatches to it, so either one is a
self-call. Delegating to the retained handle is safe, because DataFusion's
built-in physical planner never re-dispatches through [`Session`](../operations/datafusion_session.session.Session.md#op-75302dfa669885e17a5c9093).

Retain the planner rather than the session. [`FFI_QueryPlanner`](../operations/datafusion_ffi.query_planner.FFI_QueryPlanner.md#op-3c95c5a8ef88787dcecb3bd4) owns a
reference-counted planner, so it outlives A's original session, whereas
`FFI_SessionRef` borrows its session with the lifetime erased.
