# `datafusion_optimizer::simplify_expressions::simplify_predicates::simplify_predicates`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.simplify_expressions.simplify_predicates.simplify_predicates.json).

<a id="op-9eac77fdefde585c4aaa38fa"></a>
## simplify_predicates

`function` · `datafusion_optimizer::simplify_expressions::simplify_predicates::simplify_predicates` · datafusion-optimizer 55.1.0

```rust
fn simplify_predicates(predicates: Vec<datafusion_expr::Expr>) -> datafusion_common::Result<Vec<datafusion_expr::Expr>>
```

Source: `src/simplify_expressions/simplify_predicates.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Simplifies a list of predicates by removing redundancies.

This function takes a vector of predicate expressions and groups them by the column they reference.
Predicates that reference a single column and are comparison operations (e.g., >, >=, <, <=, =)
are analyzed to remove redundant conditions. For instance, `x > 5 AND x > 6` is simplified to
`x > 6`. Other predicates that do not fit this pattern are retained as-is.

# Arguments
* `predicates` - A vector of `Expr` representing the predicates to simplify.

# Returns
A `Result` containing a vector of simplified `Expr` predicates.
