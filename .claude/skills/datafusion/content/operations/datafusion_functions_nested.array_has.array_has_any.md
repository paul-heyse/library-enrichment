# `datafusion_functions_nested::array_has::array_has_any`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.array_has.array_has_any.json).

<a id="op-e59f669d20980524de08136f"></a>
## array_has_any

`function` · `datafusion_functions_nested::array_has::array_has_any` · datafusion-functions-nested 55.1.0

```rust
fn array_has_any(first_array: datafusion_expr::Expr, second_array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/array_has.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns true if at least one element of the second array appears in the first array; otherwise, it returns false.
