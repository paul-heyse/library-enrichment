# `datafusion_expr::logical_plan::invariants::check_subquery_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.invariants.check_subquery_expr.json).

<a id="op-d286837ba517f6c0c5ec8826"></a>
## check_subquery_expr

`function` · `datafusion_expr::logical_plan::invariants::check_subquery_expr` · datafusion-expr 55.1.0

```rust
fn check_subquery_expr(outer_plan: &LogicalPlan, inner_plan: &LogicalPlan, expr: &Expr) -> datafusion_common::Result<()>
```

Source: `src/logical_plan/invariants.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Do necessary check on subquery expressions and fail the invalid plan
1) Check whether the outer plan is in the allowed outer plans list to use subquery expressions,
   the allowed while list: [Projection, Filter, Window, Aggregate, Join].
2) Check whether the inner plan is in the allowed inner plans list to use correlated(outer) expressions.
3) Check and validate unsupported cases to use the correlated(outer) expressions inside the subquery(inner) plans/inner expressions.
   For example, we do not want to support to use correlated expressions as the Join conditions in the subquery plan when the Join
   is a Full Out Join
