# `datafusion_functions_nested::string::array_to_string`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.string.array_to_string.json).

<a id="op-ad01d1bccc31f3c0a9a4d94f"></a>
## array_to_string

`function` · `datafusion_functions_nested::string::array_to_string` · datafusion-functions-nested 55.1.0

```rust
fn array_to_string(array: datafusion_expr::Expr, delimiter: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/string.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

converts each element to its text representation.
