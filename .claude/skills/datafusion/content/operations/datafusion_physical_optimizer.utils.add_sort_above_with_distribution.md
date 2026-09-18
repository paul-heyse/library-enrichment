# `datafusion_physical_optimizer::utils::add_sort_above_with_distribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.add_sort_above_with_distribution.json).

<a id="op-f2296b76f2cc45198398b07b"></a>
## add_sort_above_with_distribution

`function` · `datafusion_physical_optimizer::utils::add_sort_above_with_distribution` · datafusion-physical-optimizer 55.1.0

```rust
fn add_sort_above_with_distribution<T: Clone + Default>(node: datafusion_physical_plan::tree_node::PlanContext<T>, sort_requirements: datafusion_physical_expr::LexRequirement, fetch: Option<usize>, required_distribution: &datafusion_physical_expr::Distribution) -> datafusion_physical_plan::tree_node::PlanContext<T>
```

Source: `src/utils.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Like [`add_sort_above`](../operations/datafusion_physical_optimizer.utils.add_sort_above.md#op-d9180ab19a40fb3ad11aef88), but also inserts a [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c) when
the parent distribution requires a single partition and the input has
multiple partitions. This prevents `SortExec(preserve_partitioning=true)`
from violating `SinglePartition` requirements.
