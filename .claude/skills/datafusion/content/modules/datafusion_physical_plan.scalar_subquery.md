# `datafusion_physical_plan::scalar_subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.scalar_subquery.json).

<a id="op-f4096db81096744af346c442"></a>
## scalar_subquery

`module` · `datafusion_physical_plan::scalar_subquery` · datafusion-physical-plan 55.1.0

```rust
mod scalar_subquery
```

Source: `src/scalar_subquery.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Execution plan for uncorrelated scalar subqueries.

[`ScalarSubqueryExec`](../operations/datafusion_physical_plan.scalar_subquery.ScalarSubqueryExec.md#op-a478f6274c9e53b0018ce559) wraps a main input plan and a set of subquery plans.
At execution time, it runs each subquery exactly once, extracts the scalar
result, and populates a shared [`ScalarSubqueryResults`](../operations/datafusion_expr.physical_planning_context.ScalarSubqueryResults.md#op-6b0683e0d87a9cc813cb37e5) container that
[`ScalarSubqueryExpr`] instances hold directly and read from by index.

[`ScalarSubqueryExpr`]: datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr
