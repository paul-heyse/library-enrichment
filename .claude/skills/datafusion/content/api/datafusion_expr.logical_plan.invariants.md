# `datafusion_expr::logical_plan::invariants`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.logical_plan.invariants.json`](../model/datafusion_expr.logical_plan.invariants.json)

## InvariantLevel

`enum` · `datafusion_expr::logical_plan::invariants::InvariantLevel`

Also reachable as `datafusion::logical_expr::InvariantLevel`, `datafusion_expr::InvariantLevel`, `datafusion_expr::logical_plan::InvariantLevel`

```rust
enum InvariantLevel
```

**Variants**: `Always`, `Executable`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

---

## assert_expected_schema

`function` · `datafusion_expr::logical_plan::invariants::assert_expected_schema`

Also reachable as `datafusion::logical_expr::assert_expected_schema`, `datafusion_expr::assert_expected_schema`, `datafusion_expr::logical_plan::assert_expected_schema`

```rust
fn assert_expected_schema(schema: &datafusion_common::DFSchemaRef, plan: &LogicalPlan) -> datafusion_common::Result<()>
```

Returns an error if the plan does not have the expected schema.
Ignores metadata and nullability.

---

## check_subquery_expr

`function` · `datafusion_expr::logical_plan::invariants::check_subquery_expr`

Also reachable as `datafusion::logical_expr::check_subquery_expr`, `datafusion_expr::check_subquery_expr`, `datafusion_expr::logical_plan::check_subquery_expr`

```rust
fn check_subquery_expr(outer_plan: &LogicalPlan, inner_plan: &LogicalPlan, expr: &Expr) -> datafusion_common::Result<()>
```

Do necessary check on subquery expressions and fail the invalid plan
1) Check whether the outer plan is in the allowed outer plans list to use subquery expressions,
   the allowed while list: [Projection, Filter, Window, Aggregate, Join].
2) Check whether the inner plan is in the allowed inner plans list to use correlated(outer) expressions.
3) Check and validate unsupported cases to use the correlated(outer) expressions inside the subquery(inner) plans/inner expressions.
   For example, we do not want to support to use correlated expressions as the Join conditions in the subquery plan when the Join
   is a Full Out Join

---
