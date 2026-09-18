# `datafusion_functions_nested::resize::array_resize`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.resize.array_resize.json).

<a id="op-5835e55bd7d2c541a9e7c203"></a>
## array_resize

`function` · `datafusion_functions_nested::resize::array_resize` · datafusion-functions-nested 55.1.0

```rust
fn array_resize(array: datafusion_expr::Expr, size: datafusion_expr::Expr, value: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/resize.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns an array with the specified size filled with the given value.
