# `datafusion_functions_nested::array_has::array_has`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.array_has.array_has.json).

<a id="op-1bf96dcdb0b8c2bff56246b6"></a>
## array_has

`function` · `datafusion_functions_nested::array_has::array_has` · datafusion-functions-nested 55.1.0

```rust
fn array_has(haystack_array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/array_has.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns true, if the element appears in the first array, otherwise false.
