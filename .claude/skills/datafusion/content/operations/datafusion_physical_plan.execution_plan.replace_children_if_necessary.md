# `datafusion_physical_plan::execution_plan::replace_children_if_necessary`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.replace_children_if_necessary.json).

<a id="op-39321a5fcf4c72a03a82abd3"></a>
## replace_children_if_necessary

`function` · `datafusion_physical_plan::execution_plan::replace_children_if_necessary` · datafusion-physical-plan 55.1.0

```rust
fn replace_children_if_necessary(plan: std::sync::Arc<dyn ExecutionPlan>, children: Vec<std::sync::Arc<dyn ExecutionPlan>>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:1700`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a plan with the given children, skipping as much work as possible.

This helper is the single entry point for "rebuild a plan from new
children" and applies three layers of short-circuits, from cheapest to
most expensive:

1. **Same child pointers** — if every `children[i]` is `Arc::ptr_eq` to the
   corresponding existing child, the original `plan` is returned
   unchanged (no allocation, no [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22)
   call).
2. **Same child properties** — if the children's `PlanProperties` Arcs
   match (via [`has_same_children_properties`](../operations/datafusion_physical_plan.execution_plan.has_same_children_properties.md#op-8acc5d0710e216f47bd26239)), the plan's own
   `PlanProperties` cache can be reused. This calls
   [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22) with [`ChildrenPropertiesMode::Keep`](../operations/datafusion_physical_plan.execution_plan.ChildrenPropertiesMode.md#op-e31e5a70039a4fa83b73b6e1),
   which swaps the child pointers without recomputing `PlanProperties`.
3. **Full recompute** — otherwise, delegate to
   [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22) with [`ChildrenPropertiesMode::Recompute`](../operations/datafusion_physical_plan.execution_plan.ChildrenPropertiesMode.md#op-449a09e8483cda87bd8b69c9),
   which recomputes `PlanProperties` from scratch.

The size of `children` must be equal to the size of `ExecutionPlan::children()`.
