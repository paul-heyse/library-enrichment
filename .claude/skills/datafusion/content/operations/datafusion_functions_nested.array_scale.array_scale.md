# `datafusion_functions_nested::array_scale::array_scale`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.array_scale.array_scale.json).

<a id="op-5ffcac12e1d2aa7c0793246d"></a>
## array_scale

`function` · `datafusion_functions_nested::array_scale::array_scale` · datafusion-functions-nested 55.1.0

```rust
fn array_scale(array: datafusion_expr::Expr, scalar: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/array_scale.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

scales each element of a numeric array by a scalar.
