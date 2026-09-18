# `datafusion_physical_optimizer::limit_pushdown`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.limit_pushdown.json).

<a id="op-bdc2eb705a33e412e7328255"></a>
## limit_pushdown

`module` · `datafusion_physical_optimizer::limit_pushdown` · datafusion-physical-optimizer 55.1.0

```rust
mod limit_pushdown
```

Source: `src/limit_pushdown.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

[`LimitPushdown`](../operations/datafusion_physical_optimizer.limit_pushdown.LimitPushdown.md#op-5e4f8ca3ba0b51aa7a5956ca) pushes `LIMIT` down through `ExecutionPlan`s to reduce
data transfer as much as possible.

# Plan Limit Absorption
In addition to pushing down `GlobalLimitExec` and `LocalLimitExec` nodes in
the plan, some operators can "absorb" a limit and stop early during
execution.

## Background: vectorized volcano execution model
DataFusion uses a batched volcano model. For most operators, output is
produced in batches of `datafusion.execution.batch_size` (default 8192), so
the batch sizes typically look like:
```text
8192, 8192, ..., 8192, 100 (the final batch may be partial)
```

## Example
For a join with an expensive, selective predicate:
```text
GlobalLimitExec: skip=0, fetch=10
-- NestedLoopJoinExec(on=expr_expensive_and_selective)
--- DataSourceExec()
--- DataSourceExec()
```

Under this model, `NestedLoopJoinExec` would keep working until it can emit
a full batch (8192 rows), even though the query only needs 10. If the limit
cannot be pushed below the join, we can still embed it inside the join so it
stops once the limit is satisfied. The transformed plan looks like:

```text
NestedLoopJoinExec(on=expr_expensive_and_selective, fetch=10)
--- DataSourceExec()
--- DataSourceExec()
```

## Implementation
The current optimizer rule optionally pushes `fetch` requirements into
operators via [`ExecutionPlan::with_fetch`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-9cea7d2e63946ce72cec4604).

To support early termination in operators, [`LimitedBatchCoalescer`](https://docs.rs/datafusion/latest/datafusion/physical_plan/coalesce/struct.LimitedBatchCoalescer.html)
can help manage the output buffer.

Reference implementation in Hash Join: <https://github.com/apache/datafusion/pull/20228>
