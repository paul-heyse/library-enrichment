# `datafusion_physical_plan::execution_plan::reset_plan_states`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.reset_plan_states.json).

<a id="op-00512b9a8a9ffd967f88b79d"></a>
## reset_plan_states

`function` · `datafusion_physical_plan::execution_plan::reset_plan_states` · datafusion-physical-plan 55.1.0

```rust
fn reset_plan_states(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:1976`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Make plan ready to be re-executed returning its clone with state reset for all nodes.

Some plans will change their internal states after execution, making them unable to be executed again.
This function uses [`ExecutionPlan::reset_state`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-5c58755b3547082b3ec956bd) to reset any internal state within the plan.

An example is `CrossJoinExec`, which loads the left table into memory and stores it in the plan.
However, if the data of the left table is derived from the work table, it will become outdated
as the work table changes. When the next iteration executes this plan again, we must clear the left table.

# Limitations

While this function enables plan reuse, it does not allow the same plan to be executed if it (OR):

* uses dynamic filters,
* represents a recursive query.

