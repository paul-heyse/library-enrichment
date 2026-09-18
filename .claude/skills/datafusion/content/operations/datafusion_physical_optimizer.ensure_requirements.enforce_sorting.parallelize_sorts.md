# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::parallelize_sorts`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.parallelize_sorts.json).

<a id="op-9c4ca4c7ad0f37f542abdf27"></a>
## parallelize_sorts

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::parallelize_sorts` · datafusion-physical-optimizer 55.1.0

```rust
fn parallelize_sorts(requirements: PlanWithCorrespondingCoalescePartitions) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<PlanWithCorrespondingCoalescePartitions>>
```

Source: `src/ensure_requirements/enforce_sorting/mod.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Transform [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f) + [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) cascades into [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af)
+ [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c) cascades, as illustrated below.

A [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f) + [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) cascade combines partitions
first, and then sorts:
```text
  ┌ ─ ─ ─ ─ ─ ┐
   ┌─┬─┬─┐
  ││B│A│D│... ├──┐
   └─┴─┴─┘       │
  └ ─ ─ ─ ─ ─ ┘  │  ┌────────────────────────┐   ┌ ─ ─ ─ ─ ─ ─ ┐   ┌────────┐    ┌ ─ ─ ─ ─ ─ ─ ─ ┐
   Partition 1   │  │        Coalesce        │    ┌─┬─┬─┬─┬─┐      │        │     ┌─┬─┬─┬─┬─┐
                 ├──▶(no ordering guarantees)│──▶││B│E│A│D│C│...───▶  Sort  ├───▶││A│B│C│D│E│... │
                 │  │                        │    └─┴─┴─┴─┴─┘      │        │     └─┴─┴─┴─┴─┘
  ┌ ─ ─ ─ ─ ─ ┐  │  └────────────────────────┘   └ ─ ─ ─ ─ ─ ─ ┘   └────────┘    └ ─ ─ ─ ─ ─ ─ ─ ┘
   ┌─┬─┐         │                                 Partition                       Partition
  ││E│C│ ...  ├──┘
   └─┴─┘
  └ ─ ─ ─ ─ ─ ┘
   Partition 2
```


A [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) + [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c) cascade sorts each partition
first, then merges partitions while preserving the sort:
```text
  ┌ ─ ─ ─ ─ ─ ┐   ┌────────┐   ┌ ─ ─ ─ ─ ─ ┐
   ┌─┬─┬─┐        │        │    ┌─┬─┬─┐
  ││B│A│D│... │──▶│  Sort  │──▶││A│B│D│... │──┐
   └─┴─┴─┘        │        │    └─┴─┴─┘       │
  └ ─ ─ ─ ─ ─ ┘   └────────┘   └ ─ ─ ─ ─ ─ ┘  │  ┌─────────────────────┐    ┌ ─ ─ ─ ─ ─ ─ ─ ┐
   Partition 1                  Partition 1   │  │                     │     ┌─┬─┬─┬─┬─┐
                                              ├──▶ SortPreservingMerge ├───▶││A│B│C│D│E│... │
                                              │  │                     │     └─┴─┴─┴─┴─┘
  ┌ ─ ─ ─ ─ ─ ┐   ┌────────┐   ┌ ─ ─ ─ ─ ─ ┐  │  └─────────────────────┘    └ ─ ─ ─ ─ ─ ─ ─ ┘
   ┌─┬─┐          │        │    ┌─┬─┐         │                               Partition
  ││E│C│ ...  │──▶│  Sort  ├──▶││C│E│ ...  │──┘
   └─┴─┘          │        │    └─┴─┘
  └ ─ ─ ─ ─ ─ ┘   └────────┘   └ ─ ─ ─ ─ ─ ┘
   Partition 2                  Partition 2
```

The latter [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) + [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c) cascade performs
sorting first on a per-partition basis, thereby parallelizing the sort.

The outcome is that plans of the form
```text
     "SortExec: expr=\[a@0 ASC\]",
     "  ...nodes..."
     "    CoalescePartitionsExec",
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```
are transformed into
```text
     "SortPreservingMergeExec: \[a@0 ASC\]",
     "  SortExec: expr=\[a@0 ASC\]",
     "    ...nodes..."
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```
by following connections from [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f)s to [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af)s.
By performing sorting in parallel, we can increase performance in some
scenarios.

This optimization requires that there are no nodes between the [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af)
and the [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f), which requires single partitioning. Do
not parallelize when the following scenario occurs:
```text
     "SortExec: expr=\[a@0 ASC\]",
     "  ...nodes requiring single partitioning..."
     "    CoalescePartitionsExec",
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```

**Steps**
1. Checks if the plan is either a [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af), a [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c),
   or a [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f). Otherwise, does nothing.
2. If the plan is a [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) or a final [`SortPreservingMergeExec`](../operations/datafusion_physical_plan.sorts.sort_preserving_merge.SortPreservingMergeExec.md#op-e0ba0adb06eaa3e5277df91c)
   (i.e. output partitioning is 1):
     - Check for [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f) in children. If found, check if
       it can be removed (with possible [`RepartitionExec`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-a0cd55fc5d1d094b433c884b)s). If so, remove
       (see `remove_bottleneck_in_subplan`).
     - If the plan is satisfying the ordering requirements, add a `SortExec`.
     - Add an SPM above the plan and return.
3. If the plan is a [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f):
     - Check if it can be removed (with possible [`RepartitionExec`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-a0cd55fc5d1d094b433c884b)s).
       If so, remove (see `remove_bottleneck_in_subplan`).
