# `datafusion_ffi::query_planner::ForeignQueryPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.query_planner.ForeignQueryPlanner.json).

<a id="op-7669e102126cfc063986e52c"></a>
## ForeignQueryPlanner

`struct` · `datafusion_ffi::query_planner::ForeignQueryPlanner` · datafusion-ffi 55.1.0

```rust
struct ForeignQueryPlanner
```

Source: `src/query_planner.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Consumer-side [`QueryPlanner`](../operations/datafusion_session.planner.QueryPlanner.md#op-d105e63a68841dd69dcdcc42) adapter for an [`FFI_QueryPlanner`](../operations/datafusion_ffi.query_planner.FFI_QueryPlanner.md#op-3c95c5a8ef88787dcecb3bd4).

Calls serialize the logical plan, invoke the producing library, and
deserialize its physical-plan response.

<a id="op-213809b3526e5ab63104a520"></a>
## 0

`struct_field` · `datafusion_ffi::query_planner::ForeignQueryPlanner::0` · datafusion-ffi 55.1.0

```rust
0: FFI_QueryPlanner
```

Source: `src/query_planner.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5433290a694761d22bd1f62d"></a>
## create_physical_plan

`function` · `datafusion_ffi::query_planner::ForeignQueryPlanner::create_physical_plan` · datafusion-ffi 55.1.0

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::query_planner::ForeignQueryPlanner", "path": "ForeignQueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 1], "end": [353, 2], "filename": "src/query_planner.rs"}, "trait": {"args": null, "id": "datafusion_session::planner::QueryPlanner", "path": "QueryPlanner"}, "trait_path": "datafusion_session::planner::QueryPlanner"}`

Source: `src/query_planner.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-392c7c52486759ee01495c49"></a>
## fmt

`function` · `datafusion_ffi::query_planner::ForeignQueryPlanner::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::query_planner::ForeignQueryPlanner", "path": "ForeignQueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 10], "end": [326, 15], "filename": "src/query_planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/query_planner.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
