# `datafusion_physical_plan::projection::make_with_child`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.make_with_child.json).

<a id="op-efa3d58bb86ae96bba2e0aff"></a>
## make_with_child

`function` · `datafusion_physical_plan::projection::make_with_child` · datafusion-physical-plan 55.1.0

```rust
fn make_with_child(projection: &ProjectionExec, child: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/projection.rs:1051`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a new [`ProjectionExec`](../operations/datafusion_physical_plan.projection.ProjectionExec.md#op-b46d9dc006ec8aae1caad158) instance with the given child plan and
projected expressions, preserving the original output metadata.
