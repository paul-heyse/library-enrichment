# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::adjust_input_keys_ordering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_distribution.adjust_input_keys_ordering.json).

<a id="op-d3f9f7d3e3d4722bc4823300"></a>
## adjust_input_keys_ordering

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::adjust_input_keys_ordering` · datafusion-physical-optimizer 55.1.0

```rust
fn adjust_input_keys_ordering(requirements: PlanWithKeyRequirements) -> datafusion_common::error::Result<datafusion_common::tree_node::Transformed<PlanWithKeyRequirements>>
```

Source: `src/ensure_requirements/enforce_distribution.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

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
