# `datafusion_functions_nested::remove::array_remove_n`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.remove.array_remove_n.json).

<a id="op-c0eee564561e4b071286e3e9"></a>
## array_remove_n

`function` · `datafusion_functions_nested::remove::array_remove_n` · datafusion-functions-nested 55.1.0

```rust
fn array_remove_n(array: datafusion_expr::Expr, element: datafusion_expr::Expr, max: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/remove.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

removes the first `max` elements from the array equal to the given value. NULL elements already in the array are preserved when removing a non-NULL value. If `element` evaluates to NULL, the result is NULL rather than removing NULL entries.
