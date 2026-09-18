# `datafusion_functions::math::expr_fn::nanvl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.expr_fn.nanvl.json).

<a id="op-d59fedfddca57a90adc83876"></a>
## nanvl

`function` · `datafusion_functions::math::expr_fn::nanvl` · datafusion-functions 55.1.0

```rust
fn nanvl(x: datafusion_expr::Expr, y: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/math/mod.rs:399`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

returns x if x is not NaN otherwise returns y
