# `datafusion_functions_nested::set_ops::array_union`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.set_ops.array_union.json).

<a id="op-d00737e8d298d3cbab913237"></a>
## array_union

`function` · `datafusion_functions_nested::set_ops::array_union` · datafusion-functions-nested 55.1.0

```rust
fn array_union(array1: datafusion_expr::Expr, array2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/set_ops.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns an array of the elements in the union of array1 and array2 without duplicates.
