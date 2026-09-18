# `datafusion_physical_optimizer::utils::is_union`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.is_union.json).

<a id="op-c1065ee5fa837dc21ef1d5cb"></a>
## is_union

`function` · `datafusion_physical_optimizer::utils::is_union` · datafusion-physical-optimizer 55.1.0

```rust
fn is_union(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Source: `src/utils.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Checks whether the given operator is a [`UnionExec`](../operations/datafusion_physical_plan.union.UnionExec.md#op-480e5ca85debacca2ee29b84).
