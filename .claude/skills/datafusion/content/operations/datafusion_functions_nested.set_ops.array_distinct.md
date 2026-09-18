# `datafusion_functions_nested::set_ops::array_distinct`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.set_ops.array_distinct.json).

<a id="op-f47fb1588b2ee73c8d202a84"></a>
## array_distinct

`function` · `datafusion_functions_nested::set_ops::array_distinct` · datafusion-functions-nested 55.1.0

```rust
fn array_distinct(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/set_ops.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns distinct values from the array after removing duplicates.
