# `datafusion_functions::core::expr_fn::cast_to_type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.cast_to_type.json).

<a id="op-574e2ff95ad36b8f64a7830a"></a>
## cast_to_type

`function` · `datafusion_functions::core::expr_fn::cast_to_type` · datafusion-functions 55.1.0

```rust
fn cast_to_type(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Casts the first argument to the data type of the second argument
