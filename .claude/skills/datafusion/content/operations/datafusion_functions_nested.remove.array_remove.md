# `datafusion_functions_nested::remove::array_remove`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.remove.array_remove.json).

<a id="op-88dc6b9d6f2d50a3457477b7"></a>
## array_remove

`function` · `datafusion_functions_nested::remove::array_remove` · datafusion-functions-nested 55.1.0

```rust
fn array_remove(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/remove.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

removes the first element from the array equal to the given value. NULL elements already in the array are preserved when removing a non-NULL value. If `element` evaluates to NULL, the result is NULL rather than removing NULL entries.
