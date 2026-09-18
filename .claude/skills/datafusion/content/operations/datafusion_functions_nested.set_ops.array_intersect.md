# `datafusion_functions_nested::set_ops::array_intersect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.set_ops.array_intersect.json).

<a id="op-016b123aaa8e21a5bce24b05"></a>
## array_intersect

`function` · `datafusion_functions_nested::set_ops::array_intersect` · datafusion-functions-nested 55.1.0

```rust
fn array_intersect(first_array: datafusion_expr::Expr, second_array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/set_ops.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns an array of the elements in the intersection of array1 and array2.
