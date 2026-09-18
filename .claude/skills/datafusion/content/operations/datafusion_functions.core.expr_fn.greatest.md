# `datafusion_functions::core::expr_fn::greatest`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.greatest.json).

<a id="op-0f0f116072a82e0c7be3f7ee"></a>
## greatest

`function` · `datafusion_functions::core::expr_fn::greatest` · datafusion-functions 55.1.0

```rust
fn greatest(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns `greatest(args...)`, which evaluates to the greatest value in the list of expressions or NULL if all the expressions are NULL
