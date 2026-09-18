# `datafusion_physical_optimizer::utils::is_coalesce_partitions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.is_coalesce_partitions.json).

<a id="op-18967b791a1a47896058ea1d"></a>
## is_coalesce_partitions

`function` · `datafusion_physical_optimizer::utils::is_coalesce_partitions` · datafusion-physical-optimizer 55.1.0

```rust
fn is_coalesce_partitions(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Source: `src/utils.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Checks whether the given operator is a [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f).
