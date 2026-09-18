# `datafusion_functions::math::expr_fn::isnan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.expr_fn.isnan.json).

<a id="op-17d8d9f62cf2e35a0ef7c043"></a>
## isnan

`function` · `datafusion_functions::math::expr_fn::isnan` · datafusion-functions 55.1.0

```rust
fn isnan(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/math/mod.rs:399`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

returns true if a given number is +NaN or -NaN otherwise returns false
