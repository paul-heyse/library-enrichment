# `datafusion_physical_optimizer::utils::is_limit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.is_limit.json).

<a id="op-b3f79d8238ddffb4ee523de5"></a>
## is_limit

`function` · `datafusion_physical_optimizer::utils::is_limit` · datafusion-physical-optimizer 55.1.0

```rust
fn is_limit(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Source: `src/utils.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Checks whether the given operator is a limit;
i.e. either a [`LocalLimitExec`](../operations/datafusion_physical_plan.limit.LocalLimitExec.md#op-98fadda85e669dac5cf205a0) or a [`GlobalLimitExec`](../operations/datafusion_physical_plan.limit.GlobalLimitExec.md#op-e1b7f44642275634519186d0).
