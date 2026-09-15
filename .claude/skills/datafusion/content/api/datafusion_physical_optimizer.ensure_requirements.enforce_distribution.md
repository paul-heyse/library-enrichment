# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution`

Crate `datafusion-physical-optimizer` · 8 public items · structured records in [`model/datafusion_physical_optimizer.ensure_requirements.enforce_distribution.json`](../model/datafusion_physical_optimizer.ensure_requirements.enforce_distribution.json)

## adjust_input_keys_ordering

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::adjust_input_keys_ordering`

Also reachable as `datafusion_physical_optimizer::enforce_distribution::adjust_input_keys_ordering`

```rust
fn adjust_input_keys_ordering(requirements: PlanWithKeyRequirements) -> datafusion_common::error::Result<datafusion_common::tree_node::Transformed<PlanWithKeyRequirements>>
```

When the physical planner creates the Joins, the ordering of join keys is from the original query.
That might not match with the output partitioning of the join node's children
A Top-Down process will use this method to adjust children's output partitioning based on the parent key reordering requirements:

Example:
    TopJoin on (a, b, c)
        bottom left join on(b, a, c)
        bottom right join on(c, b, a)

 Will be adjusted to:
    TopJoin on (a, b, c)
        bottom left join on(a, b, c)
        bottom right join on(a, b, c)

Example:
    TopJoin on (a, b, c)
        Agg1 group by (b, a, c)
        Agg2 group by (c, b, a)

Will be adjusted to:
    TopJoin on (a, b, c)
         Projection(b, a, c)
            Agg1 group by (a, b, c)
         Projection(c, b, a)
            Agg2 group by (a, b, c)

Following is the explanation of the reordering process:

1) If the current plan is Partitioned HashJoin, SortMergeJoin, check whether the requirements can be satisfied by adjusting join keys ordering:
   Requirements can not be satisfied, clear the current requirements, generate new requirements(to pushdown) based on the current join keys, return the unchanged plan.
   Requirements is already satisfied, clear the current requirements, generate new requirements(to pushdown) based on the current join keys, return the unchanged plan.
   Requirements can be satisfied by adjusting keys ordering, clear the current requirements, generate new requirements(to pushdown) based on the adjusted join keys, return the changed plan.

2) If the current plan is Aggregation, check whether the requirements can be satisfied by adjusting group by keys ordering:
   Requirements can not be satisfied, clear all the requirements, return the unchanged plan.
   Requirements is already satisfied, clear all the requirements, return the unchanged plan.
   Requirements can be satisfied by adjusting keys ordering, clear all the requirements, return the changed plan.

3) If the current plan is RepartitionExec, CoalescePartitionsExec or WindowAggExec, clear all the requirements, return the unchanged plan
4) If the current plan is Projection, transform the requirements to the columns before the Projection and push down requirements
5) For other types of operators, by default, pushdown the parent requirements to children.

---

## ensure_distribution

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::ensure_distribution`

Also reachable as `datafusion_physical_optimizer::enforce_distribution::ensure_distribution`

```rust
fn ensure_distribution(dist_context: DistributionContext, config: &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<datafusion_common::tree_node::Transformed<DistributionContext>>
```

This function checks whether we need to add additional data exchange
operators to satisfy distribution requirements. Since this function
takes care of such requirements, we should avoid manually adding data
exchange operators in other places.

This function is intended to be used in a bottom up traversal, as it
can first repartition (or newly partition) at the datasources -- these
source partitions may be later repartitioned with additional data exchange operators.

---

## reorder_aggregate_keys

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::reorder_aggregate_keys`

Also reachable as `datafusion_physical_optimizer::enforce_distribution::reorder_aggregate_keys`

```rust
fn reorder_aggregate_keys(agg_node: PlanWithKeyRequirements, agg_exec: &datafusion_physical_plan::aggregates::AggregateExec) -> datafusion_common::error::Result<PlanWithKeyRequirements>
```

---

## reorder_join_keys_to_inputs

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::reorder_join_keys_to_inputs`

Also reachable as `datafusion_physical_optimizer::enforce_distribution::reorder_join_keys_to_inputs`

```rust
fn reorder_join_keys_to_inputs(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>
```

When the physical planner creates the Joins, the ordering of join keys is from the original query.
That might not match with the output partitioning of the join node's children
This method will try to change the ordering of the join keys to match with the
partitioning of the join nodes' children. If it can not match with both sides, it will try to
match with one, either the left side or the right side.

Example:
    TopJoin on (a, b, c)
        bottom left join on(b, a, c)
        bottom right join on(c, b, a)

 Will be adjusted to:
    TopJoin on (b, a, c)
        bottom left join on(b, a, c)
        bottom right join on(c, b, a)

Compared to the Top-Down reordering process, this Bottom-Up approach is much simpler, but might not reach a best result.
The Bottom-Up approach will be useful in future if we plan to support storage partition-wised Joins.
In that case, the datasources/tables might be pre-partitioned and we can't adjust the key ordering of the datasources
and then can't apply the Top-Down reordering process.

---

## reorder_partitioned_join_keys

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::reorder_partitioned_join_keys`

Also reachable as `datafusion_physical_optimizer::enforce_distribution::reorder_partitioned_join_keys`

```rust
fn reorder_partitioned_join_keys<F>(join_plan: PlanWithKeyRequirements, on: &[(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)], sort_options: &[arrow::compute::SortOptions], join_constructor: &F) -> datafusion_common::error::Result<PlanWithKeyRequirements> where F: Fn((Vec<(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)>, Vec<arrow::compute::SortOptions>)) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>
```

---

## replace_order_preserving_variants

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::replace_order_preserving_variants`

Also reachable as `datafusion_physical_optimizer::enforce_distribution::replace_order_preserving_variants`

```rust
fn replace_order_preserving_variants(context: DistributionContext) -> datafusion_common::error::Result<DistributionContext>
```

Updates the [`DistributionContext`] if preserving ordering while changing partitioning is not helpful or desirable.

Assume that following plan is given:
```text
"SortPreservingMergeExec: \[a@0 ASC]"
"  RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=10, preserve_order=true",
"    RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=2, preserve_order=true",
"      DataSourceExec: file_groups={2 groups: \[\[x], \[y]]}, projection=\[a, b, c, d, e], output_ordering=\[a@0 ASC], file_type=parquet",
```

This function converts plan above to the following:

```text
"CoalescePartitionsExec"
"  RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=10",
"    RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=2",
"      DataSourceExec: file_groups={2 groups: \[\[x], \[y]]}, projection=\[a, b, c, d, e], output_ordering=\[a@0 ASC], file_type=parquet",
```

---

## DistributionContext

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::DistributionContext`

Also reachable as `datafusion_physical_optimizer::enforce_distribution::DistributionContext`

```rust
type DistributionContext = datafusion_physical_plan::tree_node::PlanContext<bool>
```

Keeps track of distribution changing operators (like `RepartitionExec`,
`SortPreservingMergeExec`, `CoalescePartitionsExec`) and their ancestors.
Using this information, we can optimize distribution of the plan if/when
necessary.

---

## PlanWithKeyRequirements

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::PlanWithKeyRequirements`

Also reachable as `datafusion_physical_optimizer::enforce_distribution::PlanWithKeyRequirements`

```rust
type PlanWithKeyRequirements = datafusion_physical_plan::tree_node::PlanContext<Vec<std::sync::Arc<dyn PhysicalExpr>>>
```

Keeps track of parent required key orderings.

---
