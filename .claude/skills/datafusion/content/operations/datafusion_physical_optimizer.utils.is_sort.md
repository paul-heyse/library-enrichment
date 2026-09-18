# `datafusion_physical_optimizer::utils::is_sort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.is_sort.json).

<a id="op-7317b057ee201a4edcf16e23"></a>
## is_sort

`function` · `datafusion_physical_optimizer::utils::is_sort` · datafusion-physical-optimizer 55.1.0

```rust
fn is_sort(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Source: `src/utils.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Checks whether the given operator is a [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af).
