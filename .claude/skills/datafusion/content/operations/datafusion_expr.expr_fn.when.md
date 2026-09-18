# `datafusion_expr::expr_fn::when`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.when.json).

<a id="op-ca28ea0227c2217d678faa14"></a>
## when

`function` · `datafusion_expr::expr_fn::when` · datafusion-expr 55.1.0

```rust
fn when(when: Expr, then: Expr) -> conditional_expressions::CaseBuilder
```

Source: `src/expr_fn.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a CASE WHEN statement with boolean WHEN expressions and no base expression.
