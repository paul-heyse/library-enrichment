# `datafusion_physical_optimizer::utils::is_sort_preserving_merge`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.is_sort_preserving_merge.json).

<a id="op-284e7fef04e06b8570acd817"></a>
## is_sort_preserving_merge

`function` · `datafusion_physical_optimizer::utils::is_sort_preserving_merge` · datafusion-physical-optimizer 55.1.0

```rust
fn is_sort_preserving_merge(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Source: `src/utils.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Checks whether the given operator is a [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c).
