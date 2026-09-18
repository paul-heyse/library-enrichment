# `datafusion_functions_nested::remove::array_remove_all`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.remove.array_remove_all.json).

<a id="op-8e2c229c874721cde7c2d0f0"></a>
## array_remove_all

`function` · `datafusion_functions_nested::remove::array_remove_all` · datafusion-functions-nested 55.1.0

```rust
fn array_remove_all(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/remove.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

removes all elements from the array equal to the given value. NULL elements already in the array are preserved when removing a non-NULL value. If `element` evaluates to NULL, the result is NULL rather than removing NULL entries.
