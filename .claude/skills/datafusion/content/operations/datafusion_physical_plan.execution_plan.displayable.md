# `datafusion_physical_plan::execution_plan::displayable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.displayable.json).

<a id="op-727f9d5db0c6bca1a2d77add"></a>
## displayable

`function` · `datafusion_physical_plan::execution_plan::displayable` · datafusion-physical-plan 55.1.0

```rust
fn displayable(plan: &dyn ExecutionPlan) -> display::DisplayableExecutionPlan<'_>
```

Source: `src/execution_plan.rs:1747`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a [`DisplayableExecutionPlan`](../operations/datafusion_physical_plan.display.DisplayableExecutionPlan.md#op-ff5dd0052d13e66e5b983e42) wrapper around an
[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) which can be displayed in various easier to
understand ways.

See examples on [`DisplayableExecutionPlan`](../operations/datafusion_physical_plan.display.DisplayableExecutionPlan.md#op-ff5dd0052d13e66e5b983e42)
