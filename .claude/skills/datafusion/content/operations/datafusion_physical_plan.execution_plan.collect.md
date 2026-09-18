# `datafusion_physical_plan::execution_plan::collect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.collect.json).

<a id="op-037eade84bbecff723ca3134"></a>
## collect

`function` · `datafusion_physical_plan::execution_plan::collect` · datafusion-physical-plan 55.1.0

```rust
async fn collect(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<Vec<arrow::array::RecordBatch>>
```

Source: `src/execution_plan.rs:1752`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Execute the [ExecutionPlan](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) and collect the results in memory
