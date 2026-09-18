# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.json).

<a id="op-be054b334dcb08542b112e47"></a>
## enforce_sorting

`module` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting` · datafusion-physical-optimizer 55.1.0

```rust
mod enforce_sorting
```

Source: `src/ensure_requirements/enforce_sorting/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Sort enforcement helpers. The standalone `EnforceSorting` rule that
previously lived here has been retired in favour of `EnsureRequirements`
(which composes distribution and sorting enforcement into a single
idempotent pass). The helpers in this module — `ensure_sorting`,
`parallelize_sorts`, `PlanWithCorrespondingSort`, and the submodules
`replace_with_order_preserving_variants` and `sort_pushdown` — are
used directly by `EnsureRequirements`.

Sort enforcement inspects the physical plan with respect to local
sorting requirements and does the following:
- Adds a [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) when a requirement is not met,
- Removes an already-existing [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) if it is possible to prove
  that this sort is unnecessary

The helpers can work on valid *and* invalid physical plans with respect
to sorting requirements, but always produce a valid plan in this sense.

A non-realistic but easy to follow example for sort removals: assume the
fragment

```text
SortExec: expr=[nullable_col@0 ASC]
  SortExec: expr=[non_nullable_col@1 ASC]
```

reaches this stage. The first sort is unnecessary since its result is
overwritten by another [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af), so it is removed.
