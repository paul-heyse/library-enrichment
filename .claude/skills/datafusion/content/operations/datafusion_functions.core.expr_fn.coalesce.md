# `datafusion_functions::core::expr_fn::coalesce`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.coalesce.json).

<a id="op-939549e8bea4a39d41107f07"></a>
## coalesce

`function` · `datafusion_functions::core::expr_fn::coalesce` · datafusion-functions 55.1.0

```rust
fn coalesce(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns `coalesce(args...)`, which evaluates to the value of the first expr which is not NULL
