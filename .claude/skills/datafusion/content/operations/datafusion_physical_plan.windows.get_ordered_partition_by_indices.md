# `datafusion_physical_plan::windows::get_ordered_partition_by_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.get_ordered_partition_by_indices.json).

<a id="op-4216f98aa0c7bcb4c8246446"></a>
## get_ordered_partition_by_indices

`function` · `datafusion_physical_plan::windows::get_ordered_partition_by_indices` · datafusion-physical-plan 55.1.0

```rust
fn get_ordered_partition_by_indices(partition_by_exprs: &[std::sync::Arc<dyn PhysicalExpr>], input: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<Vec<usize>>
```

Source: `src/windows/mod.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

This function calculates the indices such that when partition by expressions reordered with the indices
resulting expressions define a preset for existing ordering.
For instance, if input is ordered by a, b, c and PARTITION BY b, a is used,
this vector will be [1, 0]. It means that when we iterate b, a columns with the order [1, 0]
resulting vector (a, b) is a preset of the existing ordering (a, b, c).
