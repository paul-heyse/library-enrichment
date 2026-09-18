# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_partial_sort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_partial_sort.json).

<a id="op-baf0b504c9a9d15e2c1fb1e9"></a>
## replace_with_partial_sort

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_partial_sort` · datafusion-physical-optimizer 55.1.0

```rust
fn replace_with_partial_sort(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/ensure_requirements/enforce_sorting/mod.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Only interested with [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af)s and their unbounded children.
If the plan is not a [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) or its child is not unbounded, returns the original plan.
Otherwise, by checking the requirement satisfaction searches for a replacement chance.
If there's one replaces the [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) plan with a [`PartialSortExec`](../operations/datafusion_physical_plan.sorts.partial_sort.PartialSortExec.md#op-0ac508bb4f53820d3d249464)
