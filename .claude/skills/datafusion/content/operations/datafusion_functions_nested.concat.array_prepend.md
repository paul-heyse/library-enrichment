# `datafusion_functions_nested::concat::array_prepend`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.concat.array_prepend.json).

<a id="op-5ab216808c47a98ccfe40727"></a>
## array_prepend

`function` · `datafusion_functions_nested::concat::array_prepend` · datafusion-functions-nested 55.1.0

```rust
fn array_prepend(element: datafusion_expr::Expr, array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/concat.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

Prepends an element to the beginning of an array.
