# `datafusion_functions::core::expr_fn::least`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.least.json).

<a id="op-d9f0d044d88394c2b5ba41b0"></a>
## least

`function` · `datafusion_functions::core::expr_fn::least` · datafusion-functions 55.1.0

```rust
fn least(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns `least(args...)`, which evaluates to the smallest value in the list of expressions or NULL if all the expressions are NULL
