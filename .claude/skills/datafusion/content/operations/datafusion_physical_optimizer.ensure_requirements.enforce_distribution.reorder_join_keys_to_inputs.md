# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::reorder_join_keys_to_inputs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_distribution.reorder_join_keys_to_inputs.json).

<a id="op-b225f95d14e0b7b3b4855aee"></a>
## reorder_join_keys_to_inputs

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::reorder_join_keys_to_inputs` · datafusion-physical-optimizer 55.1.0

```rust
fn reorder_join_keys_to_inputs(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/ensure_requirements/enforce_distribution.rs:450`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

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
