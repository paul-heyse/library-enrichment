# `datafusion_physical_plan::execution_plan::get_plan_string`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.get_plan_string.json).

<a id="op-d2d21fb19f93f66f827f2a63"></a>
## get_plan_string

`function` · `datafusion_physical_plan::execution_plan::get_plan_string` · datafusion-physical-plan 55.1.0

```rust
fn get_plan_string(plan: &std::sync::Arc<dyn ExecutionPlan>) -> Vec<String>
```

Source: `src/execution_plan.rs:2044`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Utility function yielding a string representation of the given [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673).
