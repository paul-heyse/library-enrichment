# `datafusion_physical_plan::execution_plan::has_same_children_properties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.has_same_children_properties.json).

<a id="op-8acc5d0710e216f47bd26239"></a>
## has_same_children_properties

`function` · `datafusion_physical_plan::execution_plan::has_same_children_properties` · datafusion-physical-plan 55.1.0

```rust
fn has_same_children_properties(plan: &dyn ExecutionPlan, children: &[std::sync::Arc<dyn ExecutionPlan>]) -> datafusion_common::Result<bool>
```

Source: `src/execution_plan.rs:1988`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Check if the `plan` children has the same properties as passed `children`.
In this case plan can avoid self properties re-computation when its children
replace is requested.
The size of `children` must be equal to the size of `ExecutionPlan::children()`.
