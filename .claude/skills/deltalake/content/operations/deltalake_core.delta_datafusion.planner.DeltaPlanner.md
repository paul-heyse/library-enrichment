# `deltalake_core::delta_datafusion::planner::DeltaPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.planner.DeltaPlanner.json).

<a id="op-f3fbfb5194bfa8cf820b3cc4"></a>
## DeltaPlanner

`struct` · `deltalake_core::delta_datafusion::planner::DeltaPlanner` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaPlanner
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs#L60).

Source: `crates/core/src/delta_datafusion/planner.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Deltaplanner

<a id="op-db0895dd25d5882f39c68010"></a>
## create_physical_plan

`function` · `deltalake_core::delta_datafusion::planner::DeltaPlanner::create_physical_plan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> DataFusionResult<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::planner::DeltaPlanner", "path": "DeltaPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [84, 2], "filename": "crates/core/src/delta_datafusion/planner.rs"}, "trait": {"args": null, "id": "datafusion_session::planner::QueryPlanner", "path": "QueryPlanner"}, "trait_path": "datafusion_session::planner::QueryPlanner"}`

Source: `crates/core/src/delta_datafusion/planner.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a4b4bf750de00ef4b10295e"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::planner::DeltaPlanner::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs#L59).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::planner::DeltaPlanner", "path": "DeltaPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 10], "end": [59, 15], "filename": "crates/core/src/delta_datafusion/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/planner.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13a2f92dac8bf697fe456a15"></a>
## new

`function` · `deltalake_core::delta_datafusion::planner::DeltaPlanner::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Arc<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::planner::DeltaPlanner", "path": "DeltaPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [70, 2], "filename": "crates/core/src/delta_datafusion/planner.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/planner.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the shared, lazily-initialized [`DeltaPlanner`](../operations/deltalake_core.delta_datafusion.planner.DeltaPlanner.md#op-f3fbfb5194bfa8cf820b3cc4) instance.

The planner is stateless, so a single cached instance is reused rather than
allocating a new one per query.
