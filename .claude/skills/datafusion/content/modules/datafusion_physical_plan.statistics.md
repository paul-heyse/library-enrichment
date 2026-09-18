# `datafusion_physical_plan::statistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.statistics.json).

<a id="op-1096461bedc2dc761d455bbb"></a>
## statistics

`module` · `datafusion_physical_plan::statistics` · datafusion-physical-plan 55.1.0

```rust
mod statistics
```

Source: `src/statistics.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics computation for physical plans.

[`StatisticsArgs`](../operations/datafusion_physical_plan.statistics.StatisticsArgs.md#op-33712b0db80fc4a46659cb66) provides external context to
[`ExecutionPlan::statistics_from_inputs`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-7dad80ba3b610abc3c532d8a).
