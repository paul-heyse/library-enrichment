# `datafusion_optimizer::simplify_expressions::simplify_predicates`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.simplify_expressions.simplify_predicates.json`](../model/datafusion_optimizer.simplify_expressions.simplify_predicates.json)

## simplify_predicates

`function` · `datafusion_optimizer::simplify_expressions::simplify_predicates::simplify_predicates`

Also reachable as `datafusion_optimizer::simplify_expressions::simplify_predicates`

```rust
fn simplify_predicates(predicates: Vec<datafusion_expr::Expr>) -> datafusion_common::Result<Vec<datafusion_expr::Expr>>
```

Simplifies a list of predicates by removing redundancies.

This function takes a vector of predicate expressions and groups them by the column they reference.
Predicates that reference a single column and are comparison operations (e.g., >, >=, <, <=, =)
are analyzed to remove redundant conditions. For instance, `x > 5 AND x > 6` is simplified to
`x > 6`. Other predicates that do not fit this pattern are retained as-is.

# Arguments
* `predicates` - A vector of `Expr` representing the predicates to simplify.

# Returns
A `Result` containing a vector of simplified `Expr` predicates.

---
