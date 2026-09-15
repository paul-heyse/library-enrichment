# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting`

Crate `datafusion-physical-optimizer` · 5 public items · structured records in [`model/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.json`](../model/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.json)

## ensure_sorting

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::ensure_sorting`

Also reachable as `datafusion_physical_optimizer::enforce_sorting::ensure_sorting`

```rust
fn ensure_sorting(requirements: PlanWithCorrespondingSort) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<PlanWithCorrespondingSort>>
```

This function enforces sorting requirements and makes optimizations without
violating these requirements whenever possible. Requires a bottom-up traversal.

**Steps**
1. Analyze if there are any immediate removals of [`SortExec`]s. If so,
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
       unnecessary [`SortExec`]s (see `adjust_window_sort_removal`).
4. Check and remove possibly unnecessary SPM:
      -  Checks if the plan is SPM and child 1 output partitions, if so
         decides this SPM is unnecessary and removes it from the plan.

---

## parallelize_sorts

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::parallelize_sorts`

Also reachable as `datafusion_physical_optimizer::enforce_sorting::parallelize_sorts`

```rust
fn parallelize_sorts(requirements: PlanWithCorrespondingCoalescePartitions) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<PlanWithCorrespondingCoalescePartitions>>
```

Transform [`CoalescePartitionsExec`] + [`SortExec`] cascades into [`SortExec`]
+ [`SortPreservingMergeExec`] cascades, as illustrated below.

A [`CoalescePartitionsExec`] + [`SortExec`] cascade combines partitions
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


A [`SortExec`] + [`SortPreservingMergeExec`] cascade sorts each partition
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

The latter [`SortExec`] + [`SortPreservingMergeExec`] cascade performs
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
by following connections from [`CoalescePartitionsExec`]s to [`SortExec`]s.
By performing sorting in parallel, we can increase performance in some
scenarios.

This optimization requires that there are no nodes between the [`SortExec`]
and the [`CoalescePartitionsExec`], which requires single partitioning. Do
not parallelize when the following scenario occurs:
```text
     "SortExec: expr=\[a@0 ASC\]",
     "  ...nodes requiring single partitioning..."
     "    CoalescePartitionsExec",
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```

**Steps**
1. Checks if the plan is either a [`SortExec`], a [`SortPreservingMergeExec`],
   or a [`CoalescePartitionsExec`]. Otherwise, does nothing.
2. If the plan is a [`SortExec`] or a final [`SortPreservingMergeExec`]
   (i.e. output partitioning is 1):
     - Check for [`CoalescePartitionsExec`] in children. If found, check if
       it can be removed (with possible [`RepartitionExec`]s). If so, remove
       (see `remove_bottleneck_in_subplan`).
     - If the plan is satisfying the ordering requirements, add a `SortExec`.
     - Add an SPM above the plan and return.
3. If the plan is a [`CoalescePartitionsExec`]:
     - Check if it can be removed (with possible [`RepartitionExec`]s).
       If so, remove (see `remove_bottleneck_in_subplan`).

---

## replace_with_partial_sort

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_partial_sort`

Also reachable as `datafusion_physical_optimizer::enforce_sorting::replace_with_partial_sort`

```rust
fn replace_with_partial_sort(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Only interested with [`SortExec`]s and their unbounded children.
If the plan is not a [`SortExec`] or its child is not unbounded, returns the original plan.
Otherwise, by checking the requirement satisfaction searches for a replacement chance.
If there's one replaces the [`SortExec`] plan with a [`PartialSortExec`]

---

## PlanWithCorrespondingCoalescePartitions

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::PlanWithCorrespondingCoalescePartitions`

Also reachable as `datafusion_physical_optimizer::enforce_sorting::PlanWithCorrespondingCoalescePartitions`

```rust
type PlanWithCorrespondingCoalescePartitions = datafusion_physical_plan::tree_node::PlanContext<bool>
```

Tracks the closest
[`CoalescePartitionsExec`] descendant(s) for every child of a plan. The data
attribute stores whether the plan is a `CoalescePartitionsExec` or is
connected to a `CoalescePartitionsExec` via its children.

The tracker halts at each [`SortExec`] (where the SPM will act to replace the coalesce).

This requires a bottom-up traversal was previously performed, updating the
children previously.

---

## PlanWithCorrespondingSort

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::PlanWithCorrespondingSort`

Also reachable as `datafusion_physical_optimizer::enforce_sorting::PlanWithCorrespondingSort`

```rust
type PlanWithCorrespondingSort = datafusion_physical_plan::tree_node::PlanContext<bool>
```

Context object used by sort enforcement to track the closest
[`SortExec`] descendant(s) for every child of a plan. The data attribute
stores whether the plan is a `SortExec` or is connected to a `SortExec`
via its children.

---
