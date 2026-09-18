# `datafusion_physical_optimizer::utils::is_repartition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.is_repartition.json).

<a id="op-c7bc3db75a21f4a797582d2c"></a>
## is_repartition

`function` · `datafusion_physical_optimizer::utils::is_repartition` · datafusion-physical-optimizer 55.1.0

```rust
fn is_repartition(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Source: `src/utils.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Checks whether the given operator is a [`RepartitionExec`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-a0cd55fc5d1d094b433c884b).
