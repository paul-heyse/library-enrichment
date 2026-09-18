# `datafusion_functions_nested::position::array_position`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.position.array_position.json).

<a id="op-812b76fb4364c2cd95bd4f96"></a>
## array_position

`function` · `datafusion_functions_nested::position::array_position` · datafusion-functions-nested 55.1.0

```rust
fn array_position(array: datafusion_expr::Expr, element: datafusion_expr::Expr, index: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/position.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

searches for an element in the array, returns first occurrence.
