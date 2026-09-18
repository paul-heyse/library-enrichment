# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::ensure_sorting`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.ensure_sorting.json).

<a id="op-7ababc0a4912d72dc5cce0f5"></a>
## ensure_sorting

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::ensure_sorting` · datafusion-physical-optimizer 55.1.0

```rust
fn ensure_sorting(requirements: PlanWithCorrespondingSort) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<PlanWithCorrespondingSort>>
```

Source: `src/ensure_requirements/enforce_sorting/mod.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This function enforces sorting requirements and makes optimizations without
violating these requirements whenever possible. Requires a bottom-up traversal.

**Steps**
1. Analyze if there are any immediate removals of [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af)s. If so,
   removes them (see `analyze_immediate_sort_removal`).
2. For each child of the plan, if the plan requires an input ordering:
     - Checks if ordering is satisfied with the child. If not:
         - If the child has an output ordering, removes the unnecessary
           `SortExec`.
         - Adds sort above the child plan.
     - (Plan not requires input ordering)
         - Checks if the `SortExec` is neutralized in the plan. If so,
           removes it.
3. Check and modify window operator:
     - Checks if the plan is a window operator, and connected with a sort.
       If so, either tries to update the window definition or removes
       unnecessary [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af)s (see `adjust_window_sort_removal`).
4. Check and remove possibly unnecessary SPM:
      -  Checks if the plan is SPM and child 1 output partitions, if so
         decides this SPM is unnecessary and removes it from the plan.
