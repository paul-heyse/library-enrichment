# `datafusion_physical_optimizer::utils`

Crate `datafusion-physical-optimizer` · 10 public items · structured records in [`model/datafusion_physical_optimizer.utils.json`](../model/datafusion_physical_optimizer.utils.json)

## add_sort_above

`function` · `datafusion_physical_optimizer::utils::add_sort_above`

```rust
fn add_sort_above<T: Clone + Default>(node: datafusion_physical_plan::tree_node::PlanContext<T>, sort_requirements: datafusion_physical_expr::LexRequirement, fetch: Option<usize>) -> datafusion_physical_plan::tree_node::PlanContext<T>
```

This utility function adds a `SortExec` above an operator according to the
given ordering requirements while preserving the original partitioning.

Note that this updates the plan in both the `PlanContext.children` and
the `PlanContext.plan`'s children. Therefore its not required to sync
the child plans with [`PlanContext::update_plan_from_children`].

---

## add_sort_above_with_check

`function` · `datafusion_physical_optimizer::utils::add_sort_above_with_check`

```rust
fn add_sort_above_with_check<T: Clone + Default>(node: datafusion_physical_plan::tree_node::PlanContext<T>, sort_requirements: datafusion_physical_expr::LexRequirement, fetch: Option<usize>) -> datafusion_common::Result<datafusion_physical_plan::tree_node::PlanContext<T>>
```

This utility function adds a `SortExec` above an operator according to the
given ordering requirements while preserving the original partitioning. If
requirement is already satisfied no `SortExec` is added.

---

## add_sort_above_with_distribution

`function` · `datafusion_physical_optimizer::utils::add_sort_above_with_distribution`

```rust
fn add_sort_above_with_distribution<T: Clone + Default>(node: datafusion_physical_plan::tree_node::PlanContext<T>, sort_requirements: datafusion_physical_expr::LexRequirement, fetch: Option<usize>, required_distribution: &datafusion_physical_expr::Distribution) -> datafusion_physical_plan::tree_node::PlanContext<T>
```

Like [`add_sort_above`], but also inserts a [`SortPreservingMergeExec`] when
the parent distribution requires a single partition and the input has
multiple partitions. This prevents `SortExec(preserve_partitioning=true)`
from violating `SinglePartition` requirements.

---

## is_coalesce_partitions

`function` · `datafusion_physical_optimizer::utils::is_coalesce_partitions`

```rust
fn is_coalesce_partitions(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Checks whether the given operator is a [`CoalescePartitionsExec`].

---

## is_limit

`function` · `datafusion_physical_optimizer::utils::is_limit`

```rust
fn is_limit(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Checks whether the given operator is a limit;
i.e. either a [`LocalLimitExec`] or a [`GlobalLimitExec`].

---

## is_repartition

`function` · `datafusion_physical_optimizer::utils::is_repartition`

```rust
fn is_repartition(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Checks whether the given operator is a [`RepartitionExec`].

---

## is_sort

`function` · `datafusion_physical_optimizer::utils::is_sort`

```rust
fn is_sort(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Checks whether the given operator is a [`SortExec`].

---

## is_sort_preserving_merge

`function` · `datafusion_physical_optimizer::utils::is_sort_preserving_merge`

```rust
fn is_sort_preserving_merge(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Checks whether the given operator is a [`SortPreservingMergeExec`].

---

## is_union

`function` · `datafusion_physical_optimizer::utils::is_union`

```rust
fn is_union(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Checks whether the given operator is a [`UnionExec`].

---

## is_window

`function` · `datafusion_physical_optimizer::utils::is_window`

```rust
fn is_window(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Checks whether the given operator is a window;
i.e. either a [`WindowAggExec`] or a [`BoundedWindowAggExec`].

---
