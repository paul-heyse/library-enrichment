# `datafusion_functions::math::expr_fn::iszero`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.expr_fn.iszero.json).

<a id="op-0772287d7c9433e1df8d07e6"></a>
## iszero

`function` · `datafusion_functions::math::expr_fn::iszero` · datafusion-functions 55.1.0

```rust
fn iszero(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/math/mod.rs:399`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

returns true if a given number is +0.0 or -0.0 otherwise returns false
