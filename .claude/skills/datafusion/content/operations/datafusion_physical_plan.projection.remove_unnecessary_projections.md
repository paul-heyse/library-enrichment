# `datafusion_physical_plan::projection::remove_unnecessary_projections`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.remove_unnecessary_projections.json).

<a id="op-3a15e48a4a38032bc9b53811"></a>
## remove_unnecessary_projections

`function` · `datafusion_physical_plan::projection::remove_unnecessary_projections` · datafusion-physical-plan 55.1.0

```rust
fn remove_unnecessary_projections(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn ExecutionPlan>>>
```

Source: `src/projection.rs:979`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

This function checks if `plan` is a [`ProjectionExec`](../operations/datafusion_physical_plan.projection.ProjectionExec.md#op-b46d9dc006ec8aae1caad158), and inspects its
input(s) to test whether it can push `plan` under its input(s). This function
will operate on the entire tree and may ultimately remove `plan` entirely
by leveraging source providers with built-in projection capabilities.
