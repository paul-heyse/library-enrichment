# `deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.planner.DeltaExtensionPlanner.json).

<a id="op-32c3e02361da5819dc4a62c4"></a>
## DeltaExtensionPlanner

`struct` · `deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaExtensionPlanner
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs#L88).

Source: `crates/core/src/delta_datafusion/planner.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Extension [`PhysicalPlanner`](datafusion::physical_planner::PhysicalPlanner) that knows
how to lower delta-rs custom logical nodes into executable physical plans.

Unresolved upstream links (retained, not inferred): `datafusion::physical_planner::PhysicalPlanner`.

<a id="op-70fa4893ba4d9182cfdfac12"></a>
## new

`function` · `deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Arc<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs#L92).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner", "path": "DeltaExtensionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [95, 2], "filename": "crates/core/src/delta_datafusion/planner.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/planner.rs:92`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Construct a new extension planner.

<a id="op-64b596d9d2f1f478ea2abd6f"></a>
## plan_extension

`function` · `deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner::plan_extension` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn plan_extension(&self, planner: &dyn PhysicalPlanner, node: &dyn UserDefinedLogicalNode, logical_inputs: &[&LogicalPlan], physical_inputs: &[Arc<dyn ExecutionPlan>], session_state: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> DataFusionResult<Option<Arc<dyn ExecutionPlan>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs#L99).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner", "path": "DeltaExtensionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [125, 2], "filename": "crates/core/src/delta_datafusion/planner.rs"}, "trait": {"args": null, "id": "datafusion_session::planner::ExtensionPlanner", "path": "ExtensionPlanner"}, "trait_path": "datafusion_session::planner::ExtensionPlanner"}`

Source: `crates/core/src/delta_datafusion/planner.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
