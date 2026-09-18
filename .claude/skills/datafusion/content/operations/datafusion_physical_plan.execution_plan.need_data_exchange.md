# `datafusion_physical_plan::execution_plan::need_data_exchange`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.need_data_exchange.json).

<a id="op-28cb3829cb8743ddf612599d"></a>
## need_data_exchange

`function` · `datafusion_physical_plan::execution_plan::need_data_exchange` · datafusion-physical-plan 55.1.0

```rust
fn need_data_exchange(plan: std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Source: `src/execution_plan.rs:1662`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Indicate whether a data exchange is needed for the input of `plan`.

This identifies physical operators that redistribute child partitions or
gather multiple child partitions into one output partition:

1. RepartitionExec for non-round-robin repartitioning
2. CoalescePartitionsExec for collapsing multiple partitions into one without ordering guarantee
3. SortPreservingMergeExec for collapsing multiple sorted partitions into one with ordering guarantee
