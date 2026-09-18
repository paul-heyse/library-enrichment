# `datafusion_functions_nested::position::array_positions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.position.array_positions.json).

<a id="op-5a88bc49e7f987fa0f4b412d"></a>
## array_positions

`function` · `datafusion_functions_nested::position::array_positions` · datafusion-functions-nested 55.1.0

```rust
fn array_positions(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/position.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

searches for an element in the array, returns all occurrences.
