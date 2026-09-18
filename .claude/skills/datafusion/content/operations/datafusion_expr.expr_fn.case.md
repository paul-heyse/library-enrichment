# `datafusion_expr::expr_fn::case`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.case.json).

<a id="op-66c70680faf6f45b1a397266"></a>
## case

`function` · `datafusion_expr::expr_fn::case` · datafusion-expr 55.1.0

```rust
fn case(expr: Expr) -> conditional_expressions::CaseBuilder
```

Source: `src/expr_fn.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a CASE WHEN statement with literal WHEN expressions for comparison to the base expression.
