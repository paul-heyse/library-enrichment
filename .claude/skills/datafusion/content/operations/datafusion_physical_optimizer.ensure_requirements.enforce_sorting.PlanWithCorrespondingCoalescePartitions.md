# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::PlanWithCorrespondingCoalescePartitions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.PlanWithCorrespondingCoalescePartitions.json).

<a id="op-535a110feded1234e2ebc3a3"></a>
## PlanWithCorrespondingCoalescePartitions

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::PlanWithCorrespondingCoalescePartitions` · datafusion-physical-optimizer 55.1.0

```rust
type PlanWithCorrespondingCoalescePartitions = datafusion_physical_plan::tree_node::PlanContext<bool>
```

Source: `src/ensure_requirements/enforce_sorting/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Tracks the closest
[`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f) descendant(s) for every child of a plan. The data
attribute stores whether the plan is a `CoalescePartitionsExec` or is
connected to a `CoalescePartitionsExec` via its children.

The tracker halts at each [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) (where the SPM will act to replace the coalesce).

This requires a bottom-up traversal was previously performed, updating the
children previously.
