# `datafusion_functions_nested::array_filter::array_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.array_filter.array_filter.json).

<a id="op-c84cc33e633c770734fc0b56"></a>
## array_filter

`function` · `datafusion_functions_nested::array_filter::array_filter` · datafusion-functions-nested 55.1.0

```rust
fn array_filter(array: datafusion_expr::Expr, lambda: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/array_filter.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

filters the values of an array using a boolean lambda
