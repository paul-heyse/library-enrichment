# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::PlanWithCorrespondingSort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.PlanWithCorrespondingSort.json).

<a id="op-30dc08ebe58f1c7ad2e0a9f3"></a>
## PlanWithCorrespondingSort

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::PlanWithCorrespondingSort` · datafusion-physical-optimizer 55.1.0

```rust
type PlanWithCorrespondingSort = datafusion_physical_plan::tree_node::PlanContext<bool>
```

Source: `src/ensure_requirements/enforce_sorting/mod.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Context object used by sort enforcement to track the closest
[`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) descendant(s) for every child of a plan. The data attribute
stores whether the plan is a `SortExec` or is connected to a `SortExec`
via its children.
